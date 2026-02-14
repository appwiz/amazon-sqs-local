use serde::{Deserialize, Serialize};

// --- CreateDeliveryStream ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateDeliveryStreamRequest {
    pub delivery_stream_name: String,
    #[serde(default = "default_stream_type")]
    pub delivery_stream_type: String,
    #[serde(default)]
    pub tags: Option<Vec<Tag>>,
}

fn default_stream_type() -> String {
    "DirectPut".to_string()
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateDeliveryStreamResponse {
    #[serde(rename = "DeliveryStreamARN")]
    pub delivery_stream_arn: String,
}

// --- DeleteDeliveryStream ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteDeliveryStreamRequest {
    pub delivery_stream_name: String,
}

// --- DescribeDeliveryStream ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DescribeDeliveryStreamRequest {
    pub delivery_stream_name: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct DescribeDeliveryStreamResponse {
    pub delivery_stream_description: DeliveryStreamDescription,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeliveryStreamDescription {
    pub delivery_stream_name: String,
    #[serde(rename = "DeliveryStreamARN")]
    pub delivery_stream_arn: String,
    pub delivery_stream_status: String,
    pub delivery_stream_type: String,
    pub version_id: String,
    pub create_timestamp: f64,
    pub last_update_timestamp: f64,
    pub destinations: Vec<DestinationDescription>,
    pub has_more_destinations: bool,
    pub delivery_stream_encryption_configuration: EncryptionConfig,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct DestinationDescription {
    pub destination_id: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct EncryptionConfig {
    pub status: String,
}

// --- ListDeliveryStreams ---

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ListDeliveryStreamsRequest {
    #[serde(default)]
    pub limit: Option<usize>,
    #[serde(default)]
    pub delivery_stream_type: Option<String>,
    #[serde(default)]
    pub exclusive_start_delivery_stream_name: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListDeliveryStreamsResponse {
    pub delivery_stream_names: Vec<String>,
    pub has_more_delivery_streams: bool,
}

// --- UpdateDestination ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UpdateDestinationRequest {
    pub delivery_stream_name: String,
    pub current_delivery_stream_version_id: String,
    pub destination_id: String,
}

// --- PutRecord ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PutRecordRequest {
    pub delivery_stream_name: String,
    pub record: RecordInput,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RecordInput {
    pub data: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PutRecordResponse {
    pub record_id: String,
    pub encrypted: bool,
}

// --- PutRecordBatch ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PutRecordBatchRequest {
    pub delivery_stream_name: String,
    pub records: Vec<RecordInput>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PutRecordBatchResponse {
    pub failed_put_count: i32,
    pub encrypted: bool,
    pub request_responses: Vec<PutRecordBatchResponseEntry>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PutRecordBatchResponseEntry {
    pub record_id: String,
}

// --- TagDeliveryStream ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TagDeliveryStreamRequest {
    pub delivery_stream_name: String,
    pub tags: Vec<Tag>,
}

// --- UntagDeliveryStream ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UntagDeliveryStreamRequest {
    pub delivery_stream_name: String,
    pub tag_keys: Vec<String>,
}

// --- ListTagsForDeliveryStream ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListTagsForDeliveryStreamRequest {
    pub delivery_stream_name: String,
    #[serde(default)]
    pub exclusive_start_tag_key: Option<String>,
    #[serde(default)]
    pub limit: Option<usize>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListTagsForDeliveryStreamResponse {
    pub tags: Vec<Tag>,
    pub has_more_tags: bool,
}

// --- Tag ---

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Tag {
    pub key: String,
    #[serde(default)]
    pub value: Option<String>,
}
