use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateStreamRequest {
    #[serde(rename = "StreamName")]
    pub stream_name: String,
    #[serde(rename = "ShardCount")]
    pub shard_count: Option<u32>,
    #[serde(rename = "StreamModeDetails")]
    pub stream_mode_details: Option<StreamModeDetails>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct StreamModeDetails {
    #[serde(rename = "StreamMode")]
    pub stream_mode: String,
}

#[derive(Debug, Deserialize)]
pub struct DeleteStreamRequest {
    #[serde(rename = "StreamName")]
    pub stream_name: Option<String>,
    #[serde(rename = "StreamARN")]
    pub stream_arn: Option<String>,
    #[serde(rename = "EnforceConsumerDeletion")]
    pub enforce_consumer_deletion: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct DescribeStreamRequest {
    #[serde(rename = "StreamName")]
    pub stream_name: Option<String>,
    #[serde(rename = "StreamARN")]
    pub stream_arn: Option<String>,
    #[serde(rename = "Limit")]
    pub limit: Option<u32>,
    #[serde(rename = "ExclusiveStartShardId")]
    pub exclusive_start_shard_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct DescribeStreamResponse {
    #[serde(rename = "StreamDescription")]
    pub stream_description: StreamDescription,
}

#[derive(Debug, Serialize, Clone)]
pub struct StreamDescription {
    #[serde(rename = "StreamName")]
    pub stream_name: String,
    #[serde(rename = "StreamARN")]
    pub stream_arn: String,
    #[serde(rename = "StreamStatus")]
    pub stream_status: String,
    #[serde(rename = "StreamModeDetails")]
    pub stream_mode_details: StreamModeDetails,
    #[serde(rename = "Shards")]
    pub shards: Vec<Shard>,
    #[serde(rename = "HasMoreShards")]
    pub has_more_shards: bool,
    #[serde(rename = "RetentionPeriodHours")]
    pub retention_period_hours: u32,
    #[serde(rename = "StreamCreationTimestamp")]
    pub stream_creation_timestamp: f64,
    #[serde(rename = "EnhancedMonitoring")]
    pub enhanced_monitoring: Vec<serde_json::Value>,
}

#[derive(Debug, Serialize, Clone)]
pub struct Shard {
    #[serde(rename = "ShardId")]
    pub shard_id: String,
    #[serde(rename = "HashKeyRange")]
    pub hash_key_range: HashKeyRange,
    #[serde(rename = "SequenceNumberRange")]
    pub sequence_number_range: SequenceNumberRange,
}

#[derive(Debug, Serialize, Clone)]
pub struct HashKeyRange {
    #[serde(rename = "StartingHashKey")]
    pub starting_hash_key: String,
    #[serde(rename = "EndingHashKey")]
    pub ending_hash_key: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct SequenceNumberRange {
    #[serde(rename = "StartingSequenceNumber")]
    pub starting_sequence_number: String,
    #[serde(rename = "EndingSequenceNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_sequence_number: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DescribeStreamSummaryRequest {
    #[serde(rename = "StreamName")]
    pub stream_name: Option<String>,
    #[serde(rename = "StreamARN")]
    pub stream_arn: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct DescribeStreamSummaryResponse {
    #[serde(rename = "StreamDescriptionSummary")]
    pub stream_description_summary: StreamDescriptionSummary,
}

#[derive(Debug, Serialize)]
pub struct StreamDescriptionSummary {
    #[serde(rename = "StreamName")]
    pub stream_name: String,
    #[serde(rename = "StreamARN")]
    pub stream_arn: String,
    #[serde(rename = "StreamStatus")]
    pub stream_status: String,
    #[serde(rename = "StreamModeDetails")]
    pub stream_mode_details: StreamModeDetails,
    #[serde(rename = "RetentionPeriodHours")]
    pub retention_period_hours: u32,
    #[serde(rename = "StreamCreationTimestamp")]
    pub stream_creation_timestamp: f64,
    #[serde(rename = "OpenShardCount")]
    pub open_shard_count: u32,
    #[serde(rename = "EnhancedMonitoring")]
    pub enhanced_monitoring: Vec<serde_json::Value>,
}

#[derive(Debug, Deserialize, Default)]
pub struct ListStreamsRequest {
    #[serde(rename = "Limit")]
    pub limit: Option<usize>,
    #[serde(rename = "ExclusiveStartStreamName")]
    pub exclusive_start_stream_name: Option<String>,
    #[serde(rename = "NextToken")]
    pub next_token: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ListStreamsResponse {
    #[serde(rename = "StreamNames")]
    pub stream_names: Vec<String>,
    #[serde(rename = "HasMoreStreams")]
    pub has_more_streams: bool,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PutRecordRequest {
    #[serde(rename = "StreamName")]
    pub stream_name: Option<String>,
    #[serde(rename = "StreamARN")]
    pub stream_arn: Option<String>,
    #[serde(rename = "Data")]
    pub data: String, // base64
    #[serde(rename = "PartitionKey")]
    pub partition_key: String,
    #[serde(rename = "ExplicitHashKey")]
    pub explicit_hash_key: Option<String>,
    #[serde(rename = "SequenceNumberForOrdering")]
    pub sequence_number_for_ordering: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PutRecordResponse {
    #[serde(rename = "ShardId")]
    pub shard_id: String,
    #[serde(rename = "SequenceNumber")]
    pub sequence_number: String,
    #[serde(rename = "EncryptionType")]
    pub encryption_type: String,
}

#[derive(Debug, Deserialize)]
pub struct PutRecordsRequest {
    #[serde(rename = "StreamName")]
    pub stream_name: Option<String>,
    #[serde(rename = "StreamARN")]
    pub stream_arn: Option<String>,
    #[serde(rename = "Records")]
    pub records: Vec<PutRecordsRequestEntry>,
}

#[derive(Debug, Deserialize)]
pub struct PutRecordsRequestEntry {
    #[serde(rename = "Data")]
    pub data: String,
    #[serde(rename = "PartitionKey")]
    pub partition_key: String,
    #[serde(rename = "ExplicitHashKey")]
    pub explicit_hash_key: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PutRecordsResponse {
    #[serde(rename = "FailedRecordCount")]
    pub failed_record_count: u32,
    #[serde(rename = "Records")]
    pub records: Vec<PutRecordsResultEntry>,
    #[serde(rename = "EncryptionType")]
    pub encryption_type: String,
}

#[derive(Debug, Serialize)]
pub struct PutRecordsResultEntry {
    #[serde(rename = "ShardId")]
    pub shard_id: String,
    #[serde(rename = "SequenceNumber")]
    pub sequence_number: String,
}

#[derive(Debug, Deserialize)]
pub struct GetShardIteratorRequest {
    #[serde(rename = "StreamName")]
    pub stream_name: Option<String>,
    #[serde(rename = "StreamARN")]
    pub stream_arn: Option<String>,
    #[serde(rename = "ShardId")]
    pub shard_id: String,
    #[serde(rename = "ShardIteratorType")]
    pub shard_iterator_type: String,
    #[serde(rename = "StartingSequenceNumber")]
    pub starting_sequence_number: Option<String>,
    #[serde(rename = "Timestamp")]
    pub timestamp: Option<f64>,
}

#[derive(Debug, Serialize)]
pub struct GetShardIteratorResponse {
    #[serde(rename = "ShardIterator")]
    pub shard_iterator: String,
}

#[derive(Debug, Deserialize)]
pub struct GetRecordsRequest {
    #[serde(rename = "ShardIterator")]
    pub shard_iterator: String,
    #[serde(rename = "Limit")]
    pub limit: Option<usize>,
    #[serde(rename = "StreamARN")]
    pub stream_arn: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct GetRecordsResponse {
    #[serde(rename = "Records")]
    pub records: Vec<Record>,
    #[serde(rename = "NextShardIterator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_shard_iterator: Option<String>,
    #[serde(rename = "MillisBehindLatest")]
    pub millis_behind_latest: u64,
}

#[derive(Debug, Serialize, Clone)]
pub struct Record {
    #[serde(rename = "SequenceNumber")]
    pub sequence_number: String,
    #[serde(rename = "ApproximateArrivalTimestamp")]
    pub approximate_arrival_timestamp: f64,
    #[serde(rename = "Data")]
    pub data: String, // base64
    #[serde(rename = "PartitionKey")]
    pub partition_key: String,
    #[serde(rename = "EncryptionType")]
    pub encryption_type: String,
}

#[derive(Debug, Deserialize, Default)]
pub struct ListShardsRequest {
    #[serde(rename = "StreamName")]
    pub stream_name: Option<String>,
    #[serde(rename = "StreamARN")]
    pub stream_arn: Option<String>,
    #[serde(rename = "NextToken")]
    pub next_token: Option<String>,
    #[serde(rename = "MaxResults")]
    pub max_results: Option<usize>,
}

#[derive(Debug, Serialize)]
pub struct ListShardsResponse {
    #[serde(rename = "Shards")]
    pub shards: Vec<Shard>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AddTagsToStreamRequest {
    #[serde(rename = "StreamName")]
    pub stream_name: Option<String>,
    #[serde(rename = "StreamARN")]
    pub stream_arn: Option<String>,
    #[serde(rename = "Tags")]
    pub tags: std::collections::HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub struct RemoveTagsFromStreamRequest {
    #[serde(rename = "StreamName")]
    pub stream_name: Option<String>,
    #[serde(rename = "StreamARN")]
    pub stream_arn: Option<String>,
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct ListTagsForStreamRequest {
    #[serde(rename = "StreamName")]
    pub stream_name: Option<String>,
    #[serde(rename = "StreamARN")]
    pub stream_arn: Option<String>,
    #[serde(rename = "Limit")]
    pub limit: Option<usize>,
    #[serde(rename = "ExclusiveStartTagKey")]
    pub exclusive_start_tag_key: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ListTagsForStreamResponse {
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
    #[serde(rename = "HasMoreTags")]
    pub has_more_tags: bool,
}

#[derive(Debug, Serialize)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,
}

#[derive(Debug, Deserialize)]
pub struct IncreaseStreamRetentionPeriodRequest {
    #[serde(rename = "StreamName")]
    pub stream_name: Option<String>,
    #[serde(rename = "StreamARN")]
    pub stream_arn: Option<String>,
    #[serde(rename = "RetentionPeriodHours")]
    pub retention_period_hours: u32,
}

#[derive(Debug, Deserialize)]
pub struct DecreaseStreamRetentionPeriodRequest {
    #[serde(rename = "StreamName")]
    pub stream_name: Option<String>,
    #[serde(rename = "StreamARN")]
    pub stream_arn: Option<String>,
    #[serde(rename = "RetentionPeriodHours")]
    pub retention_period_hours: u32,
}
