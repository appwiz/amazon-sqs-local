use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::GuarddutyError;
use super::types::*;

#[allow(dead_code)]
struct GuarddutyStateInner {
    detectors: HashMap<String, StoredDetector>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredDetector {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct GuarddutyState {
    inner: Arc<Mutex<GuarddutyStateInner>>,
}

impl GuarddutyState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        GuarddutyState {
            inner: Arc::new(Mutex::new(GuarddutyStateInner {
                detectors: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_detector(&self, req: CreateDetectorRequest) -> Result<DetectorDetail, GuarddutyError> {
        let mut state = self.inner.lock().await;
        let name = req.name.or(req.name_pascal).unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        if state.detectors.contains_key(&name) {
            return Err(GuarddutyError::ResourceAlreadyExistsException(format!("Detector {} already exists", name)));
        }
        let arn = format!("arn:aws:guardduty:{}:{}:detectors/{}", state.region, state.account_id, name);
        let detail = DetectorDetail {
            name: name.clone(),
            arn: arn.clone(),
            status: "ACTIVE".to_string(),
            tags: req.tags.clone(),
        };
        state.detectors.insert(name, StoredDetector {
            name: detail.name.clone(),
            arn,
            tags: req.tags.unwrap_or_default(),
        });
        Ok(detail)
    }

    #[allow(dead_code)]
    pub async fn get_detector(&self, name: &str) -> Result<DetectorDetail, GuarddutyError> {
        let state = self.inner.lock().await;
        let stored = state.detectors.get(name)
            .ok_or_else(|| GuarddutyError::ResourceNotFoundException(format!("Detector {} not found", name)))?;
        Ok(DetectorDetail {
            name: stored.name.clone(),
            arn: stored.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(stored.tags.clone()),
        })
    }

    #[allow(dead_code)]
    pub async fn list_detectors(&self) -> Result<ListDetectorsResponse, GuarddutyError> {
        let state = self.inner.lock().await;
        let items: Vec<DetectorDetail> = state.detectors.values().map(|s| DetectorDetail {
            name: s.name.clone(),
            arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListDetectorsResponse {
            items: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_detector(&self, name: &str) -> Result<(), GuarddutyError> {
        let mut state = self.inner.lock().await;
        state.detectors.remove(name)
            .ok_or_else(|| GuarddutyError::ResourceNotFoundException(format!("Detector {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = GuarddutyState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_detector() {
        let state = GuarddutyState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateDetectorRequest::default();
        let result = state.create_detector(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_get_detector_not_found() {
        let state = GuarddutyState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.get_detector("nonexistent").await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_detectors() {
        let state = GuarddutyState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.list_detectors().await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_detector_not_found() {
        let state = GuarddutyState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.delete_detector("nonexistent").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_detector_full_crud() {
        let state = GuarddutyState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateDetectorRequest::default();
        create_req.name = Some("test-crud-resource".to_string());
        let create_result = state.create_detector(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let get_result = state.get_detector("test-crud-resource").await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let del_result = state.delete_detector("test-crud-resource").await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
