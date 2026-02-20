use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct CreateLogGroupRequest {
    #[serde(rename = "logGroupName")]
    pub log_group_name: String,
    #[serde(rename = "kmsKeyId")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "tags")]
    pub tags: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize)]
pub struct DeleteLogGroupRequest {
    #[serde(rename = "logGroupName")]
    pub log_group_name: String,
}

#[derive(Debug, Deserialize, Default)]
pub struct DescribeLogGroupsRequest {
    #[serde(rename = "logGroupNamePrefix")]
    pub log_group_name_prefix: Option<String>,
    #[serde(rename = "logGroupNamePattern")]
    pub log_group_name_pattern: Option<String>,
    #[serde(rename = "nextToken")]
    pub next_token: Option<String>,
    #[serde(rename = "limit")]
    pub limit: Option<usize>,
}

#[derive(Debug, Serialize)]
pub struct DescribeLogGroupsResponse {
    #[serde(rename = "logGroups")]
    pub log_groups: Vec<LogGroup>,
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct LogGroup {
    #[serde(rename = "logGroupName")]
    pub log_group_name: String,
    #[serde(rename = "arn")]
    pub arn: String,
    #[serde(rename = "creationTime")]
    pub creation_time: i64,
    #[serde(rename = "retentionInDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_in_days: Option<i64>,
    #[serde(rename = "metricFilterCount")]
    pub metric_filter_count: i64,
    #[serde(rename = "storedBytes")]
    pub stored_bytes: i64,
}

#[derive(Debug, Deserialize)]
pub struct CreateLogStreamRequest {
    #[serde(rename = "logGroupName")]
    pub log_group_name: String,
    #[serde(rename = "logStreamName")]
    pub log_stream_name: String,
}

#[derive(Debug, Deserialize)]
pub struct DeleteLogStreamRequest {
    #[serde(rename = "logGroupName")]
    pub log_group_name: String,
    #[serde(rename = "logStreamName")]
    pub log_stream_name: String,
}

#[derive(Debug, Deserialize, Default)]
pub struct DescribeLogStreamsRequest {
    #[serde(rename = "logGroupName")]
    pub log_group_name: Option<String>,
    #[serde(rename = "logStreamNamePrefix")]
    pub log_stream_name_prefix: Option<String>,
    #[serde(rename = "orderBy")]
    pub order_by: Option<String>,
    #[serde(rename = "descending")]
    pub descending: Option<bool>,
    #[serde(rename = "nextToken")]
    pub next_token: Option<String>,
    #[serde(rename = "limit")]
    pub limit: Option<usize>,
}

#[derive(Debug, Serialize)]
pub struct DescribeLogStreamsResponse {
    #[serde(rename = "logStreams")]
    pub log_streams: Vec<LogStream>,
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct LogStream {
    #[serde(rename = "logStreamName")]
    pub log_stream_name: String,
    #[serde(rename = "creationTime")]
    pub creation_time: i64,
    #[serde(rename = "firstEventTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_event_timestamp: Option<i64>,
    #[serde(rename = "lastEventTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_event_timestamp: Option<i64>,
    #[serde(rename = "lastIngestionTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_ingestion_time: Option<i64>,
    #[serde(rename = "uploadSequenceToken")]
    pub upload_sequence_token: String,
    #[serde(rename = "arn")]
    pub arn: String,
    #[serde(rename = "storedBytes")]
    pub stored_bytes: i64,
}

#[derive(Debug, Deserialize)]
pub struct PutLogEventsRequest {
    #[serde(rename = "logGroupName")]
    pub log_group_name: String,
    #[serde(rename = "logStreamName")]
    pub log_stream_name: String,
    #[serde(rename = "logEvents")]
    pub log_events: Vec<InputLogEvent>,
    #[serde(rename = "sequenceToken")]
    pub sequence_token: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct InputLogEvent {
    #[serde(rename = "timestamp")]
    pub timestamp: i64,
    #[serde(rename = "message")]
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct PutLogEventsResponse {
    #[serde(rename = "nextSequenceToken")]
    pub next_sequence_token: String,
    #[serde(rename = "rejectedLogEventsInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rejected_log_events_info: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct GetLogEventsRequest {
    #[serde(rename = "logGroupName")]
    pub log_group_name: String,
    #[serde(rename = "logStreamName")]
    pub log_stream_name: String,
    #[serde(rename = "startTime")]
    pub start_time: Option<i64>,
    #[serde(rename = "endTime")]
    pub end_time: Option<i64>,
    #[serde(rename = "nextToken")]
    pub next_token: Option<String>,
    #[serde(rename = "limit")]
    pub limit: Option<usize>,
    #[serde(rename = "startFromHead")]
    pub start_from_head: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct GetLogEventsResponse {
    #[serde(rename = "events")]
    pub events: Vec<OutputLogEvent>,
    #[serde(rename = "nextForwardToken")]
    pub next_forward_token: String,
    #[serde(rename = "nextBackwardToken")]
    pub next_backward_token: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct OutputLogEvent {
    #[serde(rename = "timestamp")]
    pub timestamp: i64,
    #[serde(rename = "message")]
    pub message: String,
    #[serde(rename = "ingestionTime")]
    pub ingestion_time: i64,
}

#[derive(Debug, Deserialize)]
pub struct FilterLogEventsRequest {
    #[serde(rename = "logGroupName")]
    pub log_group_name: String,
    #[serde(rename = "logStreamNames")]
    pub log_stream_names: Option<Vec<String>>,
    #[serde(rename = "startTime")]
    pub start_time: Option<i64>,
    #[serde(rename = "endTime")]
    pub end_time: Option<i64>,
    #[serde(rename = "filterPattern")]
    pub filter_pattern: Option<String>,
    #[serde(rename = "nextToken")]
    pub next_token: Option<String>,
    #[serde(rename = "limit")]
    pub limit: Option<usize>,
}

#[derive(Debug, Serialize)]
pub struct FilterLogEventsResponse {
    #[serde(rename = "events")]
    pub events: Vec<FilteredLogEvent>,
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct FilteredLogEvent {
    #[serde(rename = "logStreamName")]
    pub log_stream_name: String,
    #[serde(rename = "timestamp")]
    pub timestamp: i64,
    #[serde(rename = "message")]
    pub message: String,
    #[serde(rename = "ingestionTime")]
    pub ingestion_time: i64,
    #[serde(rename = "eventId")]
    pub event_id: String,
}

#[derive(Debug, Deserialize)]
pub struct PutRetentionPolicyRequest {
    #[serde(rename = "logGroupName")]
    pub log_group_name: String,
    #[serde(rename = "retentionInDays")]
    pub retention_in_days: i64,
}

#[derive(Debug, Deserialize)]
pub struct DeleteRetentionPolicyRequest {
    #[serde(rename = "logGroupName")]
    pub log_group_name: String,
}

#[derive(Debug, Deserialize)]
pub struct TagLogGroupRequest {
    #[serde(rename = "logGroupName")]
    pub log_group_name: String,
    #[serde(rename = "tags")]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub struct UntagLogGroupRequest {
    #[serde(rename = "logGroupName")]
    pub log_group_name: String,
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct ListTagsLogGroupRequest {
    #[serde(rename = "logGroupName")]
    pub log_group_name: String,
}

#[derive(Debug, Serialize)]
pub struct ListTagsLogGroupResponse {
    #[serde(rename = "tags")]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub struct TagResourceRequest {
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    #[serde(rename = "tags")]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub struct UntagResourceRequest {
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    #[serde(rename = "tagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct ListTagsForResourceRequest {
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
}

#[derive(Debug, Serialize)]
pub struct ListTagsForResourceResponse {
    #[serde(rename = "tags")]
    pub tags: HashMap<String, String>,
}
