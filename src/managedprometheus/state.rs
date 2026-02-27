use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::ManagedprometheusError;
use super::types::*;

#[allow(dead_code)]
struct ManagedprometheusStateInner {
    workspaces: HashMap<String, StoredWorkspace>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredWorkspace {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct ManagedprometheusState {
    inner: Arc<Mutex<ManagedprometheusStateInner>>,
}

impl ManagedprometheusState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        ManagedprometheusState {
            inner: Arc::new(Mutex::new(ManagedprometheusStateInner {
                workspaces: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_workspace(&self, req: CreateWorkspaceRequest) -> Result<WorkspaceDetail, ManagedprometheusError> {
        let mut state = self.inner.lock().await;
        let name = req.name.or(req.name_pascal).unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        if state.workspaces.contains_key(&name) {
            return Err(ManagedprometheusError::ResourceAlreadyExistsException(format!("Workspace {} already exists", name)));
        }
        let arn = format!("arn:aws:aps:{}:{}:workspaces/{}", state.region, state.account_id, name);
        let detail = WorkspaceDetail {
            name: name.clone(),
            arn: arn.clone(),
            status: "ACTIVE".to_string(),
            tags: req.tags.clone(),
        };
        state.workspaces.insert(name, StoredWorkspace {
            name: detail.name.clone(),
            arn,
            tags: req.tags.unwrap_or_default(),
        });
        Ok(detail)
    }

    #[allow(dead_code)]
    pub async fn get_workspace(&self, name: &str) -> Result<WorkspaceDetail, ManagedprometheusError> {
        let state = self.inner.lock().await;
        let stored = state.workspaces.get(name)
            .ok_or_else(|| ManagedprometheusError::ResourceNotFoundException(format!("Workspace {} not found", name)))?;
        Ok(WorkspaceDetail {
            name: stored.name.clone(),
            arn: stored.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(stored.tags.clone()),
        })
    }

    #[allow(dead_code)]
    pub async fn list_workspaces(&self) -> Result<ListWorkspacesResponse, ManagedprometheusError> {
        let state = self.inner.lock().await;
        let items: Vec<WorkspaceDetail> = state.workspaces.values().map(|s| WorkspaceDetail {
            name: s.name.clone(),
            arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListWorkspacesResponse {
            items: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_workspace(&self, name: &str) -> Result<(), ManagedprometheusError> {
        let mut state = self.inner.lock().await;
        state.workspaces.remove(name)
            .ok_or_else(|| ManagedprometheusError::ResourceNotFoundException(format!("Workspace {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = ManagedprometheusState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_workspace() {
        let state = ManagedprometheusState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateWorkspaceRequest::default();
        let result = state.create_workspace(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_get_workspace_not_found() {
        let state = ManagedprometheusState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.get_workspace("nonexistent").await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_workspaces() {
        let state = ManagedprometheusState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.list_workspaces().await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_workspace_not_found() {
        let state = ManagedprometheusState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.delete_workspace("nonexistent").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_workspace_full_crud() {
        let state = ManagedprometheusState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateWorkspaceRequest::default();
        create_req.name = Some("test-crud-resource".to_string());
        let create_result = state.create_workspace(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let get_result = state.get_workspace("test-crud-resource").await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let del_result = state.delete_workspace("test-crud-resource").await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
