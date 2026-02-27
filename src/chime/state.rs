use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::ChimeError;
use super::types::*;

#[allow(dead_code)]
struct ChimeStateInner {
    accounts: HashMap<String, StoredAccount>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredAccount {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct ChimeState {
    inner: Arc<Mutex<ChimeStateInner>>,
}

impl ChimeState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        ChimeState {
            inner: Arc::new(Mutex::new(ChimeStateInner {
                accounts: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_account(&self, req: CreateAccountRequest) -> Result<AccountDetail, ChimeError> {
        let mut state = self.inner.lock().await;
        let name = req.name.or(req.name_pascal).unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        if state.accounts.contains_key(&name) {
            return Err(ChimeError::ResourceAlreadyExistsException(format!("Account {} already exists", name)));
        }
        let arn = format!("arn:aws:chime:{}:{}:accounts/{}", state.region, state.account_id, name);
        let detail = AccountDetail {
            name: name.clone(),
            arn: arn.clone(),
            status: "ACTIVE".to_string(),
            tags: req.tags.clone(),
        };
        state.accounts.insert(name, StoredAccount {
            name: detail.name.clone(),
            arn,
            tags: req.tags.unwrap_or_default(),
        });
        Ok(detail)
    }

    #[allow(dead_code)]
    pub async fn get_account(&self, name: &str) -> Result<AccountDetail, ChimeError> {
        let state = self.inner.lock().await;
        let stored = state.accounts.get(name)
            .ok_or_else(|| ChimeError::ResourceNotFoundException(format!("Account {} not found", name)))?;
        Ok(AccountDetail {
            name: stored.name.clone(),
            arn: stored.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(stored.tags.clone()),
        })
    }

    #[allow(dead_code)]
    pub async fn list_accounts(&self) -> Result<ListAccountsResponse, ChimeError> {
        let state = self.inner.lock().await;
        let items: Vec<AccountDetail> = state.accounts.values().map(|s| AccountDetail {
            name: s.name.clone(),
            arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListAccountsResponse {
            items: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_account(&self, name: &str) -> Result<(), ChimeError> {
        let mut state = self.inner.lock().await;
        state.accounts.remove(name)
            .ok_or_else(|| ChimeError::ResourceNotFoundException(format!("Account {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = ChimeState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_account() {
        let state = ChimeState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateAccountRequest::default();
        let result = state.create_account(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_get_account_not_found() {
        let state = ChimeState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.get_account("nonexistent").await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_accounts() {
        let state = ChimeState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.list_accounts().await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_account_not_found() {
        let state = ChimeState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.delete_account("nonexistent").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_account_full_crud() {
        let state = ChimeState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateAccountRequest::default();
        create_req.name = Some("test-crud-resource".to_string());
        let create_result = state.create_account(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let get_result = state.get_account("test-crud-resource").await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let del_result = state.delete_account("test-crud-resource").await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
