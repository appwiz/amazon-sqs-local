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
            .unwrap()
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

        stream.records.push(StoredRecord {
            record_id: record_id.clone(),
            data: req.record.data,
        });

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

        for record in req.records {
            let record_id = Uuid::new_v4().to_string();
            stream.records.push(StoredRecord {
                record_id: record_id.clone(),
                data: record.data,
            });
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
