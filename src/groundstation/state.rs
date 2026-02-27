use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::GroundstationError;
use super::types::*;

#[allow(dead_code)]
struct GroundstationStateInner {
    configs: HashMap<String, StoredConfig>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredConfig {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct GroundstationState {
    inner: Arc<Mutex<GroundstationStateInner>>,
}

impl GroundstationState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        GroundstationState {
            inner: Arc::new(Mutex::new(GroundstationStateInner {
                configs: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_config(&self, req: CreateConfigRequest) -> Result<ConfigDetail, GroundstationError> {
        let mut state = self.inner.lock().await;
        let name = req.name.or(req.name_pascal).unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        if state.configs.contains_key(&name) {
            return Err(GroundstationError::ResourceAlreadyExistsException(format!("Config {} already exists", name)));
        }
        let arn = format!("arn:aws:groundstation:{}:{}:configs/{}", state.region, state.account_id, name);
        let detail = ConfigDetail {
            name: name.clone(),
            arn: arn.clone(),
            status: "ACTIVE".to_string(),
            tags: req.tags.clone(),
        };
        state.configs.insert(name, StoredConfig {
            name: detail.name.clone(),
            arn,
            tags: req.tags.unwrap_or_default(),
        });
        Ok(detail)
    }

    #[allow(dead_code)]
    pub async fn get_config(&self, name: &str) -> Result<ConfigDetail, GroundstationError> {
        let state = self.inner.lock().await;
        let stored = state.configs.get(name)
            .ok_or_else(|| GroundstationError::ResourceNotFoundException(format!("Config {} not found", name)))?;
        Ok(ConfigDetail {
            name: stored.name.clone(),
            arn: stored.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(stored.tags.clone()),
        })
    }

    #[allow(dead_code)]
    pub async fn list_configs(&self) -> Result<ListConfigsResponse, GroundstationError> {
        let state = self.inner.lock().await;
        let items: Vec<ConfigDetail> = state.configs.values().map(|s| ConfigDetail {
            name: s.name.clone(),
            arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListConfigsResponse {
            items: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_config(&self, name: &str) -> Result<(), GroundstationError> {
        let mut state = self.inner.lock().await;
        state.configs.remove(name)
            .ok_or_else(|| GroundstationError::ResourceNotFoundException(format!("Config {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = GroundstationState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_config() {
        let state = GroundstationState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateConfigRequest::default();
        let result = state.create_config(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_get_config_not_found() {
        let state = GroundstationState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.get_config("nonexistent").await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_configs() {
        let state = GroundstationState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.list_configs().await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_config_not_found() {
        let state = GroundstationState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.delete_config("nonexistent").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_config_full_crud() {
        let state = GroundstationState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateConfigRequest::default();
        create_req.name = Some("test-crud-resource".to_string());
        let create_result = state.create_config(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let get_result = state.get_config("test-crud-resource").await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let del_result = state.delete_config("test-crud-resource").await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
