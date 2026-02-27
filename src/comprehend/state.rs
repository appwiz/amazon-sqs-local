use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::ComprehendError;
use super::types::*;

#[allow(dead_code)]
struct ComprehendStateInner {
    document_classifiers: HashMap<String, StoredDocumentClassifier>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredDocumentClassifier {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct ComprehendState {
    inner: Arc<Mutex<ComprehendStateInner>>,
}

impl ComprehendState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        ComprehendState {
            inner: Arc::new(Mutex::new(ComprehendStateInner {
                document_classifiers: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_document_classifier(&self, req: CreateDocumentClassifierRequest) -> Result<CreateDocumentClassifierResponse, ComprehendError> {
        let mut state = self.inner.lock().await;
        let name = req.document_classifier_name.clone();
        if state.document_classifiers.contains_key(&name) {
            return Err(ComprehendError::ResourceAlreadyExistsException(format!("DocumentClassifier {} already exists", name)));
        }
        let arn = format!("arn:aws:comprehend:{}:{}:document-classifiers/{}", state.region, state.account_id, name);
        state.document_classifiers.insert(name.clone(), StoredDocumentClassifier {
            name: name.clone(),
            arn: arn.clone(),
            tags: req.tags.unwrap_or_default(),
        });
        Ok(CreateDocumentClassifierResponse {
            document_classifier_arn: Some(arn),
            document_classifier_name: Some(name),
        })
    }

    #[allow(dead_code)]
    pub async fn describe_document_classifier(&self, req: DescribeDocumentClassifierRequest) -> Result<DescribeDocumentClassifierResponse, ComprehendError> {
        let state = self.inner.lock().await;
        let name = req.document_classifier_name.or_else(|| req.document_classifier_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| ComprehendError::ValidationException("Name or ARN required".to_string()))?;
        let stored = state.document_classifiers.get(&name)
            .ok_or_else(|| ComprehendError::ResourceNotFoundException(format!("DocumentClassifier {} not found", name)))?;
        Ok(DescribeDocumentClassifierResponse {
            document_classifier: DocumentClassifierDetail {
                document_classifier_name: stored.name.clone(),
                document_classifier_arn: stored.arn.clone(),
                status: "ACTIVE".to_string(),
                tags: Some(stored.tags.clone()),
            },
        })
    }

    #[allow(dead_code)]
    pub async fn list_document_classifiers(&self, _req: ListDocumentClassifiersRequest) -> Result<ListDocumentClassifiersResponse, ComprehendError> {
        let state = self.inner.lock().await;
        let items: Vec<DocumentClassifierDetail> = state.document_classifiers.values().map(|s| DocumentClassifierDetail {
            document_classifier_name: s.name.clone(),
            document_classifier_arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListDocumentClassifiersResponse {
            document_classifiers: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_document_classifier(&self, req: DeleteDocumentClassifierRequest) -> Result<(), ComprehendError> {
        let mut state = self.inner.lock().await;
        let name = req.document_classifier_name.or_else(|| req.document_classifier_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| ComprehendError::ValidationException("Name or ARN required".to_string()))?;
        state.document_classifiers.remove(&name)
            .ok_or_else(|| ComprehendError::ResourceNotFoundException(format!("DocumentClassifier {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = ComprehendState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_document_classifier() {
        let state = ComprehendState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateDocumentClassifierRequest::default();
        let result = state.create_document_classifier(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_describe_document_classifier_not_found() {
        let state = ComprehendState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DescribeDocumentClassifierRequest::default();
        let result = state.describe_document_classifier(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_document_classifiers_empty() {
        let state = ComprehendState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = ListDocumentClassifiersRequest::default();
        let result = state.list_document_classifiers(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_document_classifier_not_found() {
        let state = ComprehendState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DeleteDocumentClassifierRequest::default();
        let result = state.delete_document_classifier(req).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_document_classifier_create_and_list() {
        let state = ComprehendState::new("123456789012".to_string(), "us-east-1".to_string());
        let create_req = CreateDocumentClassifierRequest::default();
        let _created = state.create_document_classifier(create_req).await.unwrap();
        let list_req = ListDocumentClassifiersRequest::default();
        let listed = state.list_document_classifiers(list_req).await.unwrap();
        let _ = listed;
    }

    #[tokio::test]
    async fn test_document_classifier_full_crud() {
        let state = ComprehendState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateDocumentClassifierRequest::default();
        create_req.document_classifier_name = "test-crud-resource".to_string();
        let create_result = state.create_document_classifier(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let mut get_req = DescribeDocumentClassifierRequest::default();
        get_req.document_classifier_name = Some("test-crud-resource".to_string());
        let get_result = state.describe_document_classifier(get_req).await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let mut del_req = DeleteDocumentClassifierRequest::default();
        del_req.document_classifier_name = Some("test-crud-resource".to_string());
        let del_result = state.delete_document_classifier(del_req).await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
