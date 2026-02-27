use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::PollyError;
use super::types::*;

#[allow(dead_code)]
struct PollyStateInner {
    lexicons: HashMap<String, StoredLexicon>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredLexicon {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct PollyState {
    inner: Arc<Mutex<PollyStateInner>>,
}

impl PollyState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        PollyState {
            inner: Arc::new(Mutex::new(PollyStateInner {
                lexicons: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_lexicon(&self, req: CreateLexiconRequest) -> Result<LexiconDetail, PollyError> {
        let mut state = self.inner.lock().await;
        let name = req.name.or(req.name_pascal).unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        if state.lexicons.contains_key(&name) {
            return Err(PollyError::ResourceAlreadyExistsException(format!("Lexicon {} already exists", name)));
        }
        let arn = format!("arn:aws:polly:{}:{}:lexicons/{}", state.region, state.account_id, name);
        let detail = LexiconDetail {
            name: name.clone(),
            arn: arn.clone(),
            status: "ACTIVE".to_string(),
            tags: req.tags.clone(),
        };
        state.lexicons.insert(name, StoredLexicon {
            name: detail.name.clone(),
            arn,
            tags: req.tags.unwrap_or_default(),
        });
        Ok(detail)
    }

    #[allow(dead_code)]
    pub async fn get_lexicon(&self, name: &str) -> Result<LexiconDetail, PollyError> {
        let state = self.inner.lock().await;
        let stored = state.lexicons.get(name)
            .ok_or_else(|| PollyError::ResourceNotFoundException(format!("Lexicon {} not found", name)))?;
        Ok(LexiconDetail {
            name: stored.name.clone(),
            arn: stored.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(stored.tags.clone()),
        })
    }

    #[allow(dead_code)]
    pub async fn list_lexicons(&self) -> Result<ListLexiconsResponse, PollyError> {
        let state = self.inner.lock().await;
        let items: Vec<LexiconDetail> = state.lexicons.values().map(|s| LexiconDetail {
            name: s.name.clone(),
            arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListLexiconsResponse {
            items: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_lexicon(&self, name: &str) -> Result<(), PollyError> {
        let mut state = self.inner.lock().await;
        state.lexicons.remove(name)
            .ok_or_else(|| PollyError::ResourceNotFoundException(format!("Lexicon {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = PollyState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_lexicon() {
        let state = PollyState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateLexiconRequest::default();
        let result = state.create_lexicon(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_get_lexicon_not_found() {
        let state = PollyState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.get_lexicon("nonexistent").await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_lexicons() {
        let state = PollyState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.list_lexicons().await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_lexicon_not_found() {
        let state = PollyState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.delete_lexicon("nonexistent").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_lexicon_full_crud() {
        let state = PollyState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateLexiconRequest::default();
        create_req.name = Some("test-crud-resource".to_string());
        let create_result = state.create_lexicon(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let get_result = state.get_lexicon("test-crud-resource").await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let del_result = state.delete_lexicon("test-crud-resource").await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
