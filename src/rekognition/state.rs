use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::RekognitionError;
use super::types::*;

#[allow(dead_code)]
struct RekognitionStateInner {
    collections: HashMap<String, StoredCollection>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredCollection {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct RekognitionState {
    inner: Arc<Mutex<RekognitionStateInner>>,
}

impl RekognitionState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        RekognitionState {
            inner: Arc::new(Mutex::new(RekognitionStateInner {
                collections: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_collection(&self, req: CreateCollectionRequest) -> Result<CreateCollectionResponse, RekognitionError> {
        let mut state = self.inner.lock().await;
        let name = req.collection_name.clone();
        if state.collections.contains_key(&name) {
            return Err(RekognitionError::ResourceAlreadyExistsException(format!("Collection {} already exists", name)));
        }
        let arn = format!("arn:aws:rekognition:{}:{}:collections/{}", state.region, state.account_id, name);
        state.collections.insert(name.clone(), StoredCollection {
            name: name.clone(),
            arn: arn.clone(),
            tags: req.tags.unwrap_or_default(),
        });
        Ok(CreateCollectionResponse {
            collection_arn: Some(arn),
            collection_name: Some(name),
        })
    }

    #[allow(dead_code)]
    pub async fn describe_collection(&self, req: DescribeCollectionRequest) -> Result<DescribeCollectionResponse, RekognitionError> {
        let state = self.inner.lock().await;
        let name = req.collection_name.or_else(|| req.collection_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| RekognitionError::ValidationException("Name or ARN required".to_string()))?;
        let stored = state.collections.get(&name)
            .ok_or_else(|| RekognitionError::ResourceNotFoundException(format!("Collection {} not found", name)))?;
        Ok(DescribeCollectionResponse {
            collection: CollectionDetail {
                collection_name: stored.name.clone(),
                collection_arn: stored.arn.clone(),
                status: "ACTIVE".to_string(),
                tags: Some(stored.tags.clone()),
            },
        })
    }

    #[allow(dead_code)]
    pub async fn list_collections(&self, _req: ListCollectionsRequest) -> Result<ListCollectionsResponse, RekognitionError> {
        let state = self.inner.lock().await;
        let items: Vec<CollectionDetail> = state.collections.values().map(|s| CollectionDetail {
            collection_name: s.name.clone(),
            collection_arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListCollectionsResponse {
            collections: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_collection(&self, req: DeleteCollectionRequest) -> Result<(), RekognitionError> {
        let mut state = self.inner.lock().await;
        let name = req.collection_name.or_else(|| req.collection_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| RekognitionError::ValidationException("Name or ARN required".to_string()))?;
        state.collections.remove(&name)
            .ok_or_else(|| RekognitionError::ResourceNotFoundException(format!("Collection {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = RekognitionState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_collection() {
        let state = RekognitionState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateCollectionRequest::default();
        let result = state.create_collection(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_describe_collection_not_found() {
        let state = RekognitionState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DescribeCollectionRequest::default();
        let result = state.describe_collection(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_collections_empty() {
        let state = RekognitionState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = ListCollectionsRequest::default();
        let result = state.list_collections(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_collection_not_found() {
        let state = RekognitionState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DeleteCollectionRequest::default();
        let result = state.delete_collection(req).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_collection_create_and_list() {
        let state = RekognitionState::new("123456789012".to_string(), "us-east-1".to_string());
        let create_req = CreateCollectionRequest::default();
        let _created = state.create_collection(create_req).await.unwrap();
        let list_req = ListCollectionsRequest::default();
        let listed = state.list_collections(list_req).await.unwrap();
        let _ = listed;
    }

    #[tokio::test]
    async fn test_collection_full_crud() {
        let state = RekognitionState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateCollectionRequest::default();
        create_req.collection_name = "test-crud-resource".to_string();
        let create_result = state.create_collection(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let mut get_req = DescribeCollectionRequest::default();
        get_req.collection_name = Some("test-crud-resource".to_string());
        let get_result = state.describe_collection(get_req).await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let mut del_req = DeleteCollectionRequest::default();
        del_req.collection_name = Some("test-crud-resource".to_string());
        let del_result = state.delete_collection(del_req).await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
