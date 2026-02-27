use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::XrayError;
use super::types::*;

#[allow(dead_code)]
struct XrayStateInner {
    groups: HashMap<String, StoredGroup>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredGroup {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct XrayState {
    inner: Arc<Mutex<XrayStateInner>>,
}

impl XrayState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        XrayState {
            inner: Arc::new(Mutex::new(XrayStateInner {
                groups: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_group(&self, req: CreateGroupRequest) -> Result<GroupDetail, XrayError> {
        let mut state = self.inner.lock().await;
        let name = req.name.or(req.name_pascal).unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        if state.groups.contains_key(&name) {
            return Err(XrayError::ResourceAlreadyExistsException(format!("Group {} already exists", name)));
        }
        let arn = format!("arn:aws:xray:{}:{}:groups/{}", state.region, state.account_id, name);
        let detail = GroupDetail {
            name: name.clone(),
            arn: arn.clone(),
            status: "ACTIVE".to_string(),
            tags: req.tags.clone(),
        };
        state.groups.insert(name, StoredGroup {
            name: detail.name.clone(),
            arn,
            tags: req.tags.unwrap_or_default(),
        });
        Ok(detail)
    }

    #[allow(dead_code)]
    pub async fn get_group(&self, name: &str) -> Result<GroupDetail, XrayError> {
        let state = self.inner.lock().await;
        let stored = state.groups.get(name)
            .ok_or_else(|| XrayError::ResourceNotFoundException(format!("Group {} not found", name)))?;
        Ok(GroupDetail {
            name: stored.name.clone(),
            arn: stored.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(stored.tags.clone()),
        })
    }

    #[allow(dead_code)]
    pub async fn list_groups(&self) -> Result<ListGroupsResponse, XrayError> {
        let state = self.inner.lock().await;
        let items: Vec<GroupDetail> = state.groups.values().map(|s| GroupDetail {
            name: s.name.clone(),
            arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListGroupsResponse {
            items: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_group(&self, name: &str) -> Result<(), XrayError> {
        let mut state = self.inner.lock().await;
        state.groups.remove(name)
            .ok_or_else(|| XrayError::ResourceNotFoundException(format!("Group {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = XrayState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_group() {
        let state = XrayState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateGroupRequest::default();
        let result = state.create_group(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_get_group_not_found() {
        let state = XrayState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.get_group("nonexistent").await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_groups() {
        let state = XrayState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.list_groups().await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_group_not_found() {
        let state = XrayState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.delete_group("nonexistent").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_group_full_crud() {
        let state = XrayState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateGroupRequest::default();
        create_req.name = Some("test-crud-resource".to_string());
        let create_result = state.create_group(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let get_result = state.get_group("test-crud-resource").await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let del_result = state.delete_group("test-crud-resource").await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
