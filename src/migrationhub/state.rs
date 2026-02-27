use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::MigrationhubError;
use super::types::*;

#[allow(dead_code)]
struct MigrationhubStateInner {
    progress_update_streams: HashMap<String, StoredProgressUpdateStream>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredProgressUpdateStream {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct MigrationhubState {
    inner: Arc<Mutex<MigrationhubStateInner>>,
}

impl MigrationhubState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        MigrationhubState {
            inner: Arc::new(Mutex::new(MigrationhubStateInner {
                progress_update_streams: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_progress_update_stream(&self, req: CreateProgressUpdateStreamRequest) -> Result<CreateProgressUpdateStreamResponse, MigrationhubError> {
        let mut state = self.inner.lock().await;
        let name = req.progress_update_stream_name.clone();
        if state.progress_update_streams.contains_key(&name) {
            return Err(MigrationhubError::ResourceAlreadyExistsException(format!("ProgressUpdateStream {} already exists", name)));
        }
        let arn = format!("arn:aws:mgh:{}:{}:progress-update-streams/{}", state.region, state.account_id, name);
        state.progress_update_streams.insert(name.clone(), StoredProgressUpdateStream {
            name: name.clone(),
            arn: arn.clone(),
            tags: req.tags.unwrap_or_default(),
        });
        Ok(CreateProgressUpdateStreamResponse {
            progress_update_stream_arn: Some(arn),
            progress_update_stream_name: Some(name),
        })
    }

    #[allow(dead_code)]
    pub async fn describe_progress_update_stream(&self, req: DescribeProgressUpdateStreamRequest) -> Result<DescribeProgressUpdateStreamResponse, MigrationhubError> {
        let state = self.inner.lock().await;
        let name = req.progress_update_stream_name.or_else(|| req.progress_update_stream_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| MigrationhubError::ValidationException("Name or ARN required".to_string()))?;
        let stored = state.progress_update_streams.get(&name)
            .ok_or_else(|| MigrationhubError::ResourceNotFoundException(format!("ProgressUpdateStream {} not found", name)))?;
        Ok(DescribeProgressUpdateStreamResponse {
            progress_update_stream: ProgressUpdateStreamDetail {
                progress_update_stream_name: stored.name.clone(),
                progress_update_stream_arn: stored.arn.clone(),
                status: "ACTIVE".to_string(),
                tags: Some(stored.tags.clone()),
            },
        })
    }

    #[allow(dead_code)]
    pub async fn list_progress_update_streams(&self, _req: ListProgressUpdateStreamsRequest) -> Result<ListProgressUpdateStreamsResponse, MigrationhubError> {
        let state = self.inner.lock().await;
        let items: Vec<ProgressUpdateStreamDetail> = state.progress_update_streams.values().map(|s| ProgressUpdateStreamDetail {
            progress_update_stream_name: s.name.clone(),
            progress_update_stream_arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListProgressUpdateStreamsResponse {
            progress_update_streams: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_progress_update_stream(&self, req: DeleteProgressUpdateStreamRequest) -> Result<(), MigrationhubError> {
        let mut state = self.inner.lock().await;
        let name = req.progress_update_stream_name.or_else(|| req.progress_update_stream_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| MigrationhubError::ValidationException("Name or ARN required".to_string()))?;
        state.progress_update_streams.remove(&name)
            .ok_or_else(|| MigrationhubError::ResourceNotFoundException(format!("ProgressUpdateStream {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = MigrationhubState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_progress_update_stream() {
        let state = MigrationhubState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateProgressUpdateStreamRequest::default();
        let result = state.create_progress_update_stream(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_describe_progress_update_stream_not_found() {
        let state = MigrationhubState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DescribeProgressUpdateStreamRequest::default();
        let result = state.describe_progress_update_stream(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_progress_update_streams_empty() {
        let state = MigrationhubState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = ListProgressUpdateStreamsRequest::default();
        let result = state.list_progress_update_streams(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_progress_update_stream_not_found() {
        let state = MigrationhubState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DeleteProgressUpdateStreamRequest::default();
        let result = state.delete_progress_update_stream(req).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_progress_update_stream_create_and_list() {
        let state = MigrationhubState::new("123456789012".to_string(), "us-east-1".to_string());
        let create_req = CreateProgressUpdateStreamRequest::default();
        let _created = state.create_progress_update_stream(create_req).await.unwrap();
        let list_req = ListProgressUpdateStreamsRequest::default();
        let listed = state.list_progress_update_streams(list_req).await.unwrap();
        let _ = listed;
    }

    #[tokio::test]
    async fn test_progress_update_stream_full_crud() {
        let state = MigrationhubState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateProgressUpdateStreamRequest::default();
        create_req.progress_update_stream_name = "test-crud-resource".to_string();
        let create_result = state.create_progress_update_stream(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let mut get_req = DescribeProgressUpdateStreamRequest::default();
        get_req.progress_update_stream_name = Some("test-crud-resource".to_string());
        let get_result = state.describe_progress_update_stream(get_req).await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let mut del_req = DeleteProgressUpdateStreamRequest::default();
        del_req.progress_update_stream_name = Some("test-crud-resource".to_string());
        let del_result = state.delete_progress_update_stream(del_req).await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
