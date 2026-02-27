use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::RAMError;
use super::types::*;

#[allow(dead_code)]
struct RAMStateInner {
    resource_shares: HashMap<String, StoredResourceShare>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredResourceShare {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct RAMState {
    inner: Arc<Mutex<RAMStateInner>>,
}

impl RAMState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        RAMState {
            inner: Arc::new(Mutex::new(RAMStateInner {
                resource_shares: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_resource_share(&self, req: CreateResourceShareRequest) -> Result<ResourceShareDetail, RAMError> {
        let mut state = self.inner.lock().await;
        let name = req.name.or(req.name_pascal).unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        if state.resource_shares.contains_key(&name) {
            return Err(RAMError::ResourceAlreadyExistsException(format!("ResourceShare {} already exists", name)));
        }
        let arn = format!("arn:aws:ram:{}:{}:resource-shares/{}", state.region, state.account_id, name);
        let detail = ResourceShareDetail {
            name: name.clone(),
            arn: arn.clone(),
            status: "ACTIVE".to_string(),
            tags: req.tags.clone(),
        };
        state.resource_shares.insert(name, StoredResourceShare {
            name: detail.name.clone(),
            arn,
            tags: req.tags.unwrap_or_default(),
        });
        Ok(detail)
    }

    #[allow(dead_code)]
    pub async fn get_resource_share(&self, name: &str) -> Result<ResourceShareDetail, RAMError> {
        let state = self.inner.lock().await;
        let stored = state.resource_shares.get(name)
            .ok_or_else(|| RAMError::ResourceNotFoundException(format!("ResourceShare {} not found", name)))?;
        Ok(ResourceShareDetail {
            name: stored.name.clone(),
            arn: stored.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(stored.tags.clone()),
        })
    }

    #[allow(dead_code)]
    pub async fn list_resource_shares(&self) -> Result<ListResourceSharesResponse, RAMError> {
        let state = self.inner.lock().await;
        let items: Vec<ResourceShareDetail> = state.resource_shares.values().map(|s| ResourceShareDetail {
            name: s.name.clone(),
            arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListResourceSharesResponse {
            items: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_resource_share(&self, name: &str) -> Result<(), RAMError> {
        let mut state = self.inner.lock().await;
        state.resource_shares.remove(name)
            .ok_or_else(|| RAMError::ResourceNotFoundException(format!("ResourceShare {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = RAMState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_resource_share() {
        let state = RAMState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateResourceShareRequest::default();
        let result = state.create_resource_share(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_get_resource_share_not_found() {
        let state = RAMState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.get_resource_share("nonexistent").await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_resource_shares() {
        let state = RAMState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.list_resource_shares().await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_resource_share_not_found() {
        let state = RAMState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.delete_resource_share("nonexistent").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_resource_share_full_crud() {
        let state = RAMState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateResourceShareRequest::default();
        create_req.name = Some("test-crud-resource".to_string());
        let create_result = state.create_resource_share(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let get_result = state.get_resource_share("test-crud-resource").await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let del_result = state.delete_resource_share("test-crud-resource").await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
