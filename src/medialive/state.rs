use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::MedialiveError;
use super::types::*;

#[allow(dead_code)]
struct MedialiveStateInner {
    channels: HashMap<String, StoredChannel>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredChannel {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct MedialiveState {
    inner: Arc<Mutex<MedialiveStateInner>>,
}

impl MedialiveState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        MedialiveState {
            inner: Arc::new(Mutex::new(MedialiveStateInner {
                channels: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_channel(&self, req: CreateChannelRequest) -> Result<ChannelDetail, MedialiveError> {
        let mut state = self.inner.lock().await;
        let name = req.name.or(req.name_pascal).unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        if state.channels.contains_key(&name) {
            return Err(MedialiveError::ResourceAlreadyExistsException(format!("Channel {} already exists", name)));
        }
        let arn = format!("arn:aws:medialive:{}:{}:channels/{}", state.region, state.account_id, name);
        let detail = ChannelDetail {
            name: name.clone(),
            arn: arn.clone(),
            status: "ACTIVE".to_string(),
            tags: req.tags.clone(),
        };
        state.channels.insert(name, StoredChannel {
            name: detail.name.clone(),
            arn,
            tags: req.tags.unwrap_or_default(),
        });
        Ok(detail)
    }

    #[allow(dead_code)]
    pub async fn get_channel(&self, name: &str) -> Result<ChannelDetail, MedialiveError> {
        let state = self.inner.lock().await;
        let stored = state.channels.get(name)
            .ok_or_else(|| MedialiveError::ResourceNotFoundException(format!("Channel {} not found", name)))?;
        Ok(ChannelDetail {
            name: stored.name.clone(),
            arn: stored.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(stored.tags.clone()),
        })
    }

    #[allow(dead_code)]
    pub async fn list_channels(&self) -> Result<ListChannelsResponse, MedialiveError> {
        let state = self.inner.lock().await;
        let items: Vec<ChannelDetail> = state.channels.values().map(|s| ChannelDetail {
            name: s.name.clone(),
            arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListChannelsResponse {
            items: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_channel(&self, name: &str) -> Result<(), MedialiveError> {
        let mut state = self.inner.lock().await;
        state.channels.remove(name)
            .ok_or_else(|| MedialiveError::ResourceNotFoundException(format!("Channel {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = MedialiveState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_channel() {
        let state = MedialiveState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateChannelRequest::default();
        let result = state.create_channel(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_get_channel_not_found() {
        let state = MedialiveState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.get_channel("nonexistent").await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_channels() {
        let state = MedialiveState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.list_channels().await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_channel_not_found() {
        let state = MedialiveState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.delete_channel("nonexistent").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_channel_full_crud() {
        let state = MedialiveState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateChannelRequest::default();
        create_req.name = Some("test-crud-resource".to_string());
        let create_result = state.create_channel(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let get_result = state.get_channel("test-crud-resource").await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let del_result = state.delete_channel("test-crud-resource").await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
