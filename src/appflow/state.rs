use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::AppflowError;
use super::types::*;

#[allow(dead_code)]
struct AppflowStateInner {
    flows: HashMap<String, StoredFlow>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredFlow {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct AppflowState {
    inner: Arc<Mutex<AppflowStateInner>>,
}

impl AppflowState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        AppflowState {
            inner: Arc::new(Mutex::new(AppflowStateInner {
                flows: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_flow(&self, req: CreateFlowRequest) -> Result<FlowDetail, AppflowError> {
        let mut state = self.inner.lock().await;
        let name = req.name.or(req.name_pascal).unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        if state.flows.contains_key(&name) {
            return Err(AppflowError::ResourceAlreadyExistsException(format!("Flow {} already exists", name)));
        }
        let arn = format!("arn:aws:appflow:{}:{}:flows/{}", state.region, state.account_id, name);
        let detail = FlowDetail {
            name: name.clone(),
            arn: arn.clone(),
            status: "ACTIVE".to_string(),
            tags: req.tags.clone(),
        };
        state.flows.insert(name, StoredFlow {
            name: detail.name.clone(),
            arn,
            tags: req.tags.unwrap_or_default(),
        });
        Ok(detail)
    }

    #[allow(dead_code)]
    pub async fn get_flow(&self, name: &str) -> Result<FlowDetail, AppflowError> {
        let state = self.inner.lock().await;
        let stored = state.flows.get(name)
            .ok_or_else(|| AppflowError::ResourceNotFoundException(format!("Flow {} not found", name)))?;
        Ok(FlowDetail {
            name: stored.name.clone(),
            arn: stored.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(stored.tags.clone()),
        })
    }

    #[allow(dead_code)]
    pub async fn list_flows(&self) -> Result<ListFlowsResponse, AppflowError> {
        let state = self.inner.lock().await;
        let items: Vec<FlowDetail> = state.flows.values().map(|s| FlowDetail {
            name: s.name.clone(),
            arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListFlowsResponse {
            items: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_flow(&self, name: &str) -> Result<(), AppflowError> {
        let mut state = self.inner.lock().await;
        state.flows.remove(name)
            .ok_or_else(|| AppflowError::ResourceNotFoundException(format!("Flow {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = AppflowState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_flow() {
        let state = AppflowState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateFlowRequest::default();
        let result = state.create_flow(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_get_flow_not_found() {
        let state = AppflowState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.get_flow("nonexistent").await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_flows() {
        let state = AppflowState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.list_flows().await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_flow_not_found() {
        let state = AppflowState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.delete_flow("nonexistent").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_flow_full_crud() {
        let state = AppflowState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateFlowRequest::default();
        create_req.name = Some("test-crud-resource".to_string());
        let create_result = state.create_flow(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let get_result = state.get_flow("test-crud-resource").await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let del_result = state.delete_flow("test-crud-resource").await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
