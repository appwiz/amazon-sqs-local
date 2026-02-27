use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::LightsailError;
use super::types::*;

#[allow(dead_code)]
struct LightsailStateInner {
    instances: HashMap<String, StoredInstance>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredInstance {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct LightsailState {
    inner: Arc<Mutex<LightsailStateInner>>,
}

impl LightsailState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        LightsailState {
            inner: Arc::new(Mutex::new(LightsailStateInner {
                instances: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_instance(&self, req: CreateInstanceRequest) -> Result<CreateInstanceResponse, LightsailError> {
        let mut state = self.inner.lock().await;
        let name = req.instance_name.clone();
        if state.instances.contains_key(&name) {
            return Err(LightsailError::ResourceAlreadyExistsException(format!("Instance {} already exists", name)));
        }
        let arn = format!("arn:aws:lightsail:{}:{}:instances/{}", state.region, state.account_id, name);
        state.instances.insert(name.clone(), StoredInstance {
            name: name.clone(),
            arn: arn.clone(),
            tags: req.tags.unwrap_or_default(),
        });
        Ok(CreateInstanceResponse {
            instance_arn: Some(arn),
            instance_name: Some(name),
        })
    }

    #[allow(dead_code)]
    pub async fn describe_instance(&self, req: DescribeInstanceRequest) -> Result<DescribeInstanceResponse, LightsailError> {
        let state = self.inner.lock().await;
        let name = req.instance_name.or_else(|| req.instance_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| LightsailError::ValidationException("Name or ARN required".to_string()))?;
        let stored = state.instances.get(&name)
            .ok_or_else(|| LightsailError::ResourceNotFoundException(format!("Instance {} not found", name)))?;
        Ok(DescribeInstanceResponse {
            instance: InstanceDetail {
                instance_name: stored.name.clone(),
                instance_arn: stored.arn.clone(),
                status: "ACTIVE".to_string(),
                tags: Some(stored.tags.clone()),
            },
        })
    }

    #[allow(dead_code)]
    pub async fn list_instances(&self, _req: ListInstancesRequest) -> Result<ListInstancesResponse, LightsailError> {
        let state = self.inner.lock().await;
        let items: Vec<InstanceDetail> = state.instances.values().map(|s| InstanceDetail {
            instance_name: s.name.clone(),
            instance_arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListInstancesResponse {
            instances: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_instance(&self, req: DeleteInstanceRequest) -> Result<(), LightsailError> {
        let mut state = self.inner.lock().await;
        let name = req.instance_name.or_else(|| req.instance_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| LightsailError::ValidationException("Name or ARN required".to_string()))?;
        state.instances.remove(&name)
            .ok_or_else(|| LightsailError::ResourceNotFoundException(format!("Instance {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = LightsailState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_instance() {
        let state = LightsailState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateInstanceRequest::default();
        let result = state.create_instance(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_describe_instance_not_found() {
        let state = LightsailState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DescribeInstanceRequest::default();
        let result = state.describe_instance(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_instances_empty() {
        let state = LightsailState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = ListInstancesRequest::default();
        let result = state.list_instances(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_instance_not_found() {
        let state = LightsailState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DeleteInstanceRequest::default();
        let result = state.delete_instance(req).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_instance_create_and_list() {
        let state = LightsailState::new("123456789012".to_string(), "us-east-1".to_string());
        let create_req = CreateInstanceRequest::default();
        let _created = state.create_instance(create_req).await.unwrap();
        let list_req = ListInstancesRequest::default();
        let listed = state.list_instances(list_req).await.unwrap();
        let _ = listed;
    }

    #[tokio::test]
    async fn test_instance_full_crud() {
        let state = LightsailState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateInstanceRequest::default();
        create_req.instance_name = "test-crud-resource".to_string();
        let create_result = state.create_instance(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let mut get_req = DescribeInstanceRequest::default();
        get_req.instance_name = Some("test-crud-resource".to_string());
        let get_result = state.describe_instance(get_req).await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let mut del_req = DeleteInstanceRequest::default();
        del_req.instance_name = Some("test-crud-resource".to_string());
        let del_result = state.delete_instance(del_req).await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
