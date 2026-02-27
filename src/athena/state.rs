use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::AthenaError;
use super::types::*;

#[allow(dead_code)]
struct AthenaStateInner {
    work_groups: HashMap<String, StoredWorkGroup>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredWorkGroup {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct AthenaState {
    inner: Arc<Mutex<AthenaStateInner>>,
}

impl AthenaState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        AthenaState {
            inner: Arc::new(Mutex::new(AthenaStateInner {
                work_groups: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_work_group(&self, req: CreateWorkGroupRequest) -> Result<CreateWorkGroupResponse, AthenaError> {
        let mut state = self.inner.lock().await;
        let name = req.work_group_name.clone();
        if state.work_groups.contains_key(&name) {
            return Err(AthenaError::ResourceAlreadyExistsException(format!("WorkGroup {} already exists", name)));
        }
        let arn = format!("arn:aws:athena:{}:{}:work-groups/{}", state.region, state.account_id, name);
        state.work_groups.insert(name.clone(), StoredWorkGroup {
            name: name.clone(),
            arn: arn.clone(),
            tags: req.tags.unwrap_or_default(),
        });
        Ok(CreateWorkGroupResponse {
            work_group_arn: Some(arn),
            work_group_name: Some(name),
        })
    }

    #[allow(dead_code)]
    pub async fn describe_work_group(&self, req: DescribeWorkGroupRequest) -> Result<DescribeWorkGroupResponse, AthenaError> {
        let state = self.inner.lock().await;
        let name = req.work_group_name.or_else(|| req.work_group_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| AthenaError::ValidationException("Name or ARN required".to_string()))?;
        let stored = state.work_groups.get(&name)
            .ok_or_else(|| AthenaError::ResourceNotFoundException(format!("WorkGroup {} not found", name)))?;
        Ok(DescribeWorkGroupResponse {
            work_group: WorkGroupDetail {
                work_group_name: stored.name.clone(),
                work_group_arn: stored.arn.clone(),
                status: "ACTIVE".to_string(),
                tags: Some(stored.tags.clone()),
            },
        })
    }

    #[allow(dead_code)]
    pub async fn list_work_groups(&self, _req: ListWorkGroupsRequest) -> Result<ListWorkGroupsResponse, AthenaError> {
        let state = self.inner.lock().await;
        let items: Vec<WorkGroupDetail> = state.work_groups.values().map(|s| WorkGroupDetail {
            work_group_name: s.name.clone(),
            work_group_arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListWorkGroupsResponse {
            work_groups: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_work_group(&self, req: DeleteWorkGroupRequest) -> Result<(), AthenaError> {
        let mut state = self.inner.lock().await;
        let name = req.work_group_name.or_else(|| req.work_group_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| AthenaError::ValidationException("Name or ARN required".to_string()))?;
        state.work_groups.remove(&name)
            .ok_or_else(|| AthenaError::ResourceNotFoundException(format!("WorkGroup {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = AthenaState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_work_group() {
        let state = AthenaState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateWorkGroupRequest::default();
        let result = state.create_work_group(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_describe_work_group_not_found() {
        let state = AthenaState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DescribeWorkGroupRequest::default();
        let result = state.describe_work_group(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_work_groups_empty() {
        let state = AthenaState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = ListWorkGroupsRequest::default();
        let result = state.list_work_groups(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_work_group_not_found() {
        let state = AthenaState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DeleteWorkGroupRequest::default();
        let result = state.delete_work_group(req).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_work_group_create_and_list() {
        let state = AthenaState::new("123456789012".to_string(), "us-east-1".to_string());
        let create_req = CreateWorkGroupRequest::default();
        let _created = state.create_work_group(create_req).await.unwrap();
        let list_req = ListWorkGroupsRequest::default();
        let listed = state.list_work_groups(list_req).await.unwrap();
        let _ = listed;
    }

    #[tokio::test]
    async fn test_work_group_full_crud() {
        let state = AthenaState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateWorkGroupRequest::default();
        create_req.work_group_name = "test-crud-resource".to_string();
        let create_result = state.create_work_group(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let mut get_req = DescribeWorkGroupRequest::default();
        get_req.work_group_name = Some("test-crud-resource".to_string());
        let get_result = state.describe_work_group(get_req).await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let mut del_req = DeleteWorkGroupRequest::default();
        del_req.work_group_name = Some("test-crud-resource".to_string());
        let del_result = state.delete_work_group(del_req).await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
