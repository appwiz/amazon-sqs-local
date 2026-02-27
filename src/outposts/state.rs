use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::OutpostsError;
use super::types::*;

#[allow(dead_code)]
struct OutpostsStateInner {
    outposts: HashMap<String, StoredOutpost>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredOutpost {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct OutpostsState {
    inner: Arc<Mutex<OutpostsStateInner>>,
}

impl OutpostsState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        OutpostsState {
            inner: Arc::new(Mutex::new(OutpostsStateInner {
                outposts: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_outpost(&self, req: CreateOutpostRequest) -> Result<OutpostDetail, OutpostsError> {
        let mut state = self.inner.lock().await;
        let name = req.name.or(req.name_pascal).unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        if state.outposts.contains_key(&name) {
            return Err(OutpostsError::ResourceAlreadyExistsException(format!("Outpost {} already exists", name)));
        }
        let arn = format!("arn:aws:outposts:{}:{}:outposts/{}", state.region, state.account_id, name);
        let detail = OutpostDetail {
            name: name.clone(),
            arn: arn.clone(),
            status: "ACTIVE".to_string(),
            tags: req.tags.clone(),
        };
        state.outposts.insert(name, StoredOutpost {
            name: detail.name.clone(),
            arn,
            tags: req.tags.unwrap_or_default(),
        });
        Ok(detail)
    }

    #[allow(dead_code)]
    pub async fn get_outpost(&self, name: &str) -> Result<OutpostDetail, OutpostsError> {
        let state = self.inner.lock().await;
        let stored = state.outposts.get(name)
            .ok_or_else(|| OutpostsError::ResourceNotFoundException(format!("Outpost {} not found", name)))?;
        Ok(OutpostDetail {
            name: stored.name.clone(),
            arn: stored.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(stored.tags.clone()),
        })
    }

    #[allow(dead_code)]
    pub async fn list_outposts(&self) -> Result<ListOutpostsResponse, OutpostsError> {
        let state = self.inner.lock().await;
        let items: Vec<OutpostDetail> = state.outposts.values().map(|s| OutpostDetail {
            name: s.name.clone(),
            arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListOutpostsResponse {
            items: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_outpost(&self, name: &str) -> Result<(), OutpostsError> {
        let mut state = self.inner.lock().await;
        state.outposts.remove(name)
            .ok_or_else(|| OutpostsError::ResourceNotFoundException(format!("Outpost {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = OutpostsState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_outpost() {
        let state = OutpostsState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateOutpostRequest::default();
        let result = state.create_outpost(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_get_outpost_not_found() {
        let state = OutpostsState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.get_outpost("nonexistent").await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_outposts() {
        let state = OutpostsState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.list_outposts().await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_outpost_not_found() {
        let state = OutpostsState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.delete_outpost("nonexistent").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_outpost_full_crud() {
        let state = OutpostsState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateOutpostRequest::default();
        create_req.name = Some("test-crud-resource".to_string());
        let create_result = state.create_outpost(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let get_result = state.get_outpost("test-crud-resource").await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let del_result = state.delete_outpost("test-crud-resource").await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
