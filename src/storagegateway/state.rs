use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::StoragegatewayError;
use super::types::*;

#[allow(dead_code)]
struct StoragegatewayStateInner {
    gateways: HashMap<String, StoredGateway>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredGateway {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct StoragegatewayState {
    inner: Arc<Mutex<StoragegatewayStateInner>>,
}

impl StoragegatewayState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        StoragegatewayState {
            inner: Arc::new(Mutex::new(StoragegatewayStateInner {
                gateways: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_gateway(&self, req: CreateGatewayRequest) -> Result<CreateGatewayResponse, StoragegatewayError> {
        let mut state = self.inner.lock().await;
        let name = req.gateway_name.clone();
        if state.gateways.contains_key(&name) {
            return Err(StoragegatewayError::ResourceAlreadyExistsException(format!("Gateway {} already exists", name)));
        }
        let arn = format!("arn:aws:storagegateway:{}:{}:gateways/{}", state.region, state.account_id, name);
        state.gateways.insert(name.clone(), StoredGateway {
            name: name.clone(),
            arn: arn.clone(),
            tags: req.tags.unwrap_or_default(),
        });
        Ok(CreateGatewayResponse {
            gateway_arn: Some(arn),
            gateway_name: Some(name),
        })
    }

    #[allow(dead_code)]
    pub async fn describe_gateway(&self, req: DescribeGatewayRequest) -> Result<DescribeGatewayResponse, StoragegatewayError> {
        let state = self.inner.lock().await;
        let name = req.gateway_name.or_else(|| req.gateway_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| StoragegatewayError::ValidationException("Name or ARN required".to_string()))?;
        let stored = state.gateways.get(&name)
            .ok_or_else(|| StoragegatewayError::ResourceNotFoundException(format!("Gateway {} not found", name)))?;
        Ok(DescribeGatewayResponse {
            gateway: GatewayDetail {
                gateway_name: stored.name.clone(),
                gateway_arn: stored.arn.clone(),
                status: "ACTIVE".to_string(),
                tags: Some(stored.tags.clone()),
            },
        })
    }

    #[allow(dead_code)]
    pub async fn list_gateways(&self, _req: ListGatewaysRequest) -> Result<ListGatewaysResponse, StoragegatewayError> {
        let state = self.inner.lock().await;
        let items: Vec<GatewayDetail> = state.gateways.values().map(|s| GatewayDetail {
            gateway_name: s.name.clone(),
            gateway_arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListGatewaysResponse {
            gateways: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_gateway(&self, req: DeleteGatewayRequest) -> Result<(), StoragegatewayError> {
        let mut state = self.inner.lock().await;
        let name = req.gateway_name.or_else(|| req.gateway_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| StoragegatewayError::ValidationException("Name or ARN required".to_string()))?;
        state.gateways.remove(&name)
            .ok_or_else(|| StoragegatewayError::ResourceNotFoundException(format!("Gateway {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = StoragegatewayState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_gateway() {
        let state = StoragegatewayState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateGatewayRequest::default();
        let result = state.create_gateway(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_describe_gateway_not_found() {
        let state = StoragegatewayState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DescribeGatewayRequest::default();
        let result = state.describe_gateway(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_gateways_empty() {
        let state = StoragegatewayState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = ListGatewaysRequest::default();
        let result = state.list_gateways(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_gateway_not_found() {
        let state = StoragegatewayState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DeleteGatewayRequest::default();
        let result = state.delete_gateway(req).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_gateway_create_and_list() {
        let state = StoragegatewayState::new("123456789012".to_string(), "us-east-1".to_string());
        let create_req = CreateGatewayRequest::default();
        let _created = state.create_gateway(create_req).await.unwrap();
        let list_req = ListGatewaysRequest::default();
        let listed = state.list_gateways(list_req).await.unwrap();
        let _ = listed;
    }

    #[tokio::test]
    async fn test_gateway_full_crud() {
        let state = StoragegatewayState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateGatewayRequest::default();
        create_req.gateway_name = "test-crud-resource".to_string();
        let create_result = state.create_gateway(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let mut get_req = DescribeGatewayRequest::default();
        get_req.gateway_name = Some("test-crud-resource".to_string());
        let get_result = state.describe_gateway(get_req).await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let mut del_req = DeleteGatewayRequest::default();
        del_req.gateway_name = Some("test-crud-resource".to_string());
        let del_result = state.delete_gateway(del_req).await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
