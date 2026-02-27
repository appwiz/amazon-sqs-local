use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::ConnectError;
use super::types::*;

#[allow(dead_code)]
struct ConnectStateInner {
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
pub struct ConnectState {
    inner: Arc<Mutex<ConnectStateInner>>,
}

impl ConnectState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        ConnectState {
            inner: Arc::new(Mutex::new(ConnectStateInner {
                instances: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_instance(&self, req: CreateInstanceRequest) -> Result<InstanceDetail, ConnectError> {
        let mut state = self.inner.lock().await;
        let name = req.name.or(req.name_pascal).unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        if state.instances.contains_key(&name) {
            return Err(ConnectError::ResourceAlreadyExistsException(format!("Instance {} already exists", name)));
        }
        let arn = format!("arn:aws:connect:{}:{}:instances/{}", state.region, state.account_id, name);
        let detail = InstanceDetail {
            name: name.clone(),
            arn: arn.clone(),
            status: "ACTIVE".to_string(),
            tags: req.tags.clone(),
        };
        state.instances.insert(name, StoredInstance {
            name: detail.name.clone(),
            arn,
            tags: req.tags.unwrap_or_default(),
        });
        Ok(detail)
    }

    #[allow(dead_code)]
    pub async fn get_instance(&self, name: &str) -> Result<InstanceDetail, ConnectError> {
        let state = self.inner.lock().await;
        let stored = state.instances.get(name)
            .ok_or_else(|| ConnectError::ResourceNotFoundException(format!("Instance {} not found", name)))?;
        Ok(InstanceDetail {
            name: stored.name.clone(),
            arn: stored.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(stored.tags.clone()),
        })
    }

    #[allow(dead_code)]
    pub async fn list_instances(&self) -> Result<ListInstancesResponse, ConnectError> {
        let state = self.inner.lock().await;
        let items: Vec<InstanceDetail> = state.instances.values().map(|s| InstanceDetail {
            name: s.name.clone(),
            arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListInstancesResponse {
            items: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_instance(&self, name: &str) -> Result<(), ConnectError> {
        let mut state = self.inner.lock().await;
        state.instances.remove(name)
            .ok_or_else(|| ConnectError::ResourceNotFoundException(format!("Instance {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = ConnectState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_instance() {
        let state = ConnectState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateInstanceRequest::default();
        let result = state.create_instance(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_get_instance_not_found() {
        let state = ConnectState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.get_instance("nonexistent").await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_instances() {
        let state = ConnectState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.list_instances().await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_instance_not_found() {
        let state = ConnectState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.delete_instance("nonexistent").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_instance_full_crud() {
        let state = ConnectState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateInstanceRequest::default();
        create_req.name = Some("test-crud-resource".to_string());
        let create_result = state.create_instance(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let get_result = state.get_instance("test-crud-resource").await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let del_result = state.delete_instance("test-crud-resource").await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
