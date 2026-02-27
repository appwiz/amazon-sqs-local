use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::ImagebuilderError;
use super::types::*;

#[allow(dead_code)]
struct ImagebuilderStateInner {
    image_pipelines: HashMap<String, StoredImagePipeline>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredImagePipeline {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct ImagebuilderState {
    inner: Arc<Mutex<ImagebuilderStateInner>>,
}

impl ImagebuilderState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        ImagebuilderState {
            inner: Arc::new(Mutex::new(ImagebuilderStateInner {
                image_pipelines: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_image_pipeline(&self, req: CreateImagePipelineRequest) -> Result<ImagePipelineDetail, ImagebuilderError> {
        let mut state = self.inner.lock().await;
        let name = req.name.or(req.name_pascal).unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        if state.image_pipelines.contains_key(&name) {
            return Err(ImagebuilderError::ResourceAlreadyExistsException(format!("ImagePipeline {} already exists", name)));
        }
        let arn = format!("arn:aws:imagebuilder:{}:{}:image-pipelines/{}", state.region, state.account_id, name);
        let detail = ImagePipelineDetail {
            name: name.clone(),
            arn: arn.clone(),
            status: "ACTIVE".to_string(),
            tags: req.tags.clone(),
        };
        state.image_pipelines.insert(name, StoredImagePipeline {
            name: detail.name.clone(),
            arn,
            tags: req.tags.unwrap_or_default(),
        });
        Ok(detail)
    }

    #[allow(dead_code)]
    pub async fn get_image_pipeline(&self, name: &str) -> Result<ImagePipelineDetail, ImagebuilderError> {
        let state = self.inner.lock().await;
        let stored = state.image_pipelines.get(name)
            .ok_or_else(|| ImagebuilderError::ResourceNotFoundException(format!("ImagePipeline {} not found", name)))?;
        Ok(ImagePipelineDetail {
            name: stored.name.clone(),
            arn: stored.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(stored.tags.clone()),
        })
    }

    #[allow(dead_code)]
    pub async fn list_image_pipelines(&self) -> Result<ListImagePipelinesResponse, ImagebuilderError> {
        let state = self.inner.lock().await;
        let items: Vec<ImagePipelineDetail> = state.image_pipelines.values().map(|s| ImagePipelineDetail {
            name: s.name.clone(),
            arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListImagePipelinesResponse {
            items: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_image_pipeline(&self, name: &str) -> Result<(), ImagebuilderError> {
        let mut state = self.inner.lock().await;
        state.image_pipelines.remove(name)
            .ok_or_else(|| ImagebuilderError::ResourceNotFoundException(format!("ImagePipeline {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = ImagebuilderState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_image_pipeline() {
        let state = ImagebuilderState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateImagePipelineRequest::default();
        let result = state.create_image_pipeline(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_get_image_pipeline_not_found() {
        let state = ImagebuilderState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.get_image_pipeline("nonexistent").await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_image_pipelines() {
        let state = ImagebuilderState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.list_image_pipelines().await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_image_pipeline_not_found() {
        let state = ImagebuilderState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.delete_image_pipeline("nonexistent").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_image_pipeline_full_crud() {
        let state = ImagebuilderState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateImagePipelineRequest::default();
        create_req.name = Some("test-crud-resource".to_string());
        let create_result = state.create_image_pipeline(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let get_result = state.get_image_pipeline("test-crud-resource").await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let del_result = state.delete_image_pipeline("test-crud-resource").await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
