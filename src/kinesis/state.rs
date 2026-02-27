use std::collections::HashMap;
use std::sync::Arc;

use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use tokio::sync::Mutex;

use super::error::KinesisError;
use super::types::*;

struct StoredRecord {
    sequence_number: String,
    data: String,
    partition_key: String,
    arrival_timestamp: f64,
}

struct KinesisStream {
    name: String,
    arn: String,
    status: String,
    shard_count: u32,
    retention_period_hours: u32,
    created: f64,
    tags: HashMap<String, String>,
    records: Vec<StoredRecord>, // all records across all shards
    next_sequence: u64,
}

// Shard iterator encoding: "stream_name:shard_id:sequence_position"
struct IteratorState {
    stream_name: String,
    shard_id: String,
    position: usize, // index into stream records
}

struct KinesisStateInner {
    streams: HashMap<String, KinesisStream>,
    iterators: HashMap<String, IteratorState>,
    account_id: String,
    region: String,
}

pub struct KinesisState {
    inner: Arc<Mutex<KinesisStateInner>>,
}

impl KinesisState {
    pub fn new(account_id: String, region: String) -> Self {
        KinesisState {
            inner: Arc::new(Mutex::new(KinesisStateInner {
                streams: HashMap::new(),
                iterators: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    fn now() -> f64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64()
    }

    fn make_shards(count: u32) -> Vec<Shard> {
        let max = u128::MAX;
        let step = max / count as u128;
        (0..count).map(|i| {
            let start = i as u128 * step;
            let end = if i + 1 == count { max } else { (i + 1) as u128 * step - 1 };
            Shard {
                shard_id: format!("shardId-{:012}", i),
                hash_key_range: HashKeyRange {
                    starting_hash_key: start.to_string(),
                    ending_hash_key: end.to_string(),
                },
                sequence_number_range: SequenceNumberRange {
                    starting_sequence_number: format!("{:049}", 0u64),
                    ending_sequence_number: None,
                },
            }
        }).collect()
    }

    fn resolve<'a>(state: &'a KinesisStateInner, name: Option<&'a str>, arn: Option<&str>) -> Option<&'a str> {
        if let Some(n) = name {
            if state.streams.contains_key(n) {
                return Some(n);
            }
        }
        if let Some(a) = arn {
            for (k, s) in &state.streams {
                if s.arn == a {
                    return Some(k.as_str());
                }
            }
        }
        None
    }

    pub async fn create_stream(&self, req: CreateStreamRequest) -> Result<(), KinesisError> {
        let mut state = self.inner.lock().await;
        if state.streams.contains_key(&req.stream_name) {
            return Err(KinesisError::ResourceInUseException(format!(
                "Stream {} already exists", req.stream_name
            )));
        }
        let shard_count = req.shard_count.unwrap_or(1);
        let arn = format!(
            "arn:aws:kinesis:{}:{}:stream/{}",
            state.region, state.account_id, req.stream_name
        );
        let mode = req.stream_mode_details.unwrap_or(StreamModeDetails {
            stream_mode: "PROVISIONED".to_string(),
        });
        state.streams.insert(req.stream_name.clone(), KinesisStream {
            name: req.stream_name,
            arn,
            status: "ACTIVE".to_string(),
            shard_count,
            retention_period_hours: 24,
            created: Self::now(),
            tags: HashMap::new(),
            records: Vec::new(),
            next_sequence: 1,
        });
        let _ = mode;
        Ok(())
    }

    pub async fn delete_stream(&self, req: DeleteStreamRequest) -> Result<(), KinesisError> {
        let mut state = self.inner.lock().await;
        let name = Self::resolve(&state, req.stream_name.as_deref(), req.stream_arn.as_deref())
            .ok_or_else(|| KinesisError::ResourceNotFoundException("Stream not found".to_string()))?
            .to_string();
        state.streams.remove(&name);
        Ok(())
    }

    pub async fn describe_stream(
        &self,
        req: DescribeStreamRequest,
    ) -> Result<DescribeStreamResponse, KinesisError> {
        let state = self.inner.lock().await;
        let name = Self::resolve(&state, req.stream_name.as_deref(), req.stream_arn.as_deref())
            .ok_or_else(|| KinesisError::ResourceNotFoundException("Stream not found".to_string()))?
            .to_string();
        let stream = &state.streams[&name];
        let shards = Self::make_shards(stream.shard_count);
        Ok(DescribeStreamResponse {
            stream_description: StreamDescription {
                stream_name: stream.name.clone(),
                stream_arn: stream.arn.clone(),
                stream_status: stream.status.clone(),
                stream_mode_details: StreamModeDetails { stream_mode: "PROVISIONED".to_string() },
                shards,
                has_more_shards: false,
                retention_period_hours: stream.retention_period_hours,
                stream_creation_timestamp: stream.created,
                enhanced_monitoring: vec![],
            },
        })
    }

    pub async fn describe_stream_summary(
        &self,
        req: DescribeStreamSummaryRequest,
    ) -> Result<DescribeStreamSummaryResponse, KinesisError> {
        let state = self.inner.lock().await;
        let name = Self::resolve(&state, req.stream_name.as_deref(), req.stream_arn.as_deref())
            .ok_or_else(|| KinesisError::ResourceNotFoundException("Stream not found".to_string()))?
            .to_string();
        let stream = &state.streams[&name];
        Ok(DescribeStreamSummaryResponse {
            stream_description_summary: StreamDescriptionSummary {
                stream_name: stream.name.clone(),
                stream_arn: stream.arn.clone(),
                stream_status: stream.status.clone(),
                stream_mode_details: StreamModeDetails { stream_mode: "PROVISIONED".to_string() },
                retention_period_hours: stream.retention_period_hours,
                stream_creation_timestamp: stream.created,
                open_shard_count: stream.shard_count,
                enhanced_monitoring: vec![],
            },
        })
    }

    pub async fn list_streams(&self, req: ListStreamsRequest) -> Result<ListStreamsResponse, KinesisError> {
        let state = self.inner.lock().await;
        let mut names: Vec<String> = state.streams.keys().cloned().collect();
        names.sort();
        if let Some(ref start) = req.exclusive_start_stream_name {
            names.retain(|n| n > start);
        }
        let limit = req.limit.unwrap_or(100);
        let has_more = names.len() > limit;
        names.truncate(limit);
        Ok(ListStreamsResponse {
            stream_names: names,
            has_more_streams: has_more,
            next_token: None,
        })
    }

    pub async fn put_record(&self, req: PutRecordRequest) -> Result<PutRecordResponse, KinesisError> {
        let mut state = self.inner.lock().await;
        let name = Self::resolve(&state, req.stream_name.as_deref(), req.stream_arn.as_deref())
            .ok_or_else(|| KinesisError::ResourceNotFoundException("Stream not found".to_string()))?
            .to_string();
        let stream = state.streams.get_mut(&name).unwrap();
        let seq = stream.next_sequence;
        stream.next_sequence += 1;
        let shard_idx = 0u32; // simple: all records go to shard 0
        let sequence_number = format!("{:049}", seq);
        stream.records.push(StoredRecord {
            sequence_number: sequence_number.clone(),
            data: req.data,
            partition_key: req.partition_key,
            arrival_timestamp: Self::now(),
        });
        Ok(PutRecordResponse {
            shard_id: format!("shardId-{:012}", shard_idx),
            sequence_number,
            encryption_type: "NONE".to_string(),
        })
    }

    pub async fn put_records(&self, req: PutRecordsRequest) -> Result<PutRecordsResponse, KinesisError> {
        let mut state = self.inner.lock().await;
        let name = Self::resolve(&state, req.stream_name.as_deref(), req.stream_arn.as_deref())
            .ok_or_else(|| KinesisError::ResourceNotFoundException("Stream not found".to_string()))?
            .to_string();
        let stream = state.streams.get_mut(&name).unwrap();
        let mut results = Vec::with_capacity(req.records.len());
        for record in req.records {
            let seq = stream.next_sequence;
            stream.next_sequence += 1;
            let sequence_number = format!("{:049}", seq);
            stream.records.push(StoredRecord {
                sequence_number: sequence_number.clone(),
                data: record.data,
                partition_key: record.partition_key,
                arrival_timestamp: Self::now(),
            });
            results.push(PutRecordsResultEntry {
                shard_id: "shardId-000000000000".to_string(),
                sequence_number,
            });
        }
        Ok(PutRecordsResponse {
            failed_record_count: 0,
            records: results,
            encryption_type: "NONE".to_string(),
        })
    }

    pub async fn get_shard_iterator(
        &self,
        req: GetShardIteratorRequest,
    ) -> Result<GetShardIteratorResponse, KinesisError> {
        let mut state = self.inner.lock().await;
        let name = Self::resolve(&state, req.stream_name.as_deref(), req.stream_arn.as_deref())
            .ok_or_else(|| KinesisError::ResourceNotFoundException("Stream not found".to_string()))?
            .to_string();
        let stream = &state.streams[&name];
        let position = match req.shard_iterator_type.as_str() {
            "TRIM_HORIZON" => 0,
            "LATEST" => stream.records.len(),
            "AT_SEQUENCE_NUMBER" | "AFTER_SEQUENCE_NUMBER" => {
                if let Some(seq) = &req.starting_sequence_number {
                    let pos = stream.records.iter().position(|r| &r.sequence_number == seq);
                    match (req.shard_iterator_type.as_str(), pos) {
                        ("AT_SEQUENCE_NUMBER", Some(p)) => p,
                        ("AFTER_SEQUENCE_NUMBER", Some(p)) => p + 1,
                        _ => stream.records.len(),
                    }
                } else {
                    0
                }
            }
            _ => stream.records.len(),
        };
        // Encode iterator as base64(stream_name:shard_id:position)
        let iter_data = format!("{}:{}:{}", name, req.shard_id, position);
        let shard_iterator = BASE64.encode(iter_data.as_bytes());
        state.iterators.insert(shard_iterator.clone(), IteratorState {
            stream_name: name,
            shard_id: req.shard_id,
            position,
        });
        Ok(GetShardIteratorResponse { shard_iterator })
    }

    pub async fn get_records(&self, req: GetRecordsRequest) -> Result<GetRecordsResponse, KinesisError> {
        let mut state = self.inner.lock().await;
        let iter_state = state.iterators.get(&req.shard_iterator)
            .ok_or_else(|| KinesisError::ExpiredIteratorException("Iterator expired or invalid".to_string()))?;
        let stream_name = iter_state.stream_name.clone();
        let shard_id = iter_state.shard_id.clone();
        let position = iter_state.position;

        let stream = state.streams.get(&stream_name)
            .ok_or_else(|| KinesisError::ResourceNotFoundException("Stream not found".to_string()))?;

        let limit = req.limit.unwrap_or(10000).min(10000);
        let records_slice = &stream.records[position.min(stream.records.len())..];
        let take = records_slice.len().min(limit);
        let records: Vec<Record> = records_slice[..take].iter().map(|r| Record {
            sequence_number: r.sequence_number.clone(),
            approximate_arrival_timestamp: r.arrival_timestamp,
            data: r.data.clone(),
            partition_key: r.partition_key.clone(),
            encryption_type: "NONE".to_string(),
        }).collect();

        let new_position = position + take;
        let total_records = stream.records.len();

        // Create next shard iterator
        let iter_data = format!("{}:{}:{}", stream_name, shard_id, new_position);
        let next_iterator = BASE64.encode(iter_data.as_bytes());
        state.iterators.insert(next_iterator.clone(), IteratorState {
            stream_name: stream_name.clone(),
            shard_id: shard_id.clone(),
            position: new_position,
        });
        // Remove old iterator
        state.iterators.remove(&req.shard_iterator);

        let millis_behind = if new_position >= total_records { 0 } else { 1000 };

        Ok(GetRecordsResponse {
            records,
            next_shard_iterator: Some(next_iterator),
            millis_behind_latest: millis_behind,
        })
    }

    pub async fn list_shards(&self, req: ListShardsRequest) -> Result<ListShardsResponse, KinesisError> {
        let state = self.inner.lock().await;
        let name = Self::resolve(&state, req.stream_name.as_deref(), req.stream_arn.as_deref())
            .ok_or_else(|| KinesisError::ResourceNotFoundException("Stream not found".to_string()))?
            .to_string();
        let stream = &state.streams[&name];
        let shards = Self::make_shards(stream.shard_count);
        Ok(ListShardsResponse {
            shards,
            next_token: None,
        })
    }

    pub async fn add_tags_to_stream(&self, req: AddTagsToStreamRequest) -> Result<(), KinesisError> {
        let mut state = self.inner.lock().await;
        let name = Self::resolve(&state, req.stream_name.as_deref(), req.stream_arn.as_deref())
            .ok_or_else(|| KinesisError::ResourceNotFoundException("Stream not found".to_string()))?
            .to_string();
        let stream = state.streams.get_mut(&name).unwrap();
        for (k, v) in req.tags {
            stream.tags.insert(k, v);
        }
        Ok(())
    }

    pub async fn remove_tags_from_stream(&self, req: RemoveTagsFromStreamRequest) -> Result<(), KinesisError> {
        let mut state = self.inner.lock().await;
        let name = Self::resolve(&state, req.stream_name.as_deref(), req.stream_arn.as_deref())
            .ok_or_else(|| KinesisError::ResourceNotFoundException("Stream not found".to_string()))?
            .to_string();
        let stream = state.streams.get_mut(&name).unwrap();
        for k in &req.tag_keys {
            stream.tags.remove(k);
        }
        Ok(())
    }

    pub async fn list_tags_for_stream(
        &self,
        req: ListTagsForStreamRequest,
    ) -> Result<ListTagsForStreamResponse, KinesisError> {
        let state = self.inner.lock().await;
        let name = Self::resolve(&state, req.stream_name.as_deref(), req.stream_arn.as_deref())
            .ok_or_else(|| KinesisError::ResourceNotFoundException("Stream not found".to_string()))?
            .to_string();
        let stream = &state.streams[&name];
        let mut tags: Vec<Tag> = stream.tags.iter().map(|(k, v)| Tag {
            key: k.clone(),
            value: v.clone(),
        }).collect();
        tags.sort_by(|a, b| a.key.cmp(&b.key));
        let limit = req.limit.unwrap_or(10);
        let has_more = tags.len() > limit;
        tags.truncate(limit);
        Ok(ListTagsForStreamResponse { tags, has_more_tags: has_more })
    }

    pub async fn increase_stream_retention_period(
        &self,
        req: IncreaseStreamRetentionPeriodRequest,
    ) -> Result<(), KinesisError> {
        let mut state = self.inner.lock().await;
        let name = Self::resolve(&state, req.stream_name.as_deref(), req.stream_arn.as_deref())
            .ok_or_else(|| KinesisError::ResourceNotFoundException("Stream not found".to_string()))?
            .to_string();
        let stream = state.streams.get_mut(&name).unwrap();
        if req.retention_period_hours <= stream.retention_period_hours {
            return Err(KinesisError::InvalidArgumentException(
                "New retention period must be greater than current".to_string(),
            ));
        }
        stream.retention_period_hours = req.retention_period_hours;
        Ok(())
    }

    pub async fn decrease_stream_retention_period(
        &self,
        req: DecreaseStreamRetentionPeriodRequest,
    ) -> Result<(), KinesisError> {
        let mut state = self.inner.lock().await;
        let name = Self::resolve(&state, req.stream_name.as_deref(), req.stream_arn.as_deref())
            .ok_or_else(|| KinesisError::ResourceNotFoundException("Stream not found".to_string()))?
            .to_string();
        let stream = state.streams.get_mut(&name).unwrap();
        if req.retention_period_hours >= stream.retention_period_hours {
            return Err(KinesisError::InvalidArgumentException(
                "New retention period must be less than current".to_string(),
            ));
        }
        stream.retention_period_hours = req.retention_period_hours;
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = KinesisState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_stream() {
        let state = KinesisState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateStreamRequest::default();
        let result = state.create_stream(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_stream_not_found() {
        let state = KinesisState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DeleteStreamRequest::default();
        let result = state.delete_stream(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_streams_empty() {
        let state = KinesisState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = ListStreamsRequest::default();
        let result = state.list_streams(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_put_record() {
        let state = KinesisState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = PutRecordRequest::default();
        let _ = state.put_record(req).await;
    }
    #[tokio::test]
    async fn test_put_records() {
        let state = KinesisState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = PutRecordsRequest::default();
        let _ = state.put_records(req).await;
    }
    #[tokio::test]
    async fn test_get_records_not_found() {
        let state = KinesisState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = GetRecordsRequest::default();
        let result = state.get_records(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_shards_empty() {
        let state = KinesisState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = ListShardsRequest::default();
        let result = state.list_shards(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_add_tags_to_stream() {
        let state = KinesisState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = AddTagsToStreamRequest::default();
        let _ = state.add_tags_to_stream(req).await;
    }
    #[tokio::test]
    async fn test_remove_tags_from_stream() {
        let state = KinesisState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = RemoveTagsFromStreamRequest::default();
        let _ = state.remove_tags_from_stream(req).await;
    }

    #[tokio::test]
    async fn test_stream_create_and_list() {
        let state = KinesisState::new("123456789012".to_string(), "us-east-1".to_string());
        let create_req = CreateStreamRequest::default();
        let _created = state.create_stream(create_req).await.unwrap();
        let list_req = ListStreamsRequest::default();
        let listed = state.list_streams(list_req).await.unwrap();
        let _ = listed;
    }

    // --- Comprehensive additional tests ---

    fn make_state() -> KinesisState {
        KinesisState::new("123456789012".to_string(), "us-east-1".to_string())
    }

    async fn create_stream(state: &KinesisState, name: &str) {
        state.create_stream(CreateStreamRequest {
            stream_name: name.to_string(),
            shard_count: Some(1),
            ..Default::default()
        }).await.unwrap();
    }

    #[tokio::test]
    async fn test_create_stream_duplicate() {
        let state = make_state();
        create_stream(&state, "dup-stream").await;
        let result = state.create_stream(CreateStreamRequest {
            stream_name: "dup-stream".to_string(),
            ..Default::default()
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_stream_success() {
        let state = make_state();
        create_stream(&state, "del-stream").await;
        let result = state.delete_stream(DeleteStreamRequest {
            stream_name: Some("del-stream".to_string()),
            ..Default::default()
        }).await;
        assert!(result.is_ok());
        // Verify gone
        let list = state.list_streams(ListStreamsRequest::default()).await.unwrap();
        assert!(list.stream_names.is_empty());
    }

    #[tokio::test]
    async fn test_describe_stream() {
        let state = make_state();
        create_stream(&state, "desc-stream").await;
        let result = state.describe_stream(DescribeStreamRequest {
            stream_name: Some("desc-stream".to_string()),
            ..Default::default()
        }).await.unwrap();
        assert_eq!(result.stream_description.stream_name, "desc-stream");
        assert_eq!(result.stream_description.stream_status, "ACTIVE");
        assert!(!result.stream_description.shards.is_empty());
    }

    #[tokio::test]
    async fn test_describe_stream_not_found() {
        let state = make_state();
        let result = state.describe_stream(DescribeStreamRequest {
            stream_name: Some("nope".to_string()),
            ..Default::default()
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_describe_stream_summary() {
        let state = make_state();
        create_stream(&state, "sum-stream").await;
        let result = state.describe_stream_summary(DescribeStreamSummaryRequest {
            stream_name: Some("sum-stream".to_string()),
            ..Default::default()
        }).await.unwrap();
        assert_eq!(result.stream_description_summary.stream_name, "sum-stream");
        assert_eq!(result.stream_description_summary.open_shard_count, 1);
    }

    #[tokio::test]
    async fn test_describe_stream_summary_not_found() {
        let state = make_state();
        let result = state.describe_stream_summary(DescribeStreamSummaryRequest {
            stream_name: Some("nope".to_string()),
            ..Default::default()
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_streams_multiple() {
        let state = make_state();
        create_stream(&state, "stream-a").await;
        create_stream(&state, "stream-b").await;
        create_stream(&state, "stream-c").await;
        let result = state.list_streams(ListStreamsRequest::default()).await.unwrap();
        assert_eq!(result.stream_names.len(), 3);
        assert!(!result.has_more_streams);
    }

    #[tokio::test]
    async fn test_list_streams_with_limit() {
        let state = make_state();
        create_stream(&state, "s1").await;
        create_stream(&state, "s2").await;
        create_stream(&state, "s3").await;
        let result = state.list_streams(ListStreamsRequest {
            limit: Some(2),
            ..Default::default()
        }).await.unwrap();
        assert_eq!(result.stream_names.len(), 2);
        assert!(result.has_more_streams);
    }

    #[tokio::test]
    async fn test_put_record_and_get_records() {
        let state = make_state();
        create_stream(&state, "rec-stream").await;

        // Put a record
        let put_result = state.put_record(PutRecordRequest {
            stream_name: Some("rec-stream".to_string()),
            data: "dGVzdCBkYXRh".to_string(), // base64("test data")
            partition_key: "pk1".to_string(),
            ..Default::default()
        }).await.unwrap();
        assert!(!put_result.sequence_number.is_empty());
        assert_eq!(put_result.shard_id, "shardId-000000000000");

        // Get shard iterator
        let iter_resp = state.get_shard_iterator(GetShardIteratorRequest {
            stream_name: Some("rec-stream".to_string()),
            shard_id: "shardId-000000000000".to_string(),
            shard_iterator_type: "TRIM_HORIZON".to_string(),
            ..Default::default()
        }).await.unwrap();

        // Get records
        let get_resp = state.get_records(GetRecordsRequest {
            shard_iterator: iter_resp.shard_iterator,
            ..Default::default()
        }).await.unwrap();
        assert_eq!(get_resp.records.len(), 1);
        assert_eq!(get_resp.records[0].data, "dGVzdCBkYXRh");
        assert_eq!(get_resp.records[0].partition_key, "pk1");
        assert!(get_resp.next_shard_iterator.is_some());
    }

    #[tokio::test]
    async fn test_put_record_stream_not_found() {
        let state = make_state();
        let result = state.put_record(PutRecordRequest {
            stream_name: Some("nope".to_string()),
            data: "data".to_string(),
            partition_key: "pk".to_string(),
            ..Default::default()
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_put_records_multiple() {
        let state = make_state();
        create_stream(&state, "multi-stream").await;
        let result = state.put_records(PutRecordsRequest {
            stream_name: Some("multi-stream".to_string()),
            records: vec![
                PutRecordsRequestEntry { data: "d1".to_string(), partition_key: "p1".to_string() },
                PutRecordsRequestEntry { data: "d2".to_string(), partition_key: "p2".to_string() },
                PutRecordsRequestEntry { data: "d3".to_string(), partition_key: "p3".to_string() },
            ],
            ..Default::default()
        }).await.unwrap();
        assert_eq!(result.records.len(), 3);
        assert_eq!(result.failed_record_count, 0);
    }

    #[tokio::test]
    async fn test_put_records_stream_not_found() {
        let state = make_state();
        let result = state.put_records(PutRecordsRequest {
            stream_name: Some("nope".to_string()),
            records: vec![PutRecordsRequestEntry { data: "d".to_string(), partition_key: "p".to_string() }],
            ..Default::default()
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_shard_iterator_types() {
        let state = make_state();
        create_stream(&state, "iter-stream").await;

        // Put some records first
        state.put_record(PutRecordRequest {
            stream_name: Some("iter-stream".to_string()),
            data: "data1".to_string(),
            partition_key: "pk".to_string(),
            ..Default::default()
        }).await.unwrap();

        // TRIM_HORIZON
        let iter = state.get_shard_iterator(GetShardIteratorRequest {
            stream_name: Some("iter-stream".to_string()),
            shard_id: "shardId-000000000000".to_string(),
            shard_iterator_type: "TRIM_HORIZON".to_string(),
            ..Default::default()
        }).await.unwrap();
        assert!(!iter.shard_iterator.is_empty());

        // LATEST
        let iter = state.get_shard_iterator(GetShardIteratorRequest {
            stream_name: Some("iter-stream".to_string()),
            shard_id: "shardId-000000000000".to_string(),
            shard_iterator_type: "LATEST".to_string(),
            ..Default::default()
        }).await.unwrap();
        assert!(!iter.shard_iterator.is_empty());
    }

    #[tokio::test]
    async fn test_get_shard_iterator_stream_not_found() {
        let state = make_state();
        let result = state.get_shard_iterator(GetShardIteratorRequest {
            stream_name: Some("nope".to_string()),
            shard_id: "shardId-000000000000".to_string(),
            shard_iterator_type: "TRIM_HORIZON".to_string(),
            ..Default::default()
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_records_with_limit() {
        let state = make_state();
        create_stream(&state, "lim-stream").await;
        for i in 0..5 {
            state.put_record(PutRecordRequest {
                stream_name: Some("lim-stream".to_string()),
                data: format!("data{}", i),
                partition_key: "pk".to_string(),
                ..Default::default()
            }).await.unwrap();
        }

        let iter = state.get_shard_iterator(GetShardIteratorRequest {
            stream_name: Some("lim-stream".to_string()),
            shard_id: "shardId-000000000000".to_string(),
            shard_iterator_type: "TRIM_HORIZON".to_string(),
            ..Default::default()
        }).await.unwrap();

        let result = state.get_records(GetRecordsRequest {
            shard_iterator: iter.shard_iterator,
            limit: Some(3),
        }).await.unwrap();
        assert_eq!(result.records.len(), 3);
        assert!(result.millis_behind_latest > 0);
    }

    #[tokio::test]
    async fn test_get_records_empty_stream() {
        let state = make_state();
        create_stream(&state, "empty-stream").await;

        let iter = state.get_shard_iterator(GetShardIteratorRequest {
            stream_name: Some("empty-stream".to_string()),
            shard_id: "shardId-000000000000".to_string(),
            shard_iterator_type: "TRIM_HORIZON".to_string(),
            ..Default::default()
        }).await.unwrap();

        let result = state.get_records(GetRecordsRequest {
            shard_iterator: iter.shard_iterator,
            ..Default::default()
        }).await.unwrap();
        assert!(result.records.is_empty());
        assert_eq!(result.millis_behind_latest, 0);
    }

    #[tokio::test]
    async fn test_list_shards_success() {
        let state = make_state();
        state.create_stream(CreateStreamRequest {
            stream_name: "shard-stream".to_string(),
            shard_count: Some(3),
            ..Default::default()
        }).await.unwrap();

        let result = state.list_shards(ListShardsRequest {
            stream_name: Some("shard-stream".to_string()),
            ..Default::default()
        }).await.unwrap();
        assert_eq!(result.shards.len(), 3);
    }

    #[tokio::test]
    async fn test_add_and_list_tags() {
        let state = make_state();
        create_stream(&state, "tag-stream").await;
        let mut tags = std::collections::HashMap::new();
        tags.insert("env".to_string(), "test".to_string());
        tags.insert("team".to_string(), "backend".to_string());

        state.add_tags_to_stream(AddTagsToStreamRequest {
            stream_name: Some("tag-stream".to_string()),
            tags,
            ..Default::default()
        }).await.unwrap();

        let result = state.list_tags_for_stream(ListTagsForStreamRequest {
            stream_name: Some("tag-stream".to_string()),
            ..Default::default()
        }).await.unwrap();
        assert_eq!(result.tags.len(), 2);
        assert!(!result.has_more_tags);
    }

    #[tokio::test]
    async fn test_remove_tags_from_stream_success() {
        let state = make_state();
        create_stream(&state, "untag-stream").await;
        let mut tags = std::collections::HashMap::new();
        tags.insert("env".to_string(), "test".to_string());
        tags.insert("team".to_string(), "backend".to_string());

        state.add_tags_to_stream(AddTagsToStreamRequest {
            stream_name: Some("untag-stream".to_string()),
            tags,
            ..Default::default()
        }).await.unwrap();

        state.remove_tags_from_stream(RemoveTagsFromStreamRequest {
            stream_name: Some("untag-stream".to_string()),
            tag_keys: vec!["env".to_string()],
            ..Default::default()
        }).await.unwrap();

        let result = state.list_tags_for_stream(ListTagsForStreamRequest {
            stream_name: Some("untag-stream".to_string()),
            ..Default::default()
        }).await.unwrap();
        assert_eq!(result.tags.len(), 1);
        assert_eq!(result.tags[0].key, "team");
    }

    #[tokio::test]
    async fn test_add_tags_stream_not_found() {
        let state = make_state();
        let result = state.add_tags_to_stream(AddTagsToStreamRequest {
            stream_name: Some("nope".to_string()),
            tags: std::collections::HashMap::new(),
            ..Default::default()
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_remove_tags_stream_not_found() {
        let state = make_state();
        let result = state.remove_tags_from_stream(RemoveTagsFromStreamRequest {
            stream_name: Some("nope".to_string()),
            tag_keys: vec!["env".to_string()],
            ..Default::default()
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_tags_stream_not_found() {
        let state = make_state();
        let result = state.list_tags_for_stream(ListTagsForStreamRequest {
            stream_name: Some("nope".to_string()),
            ..Default::default()
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_increase_retention_period() {
        let state = make_state();
        create_stream(&state, "ret-stream").await;
        let result = state.increase_stream_retention_period(IncreaseStreamRetentionPeriodRequest {
            stream_name: Some("ret-stream".to_string()),
            retention_period_hours: 48,
            ..Default::default()
        }).await;
        assert!(result.is_ok());

        let desc = state.describe_stream(DescribeStreamRequest {
            stream_name: Some("ret-stream".to_string()),
            ..Default::default()
        }).await.unwrap();
        assert_eq!(desc.stream_description.retention_period_hours, 48);
    }

    #[tokio::test]
    async fn test_increase_retention_period_too_low() {
        let state = make_state();
        create_stream(&state, "ret2-stream").await;
        let result = state.increase_stream_retention_period(IncreaseStreamRetentionPeriodRequest {
            stream_name: Some("ret2-stream".to_string()),
            retention_period_hours: 12, // less than default 24
            ..Default::default()
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_decrease_retention_period() {
        let state = make_state();
        create_stream(&state, "dec-stream").await;
        // First increase
        state.increase_stream_retention_period(IncreaseStreamRetentionPeriodRequest {
            stream_name: Some("dec-stream".to_string()),
            retention_period_hours: 48,
            ..Default::default()
        }).await.unwrap();

        // Then decrease
        let result = state.decrease_stream_retention_period(DecreaseStreamRetentionPeriodRequest {
            stream_name: Some("dec-stream".to_string()),
            retention_period_hours: 24,
            ..Default::default()
        }).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_decrease_retention_period_too_high() {
        let state = make_state();
        create_stream(&state, "dec2-stream").await;
        let result = state.decrease_stream_retention_period(DecreaseStreamRetentionPeriodRequest {
            stream_name: Some("dec2-stream".to_string()),
            retention_period_hours: 48, // greater than default 24
            ..Default::default()
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_increase_retention_not_found() {
        let state = make_state();
        let result = state.increase_stream_retention_period(IncreaseStreamRetentionPeriodRequest {
            stream_name: Some("nope".to_string()),
            retention_period_hours: 48,
            ..Default::default()
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_decrease_retention_not_found() {
        let state = make_state();
        let result = state.decrease_stream_retention_period(DecreaseStreamRetentionPeriodRequest {
            stream_name: Some("nope".to_string()),
            retention_period_hours: 12,
            ..Default::default()
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_records_chained_iterators() {
        let state = make_state();
        create_stream(&state, "chain-stream").await;

        // Put 3 records
        for i in 0..3 {
            state.put_record(PutRecordRequest {
                stream_name: Some("chain-stream".to_string()),
                data: format!("rec{}", i),
                partition_key: "pk".to_string(),
                ..Default::default()
            }).await.unwrap();
        }

        // Get first 2
        let iter = state.get_shard_iterator(GetShardIteratorRequest {
            stream_name: Some("chain-stream".to_string()),
            shard_id: "shardId-000000000000".to_string(),
            shard_iterator_type: "TRIM_HORIZON".to_string(),
            ..Default::default()
        }).await.unwrap();

        let first = state.get_records(GetRecordsRequest {
            shard_iterator: iter.shard_iterator,
            limit: Some(2),
        }).await.unwrap();
        assert_eq!(first.records.len(), 2);
        let next_iter = first.next_shard_iterator.unwrap();

        // Get remaining with chained iterator
        let second = state.get_records(GetRecordsRequest {
            shard_iterator: next_iter,
            ..Default::default()
        }).await.unwrap();
        assert_eq!(second.records.len(), 1);
    }

    #[tokio::test]
    async fn test_list_tags_with_limit() {
        let state = make_state();
        create_stream(&state, "tags-lim").await;
        let mut tags = std::collections::HashMap::new();
        for i in 0..15 {
            tags.insert(format!("key{:02}", i), format!("val{}", i));
        }
        state.add_tags_to_stream(AddTagsToStreamRequest {
            stream_name: Some("tags-lim".to_string()),
            tags,
            ..Default::default()
        }).await.unwrap();

        let result = state.list_tags_for_stream(ListTagsForStreamRequest {
            stream_name: Some("tags-lim".to_string()),
            limit: Some(5),
            ..Default::default()
        }).await.unwrap();
        assert_eq!(result.tags.len(), 5);
        assert!(result.has_more_tags);
    }
}
