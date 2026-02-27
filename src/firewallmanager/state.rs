use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::FirewallmanagerError;
use super::types::*;

#[allow(dead_code)]
struct FirewallmanagerStateInner {
    policys: HashMap<String, StoredPolicy>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredPolicy {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct FirewallmanagerState {
    inner: Arc<Mutex<FirewallmanagerStateInner>>,
}

impl FirewallmanagerState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        FirewallmanagerState {
            inner: Arc::new(Mutex::new(FirewallmanagerStateInner {
                policys: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_policy(&self, req: CreatePolicyRequest) -> Result<CreatePolicyResponse, FirewallmanagerError> {
        let mut state = self.inner.lock().await;
        let name = req.policy_name.clone();
        if state.policys.contains_key(&name) {
            return Err(FirewallmanagerError::ResourceAlreadyExistsException(format!("Policy {} already exists", name)));
        }
        let arn = format!("arn:aws:fms:{}:{}:policies/{}", state.region, state.account_id, name);
        state.policys.insert(name.clone(), StoredPolicy {
            name: name.clone(),
            arn: arn.clone(),
            tags: req.tags.unwrap_or_default(),
        });
        Ok(CreatePolicyResponse {
            policy_arn: Some(arn),
            policy_name: Some(name),
        })
    }

    #[allow(dead_code)]
    pub async fn describe_policy(&self, req: DescribePolicyRequest) -> Result<DescribePolicyResponse, FirewallmanagerError> {
        let state = self.inner.lock().await;
        let name = req.policy_name.or_else(|| req.policy_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| FirewallmanagerError::ValidationException("Name or ARN required".to_string()))?;
        let stored = state.policys.get(&name)
            .ok_or_else(|| FirewallmanagerError::ResourceNotFoundException(format!("Policy {} not found", name)))?;
        Ok(DescribePolicyResponse {
            policy: PolicyDetail {
                policy_name: stored.name.clone(),
                policy_arn: stored.arn.clone(),
                status: "ACTIVE".to_string(),
                tags: Some(stored.tags.clone()),
            },
        })
    }

    #[allow(dead_code)]
    pub async fn list_policys(&self, _req: ListPolicysRequest) -> Result<ListPolicysResponse, FirewallmanagerError> {
        let state = self.inner.lock().await;
        let items: Vec<PolicyDetail> = state.policys.values().map(|s| PolicyDetail {
            policy_name: s.name.clone(),
            policy_arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListPolicysResponse {
            policys: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_policy(&self, req: DeletePolicyRequest) -> Result<(), FirewallmanagerError> {
        let mut state = self.inner.lock().await;
        let name = req.policy_name.or_else(|| req.policy_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| FirewallmanagerError::ValidationException("Name or ARN required".to_string()))?;
        state.policys.remove(&name)
            .ok_or_else(|| FirewallmanagerError::ResourceNotFoundException(format!("Policy {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = FirewallmanagerState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_policy() {
        let state = FirewallmanagerState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreatePolicyRequest::default();
        let result = state.create_policy(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_describe_policy_not_found() {
        let state = FirewallmanagerState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DescribePolicyRequest::default();
        let result = state.describe_policy(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_policys_empty() {
        let state = FirewallmanagerState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = ListPolicysRequest::default();
        let result = state.list_policys(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_policy_not_found() {
        let state = FirewallmanagerState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DeletePolicyRequest::default();
        let result = state.delete_policy(req).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_policy_create_and_list() {
        let state = FirewallmanagerState::new("123456789012".to_string(), "us-east-1".to_string());
        let create_req = CreatePolicyRequest::default();
        let _created = state.create_policy(create_req).await.unwrap();
        let list_req = ListPolicysRequest::default();
        let listed = state.list_policys(list_req).await.unwrap();
        let _ = listed;
    }

    #[tokio::test]
    async fn test_policy_full_crud() {
        let state = FirewallmanagerState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreatePolicyRequest::default();
        create_req.policy_name = "test-crud-resource".to_string();
        let create_result = state.create_policy(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let mut get_req = DescribePolicyRequest::default();
        get_req.policy_name = Some("test-crud-resource".to_string());
        let get_result = state.describe_policy(get_req).await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let mut del_req = DeletePolicyRequest::default();
        del_req.policy_name = Some("test-crud-resource".to_string());
        let del_result = state.delete_policy(del_req).await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
