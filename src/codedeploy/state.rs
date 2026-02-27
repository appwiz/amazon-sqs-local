use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::CodedeployError;
use super::types::*;

#[allow(dead_code)]
struct CodedeployStateInner {
    applications: HashMap<String, StoredApplication>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredApplication {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct CodedeployState {
    inner: Arc<Mutex<CodedeployStateInner>>,
}

impl CodedeployState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        CodedeployState {
            inner: Arc::new(Mutex::new(CodedeployStateInner {
                applications: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_application(&self, req: CreateApplicationRequest) -> Result<CreateApplicationResponse, CodedeployError> {
        let mut state = self.inner.lock().await;
        let name = req.application_name.clone();
        if state.applications.contains_key(&name) {
            return Err(CodedeployError::ResourceAlreadyExistsException(format!("Application {} already exists", name)));
        }
        let arn = format!("arn:aws:codedeploy:{}:{}:applications/{}", state.region, state.account_id, name);
        state.applications.insert(name.clone(), StoredApplication {
            name: name.clone(),
            arn: arn.clone(),
            tags: req.tags.unwrap_or_default(),
        });
        Ok(CreateApplicationResponse {
            application_arn: Some(arn),
            application_name: Some(name),
        })
    }

    #[allow(dead_code)]
    pub async fn describe_application(&self, req: DescribeApplicationRequest) -> Result<DescribeApplicationResponse, CodedeployError> {
        let state = self.inner.lock().await;
        let name = req.application_name.or_else(|| req.application_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| CodedeployError::ValidationException("Name or ARN required".to_string()))?;
        let stored = state.applications.get(&name)
            .ok_or_else(|| CodedeployError::ResourceNotFoundException(format!("Application {} not found", name)))?;
        Ok(DescribeApplicationResponse {
            application: ApplicationDetail {
                application_name: stored.name.clone(),
                application_arn: stored.arn.clone(),
                status: "ACTIVE".to_string(),
                tags: Some(stored.tags.clone()),
            },
        })
    }

    #[allow(dead_code)]
    pub async fn list_applications(&self, _req: ListApplicationsRequest) -> Result<ListApplicationsResponse, CodedeployError> {
        let state = self.inner.lock().await;
        let items: Vec<ApplicationDetail> = state.applications.values().map(|s| ApplicationDetail {
            application_name: s.name.clone(),
            application_arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListApplicationsResponse {
            applications: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_application(&self, req: DeleteApplicationRequest) -> Result<(), CodedeployError> {
        let mut state = self.inner.lock().await;
        let name = req.application_name.or_else(|| req.application_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| CodedeployError::ValidationException("Name or ARN required".to_string()))?;
        state.applications.remove(&name)
            .ok_or_else(|| CodedeployError::ResourceNotFoundException(format!("Application {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = CodedeployState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_application() {
        let state = CodedeployState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateApplicationRequest::default();
        let result = state.create_application(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_describe_application_not_found() {
        let state = CodedeployState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DescribeApplicationRequest::default();
        let result = state.describe_application(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_applications_empty() {
        let state = CodedeployState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = ListApplicationsRequest::default();
        let result = state.list_applications(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_application_not_found() {
        let state = CodedeployState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DeleteApplicationRequest::default();
        let result = state.delete_application(req).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_application_create_and_list() {
        let state = CodedeployState::new("123456789012".to_string(), "us-east-1".to_string());
        let create_req = CreateApplicationRequest::default();
        let _created = state.create_application(create_req).await.unwrap();
        let list_req = ListApplicationsRequest::default();
        let listed = state.list_applications(list_req).await.unwrap();
        let _ = listed;
    }

    #[tokio::test]
    async fn test_application_full_crud() {
        let state = CodedeployState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateApplicationRequest::default();
        create_req.application_name = "test-crud-resource".to_string();
        let create_result = state.create_application(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let mut get_req = DescribeApplicationRequest::default();
        get_req.application_name = Some("test-crud-resource".to_string());
        let get_result = state.describe_application(get_req).await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let mut del_req = DeleteApplicationRequest::default();
        del_req.application_name = Some("test-crud-resource".to_string());
        let del_result = state.delete_application(del_req).await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
