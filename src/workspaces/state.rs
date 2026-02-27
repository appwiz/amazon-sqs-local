use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::WorkspacesError;
use super::types::*;

#[allow(dead_code)]
struct WorkspacesStateInner {
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
pub struct WorkspacesState {
    inner: Arc<Mutex<WorkspacesStateInner>>,
}

impl WorkspacesState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        WorkspacesState {
            inner: Arc::new(Mutex::new(WorkspacesStateInner {
                workspaces: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_workspace(&self, req: CreateWorkspaceRequest) -> Result<CreateWorkspaceResponse, WorkspacesError> {
        let mut state = self.inner.lock().await;
        let name = req.workspace_name.clone();
        if state.workspaces.contains_key(&name) {
            return Err(WorkspacesError::ResourceAlreadyExistsException(format!("Workspace {} already exists", name)));
        }
        let arn = format!("arn:aws:workspaces:{}:{}:workspaces/{}", state.region, state.account_id, name);
        state.workspaces.insert(name.clone(), StoredWorkspace {
            name: name.clone(),
            arn: arn.clone(),
            tags: req.tags.unwrap_or_default(),
        });
        Ok(CreateWorkspaceResponse {
            workspace_arn: Some(arn),
            workspace_name: Some(name),
        })
    }

    #[allow(dead_code)]
    pub async fn describe_workspace(&self, req: DescribeWorkspaceRequest) -> Result<DescribeWorkspaceResponse, WorkspacesError> {
        let state = self.inner.lock().await;
        let name = req.workspace_name.or_else(|| req.workspace_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| WorkspacesError::ValidationException("Name or ARN required".to_string()))?;
        let stored = state.workspaces.get(&name)
            .ok_or_else(|| WorkspacesError::ResourceNotFoundException(format!("Workspace {} not found", name)))?;
        Ok(DescribeWorkspaceResponse {
            workspace: WorkspaceDetail {
                workspace_name: stored.name.clone(),
                workspace_arn: stored.arn.clone(),
                status: "ACTIVE".to_string(),
                tags: Some(stored.tags.clone()),
            },
        })
    }

    #[allow(dead_code)]
    pub async fn list_workspaces(&self, _req: ListWorkspacesRequest) -> Result<ListWorkspacesResponse, WorkspacesError> {
        let state = self.inner.lock().await;
        let items: Vec<WorkspaceDetail> = state.workspaces.values().map(|s| WorkspaceDetail {
            workspace_name: s.name.clone(),
            workspace_arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListWorkspacesResponse {
            workspaces: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_workspace(&self, req: DeleteWorkspaceRequest) -> Result<(), WorkspacesError> {
        let mut state = self.inner.lock().await;
        let name = req.workspace_name.or_else(|| req.workspace_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| WorkspacesError::ValidationException("Name or ARN required".to_string()))?;
        state.workspaces.remove(&name)
            .ok_or_else(|| WorkspacesError::ResourceNotFoundException(format!("Workspace {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = WorkspacesState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_workspace() {
        let state = WorkspacesState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateWorkspaceRequest::default();
        let result = state.create_workspace(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_describe_workspace_not_found() {
        let state = WorkspacesState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DescribeWorkspaceRequest::default();
        let result = state.describe_workspace(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_workspaces_empty() {
        let state = WorkspacesState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = ListWorkspacesRequest::default();
        let result = state.list_workspaces(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_workspace_not_found() {
        let state = WorkspacesState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DeleteWorkspaceRequest::default();
        let result = state.delete_workspace(req).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_workspace_create_and_list() {
        let state = WorkspacesState::new("123456789012".to_string(), "us-east-1".to_string());
        let create_req = CreateWorkspaceRequest::default();
        let _created = state.create_workspace(create_req).await.unwrap();
        let list_req = ListWorkspacesRequest::default();
        let listed = state.list_workspaces(list_req).await.unwrap();
        let _ = listed;
    }

    #[tokio::test]
    async fn test_workspace_full_crud() {
        let state = WorkspacesState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateWorkspaceRequest::default();
        create_req.workspace_name = "test-crud-resource".to_string();
        let create_result = state.create_workspace(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let mut get_req = DescribeWorkspaceRequest::default();
        get_req.workspace_name = Some("test-crud-resource".to_string());
        let get_result = state.describe_workspace(get_req).await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let mut del_req = DeleteWorkspaceRequest::default();
        del_req.workspace_name = Some("test-crud-resource".to_string());
        let del_result = state.delete_workspace(del_req).await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
