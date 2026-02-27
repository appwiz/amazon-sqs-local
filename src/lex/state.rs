use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::LexError;
use super::types::*;

#[allow(dead_code)]
struct LexStateInner {
    bots: HashMap<String, StoredBot>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredBot {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct LexState {
    inner: Arc<Mutex<LexStateInner>>,
}

impl LexState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        LexState {
            inner: Arc::new(Mutex::new(LexStateInner {
                bots: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_bot(&self, req: CreateBotRequest) -> Result<BotDetail, LexError> {
        let mut state = self.inner.lock().await;
        let name = req.name.or(req.name_pascal).unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        if state.bots.contains_key(&name) {
            return Err(LexError::ResourceAlreadyExistsException(format!("Bot {} already exists", name)));
        }
        let arn = format!("arn:aws:lex:{}:{}:bots/{}", state.region, state.account_id, name);
        let detail = BotDetail {
            name: name.clone(),
            arn: arn.clone(),
            status: "ACTIVE".to_string(),
            tags: req.tags.clone(),
        };
        state.bots.insert(name, StoredBot {
            name: detail.name.clone(),
            arn,
            tags: req.tags.unwrap_or_default(),
        });
        Ok(detail)
    }

    #[allow(dead_code)]
    pub async fn get_bot(&self, name: &str) -> Result<BotDetail, LexError> {
        let state = self.inner.lock().await;
        let stored = state.bots.get(name)
            .ok_or_else(|| LexError::ResourceNotFoundException(format!("Bot {} not found", name)))?;
        Ok(BotDetail {
            name: stored.name.clone(),
            arn: stored.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(stored.tags.clone()),
        })
    }

    #[allow(dead_code)]
    pub async fn list_bots(&self) -> Result<ListBotsResponse, LexError> {
        let state = self.inner.lock().await;
        let items: Vec<BotDetail> = state.bots.values().map(|s| BotDetail {
            name: s.name.clone(),
            arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListBotsResponse {
            items: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_bot(&self, name: &str) -> Result<(), LexError> {
        let mut state = self.inner.lock().await;
        state.bots.remove(name)
            .ok_or_else(|| LexError::ResourceNotFoundException(format!("Bot {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = LexState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_bot() {
        let state = LexState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateBotRequest::default();
        let result = state.create_bot(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_get_bot_not_found() {
        let state = LexState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.get_bot("nonexistent").await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_bots() {
        let state = LexState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.list_bots().await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_bot_not_found() {
        let state = LexState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.delete_bot("nonexistent").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_bot_full_crud() {
        let state = LexState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateBotRequest::default();
        create_req.name = Some("test-crud-resource".to_string());
        let create_result = state.create_bot(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let get_result = state.get_bot("test-crud-resource").await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let del_result = state.delete_bot("test-crud-resource").await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
