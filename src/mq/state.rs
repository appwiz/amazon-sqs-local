use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::MQError;
use super::types::*;

#[allow(dead_code)]
struct MQStateInner {
    brokers: HashMap<String, StoredBroker>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredBroker {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct MQState {
    inner: Arc<Mutex<MQStateInner>>,
}

impl MQState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        MQState {
            inner: Arc::new(Mutex::new(MQStateInner {
                brokers: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_broker(&self, req: CreateBrokerRequest) -> Result<BrokerDetail, MQError> {
        let mut state = self.inner.lock().await;
        let name = req.name.or(req.name_pascal).unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        if state.brokers.contains_key(&name) {
            return Err(MQError::ResourceAlreadyExistsException(format!("Broker {} already exists", name)));
        }
        let arn = format!("arn:aws:mq:{}:{}:brokers/{}", state.region, state.account_id, name);
        let detail = BrokerDetail {
            name: name.clone(),
            arn: arn.clone(),
            status: "ACTIVE".to_string(),
            tags: req.tags.clone(),
        };
        state.brokers.insert(name, StoredBroker {
            name: detail.name.clone(),
            arn,
            tags: req.tags.unwrap_or_default(),
        });
        Ok(detail)
    }

    #[allow(dead_code)]
    pub async fn get_broker(&self, name: &str) -> Result<BrokerDetail, MQError> {
        let state = self.inner.lock().await;
        let stored = state.brokers.get(name)
            .ok_or_else(|| MQError::ResourceNotFoundException(format!("Broker {} not found", name)))?;
        Ok(BrokerDetail {
            name: stored.name.clone(),
            arn: stored.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(stored.tags.clone()),
        })
    }

    #[allow(dead_code)]
    pub async fn list_brokers(&self) -> Result<ListBrokersResponse, MQError> {
        let state = self.inner.lock().await;
        let items: Vec<BrokerDetail> = state.brokers.values().map(|s| BrokerDetail {
            name: s.name.clone(),
            arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListBrokersResponse {
            items: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_broker(&self, name: &str) -> Result<(), MQError> {
        let mut state = self.inner.lock().await;
        state.brokers.remove(name)
            .ok_or_else(|| MQError::ResourceNotFoundException(format!("Broker {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = MQState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_broker() {
        let state = MQState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateBrokerRequest::default();
        let result = state.create_broker(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_get_broker_not_found() {
        let state = MQState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.get_broker("nonexistent").await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_brokers() {
        let state = MQState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.list_brokers().await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_broker_not_found() {
        let state = MQState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.delete_broker("nonexistent").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_broker_full_crud() {
        let state = MQState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateBrokerRequest::default();
        create_req.name = Some("test-crud-resource".to_string());
        let create_result = state.create_broker(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let get_result = state.get_broker("test-crud-resource").await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let del_result = state.delete_broker("test-crud-resource").await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
