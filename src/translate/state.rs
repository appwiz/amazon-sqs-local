use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::TranslateError;
use super::types::*;

#[allow(dead_code)]
struct TranslateStateInner {
    terminologys: HashMap<String, StoredTerminology>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredTerminology {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct TranslateState {
    inner: Arc<Mutex<TranslateStateInner>>,
}

impl TranslateState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        TranslateState {
            inner: Arc::new(Mutex::new(TranslateStateInner {
                terminologys: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_terminology(&self, req: CreateTerminologyRequest) -> Result<CreateTerminologyResponse, TranslateError> {
        let mut state = self.inner.lock().await;
        let name = req.terminology_name.clone();
        if state.terminologys.contains_key(&name) {
            return Err(TranslateError::ResourceAlreadyExistsException(format!("Terminology {} already exists", name)));
        }
        let arn = format!("arn:aws:translate:{}:{}:terminologies/{}", state.region, state.account_id, name);
        state.terminologys.insert(name.clone(), StoredTerminology {
            name: name.clone(),
            arn: arn.clone(),
            tags: req.tags.unwrap_or_default(),
        });
        Ok(CreateTerminologyResponse {
            terminology_arn: Some(arn),
            terminology_name: Some(name),
        })
    }

    #[allow(dead_code)]
    pub async fn describe_terminology(&self, req: DescribeTerminologyRequest) -> Result<DescribeTerminologyResponse, TranslateError> {
        let state = self.inner.lock().await;
        let name = req.terminology_name.or_else(|| req.terminology_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| TranslateError::ValidationException("Name or ARN required".to_string()))?;
        let stored = state.terminologys.get(&name)
            .ok_or_else(|| TranslateError::ResourceNotFoundException(format!("Terminology {} not found", name)))?;
        Ok(DescribeTerminologyResponse {
            terminology: TerminologyDetail {
                terminology_name: stored.name.clone(),
                terminology_arn: stored.arn.clone(),
                status: "ACTIVE".to_string(),
                tags: Some(stored.tags.clone()),
            },
        })
    }

    #[allow(dead_code)]
    pub async fn list_terminologys(&self, _req: ListTerminologysRequest) -> Result<ListTerminologysResponse, TranslateError> {
        let state = self.inner.lock().await;
        let items: Vec<TerminologyDetail> = state.terminologys.values().map(|s| TerminologyDetail {
            terminology_name: s.name.clone(),
            terminology_arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListTerminologysResponse {
            terminologys: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_terminology(&self, req: DeleteTerminologyRequest) -> Result<(), TranslateError> {
        let mut state = self.inner.lock().await;
        let name = req.terminology_name.or_else(|| req.terminology_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| TranslateError::ValidationException("Name or ARN required".to_string()))?;
        state.terminologys.remove(&name)
            .ok_or_else(|| TranslateError::ResourceNotFoundException(format!("Terminology {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = TranslateState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_terminology() {
        let state = TranslateState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateTerminologyRequest::default();
        let result = state.create_terminology(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_describe_terminology_not_found() {
        let state = TranslateState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DescribeTerminologyRequest::default();
        let result = state.describe_terminology(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_terminologys_empty() {
        let state = TranslateState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = ListTerminologysRequest::default();
        let result = state.list_terminologys(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_terminology_not_found() {
        let state = TranslateState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DeleteTerminologyRequest::default();
        let result = state.delete_terminology(req).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_terminology_create_and_list() {
        let state = TranslateState::new("123456789012".to_string(), "us-east-1".to_string());
        let create_req = CreateTerminologyRequest::default();
        let _created = state.create_terminology(create_req).await.unwrap();
        let list_req = ListTerminologysRequest::default();
        let listed = state.list_terminologys(list_req).await.unwrap();
        let _ = listed;
    }

    #[tokio::test]
    async fn test_terminology_full_crud() {
        let state = TranslateState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateTerminologyRequest::default();
        create_req.terminology_name = "test-crud-resource".to_string();
        let create_result = state.create_terminology(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let mut get_req = DescribeTerminologyRequest::default();
        get_req.terminology_name = Some("test-crud-resource".to_string());
        let get_result = state.describe_terminology(get_req).await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let mut del_req = DeleteTerminologyRequest::default();
        del_req.terminology_name = Some("test-crud-resource".to_string());
        let del_result = state.delete_terminology(del_req).await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
