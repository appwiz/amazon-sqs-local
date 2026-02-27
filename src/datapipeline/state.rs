use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::DatapipelineError;
use super::types::*;

#[allow(dead_code)]
struct DatapipelineStateInner {
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
pub struct DatapipelineState {
    inner: Arc<Mutex<DatapipelineStateInner>>,
}

impl DatapipelineState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        DatapipelineState {
            inner: Arc::new(Mutex::new(DatapipelineStateInner {
                pipelines: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_pipeline(&self, req: CreatePipelineRequest) -> Result<CreatePipelineResponse, DatapipelineError> {
        let mut state = self.inner.lock().await;
        let name = req.pipeline_name.clone();
        if state.pipelines.contains_key(&name) {
            return Err(DatapipelineError::ResourceAlreadyExistsException(format!("Pipeline {} already exists", name)));
        }
        let arn = format!("arn:aws:datapipeline:{}:{}:pipelines/{}", state.region, state.account_id, name);
        state.pipelines.insert(name.clone(), StoredPipeline {
            name: name.clone(),
            arn: arn.clone(),
            tags: req.tags.unwrap_or_default(),
        });
        Ok(CreatePipelineResponse {
            pipeline_arn: Some(arn),
            pipeline_name: Some(name),
        })
    }

    #[allow(dead_code)]
    pub async fn describe_pipeline(&self, req: DescribePipelineRequest) -> Result<DescribePipelineResponse, DatapipelineError> {
        let state = self.inner.lock().await;
        let name = req.pipeline_name.or_else(|| req.pipeline_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| DatapipelineError::ValidationException("Name or ARN required".to_string()))?;
        let stored = state.pipelines.get(&name)
            .ok_or_else(|| DatapipelineError::ResourceNotFoundException(format!("Pipeline {} not found", name)))?;
        Ok(DescribePipelineResponse {
            pipeline: PipelineDetail {
                pipeline_name: stored.name.clone(),
                pipeline_arn: stored.arn.clone(),
                status: "ACTIVE".to_string(),
                tags: Some(stored.tags.clone()),
            },
        })
    }

    #[allow(dead_code)]
    pub async fn list_pipelines(&self, _req: ListPipelinesRequest) -> Result<ListPipelinesResponse, DatapipelineError> {
        let state = self.inner.lock().await;
        let items: Vec<PipelineDetail> = state.pipelines.values().map(|s| PipelineDetail {
            pipeline_name: s.name.clone(),
            pipeline_arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListPipelinesResponse {
            pipelines: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_pipeline(&self, req: DeletePipelineRequest) -> Result<(), DatapipelineError> {
        let mut state = self.inner.lock().await;
        let name = req.pipeline_name.or_else(|| req.pipeline_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| DatapipelineError::ValidationException("Name or ARN required".to_string()))?;
        state.pipelines.remove(&name)
            .ok_or_else(|| DatapipelineError::ResourceNotFoundException(format!("Pipeline {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = DatapipelineState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_pipeline() {
        let state = DatapipelineState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreatePipelineRequest::default();
        let result = state.create_pipeline(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_describe_pipeline_not_found() {
        let state = DatapipelineState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DescribePipelineRequest::default();
        let result = state.describe_pipeline(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_pipelines_empty() {
        let state = DatapipelineState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = ListPipelinesRequest::default();
        let result = state.list_pipelines(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_pipeline_not_found() {
        let state = DatapipelineState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DeletePipelineRequest::default();
        let result = state.delete_pipeline(req).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_pipeline_create_and_list() {
        let state = DatapipelineState::new("123456789012".to_string(), "us-east-1".to_string());
        let create_req = CreatePipelineRequest::default();
        let _created = state.create_pipeline(create_req).await.unwrap();
        let list_req = ListPipelinesRequest::default();
        let listed = state.list_pipelines(list_req).await.unwrap();
        let _ = listed;
    }

    #[tokio::test]
    async fn test_pipeline_full_crud() {
        let state = DatapipelineState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreatePipelineRequest::default();
        create_req.pipeline_name = "test-crud-resource".to_string();
        let create_result = state.create_pipeline(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let mut get_req = DescribePipelineRequest::default();
        get_req.pipeline_name = Some("test-crud-resource".to_string());
        let get_result = state.describe_pipeline(get_req).await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let mut del_req = DeletePipelineRequest::default();
        del_req.pipeline_name = Some("test-crud-resource".to_string());
        let del_result = state.delete_pipeline(del_req).await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
