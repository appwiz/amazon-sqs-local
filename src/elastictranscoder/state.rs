use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::ElastictranscoderError;
use super::types::*;

#[allow(dead_code)]
struct ElastictranscoderStateInner {
    pipelines: HashMap<String, StoredPipeline>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredPipeline {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct ElastictranscoderState {
    inner: Arc<Mutex<ElastictranscoderStateInner>>,
}

impl ElastictranscoderState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        ElastictranscoderState {
            inner: Arc::new(Mutex::new(ElastictranscoderStateInner {
                pipelines: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_pipeline(&self, req: CreatePipelineRequest) -> Result<PipelineDetail, ElastictranscoderError> {
        let mut state = self.inner.lock().await;
        let name = req.name.or(req.name_pascal).unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        if state.pipelines.contains_key(&name) {
            return Err(ElastictranscoderError::ResourceAlreadyExistsException(format!("Pipeline {} already exists", name)));
        }
        let arn = format!("arn:aws:elastictranscoder:{}:{}:pipelines/{}", state.region, state.account_id, name);
        let detail = PipelineDetail {
            name: name.clone(),
            arn: arn.clone(),
            status: "ACTIVE".to_string(),
            tags: req.tags.clone(),
        };
        state.pipelines.insert(name, StoredPipeline {
            name: detail.name.clone(),
            arn,
            tags: req.tags.unwrap_or_default(),
        });
        Ok(detail)
    }

    #[allow(dead_code)]
    pub async fn get_pipeline(&self, name: &str) -> Result<PipelineDetail, ElastictranscoderError> {
        let state = self.inner.lock().await;
        let stored = state.pipelines.get(name)
            .ok_or_else(|| ElastictranscoderError::ResourceNotFoundException(format!("Pipeline {} not found", name)))?;
        Ok(PipelineDetail {
            name: stored.name.clone(),
            arn: stored.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(stored.tags.clone()),
        })
    }

    #[allow(dead_code)]
    pub async fn list_pipelines(&self) -> Result<ListPipelinesResponse, ElastictranscoderError> {
        let state = self.inner.lock().await;
        let items: Vec<PipelineDetail> = state.pipelines.values().map(|s| PipelineDetail {
            name: s.name.clone(),
            arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListPipelinesResponse {
            items: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_pipeline(&self, name: &str) -> Result<(), ElastictranscoderError> {
        let mut state = self.inner.lock().await;
        state.pipelines.remove(name)
            .ok_or_else(|| ElastictranscoderError::ResourceNotFoundException(format!("Pipeline {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = ElastictranscoderState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_pipeline() {
        let state = ElastictranscoderState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreatePipelineRequest::default();
        let result = state.create_pipeline(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_get_pipeline_not_found() {
        let state = ElastictranscoderState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.get_pipeline("nonexistent").await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_pipelines() {
        let state = ElastictranscoderState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.list_pipelines().await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_pipeline_not_found() {
        let state = ElastictranscoderState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.delete_pipeline("nonexistent").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_pipeline_full_crud() {
        let state = ElastictranscoderState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreatePipelineRequest::default();
        create_req.name = Some("test-crud-resource".to_string());
        let create_result = state.create_pipeline(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let get_result = state.get_pipeline("test-crud-resource").await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let del_result = state.delete_pipeline("test-crud-resource").await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
