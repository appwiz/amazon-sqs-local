use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::EntityresolutionError;
use super::types::*;

#[allow(dead_code)]
struct EntityresolutionStateInner {
    matching_workflows: HashMap<String, StoredMatchingWorkflow>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredMatchingWorkflow {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct EntityresolutionState {
    inner: Arc<Mutex<EntityresolutionStateInner>>,
}

impl EntityresolutionState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        EntityresolutionState {
            inner: Arc::new(Mutex::new(EntityresolutionStateInner {
                matching_workflows: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_matching_workflow(&self, req: CreateMatchingWorkflowRequest) -> Result<MatchingWorkflowDetail, EntityresolutionError> {
        let mut state = self.inner.lock().await;
        let name = req.name.or(req.name_pascal).unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        if state.matching_workflows.contains_key(&name) {
            return Err(EntityresolutionError::ResourceAlreadyExistsException(format!("MatchingWorkflow {} already exists", name)));
        }
        let arn = format!("arn:aws:entityresolution:{}:{}:matching-workflows/{}", state.region, state.account_id, name);
        let detail = MatchingWorkflowDetail {
            name: name.clone(),
            arn: arn.clone(),
            status: "ACTIVE".to_string(),
            tags: req.tags.clone(),
        };
        state.matching_workflows.insert(name, StoredMatchingWorkflow {
            name: detail.name.clone(),
            arn,
            tags: req.tags.unwrap_or_default(),
        });
        Ok(detail)
    }

    #[allow(dead_code)]
    pub async fn get_matching_workflow(&self, name: &str) -> Result<MatchingWorkflowDetail, EntityresolutionError> {
        let state = self.inner.lock().await;
        let stored = state.matching_workflows.get(name)
            .ok_or_else(|| EntityresolutionError::ResourceNotFoundException(format!("MatchingWorkflow {} not found", name)))?;
        Ok(MatchingWorkflowDetail {
            name: stored.name.clone(),
            arn: stored.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(stored.tags.clone()),
        })
    }

    #[allow(dead_code)]
    pub async fn list_matching_workflows(&self) -> Result<ListMatchingWorkflowsResponse, EntityresolutionError> {
        let state = self.inner.lock().await;
        let items: Vec<MatchingWorkflowDetail> = state.matching_workflows.values().map(|s| MatchingWorkflowDetail {
            name: s.name.clone(),
            arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListMatchingWorkflowsResponse {
            items: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_matching_workflow(&self, name: &str) -> Result<(), EntityresolutionError> {
        let mut state = self.inner.lock().await;
        state.matching_workflows.remove(name)
            .ok_or_else(|| EntityresolutionError::ResourceNotFoundException(format!("MatchingWorkflow {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = EntityresolutionState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_matching_workflow() {
        let state = EntityresolutionState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateMatchingWorkflowRequest::default();
        let result = state.create_matching_workflow(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_get_matching_workflow_not_found() {
        let state = EntityresolutionState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.get_matching_workflow("nonexistent").await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_matching_workflows() {
        let state = EntityresolutionState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.list_matching_workflows().await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_matching_workflow_not_found() {
        let state = EntityresolutionState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.delete_matching_workflow("nonexistent").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_matching_workflow_full_crud() {
        let state = EntityresolutionState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateMatchingWorkflowRequest::default();
        create_req.name = Some("test-crud-resource".to_string());
        let create_result = state.create_matching_workflow(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let get_result = state.get_matching_workflow("test-crud-resource").await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let del_result = state.delete_matching_workflow("test-crud-resource").await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
