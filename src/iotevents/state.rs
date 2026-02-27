use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::IoteventsError;
use super::types::*;

#[allow(dead_code)]
struct IoteventsStateInner {
    detector_models: HashMap<String, StoredDetectorModel>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredDetectorModel {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct IoteventsState {
    inner: Arc<Mutex<IoteventsStateInner>>,
}

impl IoteventsState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        IoteventsState {
            inner: Arc::new(Mutex::new(IoteventsStateInner {
                detector_models: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_detector_model(&self, req: CreateDetectorModelRequest) -> Result<DetectorModelDetail, IoteventsError> {
        let mut state = self.inner.lock().await;
        let name = req.name.or(req.name_pascal).unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        if state.detector_models.contains_key(&name) {
            return Err(IoteventsError::ResourceAlreadyExistsException(format!("DetectorModel {} already exists", name)));
        }
        let arn = format!("arn:aws:iotevents:{}:{}:detector-models/{}", state.region, state.account_id, name);
        let detail = DetectorModelDetail {
            name: name.clone(),
            arn: arn.clone(),
            status: "ACTIVE".to_string(),
            tags: req.tags.clone(),
        };
        state.detector_models.insert(name, StoredDetectorModel {
            name: detail.name.clone(),
            arn,
            tags: req.tags.unwrap_or_default(),
        });
        Ok(detail)
    }

    #[allow(dead_code)]
    pub async fn get_detector_model(&self, name: &str) -> Result<DetectorModelDetail, IoteventsError> {
        let state = self.inner.lock().await;
        let stored = state.detector_models.get(name)
            .ok_or_else(|| IoteventsError::ResourceNotFoundException(format!("DetectorModel {} not found", name)))?;
        Ok(DetectorModelDetail {
            name: stored.name.clone(),
            arn: stored.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(stored.tags.clone()),
        })
    }

    #[allow(dead_code)]
    pub async fn list_detector_models(&self) -> Result<ListDetectorModelsResponse, IoteventsError> {
        let state = self.inner.lock().await;
        let items: Vec<DetectorModelDetail> = state.detector_models.values().map(|s| DetectorModelDetail {
            name: s.name.clone(),
            arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListDetectorModelsResponse {
            items: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_detector_model(&self, name: &str) -> Result<(), IoteventsError> {
        let mut state = self.inner.lock().await;
        state.detector_models.remove(name)
            .ok_or_else(|| IoteventsError::ResourceNotFoundException(format!("DetectorModel {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = IoteventsState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_detector_model() {
        let state = IoteventsState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateDetectorModelRequest::default();
        let result = state.create_detector_model(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_get_detector_model_not_found() {
        let state = IoteventsState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.get_detector_model("nonexistent").await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_detector_models() {
        let state = IoteventsState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.list_detector_models().await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_detector_model_not_found() {
        let state = IoteventsState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.delete_detector_model("nonexistent").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_detector_model_full_crud() {
        let state = IoteventsState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateDetectorModelRequest::default();
        create_req.name = Some("test-crud-resource".to_string());
        let create_result = state.create_detector_model(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let get_result = state.get_detector_model("test-crud-resource").await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let del_result = state.delete_detector_model("test-crud-resource").await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
