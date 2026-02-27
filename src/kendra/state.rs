use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::KendraError;
use super::types::*;

#[allow(dead_code)]
struct KendraStateInner {
    indexs: HashMap<String, StoredIndex>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredIndex {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct KendraState {
    inner: Arc<Mutex<KendraStateInner>>,
}

impl KendraState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        KendraState {
            inner: Arc::new(Mutex::new(KendraStateInner {
                indexs: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_index(&self, req: CreateIndexRequest) -> Result<CreateIndexResponse, KendraError> {
        let mut state = self.inner.lock().await;
        let name = req.index_name.clone();
        if state.indexs.contains_key(&name) {
            return Err(KendraError::ResourceAlreadyExistsException(format!("Index {} already exists", name)));
        }
        let arn = format!("arn:aws:kendra:{}:{}:indices/{}", state.region, state.account_id, name);
        state.indexs.insert(name.clone(), StoredIndex {
            name: name.clone(),
            arn: arn.clone(),
            tags: req.tags.unwrap_or_default(),
        });
        Ok(CreateIndexResponse {
            index_arn: Some(arn),
            index_name: Some(name),
        })
    }

    #[allow(dead_code)]
    pub async fn describe_index(&self, req: DescribeIndexRequest) -> Result<DescribeIndexResponse, KendraError> {
        let state = self.inner.lock().await;
        let name = req.index_name.or_else(|| req.index_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| KendraError::ValidationException("Name or ARN required".to_string()))?;
        let stored = state.indexs.get(&name)
            .ok_or_else(|| KendraError::ResourceNotFoundException(format!("Index {} not found", name)))?;
        Ok(DescribeIndexResponse {
            index: IndexDetail {
                index_name: stored.name.clone(),
                index_arn: stored.arn.clone(),
                status: "ACTIVE".to_string(),
                tags: Some(stored.tags.clone()),
            },
        })
    }

    #[allow(dead_code)]
    pub async fn list_indexs(&self, _req: ListIndexsRequest) -> Result<ListIndexsResponse, KendraError> {
        let state = self.inner.lock().await;
        let items: Vec<IndexDetail> = state.indexs.values().map(|s| IndexDetail {
            index_name: s.name.clone(),
            index_arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListIndexsResponse {
            indexs: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_index(&self, req: DeleteIndexRequest) -> Result<(), KendraError> {
        let mut state = self.inner.lock().await;
        let name = req.index_name.or_else(|| req.index_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| KendraError::ValidationException("Name or ARN required".to_string()))?;
        state.indexs.remove(&name)
            .ok_or_else(|| KendraError::ResourceNotFoundException(format!("Index {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = KendraState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_index() {
        let state = KendraState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateIndexRequest::default();
        let result = state.create_index(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_describe_index_not_found() {
        let state = KendraState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DescribeIndexRequest::default();
        let result = state.describe_index(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_indexs_empty() {
        let state = KendraState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = ListIndexsRequest::default();
        let result = state.list_indexs(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_index_not_found() {
        let state = KendraState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DeleteIndexRequest::default();
        let result = state.delete_index(req).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_index_create_and_list() {
        let state = KendraState::new("123456789012".to_string(), "us-east-1".to_string());
        let create_req = CreateIndexRequest::default();
        let _created = state.create_index(create_req).await.unwrap();
        let list_req = ListIndexsRequest::default();
        let listed = state.list_indexs(list_req).await.unwrap();
        let _ = listed;
    }

    #[tokio::test]
    async fn test_index_full_crud() {
        let state = KendraState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateIndexRequest::default();
        create_req.index_name = "test-crud-resource".to_string();
        let create_result = state.create_index(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let mut get_req = DescribeIndexRequest::default();
        get_req.index_name = Some("test-crud-resource".to_string());
        let get_result = state.describe_index(get_req).await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let mut del_req = DeleteIndexRequest::default();
        del_req.index_name = Some("test-crud-resource".to_string());
        let del_result = state.delete_index(del_req).await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
