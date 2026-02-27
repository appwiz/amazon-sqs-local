use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::VpclatticeError;
use super::types::*;

#[allow(dead_code)]
struct VpclatticeStateInner {
    service_networks: HashMap<String, StoredServiceNetwork>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredServiceNetwork {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct VpclatticeState {
    inner: Arc<Mutex<VpclatticeStateInner>>,
}

impl VpclatticeState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        VpclatticeState {
            inner: Arc::new(Mutex::new(VpclatticeStateInner {
                service_networks: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_service_network(&self, req: CreateServiceNetworkRequest) -> Result<ServiceNetworkDetail, VpclatticeError> {
        let mut state = self.inner.lock().await;
        let name = req.name.or(req.name_pascal).unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        if state.service_networks.contains_key(&name) {
            return Err(VpclatticeError::ResourceAlreadyExistsException(format!("ServiceNetwork {} already exists", name)));
        }
        let arn = format!("arn:aws:vpc-lattice:{}:{}:service-networks/{}", state.region, state.account_id, name);
        let detail = ServiceNetworkDetail {
            name: name.clone(),
            arn: arn.clone(),
            status: "ACTIVE".to_string(),
            tags: req.tags.clone(),
        };
        state.service_networks.insert(name, StoredServiceNetwork {
            name: detail.name.clone(),
            arn,
            tags: req.tags.unwrap_or_default(),
        });
        Ok(detail)
    }

    #[allow(dead_code)]
    pub async fn get_service_network(&self, name: &str) -> Result<ServiceNetworkDetail, VpclatticeError> {
        let state = self.inner.lock().await;
        let stored = state.service_networks.get(name)
            .ok_or_else(|| VpclatticeError::ResourceNotFoundException(format!("ServiceNetwork {} not found", name)))?;
        Ok(ServiceNetworkDetail {
            name: stored.name.clone(),
            arn: stored.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(stored.tags.clone()),
        })
    }

    #[allow(dead_code)]
    pub async fn list_service_networks(&self) -> Result<ListServiceNetworksResponse, VpclatticeError> {
        let state = self.inner.lock().await;
        let items: Vec<ServiceNetworkDetail> = state.service_networks.values().map(|s| ServiceNetworkDetail {
            name: s.name.clone(),
            arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListServiceNetworksResponse {
            items: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_service_network(&self, name: &str) -> Result<(), VpclatticeError> {
        let mut state = self.inner.lock().await;
        state.service_networks.remove(name)
            .ok_or_else(|| VpclatticeError::ResourceNotFoundException(format!("ServiceNetwork {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = VpclatticeState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_service_network() {
        let state = VpclatticeState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateServiceNetworkRequest::default();
        let result = state.create_service_network(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_get_service_network_not_found() {
        let state = VpclatticeState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.get_service_network("nonexistent").await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_service_networks() {
        let state = VpclatticeState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.list_service_networks().await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_service_network_not_found() {
        let state = VpclatticeState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.delete_service_network("nonexistent").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_service_network_full_crud() {
        let state = VpclatticeState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateServiceNetworkRequest::default();
        create_req.name = Some("test-crud-resource".to_string());
        let create_result = state.create_service_network(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let get_result = state.get_service_network("test-crud-resource").await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let del_result = state.delete_service_network("test-crud-resource").await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
