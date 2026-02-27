use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;
use uuid::Uuid;

use super::error::CwlError;
use super::types::*;

struct StoredLogEvent {
    timestamp: i64,
    message: String,
    ingestion_time: i64,
    event_id: String,
}

struct LogStreamData {
    log_stream_name: String,
    arn: String,
    creation_time: i64,
    events: Vec<StoredLogEvent>,
    sequence_token: u64,
}

struct LogGroupData {
    log_group_name: String,
    arn: String,
    creation_time: i64,
    retention_in_days: Option<i64>,
    tags: HashMap<String, String>,
    streams: HashMap<String, LogStreamData>,
}

struct CwlStateInner {
    log_groups: HashMap<String, LogGroupData>,
    account_id: String,
    region: String,
}

pub struct CwlState {
    inner: Arc<Mutex<CwlStateInner>>,
}

impl CwlState {
    pub fn new(account_id: String, region: String) -> Self {
        CwlState {
            inner: Arc::new(Mutex::new(CwlStateInner {
                log_groups: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    fn now_ms() -> i64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64
    }

    pub async fn create_log_group(&self, req: CreateLogGroupRequest) -> Result<(), CwlError> {
        let mut state = self.inner.lock().await;
        if state.log_groups.contains_key(&req.log_group_name) {
            return Err(CwlError::ResourceAlreadyExistsException(format!(
                "The specified log group already exists: {}", req.log_group_name
            )));
        }
        let arn = format!(
            "arn:aws:logs:{}:{}:log-group:{}",
            state.region, state.account_id, req.log_group_name
        );
        let now = Self::now_ms();
        let tags = req.tags.unwrap_or_default();
        state.log_groups.insert(req.log_group_name.clone(), LogGroupData {
            log_group_name: req.log_group_name,
            arn,
            creation_time: now,
            retention_in_days: None,
            tags,
            streams: HashMap::new(),
        });
        Ok(())
    }

    pub async fn delete_log_group(&self, req: DeleteLogGroupRequest) -> Result<(), CwlError> {
        let mut state = self.inner.lock().await;
        if state.log_groups.remove(&req.log_group_name).is_none() {
            return Err(CwlError::ResourceNotFoundException(format!(
                "The specified log group does not exist: {}", req.log_group_name
            )));
        }
        Ok(())
    }

    pub async fn describe_log_groups(&self, req: DescribeLogGroupsRequest) -> Result<DescribeLogGroupsResponse, CwlError> {
        let state = self.inner.lock().await;
        let mut groups: Vec<LogGroup> = state.log_groups.values()
            .filter(|g| {
                req.log_group_name_prefix.as_ref()
                    .map(|p| g.log_group_name.starts_with(p.as_str()))
                    .unwrap_or(true)
                && req.log_group_name_pattern.as_ref()
                    .map(|p| g.log_group_name.contains(p.as_str()))
                    .unwrap_or(true)
            })
            .map(|g| LogGroup {
                log_group_name: g.log_group_name.clone(),
                arn: format!("{}:*", g.arn),
                creation_time: g.creation_time,
                retention_in_days: g.retention_in_days,
                metric_filter_count: 0,
                stored_bytes: g.streams.values()
                    .map(|s| s.events.iter().map(|e| e.message.len() as i64).sum::<i64>())
                    .sum(),
            })
            .collect();
        groups.sort_by(|a, b| a.log_group_name.cmp(&b.log_group_name));
        let limit = req.limit.unwrap_or(50);
        let has_more = groups.len() > limit;
        groups.truncate(limit);
        Ok(DescribeLogGroupsResponse {
            log_groups: groups,
            next_token: if has_more { Some("next".to_string()) } else { None },
        })
    }

    pub async fn create_log_stream(&self, req: CreateLogStreamRequest) -> Result<(), CwlError> {
        let mut state = self.inner.lock().await;
        let group = state.log_groups.get_mut(&req.log_group_name)
            .ok_or_else(|| CwlError::ResourceNotFoundException(format!(
                "The specified log group does not exist: {}", req.log_group_name
            )))?;
        if group.streams.contains_key(&req.log_stream_name) {
            return Err(CwlError::ResourceAlreadyExistsException(format!(
                "The specified log stream already exists: {}", req.log_stream_name
            )));
        }
        let arn = format!(
            "arn:aws:logs:us-east-1:000000000000:log-group:{}:log-stream:{}",
            req.log_group_name, req.log_stream_name
        );
        let now = Self::now_ms();
        group.streams.insert(req.log_stream_name.clone(), LogStreamData {
            log_stream_name: req.log_stream_name,
            arn,
            creation_time: now,
            events: Vec::new(),
            sequence_token: 1,
        });
        Ok(())
    }

    pub async fn delete_log_stream(&self, req: DeleteLogStreamRequest) -> Result<(), CwlError> {
        let mut state = self.inner.lock().await;
        let group = state.log_groups.get_mut(&req.log_group_name)
            .ok_or_else(|| CwlError::ResourceNotFoundException(format!(
                "The specified log group does not exist: {}", req.log_group_name
            )))?;
        if group.streams.remove(&req.log_stream_name).is_none() {
            return Err(CwlError::ResourceNotFoundException(format!(
                "The specified log stream does not exist: {}", req.log_stream_name
            )));
        }
        Ok(())
    }

    pub async fn describe_log_streams(&self, req: DescribeLogStreamsRequest) -> Result<DescribeLogStreamsResponse, CwlError> {
        let state = self.inner.lock().await;
        let group_name = req.log_group_name.as_deref().unwrap_or_default();
        let group = state.log_groups.get(group_name)
            .ok_or_else(|| CwlError::ResourceNotFoundException(format!(
                "The specified log group does not exist: {}", group_name
            )))?;
        let mut streams: Vec<LogStream> = group.streams.values()
            .filter(|s| {
                req.log_stream_name_prefix.as_ref()
                    .map(|p| s.log_stream_name.starts_with(p.as_str()))
                    .unwrap_or(true)
            })
            .map(|s| LogStream {
                log_stream_name: s.log_stream_name.clone(),
                creation_time: s.creation_time,
                first_event_timestamp: s.events.first().map(|e| e.timestamp),
                last_event_timestamp: s.events.last().map(|e| e.timestamp),
                last_ingestion_time: s.events.last().map(|e| e.ingestion_time),
                upload_sequence_token: s.sequence_token.to_string(),
                arn: s.arn.clone(),
                stored_bytes: s.events.iter().map(|e| e.message.len() as i64).sum(),
            })
            .collect();
        streams.sort_by(|a, b| a.log_stream_name.cmp(&b.log_stream_name));
        if req.descending.unwrap_or(false) {
            streams.reverse();
        }
        let limit = req.limit.unwrap_or(50);
        let has_more = streams.len() > limit;
        streams.truncate(limit);
        Ok(DescribeLogStreamsResponse {
            log_streams: streams,
            next_token: if has_more { Some("next".to_string()) } else { None },
        })
    }

    pub async fn put_log_events(&self, req: PutLogEventsRequest) -> Result<PutLogEventsResponse, CwlError> {
        let mut state = self.inner.lock().await;
        let group = state.log_groups.get_mut(&req.log_group_name)
            .ok_or_else(|| CwlError::ResourceNotFoundException(format!(
                "The specified log group does not exist: {}", req.log_group_name
            )))?;
        let stream = group.streams.get_mut(&req.log_stream_name)
            .ok_or_else(|| CwlError::ResourceNotFoundException(format!(
                "The specified log stream does not exist: {}", req.log_stream_name
            )))?;
        let now = Self::now_ms();
        for event in req.log_events {
            stream.events.push(StoredLogEvent {
                timestamp: event.timestamp,
                message: event.message,
                ingestion_time: now,
                event_id: Uuid::new_v4().to_string(),
            });
        }
        stream.sequence_token += 1;
        let next_token = stream.sequence_token.to_string();
        Ok(PutLogEventsResponse {
            next_sequence_token: next_token,
            rejected_log_events_info: None,
        })
    }

    pub async fn get_log_events(&self, req: GetLogEventsRequest) -> Result<GetLogEventsResponse, CwlError> {
        let state = self.inner.lock().await;
        let group = state.log_groups.get(&req.log_group_name)
            .ok_or_else(|| CwlError::ResourceNotFoundException(format!(
                "The specified log group does not exist: {}", req.log_group_name
            )))?;
        let stream = group.streams.get(&req.log_stream_name)
            .ok_or_else(|| CwlError::ResourceNotFoundException(format!(
                "The specified log stream does not exist: {}", req.log_stream_name
            )))?;
        let mut events: Vec<OutputLogEvent> = stream.events.iter()
            .filter(|e| {
                req.start_time.map(|s| e.timestamp >= s).unwrap_or(true)
                && req.end_time.map(|en| e.timestamp <= en).unwrap_or(true)
            })
            .map(|e| OutputLogEvent {
                timestamp: e.timestamp,
                message: e.message.clone(),
                ingestion_time: e.ingestion_time,
            })
            .collect();
        if !req.start_from_head.unwrap_or(false) {
            // Default is from head for forward iteration
        }
        let limit = req.limit.unwrap_or(10000);
        events.truncate(limit);
        Ok(GetLogEventsResponse {
            events,
            next_forward_token: "f/next".to_string(),
            next_backward_token: "b/start".to_string(),
        })
    }

    pub async fn filter_log_events(&self, req: FilterLogEventsRequest) -> Result<FilterLogEventsResponse, CwlError> {
        let state = self.inner.lock().await;
        let group = state.log_groups.get(&req.log_group_name)
            .ok_or_else(|| CwlError::ResourceNotFoundException(format!(
                "The specified log group does not exist: {}", req.log_group_name
            )))?;
        let stream_filter = req.log_stream_names.as_ref();
        let pattern = req.filter_pattern.as_deref().unwrap_or("").to_lowercase();
        let mut events: Vec<FilteredLogEvent> = group.streams.values()
            .filter(|s| stream_filter.map(|f| f.contains(&s.log_stream_name)).unwrap_or(true))
            .flat_map(|s| s.events.iter().map(move |e| (s.log_stream_name.clone(), e)))
            .filter(|(_, e)| {
                req.start_time.map(|t| e.timestamp >= t).unwrap_or(true)
                && req.end_time.map(|t| e.timestamp <= t).unwrap_or(true)
                && (pattern.is_empty() || e.message.to_lowercase().contains(&pattern))
            })
            .map(|(stream_name, e)| FilteredLogEvent {
                log_stream_name: stream_name,
                timestamp: e.timestamp,
                message: e.message.clone(),
                ingestion_time: e.ingestion_time,
                event_id: e.event_id.clone(),
            })
            .collect();
        events.sort_by_key(|e| e.timestamp);
        let limit = req.limit.unwrap_or(10000);
        let has_more = events.len() > limit;
        events.truncate(limit);
        Ok(FilterLogEventsResponse {
            events,
            next_token: if has_more { Some("next".to_string()) } else { None },
        })
    }

    pub async fn put_retention_policy(&self, req: PutRetentionPolicyRequest) -> Result<(), CwlError> {
        let mut state = self.inner.lock().await;
        let group = state.log_groups.get_mut(&req.log_group_name)
            .ok_or_else(|| CwlError::ResourceNotFoundException(format!(
                "The specified log group does not exist: {}", req.log_group_name
            )))?;
        group.retention_in_days = Some(req.retention_in_days);
        Ok(())
    }

    pub async fn delete_retention_policy(&self, req: DeleteRetentionPolicyRequest) -> Result<(), CwlError> {
        let mut state = self.inner.lock().await;
        let group = state.log_groups.get_mut(&req.log_group_name)
            .ok_or_else(|| CwlError::ResourceNotFoundException(format!(
                "The specified log group does not exist: {}", req.log_group_name
            )))?;
        group.retention_in_days = None;
        Ok(())
    }

    pub async fn tag_log_group(&self, req: TagLogGroupRequest) -> Result<(), CwlError> {
        let mut state = self.inner.lock().await;
        let group = state.log_groups.get_mut(&req.log_group_name)
            .ok_or_else(|| CwlError::ResourceNotFoundException(format!(
                "The specified log group does not exist: {}", req.log_group_name
            )))?;
        for (k, v) in req.tags { group.tags.insert(k, v); }
        Ok(())
    }

    pub async fn untag_log_group(&self, req: UntagLogGroupRequest) -> Result<(), CwlError> {
        let mut state = self.inner.lock().await;
        let group = state.log_groups.get_mut(&req.log_group_name)
            .ok_or_else(|| CwlError::ResourceNotFoundException(format!(
                "The specified log group does not exist: {}", req.log_group_name
            )))?;
        for key in &req.tags { group.tags.remove(key); }
        Ok(())
    }

    pub async fn list_tags_log_group(&self, req: ListTagsLogGroupRequest) -> Result<ListTagsLogGroupResponse, CwlError> {
        let state = self.inner.lock().await;
        let group = state.log_groups.get(&req.log_group_name)
            .ok_or_else(|| CwlError::ResourceNotFoundException(format!(
                "The specified log group does not exist: {}", req.log_group_name
            )))?;
        Ok(ListTagsLogGroupResponse { tags: group.tags.clone() })
    }

    pub async fn tag_resource(&self, req: TagResourceRequest) -> Result<(), CwlError> {
        let mut state = self.inner.lock().await;
        // Find group by ARN
        for group in state.log_groups.values_mut() {
            if group.arn == req.resource_arn || format!("{}:*", group.arn) == req.resource_arn {
                for (k, v) in req.tags { group.tags.insert(k, v); }
                return Ok(());
            }
        }
        Err(CwlError::ResourceNotFoundException(format!("Resource not found: {}", req.resource_arn)))
    }

    pub async fn untag_resource(&self, req: UntagResourceRequest) -> Result<(), CwlError> {
        let mut state = self.inner.lock().await;
        for group in state.log_groups.values_mut() {
            if group.arn == req.resource_arn || format!("{}:*", group.arn) == req.resource_arn {
                for key in &req.tag_keys { group.tags.remove(key); }
                return Ok(());
            }
        }
        Err(CwlError::ResourceNotFoundException(format!("Resource not found: {}", req.resource_arn)))
    }

    pub async fn list_tags_for_resource(&self, req: ListTagsForResourceRequest) -> Result<ListTagsForResourceResponse, CwlError> {
        let state = self.inner.lock().await;
        for group in state.log_groups.values() {
            if group.arn == req.resource_arn || format!("{}:*", group.arn) == req.resource_arn {
                return Ok(ListTagsForResourceResponse { tags: group.tags.clone() });
            }
        }
        Err(CwlError::ResourceNotFoundException(format!("Resource not found: {}", req.resource_arn)))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = CwlState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_log_group() {
        let state = CwlState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateLogGroupRequest::default();
        let result = state.create_log_group(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_log_group_not_found() {
        let state = CwlState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DeleteLogGroupRequest::default();
        let result = state.delete_log_group(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_describe_log_groups_not_found() {
        let state = CwlState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DescribeLogGroupsRequest::default();
        let result = state.describe_log_groups(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_create_log_stream() {
        let state = CwlState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateLogStreamRequest::default();
        let result = state.create_log_stream(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_delete_log_stream_not_found() {
        let state = CwlState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DeleteLogStreamRequest::default();
        let result = state.delete_log_stream(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_describe_log_streams_not_found() {
        let state = CwlState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DescribeLogStreamsRequest::default();
        let result = state.describe_log_streams(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_put_log_events() {
        let state = CwlState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = PutLogEventsRequest::default();
        let _ = state.put_log_events(req).await;
    }
    #[tokio::test]
    async fn test_get_log_events_not_found() {
        let state = CwlState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = GetLogEventsRequest::default();
        let result = state.get_log_events(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_filter_log_events() {
        let state = CwlState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = FilterLogEventsRequest::default();
        let _ = state.filter_log_events(req).await;
    }
    #[tokio::test]
    async fn test_put_retention_policy() {
        let state = CwlState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = PutRetentionPolicyRequest::default();
        let _ = state.put_retention_policy(req).await;
    }
    #[tokio::test]
    async fn test_delete_retention_policy_not_found() {
        let state = CwlState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DeleteRetentionPolicyRequest::default();
        let result = state.delete_retention_policy(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_tag_log_group() {
        let state = CwlState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = TagLogGroupRequest::default();
        let _ = state.tag_log_group(req).await;
    }
    #[tokio::test]
    async fn test_untag_log_group() {
        let state = CwlState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = UntagLogGroupRequest::default();
        let _ = state.untag_log_group(req).await;
    }
    #[tokio::test]
    async fn test_list_tags_log_group_empty() {
        let state = CwlState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = ListTagsLogGroupRequest::default();
        let result = state.list_tags_log_group(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_tag_resource() {
        let state = CwlState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = TagResourceRequest::default();
        let _ = state.tag_resource(req).await;
    }
    #[tokio::test]
    async fn test_untag_resource() {
        let state = CwlState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = UntagResourceRequest::default();
        let _ = state.untag_resource(req).await;
    }
    #[tokio::test]
    async fn test_list_tags_for_resource_empty() {
        let state = CwlState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = ListTagsForResourceRequest::default();
        let result = state.list_tags_for_resource(req).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_log_group_create_and_list() {
        let state = CwlState::new("123456789012".to_string(), "us-east-1".to_string());
        let create_req = CreateLogGroupRequest::default();
        let _created = state.create_log_group(create_req).await.unwrap();
        let list_req = ListTagsLogGroupRequest::default();
        let listed = state.list_tags_log_group(list_req).await.unwrap();
        let _ = listed;
    }

    // --- Comprehensive additional tests ---

    fn make_state() -> CwlState {
        CwlState::new("123456789012".to_string(), "us-east-1".to_string())
    }

    async fn setup_group_and_stream(state: &CwlState, group: &str, stream: &str) {
        state.create_log_group(CreateLogGroupRequest {
            log_group_name: group.to_string(),
            ..Default::default()
        }).await.unwrap();
        state.create_log_stream(CreateLogStreamRequest {
            log_group_name: group.to_string(),
            log_stream_name: stream.to_string(),
        }).await.unwrap();
    }

    #[tokio::test]
    async fn test_create_log_group_duplicate() {
        let state = make_state();
        state.create_log_group(CreateLogGroupRequest {
            log_group_name: "dup-group".to_string(),
            ..Default::default()
        }).await.unwrap();
        let result = state.create_log_group(CreateLogGroupRequest {
            log_group_name: "dup-group".to_string(),
            ..Default::default()
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_log_group_with_tags() {
        let state = make_state();
        let mut tags = HashMap::new();
        tags.insert("env".to_string(), "test".to_string());
        state.create_log_group(CreateLogGroupRequest {
            log_group_name: "tagged-group".to_string(),
            tags: Some(tags),
        }).await.unwrap();

        let result = state.list_tags_log_group(ListTagsLogGroupRequest {
            log_group_name: "tagged-group".to_string(),
        }).await.unwrap();
        assert_eq!(result.tags.get("env").unwrap(), "test");
    }

    #[tokio::test]
    async fn test_delete_log_group_success() {
        let state = make_state();
        state.create_log_group(CreateLogGroupRequest {
            log_group_name: "del-group".to_string(),
            ..Default::default()
        }).await.unwrap();
        let result = state.delete_log_group(DeleteLogGroupRequest {
            log_group_name: "del-group".to_string(),
        }).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_describe_log_groups_with_prefix() {
        let state = make_state();
        state.create_log_group(CreateLogGroupRequest {
            log_group_name: "/app/service1".to_string(),
            ..Default::default()
        }).await.unwrap();
        state.create_log_group(CreateLogGroupRequest {
            log_group_name: "/app/service2".to_string(),
            ..Default::default()
        }).await.unwrap();
        state.create_log_group(CreateLogGroupRequest {
            log_group_name: "/other/thing".to_string(),
            ..Default::default()
        }).await.unwrap();

        let result = state.describe_log_groups(DescribeLogGroupsRequest {
            log_group_name_prefix: Some("/app".to_string()),
            ..Default::default()
        }).await.unwrap();
        assert_eq!(result.log_groups.len(), 2);
    }

    #[tokio::test]
    async fn test_describe_log_groups_with_pattern() {
        let state = make_state();
        state.create_log_group(CreateLogGroupRequest {
            log_group_name: "my-service-prod".to_string(),
            ..Default::default()
        }).await.unwrap();
        state.create_log_group(CreateLogGroupRequest {
            log_group_name: "my-service-dev".to_string(),
            ..Default::default()
        }).await.unwrap();
        state.create_log_group(CreateLogGroupRequest {
            log_group_name: "other-service".to_string(),
            ..Default::default()
        }).await.unwrap();

        let result = state.describe_log_groups(DescribeLogGroupsRequest {
            log_group_name_pattern: Some("my-service".to_string()),
            ..Default::default()
        }).await.unwrap();
        assert_eq!(result.log_groups.len(), 2);
    }

    #[tokio::test]
    async fn test_create_log_stream_success() {
        let state = make_state();
        state.create_log_group(CreateLogGroupRequest {
            log_group_name: "g1".to_string(),
            ..Default::default()
        }).await.unwrap();
        let result = state.create_log_stream(CreateLogStreamRequest {
            log_group_name: "g1".to_string(),
            log_stream_name: "s1".to_string(),
        }).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_log_stream_duplicate() {
        let state = make_state();
        setup_group_and_stream(&state, "g1", "s1").await;
        let result = state.create_log_stream(CreateLogStreamRequest {
            log_group_name: "g1".to_string(),
            log_stream_name: "s1".to_string(),
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_log_stream_group_not_found() {
        let state = make_state();
        let result = state.create_log_stream(CreateLogStreamRequest {
            log_group_name: "nope".to_string(),
            log_stream_name: "s1".to_string(),
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_log_stream_success() {
        let state = make_state();
        setup_group_and_stream(&state, "g1", "s1").await;
        let result = state.delete_log_stream(DeleteLogStreamRequest {
            log_group_name: "g1".to_string(),
            log_stream_name: "s1".to_string(),
        }).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_log_stream_stream_not_found() {
        let state = make_state();
        state.create_log_group(CreateLogGroupRequest {
            log_group_name: "g1".to_string(),
            ..Default::default()
        }).await.unwrap();
        let result = state.delete_log_stream(DeleteLogStreamRequest {
            log_group_name: "g1".to_string(),
            log_stream_name: "nope".to_string(),
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_describe_log_streams_success() {
        let state = make_state();
        setup_group_and_stream(&state, "g1", "stream-a").await;
        state.create_log_stream(CreateLogStreamRequest {
            log_group_name: "g1".to_string(),
            log_stream_name: "stream-b".to_string(),
        }).await.unwrap();

        let result = state.describe_log_streams(DescribeLogStreamsRequest {
            log_group_name: Some("g1".to_string()),
            ..Default::default()
        }).await.unwrap();
        assert_eq!(result.log_streams.len(), 2);
    }

    #[tokio::test]
    async fn test_describe_log_streams_with_prefix() {
        let state = make_state();
        setup_group_and_stream(&state, "g1", "app-stream").await;
        state.create_log_stream(CreateLogStreamRequest {
            log_group_name: "g1".to_string(),
            log_stream_name: "other-stream".to_string(),
        }).await.unwrap();

        let result = state.describe_log_streams(DescribeLogStreamsRequest {
            log_group_name: Some("g1".to_string()),
            log_stream_name_prefix: Some("app".to_string()),
            ..Default::default()
        }).await.unwrap();
        assert_eq!(result.log_streams.len(), 1);
        assert_eq!(result.log_streams[0].log_stream_name, "app-stream");
    }

    #[tokio::test]
    async fn test_describe_log_streams_descending() {
        let state = make_state();
        setup_group_and_stream(&state, "g1", "a-stream").await;
        state.create_log_stream(CreateLogStreamRequest {
            log_group_name: "g1".to_string(),
            log_stream_name: "z-stream".to_string(),
        }).await.unwrap();

        let result = state.describe_log_streams(DescribeLogStreamsRequest {
            log_group_name: Some("g1".to_string()),
            descending: Some(true),
            ..Default::default()
        }).await.unwrap();
        assert_eq!(result.log_streams[0].log_stream_name, "z-stream");
    }

    #[tokio::test]
    async fn test_put_and_get_log_events() {
        let state = make_state();
        setup_group_and_stream(&state, "g1", "s1").await;

        let put_result = state.put_log_events(PutLogEventsRequest {
            log_group_name: "g1".to_string(),
            log_stream_name: "s1".to_string(),
            log_events: vec![
                InputLogEvent { timestamp: 1000, message: "first event".to_string() },
                InputLogEvent { timestamp: 2000, message: "second event".to_string() },
            ],
        }).await.unwrap();
        assert!(!put_result.next_sequence_token.is_empty());

        let get_result = state.get_log_events(GetLogEventsRequest {
            log_group_name: "g1".to_string(),
            log_stream_name: "s1".to_string(),
            ..Default::default()
        }).await.unwrap();
        assert_eq!(get_result.events.len(), 2);
        assert_eq!(get_result.events[0].message, "first event");
    }

    #[tokio::test]
    async fn test_get_log_events_with_time_range() {
        let state = make_state();
        setup_group_and_stream(&state, "g1", "s1").await;
        state.put_log_events(PutLogEventsRequest {
            log_group_name: "g1".to_string(),
            log_stream_name: "s1".to_string(),
            log_events: vec![
                InputLogEvent { timestamp: 1000, message: "early".to_string() },
                InputLogEvent { timestamp: 5000, message: "middle".to_string() },
                InputLogEvent { timestamp: 9000, message: "late".to_string() },
            ],
        }).await.unwrap();

        let result = state.get_log_events(GetLogEventsRequest {
            log_group_name: "g1".to_string(),
            log_stream_name: "s1".to_string(),
            start_time: Some(3000),
            end_time: Some(7000),
            ..Default::default()
        }).await.unwrap();
        assert_eq!(result.events.len(), 1);
        assert_eq!(result.events[0].message, "middle");
    }

    #[tokio::test]
    async fn test_get_log_events_with_limit() {
        let state = make_state();
        setup_group_and_stream(&state, "g1", "s1").await;
        let events: Vec<InputLogEvent> = (0..10).map(|i| InputLogEvent {
            timestamp: i * 1000,
            message: format!("event {}", i),
        }).collect();
        state.put_log_events(PutLogEventsRequest {
            log_group_name: "g1".to_string(),
            log_stream_name: "s1".to_string(),
            log_events: events,
        }).await.unwrap();

        let result = state.get_log_events(GetLogEventsRequest {
            log_group_name: "g1".to_string(),
            log_stream_name: "s1".to_string(),
            limit: Some(3),
            ..Default::default()
        }).await.unwrap();
        assert_eq!(result.events.len(), 3);
    }

    #[tokio::test]
    async fn test_get_log_events_group_not_found() {
        let state = make_state();
        let result = state.get_log_events(GetLogEventsRequest {
            log_group_name: "nope".to_string(),
            log_stream_name: "s1".to_string(),
            ..Default::default()
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_log_events_stream_not_found() {
        let state = make_state();
        state.create_log_group(CreateLogGroupRequest {
            log_group_name: "g1".to_string(),
            ..Default::default()
        }).await.unwrap();
        let result = state.get_log_events(GetLogEventsRequest {
            log_group_name: "g1".to_string(),
            log_stream_name: "nope".to_string(),
            ..Default::default()
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_put_log_events_group_not_found() {
        let state = make_state();
        let result = state.put_log_events(PutLogEventsRequest {
            log_group_name: "nope".to_string(),
            log_stream_name: "s1".to_string(),
            log_events: vec![InputLogEvent { timestamp: 1000, message: "msg".to_string() }],
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_put_log_events_stream_not_found() {
        let state = make_state();
        state.create_log_group(CreateLogGroupRequest {
            log_group_name: "g1".to_string(),
            ..Default::default()
        }).await.unwrap();
        let result = state.put_log_events(PutLogEventsRequest {
            log_group_name: "g1".to_string(),
            log_stream_name: "nope".to_string(),
            log_events: vec![InputLogEvent { timestamp: 1000, message: "msg".to_string() }],
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_filter_log_events_success() {
        let state = make_state();
        setup_group_and_stream(&state, "g1", "s1").await;
        state.create_log_stream(CreateLogStreamRequest {
            log_group_name: "g1".to_string(),
            log_stream_name: "s2".to_string(),
        }).await.unwrap();

        state.put_log_events(PutLogEventsRequest {
            log_group_name: "g1".to_string(),
            log_stream_name: "s1".to_string(),
            log_events: vec![
                InputLogEvent { timestamp: 1000, message: "ERROR something broke".to_string() },
                InputLogEvent { timestamp: 2000, message: "INFO all good".to_string() },
            ],
        }).await.unwrap();
        state.put_log_events(PutLogEventsRequest {
            log_group_name: "g1".to_string(),
            log_stream_name: "s2".to_string(),
            log_events: vec![
                InputLogEvent { timestamp: 1500, message: "ERROR another failure".to_string() },
            ],
        }).await.unwrap();

        let result = state.filter_log_events(FilterLogEventsRequest {
            log_group_name: "g1".to_string(),
            filter_pattern: Some("ERROR".to_string()),
            ..Default::default()
        }).await.unwrap();
        assert_eq!(result.events.len(), 2);
    }

    #[tokio::test]
    async fn test_filter_log_events_by_stream_names() {
        let state = make_state();
        setup_group_and_stream(&state, "g1", "s1").await;
        state.create_log_stream(CreateLogStreamRequest {
            log_group_name: "g1".to_string(),
            log_stream_name: "s2".to_string(),
        }).await.unwrap();

        state.put_log_events(PutLogEventsRequest {
            log_group_name: "g1".to_string(),
            log_stream_name: "s1".to_string(),
            log_events: vec![InputLogEvent { timestamp: 1000, message: "from s1".to_string() }],
        }).await.unwrap();
        state.put_log_events(PutLogEventsRequest {
            log_group_name: "g1".to_string(),
            log_stream_name: "s2".to_string(),
            log_events: vec![InputLogEvent { timestamp: 2000, message: "from s2".to_string() }],
        }).await.unwrap();

        let result = state.filter_log_events(FilterLogEventsRequest {
            log_group_name: "g1".to_string(),
            log_stream_names: Some(vec!["s1".to_string()]),
            ..Default::default()
        }).await.unwrap();
        assert_eq!(result.events.len(), 1);
        assert_eq!(result.events[0].log_stream_name, "s1");
    }

    #[tokio::test]
    async fn test_filter_log_events_with_time_range() {
        let state = make_state();
        setup_group_and_stream(&state, "g1", "s1").await;
        state.put_log_events(PutLogEventsRequest {
            log_group_name: "g1".to_string(),
            log_stream_name: "s1".to_string(),
            log_events: vec![
                InputLogEvent { timestamp: 1000, message: "early".to_string() },
                InputLogEvent { timestamp: 5000, message: "mid".to_string() },
                InputLogEvent { timestamp: 9000, message: "late".to_string() },
            ],
        }).await.unwrap();

        let result = state.filter_log_events(FilterLogEventsRequest {
            log_group_name: "g1".to_string(),
            start_time: Some(3000),
            end_time: Some(7000),
            ..Default::default()
        }).await.unwrap();
        assert_eq!(result.events.len(), 1);
        assert_eq!(result.events[0].message, "mid");
    }

    #[tokio::test]
    async fn test_filter_log_events_group_not_found() {
        let state = make_state();
        let result = state.filter_log_events(FilterLogEventsRequest {
            log_group_name: "nope".to_string(),
            ..Default::default()
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_put_and_delete_retention_policy() {
        let state = make_state();
        state.create_log_group(CreateLogGroupRequest {
            log_group_name: "ret-group".to_string(),
            ..Default::default()
        }).await.unwrap();

        state.put_retention_policy(PutRetentionPolicyRequest {
            log_group_name: "ret-group".to_string(),
            retention_in_days: 30,
        }).await.unwrap();

        let groups = state.describe_log_groups(DescribeLogGroupsRequest {
            log_group_name_prefix: Some("ret-group".to_string()),
            ..Default::default()
        }).await.unwrap();
        assert_eq!(groups.log_groups[0].retention_in_days, Some(30));

        state.delete_retention_policy(DeleteRetentionPolicyRequest {
            log_group_name: "ret-group".to_string(),
        }).await.unwrap();

        let groups = state.describe_log_groups(DescribeLogGroupsRequest {
            log_group_name_prefix: Some("ret-group".to_string()),
            ..Default::default()
        }).await.unwrap();
        assert_eq!(groups.log_groups[0].retention_in_days, None);
    }

    #[tokio::test]
    async fn test_put_retention_policy_not_found() {
        let state = make_state();
        let result = state.put_retention_policy(PutRetentionPolicyRequest {
            log_group_name: "nope".to_string(),
            retention_in_days: 7,
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_tag_and_untag_log_group() {
        let state = make_state();
        state.create_log_group(CreateLogGroupRequest {
            log_group_name: "tag-group".to_string(),
            ..Default::default()
        }).await.unwrap();

        let mut tags = HashMap::new();
        tags.insert("env".to_string(), "prod".to_string());
        tags.insert("team".to_string(), "platform".to_string());
        state.tag_log_group(TagLogGroupRequest {
            log_group_name: "tag-group".to_string(),
            tags,
        }).await.unwrap();

        let result = state.list_tags_log_group(ListTagsLogGroupRequest {
            log_group_name: "tag-group".to_string(),
        }).await.unwrap();
        assert_eq!(result.tags.len(), 2);

        state.untag_log_group(UntagLogGroupRequest {
            log_group_name: "tag-group".to_string(),
            tags: vec!["env".to_string()],
        }).await.unwrap();

        let result = state.list_tags_log_group(ListTagsLogGroupRequest {
            log_group_name: "tag-group".to_string(),
        }).await.unwrap();
        assert_eq!(result.tags.len(), 1);
        assert!(result.tags.contains_key("team"));
    }

    #[tokio::test]
    async fn test_tag_log_group_not_found() {
        let state = make_state();
        let result = state.tag_log_group(TagLogGroupRequest {
            log_group_name: "nope".to_string(),
            tags: HashMap::new(),
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_untag_log_group_not_found() {
        let state = make_state();
        let result = state.untag_log_group(UntagLogGroupRequest {
            log_group_name: "nope".to_string(),
            tags: vec!["k".to_string()],
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_tag_resource_by_arn() {
        let state = make_state();
        state.create_log_group(CreateLogGroupRequest {
            log_group_name: "arn-group".to_string(),
            ..Default::default()
        }).await.unwrap();

        let arn = format!("arn:aws:logs:us-east-1:123456789012:log-group:arn-group");
        let mut tags = HashMap::new();
        tags.insert("via".to_string(), "arn".to_string());
        state.tag_resource(TagResourceRequest {
            resource_arn: arn.clone(),
            tags,
        }).await.unwrap();

        let result = state.list_tags_for_resource(ListTagsForResourceRequest {
            resource_arn: arn,
        }).await.unwrap();
        assert_eq!(result.tags.get("via").unwrap(), "arn");
    }

    #[tokio::test]
    async fn test_untag_resource_by_arn() {
        let state = make_state();
        state.create_log_group(CreateLogGroupRequest {
            log_group_name: "untag-arn".to_string(),
            tags: Some({
                let mut m = HashMap::new();
                m.insert("k1".to_string(), "v1".to_string());
                m.insert("k2".to_string(), "v2".to_string());
                m
            }),
        }).await.unwrap();

        let arn = "arn:aws:logs:us-east-1:123456789012:log-group:untag-arn".to_string();
        state.untag_resource(UntagResourceRequest {
            resource_arn: arn.clone(),
            tag_keys: vec!["k1".to_string()],
        }).await.unwrap();

        let result = state.list_tags_for_resource(ListTagsForResourceRequest {
            resource_arn: arn,
        }).await.unwrap();
        assert_eq!(result.tags.len(), 1);
        assert!(result.tags.contains_key("k2"));
    }

    #[tokio::test]
    async fn test_tag_resource_not_found() {
        let state = make_state();
        let result = state.tag_resource(TagResourceRequest {
            resource_arn: "arn:aws:logs:us-east-1:123456789012:log-group:nope".to_string(),
            tags: HashMap::new(),
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_untag_resource_not_found() {
        let state = make_state();
        let result = state.untag_resource(UntagResourceRequest {
            resource_arn: "arn:aws:logs:us-east-1:123456789012:log-group:nope".to_string(),
            tag_keys: vec!["k".to_string()],
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_tags_for_resource_not_found() {
        let state = make_state();
        let result = state.list_tags_for_resource(ListTagsForResourceRequest {
            resource_arn: "arn:aws:logs:us-east-1:123456789012:log-group:nope".to_string(),
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_describe_log_groups_with_limit() {
        let state = make_state();
        for i in 0..5 {
            state.create_log_group(CreateLogGroupRequest {
                log_group_name: format!("group-{}", i),
                ..Default::default()
            }).await.unwrap();
        }

        let result = state.describe_log_groups(DescribeLogGroupsRequest {
            limit: Some(3),
            ..Default::default()
        }).await.unwrap();
        assert_eq!(result.log_groups.len(), 3);
        assert!(result.next_token.is_some());
    }

    #[tokio::test]
    async fn test_describe_log_streams_with_limit() {
        let state = make_state();
        state.create_log_group(CreateLogGroupRequest {
            log_group_name: "g1".to_string(),
            ..Default::default()
        }).await.unwrap();
        for i in 0..5 {
            state.create_log_stream(CreateLogStreamRequest {
                log_group_name: "g1".to_string(),
                log_stream_name: format!("stream-{}", i),
            }).await.unwrap();
        }

        let result = state.describe_log_streams(DescribeLogStreamsRequest {
            log_group_name: Some("g1".to_string()),
            limit: Some(2),
            ..Default::default()
        }).await.unwrap();
        assert_eq!(result.log_streams.len(), 2);
        assert!(result.next_token.is_some());
    }

    #[tokio::test]
    async fn test_put_log_events_sequence_token_increments() {
        let state = make_state();
        setup_group_and_stream(&state, "g1", "s1").await;

        let r1 = state.put_log_events(PutLogEventsRequest {
            log_group_name: "g1".to_string(),
            log_stream_name: "s1".to_string(),
            log_events: vec![InputLogEvent { timestamp: 1000, message: "first".to_string() }],
        }).await.unwrap();

        let r2 = state.put_log_events(PutLogEventsRequest {
            log_group_name: "g1".to_string(),
            log_stream_name: "s1".to_string(),
            log_events: vec![InputLogEvent { timestamp: 2000, message: "second".to_string() }],
        }).await.unwrap();

        assert_ne!(r1.next_sequence_token, r2.next_sequence_token);
    }

    #[tokio::test]
    async fn test_filter_log_events_case_insensitive() {
        let state = make_state();
        setup_group_and_stream(&state, "g1", "s1").await;
        state.put_log_events(PutLogEventsRequest {
            log_group_name: "g1".to_string(),
            log_stream_name: "s1".to_string(),
            log_events: vec![
                InputLogEvent { timestamp: 1000, message: "ERROR something".to_string() },
                InputLogEvent { timestamp: 2000, message: "error something else".to_string() },
                InputLogEvent { timestamp: 3000, message: "info ok".to_string() },
            ],
        }).await.unwrap();

        let result = state.filter_log_events(FilterLogEventsRequest {
            log_group_name: "g1".to_string(),
            filter_pattern: Some("error".to_string()),
            ..Default::default()
        }).await.unwrap();
        assert_eq!(result.events.len(), 2);
    }
}
