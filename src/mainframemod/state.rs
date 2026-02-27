use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::MainframemodError;
use super::types::*;

#[allow(dead_code)]
struct MainframemodStateInner {
    applications: HashMap<String, StoredApplication>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredApplication {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct MainframemodState {
    inner: Arc<Mutex<MainframemodStateInner>>,
}

impl MainframemodState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        MainframemodState {
            inner: Arc::new(Mutex::new(MainframemodStateInner {
                applications: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_application(&self, req: CreateApplicationRequest) -> Result<ApplicationDetail, MainframemodError> {
        let mut state = self.inner.lock().await;
        let name = req.name.or(req.name_pascal).unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        if state.applications.contains_key(&name) {
            return Err(MainframemodError::ResourceAlreadyExistsException(format!("Application {} already exists", name)));
        }
        let arn = format!("arn:aws:m2:{}:{}:applications/{}", state.region, state.account_id, name);
        let detail = ApplicationDetail {
            name: name.clone(),
            arn: arn.clone(),
            status: "ACTIVE".to_string(),
            tags: req.tags.clone(),
        };
        state.applications.insert(name, StoredApplication {
            name: detail.name.clone(),
            arn,
            tags: req.tags.unwrap_or_default(),
        });
        Ok(detail)
    }

    #[allow(dead_code)]
    pub async fn get_application(&self, name: &str) -> Result<ApplicationDetail, MainframemodError> {
        let state = self.inner.lock().await;
        let stored = state.applications.get(name)
            .ok_or_else(|| MainframemodError::ResourceNotFoundException(format!("Application {} not found", name)))?;
        Ok(ApplicationDetail {
            name: stored.name.clone(),
            arn: stored.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(stored.tags.clone()),
        })
    }

    #[allow(dead_code)]
    pub async fn list_applications(&self) -> Result<ListApplicationsResponse, MainframemodError> {
        let state = self.inner.lock().await;
        let items: Vec<ApplicationDetail> = state.applications.values().map(|s| ApplicationDetail {
            name: s.name.clone(),
            arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListApplicationsResponse {
            items: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_application(&self, name: &str) -> Result<(), MainframemodError> {
        let mut state = self.inner.lock().await;
        state.applications.remove(name)
            .ok_or_else(|| MainframemodError::ResourceNotFoundException(format!("Application {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = MainframemodState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_application() {
        let state = MainframemodState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateApplicationRequest::default();
        let result = state.create_application(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_get_application_not_found() {
        let state = MainframemodState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.get_application("nonexistent").await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_applications() {
        let state = MainframemodState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.list_applications().await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_application_not_found() {
        let state = MainframemodState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.delete_application("nonexistent").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_application_full_crud() {
        let state = MainframemodState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateApplicationRequest::default();
        create_req.name = Some("test-crud-resource".to_string());
        let create_result = state.create_application(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let get_result = state.get_application("test-crud-resource").await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let del_result = state.delete_application("test-crud-resource").await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
