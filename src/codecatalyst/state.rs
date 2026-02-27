use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::CodecatalystError;
use super::types::*;

#[allow(dead_code)]
struct CodecatalystStateInner {
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
pub struct CodecatalystState {
    inner: Arc<Mutex<CodecatalystStateInner>>,
}

impl CodecatalystState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        CodecatalystState {
            inner: Arc::new(Mutex::new(CodecatalystStateInner {
                projects: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_project(&self, req: CreateProjectRequest) -> Result<ProjectDetail, CodecatalystError> {
        let mut state = self.inner.lock().await;
        let name = req.name.or(req.name_pascal).unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        if state.projects.contains_key(&name) {
            return Err(CodecatalystError::ResourceAlreadyExistsException(format!("Project {} already exists", name)));
        }
        let arn = format!("arn:aws:codecatalyst:{}:{}:projects/{}", state.region, state.account_id, name);
        let detail = ProjectDetail {
            name: name.clone(),
            arn: arn.clone(),
            status: "ACTIVE".to_string(),
            tags: req.tags.clone(),
        };
        state.projects.insert(name, StoredProject {
            name: detail.name.clone(),
            arn,
            tags: req.tags.unwrap_or_default(),
        });
        Ok(detail)
    }

    #[allow(dead_code)]
    pub async fn get_project(&self, name: &str) -> Result<ProjectDetail, CodecatalystError> {
        let state = self.inner.lock().await;
        let stored = state.projects.get(name)
            .ok_or_else(|| CodecatalystError::ResourceNotFoundException(format!("Project {} not found", name)))?;
        Ok(ProjectDetail {
            name: stored.name.clone(),
            arn: stored.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(stored.tags.clone()),
        })
    }

    #[allow(dead_code)]
    pub async fn list_projects(&self) -> Result<ListProjectsResponse, CodecatalystError> {
        let state = self.inner.lock().await;
        let items: Vec<ProjectDetail> = state.projects.values().map(|s| ProjectDetail {
            name: s.name.clone(),
            arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListProjectsResponse {
            items: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_project(&self, name: &str) -> Result<(), CodecatalystError> {
        let mut state = self.inner.lock().await;
        state.projects.remove(name)
            .ok_or_else(|| CodecatalystError::ResourceNotFoundException(format!("Project {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = CodecatalystState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_project() {
        let state = CodecatalystState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateProjectRequest::default();
        let result = state.create_project(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_get_project_not_found() {
        let state = CodecatalystState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.get_project("nonexistent").await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_projects() {
        let state = CodecatalystState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.list_projects().await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_project_not_found() {
        let state = CodecatalystState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.delete_project("nonexistent").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_project_full_crud() {
        let state = CodecatalystState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateProjectRequest::default();
        create_req.name = Some("test-crud-resource".to_string());
        let create_result = state.create_project(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let get_result = state.get_project("test-crud-resource").await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let del_result = state.delete_project("test-crud-resource").await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
