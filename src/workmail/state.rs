use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::WorkmailError;
use super::types::*;

#[allow(dead_code)]
struct WorkmailStateInner {
    organizations: HashMap<String, StoredOrganization>,
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
pub struct WorkmailState {
    inner: Arc<Mutex<WorkmailStateInner>>,
}

impl WorkmailState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        WorkmailState {
            inner: Arc::new(Mutex::new(WorkmailStateInner {
                organizations: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_organization(&self, req: CreateOrganizationRequest) -> Result<CreateOrganizationResponse, WorkmailError> {
        let mut state = self.inner.lock().await;
        let name = req.organization_name.clone();
        if state.organizations.contains_key(&name) {
            return Err(WorkmailError::ResourceAlreadyExistsException(format!("Organization {} already exists", name)));
        }
        let arn = format!("arn:aws:workmail:{}:{}:organizations/{}", state.region, state.account_id, name);
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
    pub async fn describe_organization(&self, req: DescribeOrganizationRequest) -> Result<DescribeOrganizationResponse, WorkmailError> {
        let state = self.inner.lock().await;
        let name = req.organization_name.or_else(|| req.organization_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| WorkmailError::ValidationException("Name or ARN required".to_string()))?;
        let stored = state.organizations.get(&name)
            .ok_or_else(|| WorkmailError::ResourceNotFoundException(format!("Organization {} not found", name)))?;
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
    pub async fn list_organizations(&self, _req: ListOrganizationsRequest) -> Result<ListOrganizationsResponse, WorkmailError> {
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
    pub async fn delete_organization(&self, req: DeleteOrganizationRequest) -> Result<(), WorkmailError> {
        let mut state = self.inner.lock().await;
        let name = req.organization_name.or_else(|| req.organization_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| WorkmailError::ValidationException("Name or ARN required".to_string()))?;
        state.organizations.remove(&name)
            .ok_or_else(|| WorkmailError::ResourceNotFoundException(format!("Organization {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = WorkmailState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_organization() {
        let state = WorkmailState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateOrganizationRequest::default();
        let result = state.create_organization(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_describe_organization_not_found() {
        let state = WorkmailState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DescribeOrganizationRequest::default();
        let result = state.describe_organization(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_organizations_empty() {
        let state = WorkmailState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = ListOrganizationsRequest::default();
        let result = state.list_organizations(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_organization_not_found() {
        let state = WorkmailState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DeleteOrganizationRequest::default();
        let result = state.delete_organization(req).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_organization_create_and_list() {
        let state = WorkmailState::new("123456789012".to_string(), "us-east-1".to_string());
        let create_req = CreateOrganizationRequest::default();
        let _created = state.create_organization(create_req).await.unwrap();
        let list_req = ListOrganizationsRequest::default();
        let listed = state.list_organizations(list_req).await.unwrap();
        let _ = listed;
    }

    #[tokio::test]
    async fn test_organization_full_crud() {
        let state = WorkmailState::new("123456789012".to_string(), "us-east-1".to_string());
        
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
}
