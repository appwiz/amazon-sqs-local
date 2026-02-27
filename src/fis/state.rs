use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::FISError;
use super::types::*;

#[allow(dead_code)]
struct FISStateInner {
    experiment_templates: HashMap<String, StoredExperimentTemplate>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredExperimentTemplate {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct FISState {
    inner: Arc<Mutex<FISStateInner>>,
}

impl FISState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        FISState {
            inner: Arc::new(Mutex::new(FISStateInner {
                experiment_templates: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_experiment_template(&self, req: CreateExperimentTemplateRequest) -> Result<ExperimentTemplateDetail, FISError> {
        let mut state = self.inner.lock().await;
        let name = req.name.or(req.name_pascal).unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        if state.experiment_templates.contains_key(&name) {
            return Err(FISError::ResourceAlreadyExistsException(format!("ExperimentTemplate {} already exists", name)));
        }
        let arn = format!("arn:aws:fis:{}:{}:experiment-templates/{}", state.region, state.account_id, name);
        let detail = ExperimentTemplateDetail {
            name: name.clone(),
            arn: arn.clone(),
            status: "ACTIVE".to_string(),
            tags: req.tags.clone(),
        };
        state.experiment_templates.insert(name, StoredExperimentTemplate {
            name: detail.name.clone(),
            arn,
            tags: req.tags.unwrap_or_default(),
        });
        Ok(detail)
    }

    #[allow(dead_code)]
    pub async fn get_experiment_template(&self, name: &str) -> Result<ExperimentTemplateDetail, FISError> {
        let state = self.inner.lock().await;
        let stored = state.experiment_templates.get(name)
            .ok_or_else(|| FISError::ResourceNotFoundException(format!("ExperimentTemplate {} not found", name)))?;
        Ok(ExperimentTemplateDetail {
            name: stored.name.clone(),
            arn: stored.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(stored.tags.clone()),
        })
    }

    #[allow(dead_code)]
    pub async fn list_experiment_templates(&self) -> Result<ListExperimentTemplatesResponse, FISError> {
        let state = self.inner.lock().await;
        let items: Vec<ExperimentTemplateDetail> = state.experiment_templates.values().map(|s| ExperimentTemplateDetail {
            name: s.name.clone(),
            arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListExperimentTemplatesResponse {
            items: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_experiment_template(&self, name: &str) -> Result<(), FISError> {
        let mut state = self.inner.lock().await;
        state.experiment_templates.remove(name)
            .ok_or_else(|| FISError::ResourceNotFoundException(format!("ExperimentTemplate {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = FISState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_experiment_template() {
        let state = FISState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateExperimentTemplateRequest::default();
        let result = state.create_experiment_template(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_get_experiment_template_not_found() {
        let state = FISState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.get_experiment_template("nonexistent").await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_experiment_templates() {
        let state = FISState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.list_experiment_templates().await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_experiment_template_not_found() {
        let state = FISState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.delete_experiment_template("nonexistent").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_experiment_template_full_crud() {
        let state = FISState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateExperimentTemplateRequest::default();
        create_req.name = Some("test-crud-resource".to_string());
        let create_result = state.create_experiment_template(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let get_result = state.get_experiment_template("test-crud-resource").await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let del_result = state.delete_experiment_template("test-crud-resource").await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
