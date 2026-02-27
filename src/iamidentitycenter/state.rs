use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::IamidentitycenterError;
use super::types::*;

#[allow(dead_code)]
struct IamidentitycenterStateInner {
    permission_sets: HashMap<String, StoredPermissionSet>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredPermissionSet {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct IamidentitycenterState {
    inner: Arc<Mutex<IamidentitycenterStateInner>>,
}

impl IamidentitycenterState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        IamidentitycenterState {
            inner: Arc::new(Mutex::new(IamidentitycenterStateInner {
                permission_sets: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_permission_set(&self, req: CreatePermissionSetRequest) -> Result<CreatePermissionSetResponse, IamidentitycenterError> {
        let mut state = self.inner.lock().await;
        let name = req.permission_set_name.clone();
        if state.permission_sets.contains_key(&name) {
            return Err(IamidentitycenterError::ResourceAlreadyExistsException(format!("PermissionSet {} already exists", name)));
        }
        let arn = format!("arn:aws:sso:{}:{}:permission-sets/{}", state.region, state.account_id, name);
        state.permission_sets.insert(name.clone(), StoredPermissionSet {
            name: name.clone(),
            arn: arn.clone(),
            tags: req.tags.unwrap_or_default(),
        });
        Ok(CreatePermissionSetResponse {
            permission_set_arn: Some(arn),
            permission_set_name: Some(name),
        })
    }

    #[allow(dead_code)]
    pub async fn describe_permission_set(&self, req: DescribePermissionSetRequest) -> Result<DescribePermissionSetResponse, IamidentitycenterError> {
        let state = self.inner.lock().await;
        let name = req.permission_set_name.or_else(|| req.permission_set_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| IamidentitycenterError::ValidationException("Name or ARN required".to_string()))?;
        let stored = state.permission_sets.get(&name)
            .ok_or_else(|| IamidentitycenterError::ResourceNotFoundException(format!("PermissionSet {} not found", name)))?;
        Ok(DescribePermissionSetResponse {
            permission_set: PermissionSetDetail {
                permission_set_name: stored.name.clone(),
                permission_set_arn: stored.arn.clone(),
                status: "ACTIVE".to_string(),
                tags: Some(stored.tags.clone()),
            },
        })
    }

    #[allow(dead_code)]
    pub async fn list_permission_sets(&self, _req: ListPermissionSetsRequest) -> Result<ListPermissionSetsResponse, IamidentitycenterError> {
        let state = self.inner.lock().await;
        let items: Vec<PermissionSetDetail> = state.permission_sets.values().map(|s| PermissionSetDetail {
            permission_set_name: s.name.clone(),
            permission_set_arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListPermissionSetsResponse {
            permission_sets: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_permission_set(&self, req: DeletePermissionSetRequest) -> Result<(), IamidentitycenterError> {
        let mut state = self.inner.lock().await;
        let name = req.permission_set_name.or_else(|| req.permission_set_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| IamidentitycenterError::ValidationException("Name or ARN required".to_string()))?;
        state.permission_sets.remove(&name)
            .ok_or_else(|| IamidentitycenterError::ResourceNotFoundException(format!("PermissionSet {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = IamidentitycenterState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_permission_set() {
        let state = IamidentitycenterState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreatePermissionSetRequest::default();
        let result = state.create_permission_set(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_describe_permission_set_not_found() {
        let state = IamidentitycenterState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DescribePermissionSetRequest::default();
        let result = state.describe_permission_set(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_permission_sets_empty() {
        let state = IamidentitycenterState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = ListPermissionSetsRequest::default();
        let result = state.list_permission_sets(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_permission_set_not_found() {
        let state = IamidentitycenterState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DeletePermissionSetRequest::default();
        let result = state.delete_permission_set(req).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_permission_set_create_and_list() {
        let state = IamidentitycenterState::new("123456789012".to_string(), "us-east-1".to_string());
        let create_req = CreatePermissionSetRequest::default();
        let _created = state.create_permission_set(create_req).await.unwrap();
        let list_req = ListPermissionSetsRequest::default();
        let listed = state.list_permission_sets(list_req).await.unwrap();
        let _ = listed;
    }

    #[tokio::test]
    async fn test_permission_set_full_crud() {
        let state = IamidentitycenterState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreatePermissionSetRequest::default();
        create_req.permission_set_name = "test-crud-resource".to_string();
        let create_result = state.create_permission_set(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let mut get_req = DescribePermissionSetRequest::default();
        get_req.permission_set_name = Some("test-crud-resource".to_string());
        let get_result = state.describe_permission_set(get_req).await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let mut del_req = DeletePermissionSetRequest::default();
        del_req.permission_set_name = Some("test-crud-resource".to_string());
        let del_result = state.delete_permission_set(del_req).await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
