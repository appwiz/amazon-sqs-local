use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::ManagedblockchainError;
use super::types::*;

#[allow(dead_code)]
struct ManagedblockchainStateInner {
    networks: HashMap<String, StoredNetwork>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredNetwork {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct ManagedblockchainState {
    inner: Arc<Mutex<ManagedblockchainStateInner>>,
}

impl ManagedblockchainState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        ManagedblockchainState {
            inner: Arc::new(Mutex::new(ManagedblockchainStateInner {
                networks: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_network(&self, req: CreateNetworkRequest) -> Result<NetworkDetail, ManagedblockchainError> {
        let mut state = self.inner.lock().await;
        let name = req.name.or(req.name_pascal).unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        if state.networks.contains_key(&name) {
            return Err(ManagedblockchainError::ResourceAlreadyExistsException(format!("Network {} already exists", name)));
        }
        let arn = format!("arn:aws:managedblockchain:{}:{}:networks/{}", state.region, state.account_id, name);
        let detail = NetworkDetail {
            name: name.clone(),
            arn: arn.clone(),
            status: "ACTIVE".to_string(),
            tags: req.tags.clone(),
        };
        state.networks.insert(name, StoredNetwork {
            name: detail.name.clone(),
            arn,
            tags: req.tags.unwrap_or_default(),
        });
        Ok(detail)
    }

    #[allow(dead_code)]
    pub async fn get_network(&self, name: &str) -> Result<NetworkDetail, ManagedblockchainError> {
        let state = self.inner.lock().await;
        let stored = state.networks.get(name)
            .ok_or_else(|| ManagedblockchainError::ResourceNotFoundException(format!("Network {} not found", name)))?;
        Ok(NetworkDetail {
            name: stored.name.clone(),
            arn: stored.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(stored.tags.clone()),
        })
    }

    #[allow(dead_code)]
    pub async fn list_networks(&self) -> Result<ListNetworksResponse, ManagedblockchainError> {
        let state = self.inner.lock().await;
        let items: Vec<NetworkDetail> = state.networks.values().map(|s| NetworkDetail {
            name: s.name.clone(),
            arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListNetworksResponse {
            items: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_network(&self, name: &str) -> Result<(), ManagedblockchainError> {
        let mut state = self.inner.lock().await;
        state.networks.remove(name)
            .ok_or_else(|| ManagedblockchainError::ResourceNotFoundException(format!("Network {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = ManagedblockchainState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_network() {
        let state = ManagedblockchainState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateNetworkRequest::default();
        let result = state.create_network(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_get_network_not_found() {
        let state = ManagedblockchainState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.get_network("nonexistent").await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_networks() {
        let state = ManagedblockchainState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.list_networks().await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_network_not_found() {
        let state = ManagedblockchainState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.delete_network("nonexistent").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_network_full_crud() {
        let state = ManagedblockchainState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateNetworkRequest::default();
        create_req.name = Some("test-crud-resource".to_string());
        let create_result = state.create_network(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let get_result = state.get_network("test-crud-resource").await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let del_result = state.delete_network("test-crud-resource").await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
