use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::IotsitewiseError;
use super::types::*;

#[allow(dead_code)]
struct IotsitewiseStateInner {
    assets: HashMap<String, StoredAsset>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredAsset {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct IotsitewiseState {
    inner: Arc<Mutex<IotsitewiseStateInner>>,
}

impl IotsitewiseState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        IotsitewiseState {
            inner: Arc::new(Mutex::new(IotsitewiseStateInner {
                assets: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_asset(&self, req: CreateAssetRequest) -> Result<AssetDetail, IotsitewiseError> {
        let mut state = self.inner.lock().await;
        let name = req.name.or(req.name_pascal).unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        if state.assets.contains_key(&name) {
            return Err(IotsitewiseError::ResourceAlreadyExistsException(format!("Asset {} already exists", name)));
        }
        let arn = format!("arn:aws:iotsitewise:{}:{}:assets/{}", state.region, state.account_id, name);
        let detail = AssetDetail {
            name: name.clone(),
            arn: arn.clone(),
            status: "ACTIVE".to_string(),
            tags: req.tags.clone(),
        };
        state.assets.insert(name, StoredAsset {
            name: detail.name.clone(),
            arn,
            tags: req.tags.unwrap_or_default(),
        });
        Ok(detail)
    }

    #[allow(dead_code)]
    pub async fn get_asset(&self, name: &str) -> Result<AssetDetail, IotsitewiseError> {
        let state = self.inner.lock().await;
        let stored = state.assets.get(name)
            .ok_or_else(|| IotsitewiseError::ResourceNotFoundException(format!("Asset {} not found", name)))?;
        Ok(AssetDetail {
            name: stored.name.clone(),
            arn: stored.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(stored.tags.clone()),
        })
    }

    #[allow(dead_code)]
    pub async fn list_assets(&self) -> Result<ListAssetsResponse, IotsitewiseError> {
        let state = self.inner.lock().await;
        let items: Vec<AssetDetail> = state.assets.values().map(|s| AssetDetail {
            name: s.name.clone(),
            arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListAssetsResponse {
            items: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_asset(&self, name: &str) -> Result<(), IotsitewiseError> {
        let mut state = self.inner.lock().await;
        state.assets.remove(name)
            .ok_or_else(|| IotsitewiseError::ResourceNotFoundException(format!("Asset {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = IotsitewiseState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_asset() {
        let state = IotsitewiseState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateAssetRequest::default();
        let result = state.create_asset(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_get_asset_not_found() {
        let state = IotsitewiseState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.get_asset("nonexistent").await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_assets() {
        let state = IotsitewiseState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.list_assets().await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_asset_not_found() {
        let state = IotsitewiseState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.delete_asset("nonexistent").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_asset_full_crud() {
        let state = IotsitewiseState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateAssetRequest::default();
        create_req.name = Some("test-crud-resource".to_string());
        let create_result = state.create_asset(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let get_result = state.get_asset("test-crud-resource").await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let del_result = state.delete_asset("test-crud-resource").await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
