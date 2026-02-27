use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::FrauddetectorError;
use super::types::*;

#[allow(dead_code)]
struct FrauddetectorStateInner {
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
pub struct FrauddetectorState {
    inner: Arc<Mutex<FrauddetectorStateInner>>,
}

impl FrauddetectorState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        FrauddetectorState {
            inner: Arc::new(Mutex::new(FrauddetectorStateInner {
                detectors: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_detector(&self, req: CreateDetectorRequest) -> Result<CreateDetectorResponse, FrauddetectorError> {
        let mut state = self.inner.lock().await;
        let name = req.detector_name.clone();
        if state.detectors.contains_key(&name) {
            return Err(FrauddetectorError::ResourceAlreadyExistsException(format!("Detector {} already exists", name)));
        }
        let arn = format!("arn:aws:frauddetector:{}:{}:detectors/{}", state.region, state.account_id, name);
        state.detectors.insert(name.clone(), StoredDetector {
            name: name.clone(),
            arn: arn.clone(),
            tags: req.tags.unwrap_or_default(),
        });
        Ok(CreateDetectorResponse {
            detector_arn: Some(arn),
            detector_name: Some(name),
        })
    }

    #[allow(dead_code)]
    pub async fn describe_detector(&self, req: DescribeDetectorRequest) -> Result<DescribeDetectorResponse, FrauddetectorError> {
        let state = self.inner.lock().await;
        let name = req.detector_name.or_else(|| req.detector_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| FrauddetectorError::ValidationException("Name or ARN required".to_string()))?;
        let stored = state.detectors.get(&name)
            .ok_or_else(|| FrauddetectorError::ResourceNotFoundException(format!("Detector {} not found", name)))?;
        Ok(DescribeDetectorResponse {
            detector: DetectorDetail {
                detector_name: stored.name.clone(),
                detector_arn: stored.arn.clone(),
                status: "ACTIVE".to_string(),
                tags: Some(stored.tags.clone()),
            },
        })
    }

    #[allow(dead_code)]
    pub async fn list_detectors(&self, _req: ListDetectorsRequest) -> Result<ListDetectorsResponse, FrauddetectorError> {
        let state = self.inner.lock().await;
        let items: Vec<DetectorDetail> = state.detectors.values().map(|s| DetectorDetail {
            detector_name: s.name.clone(),
            detector_arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListDetectorsResponse {
            detectors: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_detector(&self, req: DeleteDetectorRequest) -> Result<(), FrauddetectorError> {
        let mut state = self.inner.lock().await;
        let name = req.detector_name.or_else(|| req.detector_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| FrauddetectorError::ValidationException("Name or ARN required".to_string()))?;
        state.detectors.remove(&name)
            .ok_or_else(|| FrauddetectorError::ResourceNotFoundException(format!("Detector {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = FrauddetectorState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_detector() {
        let state = FrauddetectorState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateDetectorRequest::default();
        let result = state.create_detector(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_describe_detector_not_found() {
        let state = FrauddetectorState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DescribeDetectorRequest::default();
        let result = state.describe_detector(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_detectors_empty() {
        let state = FrauddetectorState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = ListDetectorsRequest::default();
        let result = state.list_detectors(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_detector_not_found() {
        let state = FrauddetectorState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DeleteDetectorRequest::default();
        let result = state.delete_detector(req).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_detector_create_and_list() {
        let state = FrauddetectorState::new("123456789012".to_string(), "us-east-1".to_string());
        let create_req = CreateDetectorRequest::default();
        let _created = state.create_detector(create_req).await.unwrap();
        let list_req = ListDetectorsRequest::default();
        let listed = state.list_detectors(list_req).await.unwrap();
        let _ = listed;
    }

    #[tokio::test]
    async fn test_detector_full_crud() {
        let state = FrauddetectorState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateDetectorRequest::default();
        create_req.detector_name = "test-crud-resource".to_string();
        let create_result = state.create_detector(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let mut get_req = DescribeDetectorRequest::default();
        get_req.detector_name = Some("test-crud-resource".to_string());
        let get_result = state.describe_detector(get_req).await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let mut del_req = DeleteDetectorRequest::default();
        del_req.detector_name = Some("test-crud-resource".to_string());
        let del_result = state.delete_detector(del_req).await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
