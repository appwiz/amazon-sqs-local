use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::LakeformationError;
use super::types::*;

#[allow(dead_code)]
struct LakeformationStateInner {
    resources: HashMap<String, StoredResource>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredResource {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct LakeformationState {
    inner: Arc<Mutex<LakeformationStateInner>>,
}

impl LakeformationState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        LakeformationState {
            inner: Arc::new(Mutex::new(LakeformationStateInner {
                resources: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_resource(&self, req: CreateResourceRequest) -> Result<CreateResourceResponse, LakeformationError> {
        let mut state = self.inner.lock().await;
        let name = req.resource_name.clone();
        if state.resources.contains_key(&name) {
            return Err(LakeformationError::ResourceAlreadyExistsException(format!("Resource {} already exists", name)));
        }
        let arn = format!("arn:aws:lakeformation:{}:{}:resources/{}", state.region, state.account_id, name);
        state.resources.insert(name.clone(), StoredResource {
            name: name.clone(),
            arn: arn.clone(),
            tags: req.tags.unwrap_or_default(),
        });
        Ok(CreateResourceResponse {
            resource_arn: Some(arn),
            resource_name: Some(name),
        })
    }

    #[allow(dead_code)]
    pub async fn describe_resource(&self, req: DescribeResourceRequest) -> Result<DescribeResourceResponse, LakeformationError> {
        let state = self.inner.lock().await;
        let name = req.resource_name.or_else(|| req.resource_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| LakeformationError::ValidationException("Name or ARN required".to_string()))?;
        let stored = state.resources.get(&name)
            .ok_or_else(|| LakeformationError::ResourceNotFoundException(format!("Resource {} not found", name)))?;
        Ok(DescribeResourceResponse {
            resource: ResourceDetail {
                resource_name: stored.name.clone(),
                resource_arn: stored.arn.clone(),
                status: "ACTIVE".to_string(),
                tags: Some(stored.tags.clone()),
            },
        })
    }

    #[allow(dead_code)]
    pub async fn list_resources(&self, _req: ListResourcesRequest) -> Result<ListResourcesResponse, LakeformationError> {
        let state = self.inner.lock().await;
        let items: Vec<ResourceDetail> = state.resources.values().map(|s| ResourceDetail {
            resource_name: s.name.clone(),
            resource_arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListResourcesResponse {
            resources: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_resource(&self, req: DeleteResourceRequest) -> Result<(), LakeformationError> {
        let mut state = self.inner.lock().await;
        let name = req.resource_name.or_else(|| req.resource_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| LakeformationError::ValidationException("Name or ARN required".to_string()))?;
        state.resources.remove(&name)
            .ok_or_else(|| LakeformationError::ResourceNotFoundException(format!("Resource {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = LakeformationState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_resource() {
        let state = LakeformationState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateResourceRequest::default();
        let result = state.create_resource(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_describe_resource_not_found() {
        let state = LakeformationState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DescribeResourceRequest::default();
        let result = state.describe_resource(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_resources_empty() {
        let state = LakeformationState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = ListResourcesRequest::default();
        let result = state.list_resources(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_resource_not_found() {
        let state = LakeformationState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DeleteResourceRequest::default();
        let result = state.delete_resource(req).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_resource_create_and_list() {
        let state = LakeformationState::new("123456789012".to_string(), "us-east-1".to_string());
        let create_req = CreateResourceRequest::default();
        let _created = state.create_resource(create_req).await.unwrap();
        let list_req = ListResourcesRequest::default();
        let listed = state.list_resources(list_req).await.unwrap();
        let _ = listed;
    }

    #[tokio::test]
    async fn test_resource_full_crud() {
        let state = LakeformationState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateResourceRequest::default();
        create_req.resource_name = "test-crud-resource".to_string();
        let create_result = state.create_resource(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let mut get_req = DescribeResourceRequest::default();
        get_req.resource_name = Some("test-crud-resource".to_string());
        let get_result = state.describe_resource(get_req).await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let mut del_req = DeleteResourceRequest::default();
        del_req.resource_name = Some("test-crud-resource".to_string());
        let del_result = state.delete_resource(del_req).await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
