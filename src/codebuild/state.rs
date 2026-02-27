use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::CodebuildError;
use super::types::*;

#[allow(dead_code)]
struct CodebuildStateInner {
    projects: HashMap<String, StoredProject>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredProject {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct CodebuildState {
    inner: Arc<Mutex<CodebuildStateInner>>,
}

impl CodebuildState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        CodebuildState {
            inner: Arc::new(Mutex::new(CodebuildStateInner {
                projects: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_project(&self, req: CreateProjectRequest) -> Result<CreateProjectResponse, CodebuildError> {
        let mut state = self.inner.lock().await;
        let name = req.project_name.clone();
        if state.projects.contains_key(&name) {
            return Err(CodebuildError::ResourceAlreadyExistsException(format!("Project {} already exists", name)));
        }
        let arn = format!("arn:aws:codebuild:{}:{}:projects/{}", state.region, state.account_id, name);
        state.projects.insert(name.clone(), StoredProject {
            name: name.clone(),
            arn: arn.clone(),
            tags: req.tags.unwrap_or_default(),
        });
        Ok(CreateProjectResponse {
            project_arn: Some(arn),
            project_name: Some(name),
        })
    }

    #[allow(dead_code)]
    pub async fn describe_project(&self, req: DescribeProjectRequest) -> Result<DescribeProjectResponse, CodebuildError> {
        let state = self.inner.lock().await;
        let name = req.project_name.or_else(|| req.project_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| CodebuildError::ValidationException("Name or ARN required".to_string()))?;
        let stored = state.projects.get(&name)
            .ok_or_else(|| CodebuildError::ResourceNotFoundException(format!("Project {} not found", name)))?;
        Ok(DescribeProjectResponse {
            project: ProjectDetail {
                project_name: stored.name.clone(),
                project_arn: stored.arn.clone(),
                status: "ACTIVE".to_string(),
                tags: Some(stored.tags.clone()),
            },
        })
    }

    #[allow(dead_code)]
    pub async fn list_projects(&self, _req: ListProjectsRequest) -> Result<ListProjectsResponse, CodebuildError> {
        let state = self.inner.lock().await;
        let items: Vec<ProjectDetail> = state.projects.values().map(|s| ProjectDetail {
            project_name: s.name.clone(),
            project_arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListProjectsResponse {
            projects: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_project(&self, req: DeleteProjectRequest) -> Result<(), CodebuildError> {
        let mut state = self.inner.lock().await;
        let name = req.project_name.or_else(|| req.project_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| CodebuildError::ValidationException("Name or ARN required".to_string()))?;
        state.projects.remove(&name)
            .ok_or_else(|| CodebuildError::ResourceNotFoundException(format!("Project {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = CodebuildState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_project() {
        let state = CodebuildState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateProjectRequest::default();
        let result = state.create_project(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_describe_project_not_found() {
        let state = CodebuildState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DescribeProjectRequest::default();
        let result = state.describe_project(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_projects_empty() {
        let state = CodebuildState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = ListProjectsRequest::default();
        let result = state.list_projects(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_project_not_found() {
        let state = CodebuildState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DeleteProjectRequest::default();
        let result = state.delete_project(req).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_project_create_and_list() {
        let state = CodebuildState::new("123456789012".to_string(), "us-east-1".to_string());
        let create_req = CreateProjectRequest::default();
        let _created = state.create_project(create_req).await.unwrap();
        let list_req = ListProjectsRequest::default();
        let listed = state.list_projects(list_req).await.unwrap();
        let _ = listed;
    }

    #[tokio::test]
    async fn test_project_full_crud() {
        let state = CodebuildState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateProjectRequest::default();
        create_req.project_name = "test-crud-resource".to_string();
        let create_result = state.create_project(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let mut get_req = DescribeProjectRequest::default();
        get_req.project_name = Some("test-crud-resource".to_string());
        let get_result = state.describe_project(get_req).await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let mut del_req = DeleteProjectRequest::default();
        del_req.project_name = Some("test-crud-resource".to_string());
        let del_result = state.delete_project(del_req).await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
