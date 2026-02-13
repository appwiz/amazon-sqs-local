use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// --- Shared types ---

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct MessageAttributeValue {
    pub data_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binary_value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BatchResultErrorEntry {
    pub id: String,
    pub code: String,
    pub message: String,
    pub sender_fault: bool,
}

// --- CreateQueue ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateQueueRequest {
    pub queue_name: String,
    #[serde(default)]
    pub attributes: Option<HashMap<String, String>>,
    #[serde(default, rename = "tags")]
    pub tags: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateQueueResponse {
    pub queue_url: String,
}

// --- DeleteQueue ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteQueueRequest {
    pub queue_url: String,
}

// --- GetQueueUrl ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(dead_code)]
pub struct GetQueueUrlRequest {
    pub queue_name: String,
    #[serde(default)]
    pub queue_owner_a_w_s_account_id: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetQueueUrlResponse {
    pub queue_url: String,
}

// --- ListQueues ---

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ListQueuesRequest {
    #[serde(default)]
    pub queue_name_prefix: Option<String>,
    #[serde(default)]
    pub max_results: Option<i32>,
    #[serde(default)]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListQueuesResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_urls: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

// --- GetQueueAttributes ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetQueueAttributesRequest {
    pub queue_url: String,
    #[serde(default)]
    pub attribute_names: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetQueueAttributesResponse {
    pub attributes: HashMap<String, String>,
}

// --- SetQueueAttributes ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SetQueueAttributesRequest {
    pub queue_url: String,
    pub attributes: HashMap<String, String>,
}

// --- PurgeQueue ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PurgeQueueRequest {
    pub queue_url: String,
}

// --- SendMessage ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SendMessageRequest {
    pub queue_url: String,
    pub message_body: String,
    #[serde(default)]
    pub delay_seconds: Option<i32>,
    #[serde(default)]
    pub message_attributes: Option<HashMap<String, MessageAttributeValue>>,
    #[serde(default)]
    pub message_system_attributes: Option<HashMap<String, MessageAttributeValue>>,
    #[serde(default)]
    pub message_deduplication_id: Option<String>,
    #[serde(default)]
    pub message_group_id: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct SendMessageResponse {
    pub message_id: String,
    #[serde(rename = "MD5OfMessageBody")]
    pub md5_of_message_body: String,
    #[serde(rename = "MD5OfMessageAttributes", skip_serializing_if = "Option::is_none")]
    pub md5_of_message_attributes: Option<String>,
    #[serde(
        rename = "MD5OfMessageSystemAttributes",
        skip_serializing_if = "Option::is_none"
    )]
    pub md5_of_message_system_attributes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence_number: Option<String>,
}

// --- SendMessageBatch ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SendMessageBatchRequest {
    pub queue_url: String,
    pub entries: Vec<SendMessageBatchEntry>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SendMessageBatchEntry {
    pub id: String,
    pub message_body: String,
    #[serde(default)]
    pub delay_seconds: Option<i32>,
    #[serde(default)]
    pub message_attributes: Option<HashMap<String, MessageAttributeValue>>,
    #[serde(default)]
    pub message_system_attributes: Option<HashMap<String, MessageAttributeValue>>,
    #[serde(default)]
    pub message_deduplication_id: Option<String>,
    #[serde(default)]
    pub message_group_id: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct SendMessageBatchResponse {
    pub successful: Vec<SendMessageBatchResultEntry>,
    pub failed: Vec<BatchResultErrorEntry>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct SendMessageBatchResultEntry {
    pub id: String,
    pub message_id: String,
    #[serde(rename = "MD5OfMessageBody")]
    pub md5_of_message_body: String,
    #[serde(rename = "MD5OfMessageAttributes", skip_serializing_if = "Option::is_none")]
    pub md5_of_message_attributes: Option<String>,
    #[serde(
        rename = "MD5OfMessageSystemAttributes",
        skip_serializing_if = "Option::is_none"
    )]
    pub md5_of_message_system_attributes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence_number: Option<String>,
}

