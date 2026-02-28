use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;
use uuid::Uuid;

use super::error::FirehoseError;
use super::stream::{DeliveryStream, StoredRecord};
use super::types::*;

struct FirehoseStateInner {
    streams: HashMap<String, DeliveryStream>,
    account_id: String,
    region: String,
}

pub struct FirehoseState {
    inner: Arc<Mutex<FirehoseStateInner>>,
}

impl FirehoseState {
    pub fn new(account_id: String, region: String) -> Self {
        FirehoseState {
            inner: Arc::new(Mutex::new(FirehoseStateInner {
                streams: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    pub async fn create_delivery_stream(
        &self,
        req: CreateDeliveryStreamRequest,
    ) -> Result<CreateDeliveryStreamResponse, FirehoseError> {
        let mut state = self.inner.lock().await;

        if state.streams.contains_key(&req.delivery_stream_name) {
            return Err(FirehoseError::ResourceInUseException(format!(
                "Delivery stream {} already exists",
                req.delivery_stream_name
            )));
        }

        let arn = format!(
            "arn:aws:firehose:{}:{}:deliverystream/{}",
            state.region, state.account_id, req.delivery_stream_name
        );

        let mut stream =
            DeliveryStream::new(req.delivery_stream_name.clone(), arn.clone(), req.delivery_stream_type);

        if let Some(tags) = req.tags {
            for tag in tags {
                stream
                    .tags
                    .insert(tag.key, tag.value.unwrap_or_default());
            }
        }

        state.streams.insert(req.delivery_stream_name, stream);

        Ok(CreateDeliveryStreamResponse {
            delivery_stream_arn: arn,
        })
    }

    pub async fn delete_delivery_stream(
        &self,
        req: DeleteDeliveryStreamRequest,
    ) -> Result<(), FirehoseError> {
        let mut state = self.inner.lock().await;

        if state.streams.remove(&req.delivery_stream_name).is_none() {
            return Err(FirehoseError::ResourceNotFoundException(format!(
                "Delivery stream {} under account {} not found.",
                req.delivery_stream_name, state.account_id
            )));
        }

        Ok(())
    }

    pub async fn describe_delivery_stream(
        &self,
        req: DescribeDeliveryStreamRequest,
    ) -> Result<DescribeDeliveryStreamResponse, FirehoseError> {
        let state = self.inner.lock().await;

        let stream =
            state
                .streams
                .get(&req.delivery_stream_name)
                .ok_or_else(|| {
                    FirehoseError::ResourceNotFoundException(format!(
                        "Delivery stream {} under account {} not found.",
                        req.delivery_stream_name, state.account_id
                    ))
                })?;

        let destinations: Vec<DestinationDescription> = stream
            .destinations
            .iter()
            .map(|d| DestinationDescription {
                destination_id: d.destination_id.clone(),
            })
            .collect();

        Ok(DescribeDeliveryStreamResponse {
            delivery_stream_description: DeliveryStreamDescription {
                delivery_stream_name: stream.name.clone(),
                delivery_stream_arn: stream.arn.clone(),
                delivery_stream_status: stream.status.clone(),
                delivery_stream_type: stream.stream_type.clone(),
                version_id: stream.version_id.clone(),
                create_timestamp: stream.create_timestamp,
                last_update_timestamp: stream.last_update_timestamp,
                destinations,
                has_more_destinations: false,
                delivery_stream_encryption_configuration: EncryptionConfig {
                    status: "DISABLED".to_string(),
                },
            },
        })
    }

    pub async fn list_delivery_streams(
        &self,
        req: ListDeliveryStreamsRequest,
    ) -> Result<ListDeliveryStreamsResponse, FirehoseError> {
        let state = self.inner.lock().await;

        let mut names: Vec<String> = state.streams.keys().cloned().collect();
        names.sort();

        // Filter by stream type if specified
        if let Some(ref stream_type) = req.delivery_stream_type {
            names.retain(|name| {
                state
                    .streams
                    .get(name)
                    .map(|s| &s.stream_type == stream_type)
                    .unwrap_or(false)
            });
        }

        // Apply exclusive start
        if let Some(ref start_name) = req.exclusive_start_delivery_stream_name {
            if let Some(pos) = names.iter().position(|n| n > start_name) {
                names = names[pos..].to_vec();
            } else {
                names.clear();
            }
        }

        let limit = req.limit.unwrap_or(10000);
        let has_more = names.len() > limit;
        names.truncate(limit);

        Ok(ListDeliveryStreamsResponse {
            delivery_stream_names: names,
            has_more_delivery_streams: has_more,
        })
    }

    pub async fn update_destination(
        &self,
        req: UpdateDestinationRequest,
    ) -> Result<(), FirehoseError> {
        let mut state = self.inner.lock().await;
        let account_id = state.account_id.clone();

        let stream =
            state
                .streams
                .get_mut(&req.delivery_stream_name)
                .ok_or_else(|| {
                    FirehoseError::ResourceNotFoundException(format!(
                        "Delivery stream {} under account {} not found.",
                        req.delivery_stream_name, account_id
                    ))
                })?;

        if stream.version_id != req.current_delivery_stream_version_id {
            return Err(FirehoseError::ConcurrentModificationException(format!(
                "Version mismatch: current version is {}, provided version is {}",
                stream.version_id, req.current_delivery_stream_version_id
            )));
        }

        // Verify destination exists
        if !stream
            .destinations
            .iter()
            .any(|d| d.destination_id == req.destination_id)
        {
            return Err(FirehoseError::InvalidArgumentException(format!(
                "Destination Id {} not found",
                req.destination_id
            )));
        }

        // Increment version
        let current_version: u64 = stream.version_id.parse().unwrap_or(1);
        stream.version_id = (current_version + 1).to_string();
        stream.last_update_timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or(std::time::Duration::from_secs(0))
            .as_secs_f64();

        Ok(())
    }

    pub async fn put_record(
        &self,
        req: PutRecordRequest,
    ) -> Result<PutRecordResponse, FirehoseError> {
        let mut state = self.inner.lock().await;
        let account_id = state.account_id.clone();

        let stream =
            state
                .streams
                .get_mut(&req.delivery_stream_name)
                .ok_or_else(|| {
                    FirehoseError::ResourceNotFoundException(format!(
                        "Delivery stream {} under account {} not found.",
                        req.delivery_stream_name, account_id
                    ))
                })?;

        let record_id = Uuid::new_v4().to_string();

        stream.records.push(StoredRecord {});

        Ok(PutRecordResponse {
            record_id,
            encrypted: false,
        })
    }

    pub async fn put_record_batch(
        &self,
        req: PutRecordBatchRequest,
    ) -> Result<PutRecordBatchResponse, FirehoseError> {
        let mut state = self.inner.lock().await;
        let account_id = state.account_id.clone();

        if req.records.is_empty() {
            return Err(FirehoseError::InvalidArgumentException(
                "Records must not be empty".to_string(),
            ));
        }

        if req.records.len() > 500 {
            return Err(FirehoseError::InvalidArgumentException(
                "Batch size must not exceed 500 records".to_string(),
            ));
        }

        let stream =
            state
                .streams
                .get_mut(&req.delivery_stream_name)
                .ok_or_else(|| {
                    FirehoseError::ResourceNotFoundException(format!(
                        "Delivery stream {} under account {} not found.",
                        req.delivery_stream_name, account_id
                    ))
                })?;

        let mut responses = Vec::with_capacity(req.records.len());

        for _record in req.records {
            let record_id = Uuid::new_v4().to_string();
            stream.records.push(StoredRecord {});
            responses.push(PutRecordBatchResponseEntry { record_id });
        }

        Ok(PutRecordBatchResponse {
            failed_put_count: 0,
            encrypted: false,
            request_responses: responses,
        })
    }

    pub async fn tag_delivery_stream(
        &self,
        req: TagDeliveryStreamRequest,
    ) -> Result<(), FirehoseError> {
        let mut state = self.inner.lock().await;
        let account_id = state.account_id.clone();

        let stream =
            state
                .streams
                .get_mut(&req.delivery_stream_name)
                .ok_or_else(|| {
                    FirehoseError::ResourceNotFoundException(format!(
                        "Delivery stream {} under account {} not found.",
                        req.delivery_stream_name, account_id
                    ))
                })?;

        for tag in req.tags {
            stream.tags.insert(tag.key, tag.value.unwrap_or_default());
        }

        if stream.tags.len() > 50 {
            return Err(FirehoseError::LimitExceededException(
                "Tag limit exceeded. Max 50 tags per delivery stream.".to_string(),
            ));
        }

        Ok(())
    }

    pub async fn untag_delivery_stream(
        &self,
        req: UntagDeliveryStreamRequest,
    ) -> Result<(), FirehoseError> {
        let mut state = self.inner.lock().await;
        let account_id = state.account_id.clone();

        let stream =
            state
                .streams
                .get_mut(&req.delivery_stream_name)
                .ok_or_else(|| {
                    FirehoseError::ResourceNotFoundException(format!(
                        "Delivery stream {} under account {} not found.",
                        req.delivery_stream_name, account_id
                    ))
                })?;

        for key in &req.tag_keys {
            stream.tags.remove(key);
        }

        Ok(())
    }

    pub async fn list_tags_for_delivery_stream(
        &self,
        req: ListTagsForDeliveryStreamRequest,
    ) -> Result<ListTagsForDeliveryStreamResponse, FirehoseError> {
        let state = self.inner.lock().await;

        let stream = state
            .streams
            .get(&req.delivery_stream_name)
            .ok_or_else(|| {
                FirehoseError::ResourceNotFoundException(format!(
                    "Delivery stream {} under account {} not found.",
                    req.delivery_stream_name, state.account_id
                ))
            })?;

        let mut tags: Vec<Tag> = stream
            .tags
            .iter()
            .map(|(k, v)| Tag {
                key: k.clone(),
                value: Some(v.clone()),
            })
            .collect();
        tags.sort_by(|a, b| a.key.cmp(&b.key));

        // Apply exclusive start
        if let Some(ref start_key) = req.exclusive_start_tag_key {
            if let Some(pos) = tags.iter().position(|t| t.key > *start_key) {
                tags = tags[pos..].to_vec();
            } else {
                tags.clear();
            }
        }

        let limit = req.limit.unwrap_or(50);
        let has_more = tags.len() > limit;
        tags.truncate(limit);

        Ok(ListTagsForDeliveryStreamResponse {
            tags,
            has_more_tags: has_more,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_state() -> FirehoseState {
        FirehoseState::new("123456789012".to_string(), "us-east-1".to_string())
    }

    async fn create_stream(state: &FirehoseState, name: &str) -> String {
        let req = CreateDeliveryStreamRequest {
            delivery_stream_name: name.to_string(),
            ..Default::default()
        };
        state.create_delivery_stream(req).await.unwrap().delivery_stream_arn
    }

    #[tokio::test]
    async fn test_new_state() {
        let _state = make_state();
    }

    #[tokio::test]
    async fn test_create_delivery_stream() {
        let state = make_state();
        let arn = create_stream(&state, "my-stream").await;
        assert!(arn.contains("my-stream"));
    }

    #[tokio::test]
    async fn test_create_delivery_stream_duplicate() {
        let state = make_state();
        create_stream(&state, "dup").await;
        let req = CreateDeliveryStreamRequest {
            delivery_stream_name: "dup".to_string(),
            ..Default::default()
        };
        assert!(state.create_delivery_stream(req).await.is_err());
    }

    #[tokio::test]
    async fn test_describe_delivery_stream() {
        let state = make_state();
        create_stream(&state, "desc-stream").await;
        let req = DescribeDeliveryStreamRequest { delivery_stream_name: "desc-stream".to_string() };
        let result = state.describe_delivery_stream(req).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().delivery_stream_description.delivery_stream_name, "desc-stream");
    }

    #[tokio::test]
    async fn test_describe_delivery_stream_not_found() {
        let state = make_state();
        let req = DescribeDeliveryStreamRequest { delivery_stream_name: "nope".to_string() };
        assert!(state.describe_delivery_stream(req).await.is_err());
    }

    #[tokio::test]
    async fn test_list_delivery_streams() {
        let state = make_state();
        create_stream(&state, "s1").await;
        create_stream(&state, "s2").await;
        let req = ListDeliveryStreamsRequest::default();
        let result = state.list_delivery_streams(req).await.unwrap();
        assert_eq!(result.delivery_stream_names.len(), 2);
    }

    #[tokio::test]
    async fn test_delete_delivery_stream() {
        let state = make_state();
        create_stream(&state, "del-stream").await;
        let req = DeleteDeliveryStreamRequest { delivery_stream_name: "del-stream".to_string() };
        assert!(state.delete_delivery_stream(req).await.is_ok());
    }

    #[tokio::test]
    async fn test_delete_delivery_stream_not_found() {
        let state = make_state();
        let req = DeleteDeliveryStreamRequest { delivery_stream_name: "nope".to_string() };
        assert!(state.delete_delivery_stream(req).await.is_err());
    }

    #[tokio::test]
    async fn test_put_record() {
        let state = make_state();
        create_stream(&state, "rec-stream").await;
        let req = PutRecordRequest { delivery_stream_name: "rec-stream".to_string() };
        let result = state.put_record(req).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_put_record_batch() {
        let state = make_state();
        create_stream(&state, "batch-stream").await;
        let req = PutRecordBatchRequest {
            delivery_stream_name: "batch-stream".to_string(),
            records: vec![RecordInput {}, RecordInput {}],
        };
        let result = state.put_record_batch(req).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().request_responses.len(), 2);
    }

    #[tokio::test]
    async fn test_tag_and_list_tags() {
        let state = make_state();
        create_stream(&state, "tag-stream").await;
        state.tag_delivery_stream(TagDeliveryStreamRequest {
            delivery_stream_name: "tag-stream".to_string(),
            tags: vec![Tag { key: "env".to_string(), value: Some("test".to_string()) }],
        }).await.unwrap();
        let result = state.list_tags_for_delivery_stream(ListTagsForDeliveryStreamRequest {
            delivery_stream_name: "tag-stream".to_string(),
            ..Default::default()
        }).await.unwrap();
        assert_eq!(result.tags.len(), 1);
    }

    #[tokio::test]
    async fn test_untag_delivery_stream() {
        let state = make_state();
        create_stream(&state, "untag-stream").await;
        state.tag_delivery_stream(TagDeliveryStreamRequest {
            delivery_stream_name: "untag-stream".to_string(),
            tags: vec![Tag { key: "env".to_string(), value: Some("test".to_string()) }],
        }).await.unwrap();
        state.untag_delivery_stream(UntagDeliveryStreamRequest {
            delivery_stream_name: "untag-stream".to_string(),
            tag_keys: vec!["env".to_string()],
        }).await.unwrap();
        let result = state.list_tags_for_delivery_stream(ListTagsForDeliveryStreamRequest {
            delivery_stream_name: "untag-stream".to_string(),
            ..Default::default()
        }).await.unwrap();
        assert!(result.tags.is_empty());
    }

    // --- Extended coverage: create with tags ---

    #[tokio::test]
    async fn test_create_delivery_stream_with_tags() {
        let state = make_state();
        let req = CreateDeliveryStreamRequest {
            delivery_stream_name: "tagged-stream".to_string(),
            tags: Some(vec![
                Tag { key: "env".to_string(), value: Some("prod".to_string()) },
                Tag { key: "team".to_string(), value: None },
            ]),
            ..Default::default()
        };
        state.create_delivery_stream(req).await.unwrap();
        let result = state.list_tags_for_delivery_stream(ListTagsForDeliveryStreamRequest {
            delivery_stream_name: "tagged-stream".to_string(),
            ..Default::default()
        }).await.unwrap();
        assert_eq!(result.tags.len(), 2);
    }

    // --- Extended coverage: list with filtering ---

    #[tokio::test]
    async fn test_list_delivery_streams_with_type_filter() {
        let state = make_state();
        // create_stream uses Default which gives empty stream_type
        // Create with explicit type to test filtering
        state.create_delivery_stream(CreateDeliveryStreamRequest {
            delivery_stream_name: "s1".to_string(),
            delivery_stream_type: "DirectPut".to_string(),
            ..Default::default()
        }).await.unwrap();
        state.create_delivery_stream(CreateDeliveryStreamRequest {
            delivery_stream_name: "s2".to_string(),
            delivery_stream_type: "KinesisStreamAsSource".to_string(),
            ..Default::default()
        }).await.unwrap();
        let result = state.list_delivery_streams(ListDeliveryStreamsRequest {
            delivery_stream_type: Some("DirectPut".to_string()),
            ..Default::default()
        }).await.unwrap();
        assert_eq!(result.delivery_stream_names.len(), 1);
        assert_eq!(result.delivery_stream_names[0], "s1");
    }

    #[tokio::test]
    async fn test_list_delivery_streams_exclusive_start() {
        let state = make_state();
        create_stream(&state, "a-stream").await;
        create_stream(&state, "b-stream").await;
        create_stream(&state, "c-stream").await;
        let result = state.list_delivery_streams(ListDeliveryStreamsRequest {
            exclusive_start_delivery_stream_name: Some("a-stream".to_string()),
            ..Default::default()
        }).await.unwrap();
        assert_eq!(result.delivery_stream_names.len(), 2);
        assert_eq!(result.delivery_stream_names[0], "b-stream");
    }

    #[tokio::test]
    async fn test_list_delivery_streams_exclusive_start_past_end() {
        let state = make_state();
        create_stream(&state, "a-stream").await;
        let result = state.list_delivery_streams(ListDeliveryStreamsRequest {
            exclusive_start_delivery_stream_name: Some("z-stream".to_string()),
            ..Default::default()
        }).await.unwrap();
        assert!(result.delivery_stream_names.is_empty());
    }

    #[tokio::test]
    async fn test_list_delivery_streams_pagination() {
        let state = make_state();
        create_stream(&state, "s1").await;
        create_stream(&state, "s2").await;
        create_stream(&state, "s3").await;
        let result = state.list_delivery_streams(ListDeliveryStreamsRequest {
            limit: Some(2),
            ..Default::default()
        }).await.unwrap();
        assert_eq!(result.delivery_stream_names.len(), 2);
        assert!(result.has_more_delivery_streams);
    }

    // --- Extended coverage: update_destination ---

    #[tokio::test]
    async fn test_update_destination_not_found() {
        let state = make_state();
        let result = state.update_destination(UpdateDestinationRequest {
            delivery_stream_name: "nope".to_string(),
            current_delivery_stream_version_id: "1".to_string(),
            destination_id: "d1".to_string(),
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_destination_version_mismatch() {
        let state = make_state();
        create_stream(&state, "my-stream").await;
        let result = state.update_destination(UpdateDestinationRequest {
            delivery_stream_name: "my-stream".to_string(),
            current_delivery_stream_version_id: "999".to_string(),
            destination_id: "destinationId-000000000001".to_string(),
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_destination_invalid_dest_id() {
        let state = make_state();
        create_stream(&state, "my-stream").await;
        let result = state.update_destination(UpdateDestinationRequest {
            delivery_stream_name: "my-stream".to_string(),
            current_delivery_stream_version_id: "1".to_string(),
            destination_id: "invalid-dest".to_string(),
        }).await;
        assert!(result.is_err());
    }

    // --- Extended coverage: put_record errors ---

    #[tokio::test]
    async fn test_put_record_not_found() {
        let state = make_state();
        let result = state.put_record(PutRecordRequest {
            delivery_stream_name: "nope".to_string(),
        }).await;
        assert!(result.is_err());
    }

    // --- Extended coverage: put_record_batch errors ---

    #[tokio::test]
    async fn test_put_record_batch_empty() {
        let state = make_state();
        create_stream(&state, "my-stream").await;
        let result = state.put_record_batch(PutRecordBatchRequest {
            delivery_stream_name: "my-stream".to_string(),
            records: vec![],
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_put_record_batch_not_found() {
        let state = make_state();
        let result = state.put_record_batch(PutRecordBatchRequest {
            delivery_stream_name: "nope".to_string(),
            records: vec![RecordInput {}],
        }).await;
        assert!(result.is_err());
    }

    // --- Extended coverage: tag operations errors ---

    #[tokio::test]
    async fn test_tag_delivery_stream_not_found() {
        let state = make_state();
        let result = state.tag_delivery_stream(TagDeliveryStreamRequest {
            delivery_stream_name: "nope".to_string(),
            tags: vec![Tag { key: "k".to_string(), value: Some("v".to_string()) }],
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_untag_delivery_stream_not_found() {
        let state = make_state();
        let result = state.untag_delivery_stream(UntagDeliveryStreamRequest {
            delivery_stream_name: "nope".to_string(),
            tag_keys: vec!["k".to_string()],
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_tags_not_found() {
        let state = make_state();
        let result = state.list_tags_for_delivery_stream(ListTagsForDeliveryStreamRequest {
            delivery_stream_name: "nope".to_string(),
            ..Default::default()
        }).await;
        assert!(result.is_err());
    }

    // --- Extended coverage: list_tags_for_delivery_stream pagination ---

    #[tokio::test]
    async fn test_list_tags_exclusive_start() {
        let state = make_state();
        create_stream(&state, "my-stream").await;
        state.tag_delivery_stream(TagDeliveryStreamRequest {
            delivery_stream_name: "my-stream".to_string(),
            tags: vec![
                Tag { key: "a".to_string(), value: Some("1".to_string()) },
                Tag { key: "b".to_string(), value: Some("2".to_string()) },
                Tag { key: "c".to_string(), value: Some("3".to_string()) },
            ],
        }).await.unwrap();
        let result = state.list_tags_for_delivery_stream(ListTagsForDeliveryStreamRequest {
            delivery_stream_name: "my-stream".to_string(),
            exclusive_start_tag_key: Some("a".to_string()),
            limit: None,
        }).await.unwrap();
        assert_eq!(result.tags.len(), 2);
        assert_eq!(result.tags[0].key, "b");
    }

    #[tokio::test]
    async fn test_list_tags_exclusive_start_past_end() {
        let state = make_state();
        create_stream(&state, "my-stream").await;
        state.tag_delivery_stream(TagDeliveryStreamRequest {
            delivery_stream_name: "my-stream".to_string(),
            tags: vec![Tag { key: "a".to_string(), value: Some("1".to_string()) }],
        }).await.unwrap();
        let result = state.list_tags_for_delivery_stream(ListTagsForDeliveryStreamRequest {
            delivery_stream_name: "my-stream".to_string(),
            exclusive_start_tag_key: Some("z".to_string()),
            limit: None,
        }).await.unwrap();
        assert!(result.tags.is_empty());
    }

    #[tokio::test]
    async fn test_list_tags_with_limit() {
        let state = make_state();
        create_stream(&state, "my-stream").await;
        state.tag_delivery_stream(TagDeliveryStreamRequest {
            delivery_stream_name: "my-stream".to_string(),
            tags: vec![
                Tag { key: "a".to_string(), value: Some("1".to_string()) },
                Tag { key: "b".to_string(), value: Some("2".to_string()) },
                Tag { key: "c".to_string(), value: Some("3".to_string()) },
            ],
        }).await.unwrap();
        let result = state.list_tags_for_delivery_stream(ListTagsForDeliveryStreamRequest {
            delivery_stream_name: "my-stream".to_string(),
            exclusive_start_tag_key: None,
            limit: Some(2),
        }).await.unwrap();
        assert_eq!(result.tags.len(), 2);
        assert!(result.has_more_tags);
    }

    // --- Extended coverage: describe stream fields ---

    #[tokio::test]
    async fn test_describe_delivery_stream_fields() {
        let state = make_state();
        create_stream(&state, "detail-stream").await;
        let result = state.describe_delivery_stream(DescribeDeliveryStreamRequest {
            delivery_stream_name: "detail-stream".to_string(),
        }).await.unwrap();
        let desc = &result.delivery_stream_description;
        assert_eq!(desc.delivery_stream_name, "detail-stream");
        assert_eq!(desc.delivery_stream_status, "ACTIVE");
        assert_eq!(desc.version_id, "1");
        assert!(!desc.has_more_destinations);
        assert_eq!(desc.delivery_stream_encryption_configuration.status, "DISABLED");
    }
}
