use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::AppfabricError;
use super::types::*;

#[allow(dead_code)]
struct AppfabricStateInner {
    app_bundles: HashMap<String, StoredAppBundle>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredAppBundle {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct AppfabricState {
    inner: Arc<Mutex<AppfabricStateInner>>,
}

impl AppfabricState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        AppfabricState {
            inner: Arc::new(Mutex::new(AppfabricStateInner {
                app_bundles: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_app_bundle(&self, req: CreateAppBundleRequest) -> Result<AppBundleDetail, AppfabricError> {
        let mut state = self.inner.lock().await;
        let name = req.name.or(req.name_pascal).unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        if state.app_bundles.contains_key(&name) {
            return Err(AppfabricError::ResourceAlreadyExistsException(format!("AppBundle {} already exists", name)));
        }
        let arn = format!("arn:aws:appfabric:{}:{}:app-bundles/{}", state.region, state.account_id, name);
        let detail = AppBundleDetail {
            name: name.clone(),
            arn: arn.clone(),
            status: "ACTIVE".to_string(),
            tags: req.tags.clone(),
        };
        state.app_bundles.insert(name, StoredAppBundle {
            name: detail.name.clone(),
            arn,
            tags: req.tags.unwrap_or_default(),
        });
        Ok(detail)
    }

    #[allow(dead_code)]
    pub async fn get_app_bundle(&self, name: &str) -> Result<AppBundleDetail, AppfabricError> {
        let state = self.inner.lock().await;
        let stored = state.app_bundles.get(name)
            .ok_or_else(|| AppfabricError::ResourceNotFoundException(format!("AppBundle {} not found", name)))?;
        Ok(AppBundleDetail {
            name: stored.name.clone(),
            arn: stored.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(stored.tags.clone()),
        })
    }

    #[allow(dead_code)]
    pub async fn list_app_bundles(&self) -> Result<ListAppBundlesResponse, AppfabricError> {
        let state = self.inner.lock().await;
        let items: Vec<AppBundleDetail> = state.app_bundles.values().map(|s| AppBundleDetail {
            name: s.name.clone(),
            arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListAppBundlesResponse {
            items: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_app_bundle(&self, name: &str) -> Result<(), AppfabricError> {
        let mut state = self.inner.lock().await;
        state.app_bundles.remove(name)
            .ok_or_else(|| AppfabricError::ResourceNotFoundException(format!("AppBundle {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = AppfabricState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_app_bundle() {
        let state = AppfabricState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateAppBundleRequest::default();
        let result = state.create_app_bundle(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_get_app_bundle_not_found() {
        let state = AppfabricState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.get_app_bundle("nonexistent").await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_app_bundles() {
        let state = AppfabricState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.list_app_bundles().await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_app_bundle_not_found() {
        let state = AppfabricState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.delete_app_bundle("nonexistent").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_app_bundle_full_crud() {
        let state = AppfabricState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateAppBundleRequest::default();
        create_req.name = Some("test-crud-resource".to_string());
        let create_result = state.create_app_bundle(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let get_result = state.get_app_bundle("test-crud-resource").await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let del_result = state.delete_app_bundle("test-crud-resource").await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
