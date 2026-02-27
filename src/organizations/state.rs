use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::OrganizationsError;
use super::types::*;

#[allow(dead_code)]
struct OrganizationsStateInner {
    organizations: HashMap<String, StoredOrganization>,
    accounts: HashMap<String, StoredAccount>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredOrganization {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
struct StoredAccount {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct OrganizationsState {
    inner: Arc<Mutex<OrganizationsStateInner>>,
}

impl OrganizationsState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        OrganizationsState {
            inner: Arc::new(Mutex::new(OrganizationsStateInner {
                organizations: HashMap::new(),
                accounts: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_organization(&self, req: CreateOrganizationRequest) -> Result<CreateOrganizationResponse, OrganizationsError> {
        let mut state = self.inner.lock().await;
        let name = req.organization_name.clone();
        if state.organizations.contains_key(&name) {
            return Err(OrganizationsError::ResourceAlreadyExistsException(format!("Organization {} already exists", name)));
        }
        let arn = format!("arn:aws:organizations:{}:{}:organizations/{}", state.region, state.account_id, name);
        state.organizations.insert(name.clone(), StoredOrganization {
            name: name.clone(),
            arn: arn.clone(),
            tags: req.tags.unwrap_or_default(),
        });
        Ok(CreateOrganizationResponse {
            organization_arn: Some(arn),
            organization_name: Some(name),
        })
    }

    #[allow(dead_code)]
    pub async fn describe_organization(&self, req: DescribeOrganizationRequest) -> Result<DescribeOrganizationResponse, OrganizationsError> {
        let state = self.inner.lock().await;
        let name = req.organization_name.or_else(|| req.organization_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| OrganizationsError::ValidationException("Name or ARN required".to_string()))?;
        let stored = state.organizations.get(&name)
            .ok_or_else(|| OrganizationsError::ResourceNotFoundException(format!("Organization {} not found", name)))?;
        Ok(DescribeOrganizationResponse {
            organization: OrganizationDetail {
                organization_name: stored.name.clone(),
                organization_arn: stored.arn.clone(),
                status: "ACTIVE".to_string(),
                tags: Some(stored.tags.clone()),
            },
        })
    }

    #[allow(dead_code)]
    pub async fn list_organizations(&self, _req: ListOrganizationsRequest) -> Result<ListOrganizationsResponse, OrganizationsError> {
        let state = self.inner.lock().await;
        let items: Vec<OrganizationDetail> = state.organizations.values().map(|s| OrganizationDetail {
            organization_name: s.name.clone(),
            organization_arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListOrganizationsResponse {
            organizations: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_organization(&self, req: DeleteOrganizationRequest) -> Result<(), OrganizationsError> {
        let mut state = self.inner.lock().await;
        let name = req.organization_name.or_else(|| req.organization_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| OrganizationsError::ValidationException("Name or ARN required".to_string()))?;
        state.organizations.remove(&name)
            .ok_or_else(|| OrganizationsError::ResourceNotFoundException(format!("Organization {} not found", name)))?;
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn create_account(&self, req: CreateAccountRequest) -> Result<CreateAccountResponse, OrganizationsError> {
        let mut state = self.inner.lock().await;
        let name = req.account_name.clone();
        if state.accounts.contains_key(&name) {
            return Err(OrganizationsError::ResourceAlreadyExistsException(format!("Account {} already exists", name)));
        }
        let arn = format!("arn:aws:organizations:{}:{}:accounts/{}", state.region, state.account_id, name);
        state.accounts.insert(name.clone(), StoredAccount {
            name: name.clone(),
            arn: arn.clone(),
            tags: req.tags.unwrap_or_default(),
        });
        Ok(CreateAccountResponse {
            account_arn: Some(arn),
            account_name: Some(name),
        })
    }

    #[allow(dead_code)]
    pub async fn describe_account(&self, req: DescribeAccountRequest) -> Result<DescribeAccountResponse, OrganizationsError> {
        let state = self.inner.lock().await;
        let name = req.account_name.or_else(|| req.account_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| OrganizationsError::ValidationException("Name or ARN required".to_string()))?;
        let stored = state.accounts.get(&name)
            .ok_or_else(|| OrganizationsError::ResourceNotFoundException(format!("Account {} not found", name)))?;
        Ok(DescribeAccountResponse {
            account: AccountDetail {
                account_name: stored.name.clone(),
                account_arn: stored.arn.clone(),
                status: "ACTIVE".to_string(),
                tags: Some(stored.tags.clone()),
            },
        })
    }

    #[allow(dead_code)]
    pub async fn list_accounts(&self, _req: ListAccountsRequest) -> Result<ListAccountsResponse, OrganizationsError> {
        let state = self.inner.lock().await;
        let items: Vec<AccountDetail> = state.accounts.values().map(|s| AccountDetail {
            account_name: s.name.clone(),
            account_arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListAccountsResponse {
            accounts: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_account(&self, req: DeleteAccountRequest) -> Result<(), OrganizationsError> {
        let mut state = self.inner.lock().await;
        let name = req.account_name.or_else(|| req.account_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| OrganizationsError::ValidationException("Name or ARN required".to_string()))?;
        state.accounts.remove(&name)
            .ok_or_else(|| OrganizationsError::ResourceNotFoundException(format!("Account {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = OrganizationsState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_organization() {
        let state = OrganizationsState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateOrganizationRequest::default();
        let result = state.create_organization(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_describe_organization_not_found() {
        let state = OrganizationsState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DescribeOrganizationRequest::default();
        let result = state.describe_organization(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_organizations_empty() {
        let state = OrganizationsState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = ListOrganizationsRequest::default();
        let result = state.list_organizations(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_organization_not_found() {
        let state = OrganizationsState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DeleteOrganizationRequest::default();
        let result = state.delete_organization(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_create_account() {
        let state = OrganizationsState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateAccountRequest::default();
        let result = state.create_account(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_describe_account_not_found() {
        let state = OrganizationsState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DescribeAccountRequest::default();
        let result = state.describe_account(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_accounts_empty() {
        let state = OrganizationsState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = ListAccountsRequest::default();
        let result = state.list_accounts(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_account_not_found() {
        let state = OrganizationsState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DeleteAccountRequest::default();
        let result = state.delete_account(req).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_organization_create_and_list() {
        let state = OrganizationsState::new("123456789012".to_string(), "us-east-1".to_string());
        let create_req = CreateOrganizationRequest::default();
        let _created = state.create_organization(create_req).await.unwrap();
        let list_req = ListOrganizationsRequest::default();
        let listed = state.list_organizations(list_req).await.unwrap();
        let _ = listed;
    }
    #[tokio::test]
    async fn test_account_create_and_list() {
        let state = OrganizationsState::new("123456789012".to_string(), "us-east-1".to_string());
        let create_req = CreateAccountRequest::default();
        let _created = state.create_account(create_req).await.unwrap();
        let list_req = ListAccountsRequest::default();
        let listed = state.list_accounts(list_req).await.unwrap();
        let _ = listed;
    }

    #[tokio::test]
    async fn test_organization_full_crud() {
        let state = OrganizationsState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateOrganizationRequest::default();
        create_req.organization_name = "test-crud-resource".to_string();
        let create_result = state.create_organization(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let mut get_req = DescribeOrganizationRequest::default();
        get_req.organization_name = Some("test-crud-resource".to_string());
        let get_result = state.describe_organization(get_req).await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let mut del_req = DeleteOrganizationRequest::default();
        del_req.organization_name = Some("test-crud-resource".to_string());
        let del_result = state.delete_organization(del_req).await;
        assert!(del_result.is_ok(), "delete should succeed");
    }

    #[tokio::test]
    async fn test_account_full_crud() {
        let state = OrganizationsState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateAccountRequest::default();
        create_req.account_name = "test-crud-resource".to_string();
        let create_result = state.create_account(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let mut get_req = DescribeAccountRequest::default();
        get_req.account_name = Some("test-crud-resource".to_string());
        let get_result = state.describe_account(get_req).await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let mut del_req = DeleteAccountRequest::default();
        del_req.account_name = Some("test-crud-resource".to_string());
        let del_result = state.delete_account(del_req).await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
