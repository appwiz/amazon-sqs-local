use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::InspectorError;
use super::types::*;

#[allow(dead_code)]
struct InspectorStateInner {
    findings: HashMap<String, StoredFinding>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredFinding {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct InspectorState {
    inner: Arc<Mutex<InspectorStateInner>>,
}

impl InspectorState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        InspectorState {
            inner: Arc::new(Mutex::new(InspectorStateInner {
                findings: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_finding(&self, req: CreateFindingRequest) -> Result<FindingDetail, InspectorError> {
        let mut state = self.inner.lock().await;
        let name = req.name.or(req.name_pascal).unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        if state.findings.contains_key(&name) {
            return Err(InspectorError::ResourceAlreadyExistsException(format!("Finding {} already exists", name)));
        }
        let arn = format!("arn:aws:inspector2:{}:{}:findings/{}", state.region, state.account_id, name);
        let detail = FindingDetail {
            name: name.clone(),
            arn: arn.clone(),
            status: "ACTIVE".to_string(),
            tags: req.tags.clone(),
        };
        state.findings.insert(name, StoredFinding {
            name: detail.name.clone(),
            arn,
            tags: req.tags.unwrap_or_default(),
        });
        Ok(detail)
    }

    #[allow(dead_code)]
    pub async fn get_finding(&self, name: &str) -> Result<FindingDetail, InspectorError> {
        let state = self.inner.lock().await;
        let stored = state.findings.get(name)
            .ok_or_else(|| InspectorError::ResourceNotFoundException(format!("Finding {} not found", name)))?;
        Ok(FindingDetail {
            name: stored.name.clone(),
            arn: stored.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(stored.tags.clone()),
        })
    }

    #[allow(dead_code)]
    pub async fn list_findings(&self) -> Result<ListFindingsResponse, InspectorError> {
        let state = self.inner.lock().await;
        let items: Vec<FindingDetail> = state.findings.values().map(|s| FindingDetail {
            name: s.name.clone(),
            arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListFindingsResponse {
            items: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_finding(&self, name: &str) -> Result<(), InspectorError> {
        let mut state = self.inner.lock().await;
        state.findings.remove(name)
            .ok_or_else(|| InspectorError::ResourceNotFoundException(format!("Finding {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = InspectorState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_finding() {
        let state = InspectorState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateFindingRequest::default();
        let result = state.create_finding(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_get_finding_not_found() {
        let state = InspectorState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.get_finding("nonexistent").await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_findings() {
        let state = InspectorState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.list_findings().await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_finding_not_found() {
        let state = InspectorState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.delete_finding("nonexistent").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_finding_full_crud() {
        let state = InspectorState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateFindingRequest::default();
        create_req.name = Some("test-crud-resource".to_string());
        let create_result = state.create_finding(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let get_result = state.get_finding("test-crud-resource").await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let del_result = state.delete_finding("test-crud-resource").await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
