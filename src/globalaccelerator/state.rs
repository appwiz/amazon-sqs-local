use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::GlobalacceleratorError;
use super::types::*;

#[allow(dead_code)]
struct GlobalacceleratorStateInner {
    accelerators: HashMap<String, StoredAccelerator>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredAccelerator {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct GlobalacceleratorState {
    inner: Arc<Mutex<GlobalacceleratorStateInner>>,
}

impl GlobalacceleratorState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        GlobalacceleratorState {
            inner: Arc::new(Mutex::new(GlobalacceleratorStateInner {
                accelerators: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_accelerator(&self, req: CreateAcceleratorRequest) -> Result<CreateAcceleratorResponse, GlobalacceleratorError> {
        let mut state = self.inner.lock().await;
        let name = req.accelerator_name.clone();
        if state.accelerators.contains_key(&name) {
            return Err(GlobalacceleratorError::ResourceAlreadyExistsException(format!("Accelerator {} already exists", name)));
        }
        let arn = format!("arn:aws:globalaccelerator:{}:{}:accelerators/{}", state.region, state.account_id, name);
        state.accelerators.insert(name.clone(), StoredAccelerator {
            name: name.clone(),
            arn: arn.clone(),
            tags: req.tags.unwrap_or_default(),
        });
        Ok(CreateAcceleratorResponse {
            accelerator_arn: Some(arn),
            accelerator_name: Some(name),
        })
    }

    #[allow(dead_code)]
    pub async fn describe_accelerator(&self, req: DescribeAcceleratorRequest) -> Result<DescribeAcceleratorResponse, GlobalacceleratorError> {
        let state = self.inner.lock().await;
        let name = req.accelerator_name.or_else(|| req.accelerator_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| GlobalacceleratorError::ValidationException("Name or ARN required".to_string()))?;
        let stored = state.accelerators.get(&name)
            .ok_or_else(|| GlobalacceleratorError::ResourceNotFoundException(format!("Accelerator {} not found", name)))?;
        Ok(DescribeAcceleratorResponse {
            accelerator: AcceleratorDetail {
                accelerator_name: stored.name.clone(),
                accelerator_arn: stored.arn.clone(),
                status: "ACTIVE".to_string(),
                tags: Some(stored.tags.clone()),
            },
        })
    }

    #[allow(dead_code)]
    pub async fn list_accelerators(&self, _req: ListAcceleratorsRequest) -> Result<ListAcceleratorsResponse, GlobalacceleratorError> {
        let state = self.inner.lock().await;
        let items: Vec<AcceleratorDetail> = state.accelerators.values().map(|s| AcceleratorDetail {
            accelerator_name: s.name.clone(),
            accelerator_arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListAcceleratorsResponse {
            accelerators: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_accelerator(&self, req: DeleteAcceleratorRequest) -> Result<(), GlobalacceleratorError> {
        let mut state = self.inner.lock().await;
        let name = req.accelerator_name.or_else(|| req.accelerator_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| GlobalacceleratorError::ValidationException("Name or ARN required".to_string()))?;
        state.accelerators.remove(&name)
            .ok_or_else(|| GlobalacceleratorError::ResourceNotFoundException(format!("Accelerator {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = GlobalacceleratorState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_accelerator() {
        let state = GlobalacceleratorState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateAcceleratorRequest::default();
        let result = state.create_accelerator(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_describe_accelerator_not_found() {
        let state = GlobalacceleratorState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DescribeAcceleratorRequest::default();
        let result = state.describe_accelerator(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_accelerators_empty() {
        let state = GlobalacceleratorState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = ListAcceleratorsRequest::default();
        let result = state.list_accelerators(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_accelerator_not_found() {
        let state = GlobalacceleratorState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DeleteAcceleratorRequest::default();
        let result = state.delete_accelerator(req).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_accelerator_create_and_list() {
        let state = GlobalacceleratorState::new("123456789012".to_string(), "us-east-1".to_string());
        let create_req = CreateAcceleratorRequest::default();
        let _created = state.create_accelerator(create_req).await.unwrap();
        let list_req = ListAcceleratorsRequest::default();
        let listed = state.list_accelerators(list_req).await.unwrap();
        let _ = listed;
    }

    #[tokio::test]
    async fn test_accelerator_full_crud() {
        let state = GlobalacceleratorState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateAcceleratorRequest::default();
        create_req.accelerator_name = "test-crud-resource".to_string();
        let create_result = state.create_accelerator(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let mut get_req = DescribeAcceleratorRequest::default();
        get_req.accelerator_name = Some("test-crud-resource".to_string());
        let get_result = state.describe_accelerator(get_req).await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let mut del_req = DeleteAcceleratorRequest::default();
        del_req.accelerator_name = Some("test-crud-resource".to_string());
        let del_result = state.delete_accelerator(del_req).await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
