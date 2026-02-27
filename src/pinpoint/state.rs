use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::PinpointError;
use super::types::*;

#[allow(dead_code)]
struct PinpointStateInner {
    apps: HashMap<String, StoredApp>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredApp {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct PinpointState {
    inner: Arc<Mutex<PinpointStateInner>>,
}

impl PinpointState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        PinpointState {
            inner: Arc::new(Mutex::new(PinpointStateInner {
                apps: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_app(&self, req: CreateAppRequest) -> Result<AppDetail, PinpointError> {
        let mut state = self.inner.lock().await;
        let name = req.name.or(req.name_pascal).unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        if state.apps.contains_key(&name) {
            return Err(PinpointError::ResourceAlreadyExistsException(format!("App {} already exists", name)));
        }
        let arn = format!("arn:aws:mobiletargeting:{}:{}:apps/{}", state.region, state.account_id, name);
        let detail = AppDetail {
            name: name.clone(),
            arn: arn.clone(),
            status: "ACTIVE".to_string(),
            tags: req.tags.clone(),
        };
        state.apps.insert(name, StoredApp {
            name: detail.name.clone(),
            arn,
            tags: req.tags.unwrap_or_default(),
        });
        Ok(detail)
    }

    #[allow(dead_code)]
    pub async fn get_app(&self, name: &str) -> Result<AppDetail, PinpointError> {
        let state = self.inner.lock().await;
        let stored = state.apps.get(name)
            .ok_or_else(|| PinpointError::ResourceNotFoundException(format!("App {} not found", name)))?;
        Ok(AppDetail {
            name: stored.name.clone(),
            arn: stored.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(stored.tags.clone()),
        })
    }

    #[allow(dead_code)]
    pub async fn list_apps(&self) -> Result<ListAppsResponse, PinpointError> {
        let state = self.inner.lock().await;
        let items: Vec<AppDetail> = state.apps.values().map(|s| AppDetail {
            name: s.name.clone(),
            arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListAppsResponse {
            items: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_app(&self, name: &str) -> Result<(), PinpointError> {
        let mut state = self.inner.lock().await;
        state.apps.remove(name)
            .ok_or_else(|| PinpointError::ResourceNotFoundException(format!("App {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = PinpointState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_app() {
        let state = PinpointState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateAppRequest::default();
        let result = state.create_app(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_get_app_not_found() {
        let state = PinpointState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.get_app("nonexistent").await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_apps() {
        let state = PinpointState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.list_apps().await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_app_not_found() {
        let state = PinpointState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.delete_app("nonexistent").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_app_full_crud() {
        let state = PinpointState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateAppRequest::default();
        create_req.name = Some("test-crud-resource".to_string());
        let create_result = state.create_app(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let get_result = state.get_app("test-crud-resource").await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let del_result = state.delete_app("test-crud-resource").await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
