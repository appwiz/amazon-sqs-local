use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::BedrockError;
use super::types::*;

#[allow(dead_code)]
struct BedrockStateInner {
    model_customization_jobs: HashMap<String, StoredModelCustomizationJob>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredModelCustomizationJob {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct BedrockState {
    inner: Arc<Mutex<BedrockStateInner>>,
}

impl BedrockState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        BedrockState {
            inner: Arc::new(Mutex::new(BedrockStateInner {
                model_customization_jobs: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_model_customization_job(&self, req: CreateModelCustomizationJobRequest) -> Result<ModelCustomizationJobDetail, BedrockError> {
        let mut state = self.inner.lock().await;
        let name = req.name.or(req.name_pascal).unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        if state.model_customization_jobs.contains_key(&name) {
            return Err(BedrockError::ResourceAlreadyExistsException(format!("ModelCustomizationJob {} already exists", name)));
        }
        let arn = format!("arn:aws:bedrock:{}:{}:model-customization-jobs/{}", state.region, state.account_id, name);
        let detail = ModelCustomizationJobDetail {
            name: name.clone(),
            arn: arn.clone(),
            status: "ACTIVE".to_string(),
            tags: req.tags.clone(),
        };
        state.model_customization_jobs.insert(name, StoredModelCustomizationJob {
            name: detail.name.clone(),
            arn,
            tags: req.tags.unwrap_or_default(),
        });
        Ok(detail)
    }

    #[allow(dead_code)]
    pub async fn get_model_customization_job(&self, name: &str) -> Result<ModelCustomizationJobDetail, BedrockError> {
        let state = self.inner.lock().await;
        let stored = state.model_customization_jobs.get(name)
            .ok_or_else(|| BedrockError::ResourceNotFoundException(format!("ModelCustomizationJob {} not found", name)))?;
        Ok(ModelCustomizationJobDetail {
            name: stored.name.clone(),
            arn: stored.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(stored.tags.clone()),
        })
    }

    #[allow(dead_code)]
    pub async fn list_model_customization_jobs(&self) -> Result<ListModelCustomizationJobsResponse, BedrockError> {
        let state = self.inner.lock().await;
        let items: Vec<ModelCustomizationJobDetail> = state.model_customization_jobs.values().map(|s| ModelCustomizationJobDetail {
            name: s.name.clone(),
            arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListModelCustomizationJobsResponse {
            items: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_model_customization_job(&self, name: &str) -> Result<(), BedrockError> {
        let mut state = self.inner.lock().await;
        state.model_customization_jobs.remove(name)
            .ok_or_else(|| BedrockError::ResourceNotFoundException(format!("ModelCustomizationJob {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = BedrockState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_model_customization_job() {
        let state = BedrockState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateModelCustomizationJobRequest::default();
        let result = state.create_model_customization_job(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_get_model_customization_job_not_found() {
        let state = BedrockState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.get_model_customization_job("nonexistent").await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_model_customization_jobs() {
        let state = BedrockState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.list_model_customization_jobs().await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_model_customization_job_not_found() {
        let state = BedrockState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.delete_model_customization_job("nonexistent").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_model_customization_job_full_crud() {
        let state = BedrockState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateModelCustomizationJobRequest::default();
        create_req.name = Some("test-crud-resource".to_string());
        let create_result = state.create_model_customization_job(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let get_result = state.get_model_customization_job("test-crud-resource").await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let del_result = state.delete_model_customization_job("test-crud-resource").await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