// --- ReceiveMessage ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(dead_code)]
pub struct ReceiveMessageRequest {
    pub queue_url: String,
    #[serde(default)]
    pub max_number_of_messages: Option<i32>,
    #[serde(default)]
    pub visibility_timeout: Option<i32>,
    #[serde(default)]
    pub wait_time_seconds: Option<i32>,
    #[serde(default)]
    pub attribute_names: Option<Vec<String>>,
    #[serde(default)]
    pub message_attribute_names: Option<Vec<String>>,
    #[serde(default)]
    pub message_system_attribute_names: Option<Vec<String>>,
    #[serde(default)]
    pub receive_request_attempt_id: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ReceiveMessageResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<ReceiveMessageResult>>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ReceiveMessageResult {
    pub message_id: String,
    pub receipt_handle: String,
    pub body: String,
    #[serde(rename = "MD5OfBody")]
    pub md5_of_body: String,
    #[serde(rename = "MD5OfMessageAttributes", skip_serializing_if = "Option::is_none")]
    pub md5_of_message_attributes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_attributes: Option<HashMap<String, MessageAttributeValue>>,
}

// --- DeleteMessage ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteMessageRequest {
    pub queue_url: String,
    pub receipt_handle: String,
}

// --- DeleteMessageBatch ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteMessageBatchRequest {
    pub queue_url: String,
    pub entries: Vec<DeleteMessageBatchEntry>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteMessageBatchEntry {
    pub id: String,
    pub receipt_handle: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteMessageBatchResponse {
    pub successful: Vec<DeleteMessageBatchResultEntry>,
    pub failed: Vec<BatchResultErrorEntry>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteMessageBatchResultEntry {
    pub id: String,
}

// --- ChangeMessageVisibility ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ChangeMessageVisibilityRequest {
    pub queue_url: String,
    pub receipt_handle: String,
    pub visibility_timeout: i32,
}

// --- ChangeMessageVisibilityBatch ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ChangeMessageVisibilityBatchRequest {
    pub queue_url: String,
    pub entries: Vec<ChangeMessageVisibilityBatchEntry>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ChangeMessageVisibilityBatchEntry {
    pub id: String,
    pub receipt_handle: String,
    pub visibility_timeout: i32,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ChangeMessageVisibilityBatchResponse {
    pub successful: Vec<ChangeMessageVisibilityBatchResultEntry>,
    pub failed: Vec<BatchResultErrorEntry>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ChangeMessageVisibilityBatchResultEntry {
    pub id: String,
}

// --- TagQueue ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TagQueueRequest {
    pub queue_url: String,
    pub tags: HashMap<String, String>,
}

// --- UntagQueue ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UntagQueueRequest {
    pub queue_url: String,
    pub tag_keys: Vec<String>,
}

// --- ListQueueTags ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListQueueTagsRequest {
    pub queue_url: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListQueueTagsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<HashMap<String, String>>,
}

// --- AddPermission ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AddPermissionRequest {
    pub queue_url: String,
    pub label: String,
    #[serde(rename = "AWSAccountIds")]
    pub aws_account_ids: Vec<String>,
    pub actions: Vec<String>,
}

// --- RemovePermission ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RemovePermissionRequest {
    pub queue_url: String,
    pub label: String,
}

// --- ListDeadLetterSourceQueues ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListDeadLetterSourceQueuesRequest {
    pub queue_url: String,
    #[serde(default)]
    pub max_results: Option<i32>,
    #[serde(default)]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ListDeadLetterSourceQueuesResponse {
    #[serde(rename = "queueUrls")]
    pub queue_urls: Vec<String>,
    #[serde(rename = "NextToken", skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

// --- StartMessageMoveTask ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StartMessageMoveTaskRequest {
    pub source_arn: String,
    #[serde(default)]
    pub destination_arn: Option<String>,
    #[serde(default)]
    pub max_number_of_messages_per_second: Option<i32>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct StartMessageMoveTaskResponse {
    pub task_handle: String,
}

// --- CancelMessageMoveTask ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CancelMessageMoveTaskRequest {
    pub task_handle: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CancelMessageMoveTaskResponse {
    pub approximate_number_of_messages_moved: i64,
}

// --- ListMessageMoveTasks ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(dead_code)]
pub struct ListMessageMoveTasksRequest {
    pub source_arn: String,
    #[serde(default)]
    pub max_results: Option<i32>,
    #[serde(default)]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListMessageMoveTasksResponse {
    pub results: Vec<MessageMoveTaskResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct MessageMoveTaskResult {
    pub task_handle: String,
    pub status: String,
    pub source_arn: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_arn: Option<String>,
    pub approximate_number_of_messages_moved: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approximate_number_of_messages_to_move: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_number_of_messages_per_second: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_timestamp: Option<i64>,
}
