use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::KinesisvideostreamsError;
use super::types::*;

#[allow(dead_code)]
struct KinesisvideostreamsStateInner {
    streams: HashMap<String, StoredStream>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredStream {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct KinesisvideostreamsState {
    inner: Arc<Mutex<KinesisvideostreamsStateInner>>,
}

impl KinesisvideostreamsState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        KinesisvideostreamsState {
            inner: Arc::new(Mutex::new(KinesisvideostreamsStateInner {
                streams: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_stream(&self, req: CreateStreamRequest) -> Result<CreateStreamResponse, KinesisvideostreamsError> {
        let mut state = self.inner.lock().await;
        let name = req.stream_name.clone();
        if state.streams.contains_key(&name) {
            return Err(KinesisvideostreamsError::ResourceAlreadyExistsException(format!("Stream {} already exists", name)));
        }
        let arn = format!("arn:aws:kinesisvideo:{}:{}:streams/{}", state.region, state.account_id, name);
        state.streams.insert(name.clone(), StoredStream {
            name: name.clone(),
            arn: arn.clone(),
            tags: req.tags.unwrap_or_default(),
        });
        Ok(CreateStreamResponse {
            stream_arn: Some(arn),
            stream_name: Some(name),
        })
    }

    #[allow(dead_code)]
    pub async fn describe_stream(&self, req: DescribeStreamRequest) -> Result<DescribeStreamResponse, KinesisvideostreamsError> {
        let state = self.inner.lock().await;
        let name = req.stream_name.or_else(|| req.stream_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| KinesisvideostreamsError::ValidationException("Name or ARN required".to_string()))?;
        let stored = state.streams.get(&name)
            .ok_or_else(|| KinesisvideostreamsError::ResourceNotFoundException(format!("Stream {} not found", name)))?;
        Ok(DescribeStreamResponse {
            stream: StreamDetail {
                stream_name: stored.name.clone(),
                stream_arn: stored.arn.clone(),
                status: "ACTIVE".to_string(),
                tags: Some(stored.tags.clone()),
            },
        })
    }

    #[allow(dead_code)]
    pub async fn list_streams(&self, _req: ListStreamsRequest) -> Result<ListStreamsResponse, KinesisvideostreamsError> {
        let state = self.inner.lock().await;
        let items: Vec<StreamDetail> = state.streams.values().map(|s| StreamDetail {
            stream_name: s.name.clone(),
            stream_arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListStreamsResponse {
            streams: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_stream(&self, req: DeleteStreamRequest) -> Result<(), KinesisvideostreamsError> {
        let mut state = self.inner.lock().await;
        let name = req.stream_name.or_else(|| req.stream_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| KinesisvideostreamsError::ValidationException("Name or ARN required".to_string()))?;
        state.streams.remove(&name)
            .ok_or_else(|| KinesisvideostreamsError::ResourceNotFoundException(format!("Stream {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = KinesisvideostreamsState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_stream() {
        let state = KinesisvideostreamsState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateStreamRequest::default();
        let result = state.create_stream(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_describe_stream_not_found() {
        let state = KinesisvideostreamsState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DescribeStreamRequest::default();
        let result = state.describe_stream(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_streams_empty() {
        let state = KinesisvideostreamsState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = ListStreamsRequest::default();
        let result = state.list_streams(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_stream_not_found() {
        let state = KinesisvideostreamsState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DeleteStreamRequest::default();
        let result = state.delete_stream(req).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_stream_create_and_list() {
        let state = KinesisvideostreamsState::new("123456789012".to_string(), "us-east-1".to_string());
        let create_req = CreateStreamRequest::default();
        let _created = state.create_stream(create_req).await.unwrap();
        let list_req = ListStreamsRequest::default();
        let listed = state.list_streams(list_req).await.unwrap();
        let _ = listed;
    }

    #[tokio::test]
    async fn test_stream_full_crud() {
        let state = KinesisvideostreamsState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateStreamRequest::default();
        create_req.stream_name = "test-crud-resource".to_string();
        let create_result = state.create_stream(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let mut get_req = DescribeStreamRequest::default();
        get_req.stream_name = Some("test-crud-resource".to_string());
        let get_result = state.describe_stream(get_req).await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let mut del_req = DeleteStreamRequest::default();
        del_req.stream_name = Some("test-crud-resource".to_string());
        let del_result = state.delete_stream(del_req).await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
