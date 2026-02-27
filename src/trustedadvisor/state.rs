use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::TrustedadvisorError;
use super::types::*;

#[allow(dead_code)]
struct TrustedadvisorStateInner {
    checks: HashMap<String, StoredCheck>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredCheck {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct TrustedadvisorState {
    inner: Arc<Mutex<TrustedadvisorStateInner>>,
}

impl TrustedadvisorState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        TrustedadvisorState {
            inner: Arc::new(Mutex::new(TrustedadvisorStateInner {
                checks: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_check(&self, req: CreateCheckRequest) -> Result<CreateCheckResponse, TrustedadvisorError> {
        let mut state = self.inner.lock().await;
        let name = req.check_name.clone();
        if state.checks.contains_key(&name) {
            return Err(TrustedadvisorError::ResourceAlreadyExistsException(format!("Check {} already exists", name)));
        }
        let arn = format!("arn:aws:support:{}:{}:checks/{}", state.region, state.account_id, name);
        state.checks.insert(name.clone(), StoredCheck {
            name: name.clone(),
            arn: arn.clone(),
            tags: req.tags.unwrap_or_default(),
        });
        Ok(CreateCheckResponse {
            check_arn: Some(arn),
            check_name: Some(name),
        })
    }

    #[allow(dead_code)]
    pub async fn describe_check(&self, req: DescribeCheckRequest) -> Result<DescribeCheckResponse, TrustedadvisorError> {
        let state = self.inner.lock().await;
        let name = req.check_name.or_else(|| req.check_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| TrustedadvisorError::ValidationException("Name or ARN required".to_string()))?;
        let stored = state.checks.get(&name)
            .ok_or_else(|| TrustedadvisorError::ResourceNotFoundException(format!("Check {} not found", name)))?;
        Ok(DescribeCheckResponse {
            check: CheckDetail {
                check_name: stored.name.clone(),
                check_arn: stored.arn.clone(),
                status: "ACTIVE".to_string(),
                tags: Some(stored.tags.clone()),
            },
        })
    }

    #[allow(dead_code)]
    pub async fn list_checks(&self, _req: ListChecksRequest) -> Result<ListChecksResponse, TrustedadvisorError> {
        let state = self.inner.lock().await;
        let items: Vec<CheckDetail> = state.checks.values().map(|s| CheckDetail {
            check_name: s.name.clone(),
            check_arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListChecksResponse {
            checks: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_check(&self, req: DeleteCheckRequest) -> Result<(), TrustedadvisorError> {
        let mut state = self.inner.lock().await;
        let name = req.check_name.or_else(|| req.check_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| TrustedadvisorError::ValidationException("Name or ARN required".to_string()))?;
        state.checks.remove(&name)
            .ok_or_else(|| TrustedadvisorError::ResourceNotFoundException(format!("Check {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = TrustedadvisorState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_check() {
        let state = TrustedadvisorState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateCheckRequest::default();
        let result = state.create_check(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_describe_check_not_found() {
        let state = TrustedadvisorState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DescribeCheckRequest::default();
        let result = state.describe_check(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_checks_empty() {
        let state = TrustedadvisorState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = ListChecksRequest::default();
        let result = state.list_checks(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_check_not_found() {
        let state = TrustedadvisorState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DeleteCheckRequest::default();
        let result = state.delete_check(req).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_check_create_and_list() {
        let state = TrustedadvisorState::new("123456789012".to_string(), "us-east-1".to_string());
        let create_req = CreateCheckRequest::default();
        let _created = state.create_check(create_req).await.unwrap();
        let list_req = ListChecksRequest::default();
        let listed = state.list_checks(list_req).await.unwrap();
        let _ = listed;
    }

    #[tokio::test]
    async fn test_check_full_crud() {
        let state = TrustedadvisorState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateCheckRequest::default();
        create_req.check_name = "test-crud-resource".to_string();
        let create_result = state.create_check(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let mut get_req = DescribeCheckRequest::default();
        get_req.check_name = Some("test-crud-resource".to_string());
        let get_result = state.describe_check(get_req).await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let mut del_req = DeleteCheckRequest::default();
        del_req.check_name = Some("test-crud-resource".to_string());
        let del_result = state.delete_check(del_req).await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
