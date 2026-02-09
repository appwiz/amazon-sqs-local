use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// --- Shared types ---

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct MessageAttributeValueJson {
    pub data_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binary_value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TagJson {
    pub key: String,
    pub value: String,
}

// --- CreateTopic ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateTopicRequest {
    pub name: String,
    #[serde(default)]
    pub attributes: Option<HashMap<String, String>>,
    #[serde(default)]
    pub tags: Option<Vec<TagJson>>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateTopicResponse {
    pub topic_arn: String,
}

// --- DeleteTopic ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteTopicRequest {
    pub topic_arn: String,
}

// --- ListTopics ---

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ListTopicsRequest {
    #[serde(default)]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListTopicsResponse {
    pub topics: Vec<TopicArnEntry>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct TopicArnEntry {
    pub topic_arn: String,
}

// --- GetTopicAttributes ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetTopicAttributesRequest {
    pub topic_arn: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetTopicAttributesResponse {
    pub attributes: HashMap<String, String>,
}

// --- SetTopicAttributes ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SetTopicAttributesRequest {
    pub topic_arn: String,
    pub attribute_name: String,
    #[serde(default)]
    pub attribute_value: Option<String>,
}

// --- Subscribe ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(dead_code)]
pub struct SubscribeRequest {
    pub topic_arn: String,
    pub protocol: String,
    #[serde(default)]
    pub endpoint: Option<String>,
    #[serde(default)]
    pub attributes: Option<HashMap<String, String>>,
    #[serde(default)]
    pub return_subscription_arn: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct SubscribeResponse {
    pub subscription_arn: String,
}

// --- Unsubscribe ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UnsubscribeRequest {
    pub subscription_arn: String,
}

// --- ConfirmSubscription ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(dead_code)]
pub struct ConfirmSubscriptionRequest {
    pub topic_arn: String,
    pub token: String,
    #[serde(default)]
    pub authenticate_on_unsubscribe: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfirmSubscriptionResponse {
    pub subscription_arn: String,
}

// --- ListSubscriptions ---

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
#[allow(dead_code)]
pub struct ListSubscriptionsRequest {
    #[serde(default)]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListSubscriptionsResponse {
    pub subscriptions: Vec<SubscriptionEntry>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct SubscriptionEntry {
    pub subscription_arn: String,
    pub owner: String,
    pub protocol: String,
    pub endpoint: String,
    pub topic_arn: String,
}

// --- ListSubscriptionsByTopic ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(dead_code)]
pub struct ListSubscriptionsByTopicRequest {
    pub topic_arn: String,
    #[serde(default)]
    pub next_token: Option<String>,
}

// --- GetSubscriptionAttributes ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetSubscriptionAttributesRequest {
    pub subscription_arn: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetSubscriptionAttributesResponse {
    pub attributes: HashMap<String, String>,
}

// --- SetSubscriptionAttributes ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SetSubscriptionAttributesRequest {
    pub subscription_arn: String,
    pub attribute_name: String,
    #[serde(default)]
    pub attribute_value: Option<String>,
}

// --- Publish ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(dead_code)]
pub struct PublishRequest {
    #[serde(default)]
    pub topic_arn: Option<String>,
    #[serde(default)]
    pub target_arn: Option<String>,
    pub message: String,
    #[serde(default)]
    pub subject: Option<String>,
    #[serde(default)]
    pub message_structure: Option<String>,
    #[serde(default)]
    pub message_attributes: Option<HashMap<String, MessageAttributeValueJson>>,
    #[serde(default)]
    pub message_deduplication_id: Option<String>,
    #[serde(default)]
    pub message_group_id: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PublishResponse {
    pub message_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence_number: Option<String>,
}

// --- PublishBatch ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PublishBatchRequest {
    pub topic_arn: String,
    pub publish_batch_request_entries: Vec<PublishBatchEntry>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(dead_code)]
pub struct PublishBatchEntry {
    pub id: String,
    pub message: String,
    #[serde(default)]
    pub subject: Option<String>,
    #[serde(default)]
    pub message_attributes: Option<HashMap<String, MessageAttributeValueJson>>,
    #[serde(default)]
    pub message_deduplication_id: Option<String>,
    #[serde(default)]
    pub message_group_id: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PublishBatchResponse {
    pub successful: Vec<PublishBatchResultEntry>,
    pub failed: Vec<BatchResultErrorEntry>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PublishBatchResultEntry {
    pub id: String,
    pub message_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence_number: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BatchResultErrorEntry {
    pub id: String,
    pub code: String,
    pub message: String,
    pub sender_fault: bool,
}

// --- TagResource ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TagResourceRequest {
    pub resource_arn: String,
    pub tags: Vec<TagJson>,
}

// --- UntagResource ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UntagResourceRequest {
    pub resource_arn: String,
    pub tag_keys: Vec<String>,
}

// --- ListTagsForResource ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListTagsForResourceRequest {
    pub resource_arn: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListTagsForResourceResponse {
    pub tags: Vec<TagJson>,
}
