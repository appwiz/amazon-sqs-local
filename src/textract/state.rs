use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::TextractError;
use super::types::*;

#[allow(dead_code)]
struct TextractStateInner {
    adapters: HashMap<String, StoredAdapter>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredAdapter {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct TextractState {
    inner: Arc<Mutex<TextractStateInner>>,
}

impl TextractState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        TextractState {
            inner: Arc::new(Mutex::new(TextractStateInner {
                adapters: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_adapter(&self, req: CreateAdapterRequest) -> Result<CreateAdapterResponse, TextractError> {
        let mut state = self.inner.lock().await;
        let name = req.adapter_name.clone();
        if state.adapters.contains_key(&name) {
            return Err(TextractError::ResourceAlreadyExistsException(format!("Adapter {} already exists", name)));
        }
        let arn = format!("arn:aws:textract:{}:{}:adapters/{}", state.region, state.account_id, name);
        state.adapters.insert(name.clone(), StoredAdapter {
            name: name.clone(),
            arn: arn.clone(),
            tags: req.tags.unwrap_or_default(),
        });
        Ok(CreateAdapterResponse {
            adapter_arn: Some(arn),
            adapter_name: Some(name),
        })
    }

    #[allow(dead_code)]
    pub async fn describe_adapter(&self, req: DescribeAdapterRequest) -> Result<DescribeAdapterResponse, TextractError> {
        let state = self.inner.lock().await;
        let name = req.adapter_name.or_else(|| req.adapter_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| TextractError::ValidationException("Name or ARN required".to_string()))?;
        let stored = state.adapters.get(&name)
            .ok_or_else(|| TextractError::ResourceNotFoundException(format!("Adapter {} not found", name)))?;
        Ok(DescribeAdapterResponse {
            adapter: AdapterDetail {
                adapter_name: stored.name.clone(),
                adapter_arn: stored.arn.clone(),
                status: "ACTIVE".to_string(),
                tags: Some(stored.tags.clone()),
            },
        })
    }

    #[allow(dead_code)]
    pub async fn list_adapters(&self, _req: ListAdaptersRequest) -> Result<ListAdaptersResponse, TextractError> {
        let state = self.inner.lock().await;
        let items: Vec<AdapterDetail> = state.adapters.values().map(|s| AdapterDetail {
            adapter_name: s.name.clone(),
            adapter_arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListAdaptersResponse {
            adapters: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_adapter(&self, req: DeleteAdapterRequest) -> Result<(), TextractError> {
        let mut state = self.inner.lock().await;
        let name = req.adapter_name.or_else(|| req.adapter_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| TextractError::ValidationException("Name or ARN required".to_string()))?;
        state.adapters.remove(&name)
            .ok_or_else(|| TextractError::ResourceNotFoundException(format!("Adapter {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = TextractState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_adapter() {
        let state = TextractState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateAdapterRequest::default();
        let result = state.create_adapter(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_describe_adapter_not_found() {
        let state = TextractState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DescribeAdapterRequest::default();
        let result = state.describe_adapter(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_adapters_empty() {
        let state = TextractState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = ListAdaptersRequest::default();
        let result = state.list_adapters(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_adapter_not_found() {
        let state = TextractState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DeleteAdapterRequest::default();
        let result = state.delete_adapter(req).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_adapter_create_and_list() {
        let state = TextractState::new("123456789012".to_string(), "us-east-1".to_string());
        let create_req = CreateAdapterRequest::default();
        let _created = state.create_adapter(create_req).await.unwrap();
        let list_req = ListAdaptersRequest::default();
        let listed = state.list_adapters(list_req).await.unwrap();
        let _ = listed;
    }

    #[tokio::test]
    async fn test_adapter_full_crud() {
        let state = TextractState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateAdapterRequest::default();
        create_req.adapter_name = "test-crud-resource".to_string();
        let create_result = state.create_adapter(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let mut get_req = DescribeAdapterRequest::default();
        get_req.adapter_name = Some("test-crud-resource".to_string());
        let get_result = state.describe_adapter(get_req).await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let mut del_req = DeleteAdapterRequest::default();
        del_req.adapter_name = Some("test-crud-resource".to_string());
        let del_result = state.delete_adapter(del_req).await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
