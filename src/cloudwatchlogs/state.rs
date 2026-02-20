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
        let mut tags = req.tags.unwrap_or_default();
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
