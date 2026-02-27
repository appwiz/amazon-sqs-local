use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::VerifiedpermissionsError;
use super::types::*;

#[allow(dead_code)]
struct VerifiedpermissionsStateInner {
    policy_stores: HashMap<String, StoredPolicyStore>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredPolicyStore {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct VerifiedpermissionsState {
    inner: Arc<Mutex<VerifiedpermissionsStateInner>>,
}

impl VerifiedpermissionsState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        VerifiedpermissionsState {
            inner: Arc::new(Mutex::new(VerifiedpermissionsStateInner {
                policy_stores: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_policy_store(&self, req: CreatePolicyStoreRequest) -> Result<CreatePolicyStoreResponse, VerifiedpermissionsError> {
        let mut state = self.inner.lock().await;
        let name = req.policy_store_name.clone();
        if state.policy_stores.contains_key(&name) {
            return Err(VerifiedpermissionsError::ResourceAlreadyExistsException(format!("PolicyStore {} already exists", name)));
        }
        let arn = format!("arn:aws:verifiedpermissions:{}:{}:policy-stores/{}", state.region, state.account_id, name);
        state.policy_stores.insert(name.clone(), StoredPolicyStore {
            name: name.clone(),
            arn: arn.clone(),
            tags: req.tags.unwrap_or_default(),
        });
        Ok(CreatePolicyStoreResponse {
            policy_store_arn: Some(arn),
            policy_store_name: Some(name),
        })
    }

    #[allow(dead_code)]
    pub async fn describe_policy_store(&self, req: DescribePolicyStoreRequest) -> Result<DescribePolicyStoreResponse, VerifiedpermissionsError> {
        let state = self.inner.lock().await;
        let name = req.policy_store_name.or_else(|| req.policy_store_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| VerifiedpermissionsError::ValidationException("Name or ARN required".to_string()))?;
        let stored = state.policy_stores.get(&name)
            .ok_or_else(|| VerifiedpermissionsError::ResourceNotFoundException(format!("PolicyStore {} not found", name)))?;
        Ok(DescribePolicyStoreResponse {
            policy_store: PolicyStoreDetail {
                policy_store_name: stored.name.clone(),
                policy_store_arn: stored.arn.clone(),
                status: "ACTIVE".to_string(),
                tags: Some(stored.tags.clone()),
            },
        })
    }

    #[allow(dead_code)]
    pub async fn list_policy_stores(&self, _req: ListPolicyStoresRequest) -> Result<ListPolicyStoresResponse, VerifiedpermissionsError> {
        let state = self.inner.lock().await;
        let items: Vec<PolicyStoreDetail> = state.policy_stores.values().map(|s| PolicyStoreDetail {
            policy_store_name: s.name.clone(),
            policy_store_arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListPolicyStoresResponse {
            policy_stores: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_policy_store(&self, req: DeletePolicyStoreRequest) -> Result<(), VerifiedpermissionsError> {
        let mut state = self.inner.lock().await;
        let name = req.policy_store_name.or_else(|| req.policy_store_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| VerifiedpermissionsError::ValidationException("Name or ARN required".to_string()))?;
        state.policy_stores.remove(&name)
            .ok_or_else(|| VerifiedpermissionsError::ResourceNotFoundException(format!("PolicyStore {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = VerifiedpermissionsState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_policy_store() {
        let state = VerifiedpermissionsState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreatePolicyStoreRequest::default();
        let result = state.create_policy_store(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_describe_policy_store_not_found() {
        let state = VerifiedpermissionsState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DescribePolicyStoreRequest::default();
        let result = state.describe_policy_store(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_policy_stores_empty() {
        let state = VerifiedpermissionsState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = ListPolicyStoresRequest::default();
        let result = state.list_policy_stores(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_policy_store_not_found() {
        let state = VerifiedpermissionsState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DeletePolicyStoreRequest::default();
        let result = state.delete_policy_store(req).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_policy_store_create_and_list() {
        let state = VerifiedpermissionsState::new("123456789012".to_string(), "us-east-1".to_string());
        let create_req = CreatePolicyStoreRequest::default();
        let _created = state.create_policy_store(create_req).await.unwrap();
        let list_req = ListPolicyStoresRequest::default();
        let listed = state.list_policy_stores(list_req).await.unwrap();
        let _ = listed;
    }

    #[tokio::test]
    async fn test_policy_store_full_crud() {
        let state = VerifiedpermissionsState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreatePolicyStoreRequest::default();
        create_req.policy_store_name = "test-crud-resource".to_string();
        let create_result = state.create_policy_store(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let mut get_req = DescribePolicyStoreRequest::default();
        get_req.policy_store_name = Some("test-crud-resource".to_string());
        let get_result = state.describe_policy_store(get_req).await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let mut del_req = DeletePolicyStoreRequest::default();
        del_req.policy_store_name = Some("test-crud-resource".to_string());
        let del_result = state.delete_policy_store(del_req).await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
