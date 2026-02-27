use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::ProtonError;
use super::types::*;

#[allow(dead_code)]
struct ProtonStateInner {
    environment_templates: HashMap<String, StoredEnvironmentTemplate>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredEnvironmentTemplate {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct ProtonState {
    inner: Arc<Mutex<ProtonStateInner>>,
}

impl ProtonState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        ProtonState {
            inner: Arc::new(Mutex::new(ProtonStateInner {
                environment_templates: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_environment_template(&self, req: CreateEnvironmentTemplateRequest) -> Result<CreateEnvironmentTemplateResponse, ProtonError> {
        let mut state = self.inner.lock().await;
        let name = req.environment_template_name.clone();
        if state.environment_templates.contains_key(&name) {
            return Err(ProtonError::ResourceAlreadyExistsException(format!("EnvironmentTemplate {} already exists", name)));
        }
        let arn = format!("arn:aws:proton:{}:{}:environment-templates/{}", state.region, state.account_id, name);
        state.environment_templates.insert(name.clone(), StoredEnvironmentTemplate {
            name: name.clone(),
            arn: arn.clone(),
            tags: req.tags.unwrap_or_default(),
        });
        Ok(CreateEnvironmentTemplateResponse {
            environment_template_arn: Some(arn),
            environment_template_name: Some(name),
        })
    }

    #[allow(dead_code)]
    pub async fn describe_environment_template(&self, req: DescribeEnvironmentTemplateRequest) -> Result<DescribeEnvironmentTemplateResponse, ProtonError> {
        let state = self.inner.lock().await;
        let name = req.environment_template_name.or_else(|| req.environment_template_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| ProtonError::ValidationException("Name or ARN required".to_string()))?;
        let stored = state.environment_templates.get(&name)
            .ok_or_else(|| ProtonError::ResourceNotFoundException(format!("EnvironmentTemplate {} not found", name)))?;
        Ok(DescribeEnvironmentTemplateResponse {
            environment_template: EnvironmentTemplateDetail {
                environment_template_name: stored.name.clone(),
                environment_template_arn: stored.arn.clone(),
                status: "ACTIVE".to_string(),
                tags: Some(stored.tags.clone()),
            },
        })
    }

    #[allow(dead_code)]
    pub async fn list_environment_templates(&self, _req: ListEnvironmentTemplatesRequest) -> Result<ListEnvironmentTemplatesResponse, ProtonError> {
        let state = self.inner.lock().await;
        let items: Vec<EnvironmentTemplateDetail> = state.environment_templates.values().map(|s| EnvironmentTemplateDetail {
            environment_template_name: s.name.clone(),
            environment_template_arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListEnvironmentTemplatesResponse {
            environment_templates: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_environment_template(&self, req: DeleteEnvironmentTemplateRequest) -> Result<(), ProtonError> {
        let mut state = self.inner.lock().await;
        let name = req.environment_template_name.or_else(|| req.environment_template_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| ProtonError::ValidationException("Name or ARN required".to_string()))?;
        state.environment_templates.remove(&name)
            .ok_or_else(|| ProtonError::ResourceNotFoundException(format!("EnvironmentTemplate {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = ProtonState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_environment_template() {
        let state = ProtonState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateEnvironmentTemplateRequest::default();
        let result = state.create_environment_template(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_describe_environment_template_not_found() {
        let state = ProtonState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DescribeEnvironmentTemplateRequest::default();
        let result = state.describe_environment_template(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_environment_templates_empty() {
        let state = ProtonState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = ListEnvironmentTemplatesRequest::default();
        let result = state.list_environment_templates(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_environment_template_not_found() {
        let state = ProtonState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DeleteEnvironmentTemplateRequest::default();
        let result = state.delete_environment_template(req).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_environment_template_create_and_list() {
        let state = ProtonState::new("123456789012".to_string(), "us-east-1".to_string());
        let create_req = CreateEnvironmentTemplateRequest::default();
        let _created = state.create_environment_template(create_req).await.unwrap();
        let list_req = ListEnvironmentTemplatesRequest::default();
        let listed = state.list_environment_templates(list_req).await.unwrap();
        let _ = listed;
    }

    #[tokio::test]
    async fn test_environment_template_full_crud() {
        let state = ProtonState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateEnvironmentTemplateRequest::default();
        create_req.environment_template_name = "test-crud-resource".to_string();
        let create_result = state.create_environment_template(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let mut get_req = DescribeEnvironmentTemplateRequest::default();
        get_req.environment_template_name = Some("test-crud-resource".to_string());
        let get_result = state.describe_environment_template(get_req).await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let mut del_req = DeleteEnvironmentTemplateRequest::default();
        del_req.environment_template_name = Some("test-crud-resource".to_string());
        let del_result = state.delete_environment_template(del_req).await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
