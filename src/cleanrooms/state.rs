use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::CleanroomsError;
use super::types::*;

#[allow(dead_code)]
struct CleanroomsStateInner {
    collaborations: HashMap<String, StoredCollaboration>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredCollaboration {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct CleanroomsState {
    inner: Arc<Mutex<CleanroomsStateInner>>,
}

impl CleanroomsState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        CleanroomsState {
            inner: Arc::new(Mutex::new(CleanroomsStateInner {
                collaborations: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_collaboration(&self, req: CreateCollaborationRequest) -> Result<CollaborationDetail, CleanroomsError> {
        let mut state = self.inner.lock().await;
        let name = req.name.or(req.name_pascal).unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        if state.collaborations.contains_key(&name) {
            return Err(CleanroomsError::ResourceAlreadyExistsException(format!("Collaboration {} already exists", name)));
        }
        let arn = format!("arn:aws:cleanrooms:{}:{}:collaborations/{}", state.region, state.account_id, name);
        let detail = CollaborationDetail {
            name: name.clone(),
            arn: arn.clone(),
            status: "ACTIVE".to_string(),
            tags: req.tags.clone(),
        };
        state.collaborations.insert(name, StoredCollaboration {
            name: detail.name.clone(),
            arn,
            tags: req.tags.unwrap_or_default(),
        });
        Ok(detail)
    }

    #[allow(dead_code)]
    pub async fn get_collaboration(&self, name: &str) -> Result<CollaborationDetail, CleanroomsError> {
        let state = self.inner.lock().await;
        let stored = state.collaborations.get(name)
            .ok_or_else(|| CleanroomsError::ResourceNotFoundException(format!("Collaboration {} not found", name)))?;
        Ok(CollaborationDetail {
            name: stored.name.clone(),
            arn: stored.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(stored.tags.clone()),
        })
    }

    #[allow(dead_code)]
    pub async fn list_collaborations(&self) -> Result<ListCollaborationsResponse, CleanroomsError> {
        let state = self.inner.lock().await;
        let items: Vec<CollaborationDetail> = state.collaborations.values().map(|s| CollaborationDetail {
            name: s.name.clone(),
            arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListCollaborationsResponse {
            items: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_collaboration(&self, name: &str) -> Result<(), CleanroomsError> {
        let mut state = self.inner.lock().await;
        state.collaborations.remove(name)
            .ok_or_else(|| CleanroomsError::ResourceNotFoundException(format!("Collaboration {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = CleanroomsState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_collaboration() {
        let state = CleanroomsState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateCollaborationRequest::default();
        let result = state.create_collaboration(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_get_collaboration_not_found() {
        let state = CleanroomsState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.get_collaboration("nonexistent").await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_collaborations() {
        let state = CleanroomsState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.list_collaborations().await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_collaboration_not_found() {
        let state = CleanroomsState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.delete_collaboration("nonexistent").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_collaboration_full_crud() {
        let state = CleanroomsState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateCollaborationRequest::default();
        create_req.name = Some("test-crud-resource".to_string());
        let create_result = state.create_collaboration(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let get_result = state.get_collaboration("test-crud-resource").await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let del_result = state.delete_collaboration("test-crud-resource").await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
