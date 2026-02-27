use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::IotgreengrassError;
use super::types::*;

#[allow(dead_code)]
struct IotgreengrassStateInner {
    components: HashMap<String, StoredComponent>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredComponent {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct IotgreengrassState {
    inner: Arc<Mutex<IotgreengrassStateInner>>,
}

impl IotgreengrassState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        IotgreengrassState {
            inner: Arc::new(Mutex::new(IotgreengrassStateInner {
                components: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_component(&self, req: CreateComponentRequest) -> Result<ComponentDetail, IotgreengrassError> {
        let mut state = self.inner.lock().await;
        let name = req.name.or(req.name_pascal).unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        if state.components.contains_key(&name) {
            return Err(IotgreengrassError::ResourceAlreadyExistsException(format!("Component {} already exists", name)));
        }
        let arn = format!("arn:aws:greengrass:{}:{}:components/{}", state.region, state.account_id, name);
        let detail = ComponentDetail {
            name: name.clone(),
            arn: arn.clone(),
            status: "ACTIVE".to_string(),
            tags: req.tags.clone(),
        };
        state.components.insert(name, StoredComponent {
            name: detail.name.clone(),
            arn,
            tags: req.tags.unwrap_or_default(),
        });
        Ok(detail)
    }

    #[allow(dead_code)]
    pub async fn get_component(&self, name: &str) -> Result<ComponentDetail, IotgreengrassError> {
        let state = self.inner.lock().await;
        let stored = state.components.get(name)
            .ok_or_else(|| IotgreengrassError::ResourceNotFoundException(format!("Component {} not found", name)))?;
        Ok(ComponentDetail {
            name: stored.name.clone(),
            arn: stored.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(stored.tags.clone()),
        })
    }

    #[allow(dead_code)]
    pub async fn list_components(&self) -> Result<ListComponentsResponse, IotgreengrassError> {
        let state = self.inner.lock().await;
        let items: Vec<ComponentDetail> = state.components.values().map(|s| ComponentDetail {
            name: s.name.clone(),
            arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListComponentsResponse {
            items: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_component(&self, name: &str) -> Result<(), IotgreengrassError> {
        let mut state = self.inner.lock().await;
        state.components.remove(name)
            .ok_or_else(|| IotgreengrassError::ResourceNotFoundException(format!("Component {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = IotgreengrassState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_component() {
        let state = IotgreengrassState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateComponentRequest::default();
        let result = state.create_component(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_get_component_not_found() {
        let state = IotgreengrassState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.get_component("nonexistent").await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_components() {
        let state = IotgreengrassState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.list_components().await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_component_not_found() {
        let state = IotgreengrassState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.delete_component("nonexistent").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_component_full_crud() {
        let state = IotgreengrassState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateComponentRequest::default();
        create_req.name = Some("test-crud-resource".to_string());
        let create_result = state.create_component(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let get_result = state.get_component("test-crud-resource").await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let del_result = state.delete_component("test-crud-resource").await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
