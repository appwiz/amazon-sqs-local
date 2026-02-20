use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct CreateEventBusRequest {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "EventSourceName")]
    pub event_source_name: Option<String>,
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Serialize)]
pub struct CreateEventBusResponse {
    #[serde(rename = "EventBusArn")]
    pub event_bus_arn: String,
}

#[derive(Debug, Deserialize)]
pub struct DeleteEventBusRequest {
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct DescribeEventBusRequest {
    #[serde(rename = "Name")]
    pub name: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct DescribeEventBusResponse {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Arn")]
    pub arn: String,
    #[serde(rename = "Policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
pub struct ListEventBusesRequest {
    #[serde(rename = "NamePrefix")]
    pub name_prefix: Option<String>,
    #[serde(rename = "Limit")]
    pub limit: Option<usize>,
    #[serde(rename = "NextToken")]
    pub next_token: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ListEventBusesResponse {
    #[serde(rename = "EventBuses")]
    pub event_buses: Vec<EventBus>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct EventBus {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Arn")]
    pub arn: String,
}

#[derive(Debug, Deserialize)]
pub struct PutEventsRequest {
    #[serde(rename = "Entries")]
    pub entries: Vec<PutEventsRequestEntry>,
}

#[derive(Debug, Deserialize)]
pub struct PutEventsRequestEntry {
    #[serde(rename = "EventBusName")]
    pub event_bus_name: Option<String>,
    #[serde(rename = "Source")]
    pub source: Option<String>,
    #[serde(rename = "DetailType")]
    pub detail_type: Option<String>,
    #[serde(rename = "Detail")]
    pub detail: Option<String>,
    #[serde(rename = "Resources")]
    pub resources: Option<Vec<String>>,
    #[serde(rename = "Time")]
    pub time: Option<f64>,
}

#[derive(Debug, Serialize)]
pub struct PutEventsResponse {
    #[serde(rename = "FailedEntryCount")]
    pub failed_entry_count: u32,
    #[serde(rename = "Entries")]
    pub entries: Vec<PutEventsResultEntry>,
}

#[derive(Debug, Serialize)]
pub struct PutEventsResultEntry {
    #[serde(rename = "EventId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    #[serde(rename = "ErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PutRuleRequest {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "EventBusName")]
    pub event_bus_name: Option<String>,
    #[serde(rename = "ScheduleExpression")]
    pub schedule_expression: Option<String>,
    #[serde(rename = "EventPattern")]
    pub event_pattern: Option<String>,
    #[serde(rename = "State")]
    pub state: Option<String>,
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Serialize)]
pub struct PutRuleResponse {
    #[serde(rename = "RuleArn")]
    pub rule_arn: String,
}

#[derive(Debug, Deserialize)]
pub struct DeleteRuleRequest {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "EventBusName")]
    pub event_bus_name: Option<String>,
    #[serde(rename = "Force")]
    pub force: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct DescribeRuleRequest {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "EventBusName")]
    pub event_bus_name: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct DescribeRuleResponse {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Arn")]
    pub arn: String,
    #[serde(rename = "EventPattern")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_pattern: Option<String>,
    #[serde(rename = "ScheduleExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_expression: Option<String>,
    #[serde(rename = "State")]
    pub state: String,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EventBusName")]
    pub event_bus_name: String,
}

#[derive(Debug, Deserialize, Default)]
pub struct ListRulesRequest {
    #[serde(rename = "EventBusName")]
    pub event_bus_name: Option<String>,
    #[serde(rename = "NamePrefix")]
    pub name_prefix: Option<String>,
    #[serde(rename = "Limit")]
    pub limit: Option<usize>,
    #[serde(rename = "NextToken")]
    pub next_token: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ListRulesResponse {
    #[serde(rename = "Rules")]
    pub rules: Vec<Rule>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct Rule {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Arn")]
    pub arn: String,
    #[serde(rename = "State")]
    pub state: String,
    #[serde(rename = "EventBusName")]
    pub event_bus_name: String,
}

#[derive(Debug, Deserialize)]
pub struct PutTargetsRequest {
    #[serde(rename = "Rule")]
    pub rule: String,
    #[serde(rename = "EventBusName")]
    pub event_bus_name: Option<String>,
    #[serde(rename = "Targets")]
    pub targets: Vec<Target>,
}

#[derive(Debug, Serialize)]
pub struct PutTargetsResponse {
    #[serde(rename = "FailedEntryCount")]
    pub failed_entry_count: u32,
    #[serde(rename = "FailedEntries")]
    pub failed_entries: Vec<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct RemoveTargetsRequest {
    #[serde(rename = "Rule")]
    pub rule: String,
    #[serde(rename = "EventBusName")]
    pub event_bus_name: Option<String>,
    #[serde(rename = "Ids")]
    pub ids: Vec<String>,
    #[serde(rename = "Force")]
    pub force: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct RemoveTargetsResponse {
    #[serde(rename = "FailedEntryCount")]
    pub failed_entry_count: u32,
    #[serde(rename = "FailedEntries")]
    pub failed_entries: Vec<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct ListTargetsByRuleRequest {
    #[serde(rename = "Rule")]
    pub rule: String,
    #[serde(rename = "EventBusName")]
    pub event_bus_name: Option<String>,
    #[serde(rename = "NextToken")]
    pub next_token: Option<String>,
    #[serde(rename = "Limit")]
    pub limit: Option<usize>,
}

#[derive(Debug, Serialize)]
pub struct ListTargetsByRuleResponse {
    #[serde(rename = "Targets")]
    pub targets: Vec<Target>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Target {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Arn")]
    pub arn: String,
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "Input")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    #[serde(rename = "InputPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_path: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,
}

#[derive(Debug, Deserialize)]
pub struct TagResourceRequest {
    #[serde(rename = "ResourceARN")]
    pub resource_arn: String,
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

#[derive(Debug, Deserialize)]
pub struct UntagResourceRequest {
    #[serde(rename = "ResourceARN")]
    pub resource_arn: String,
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct ListTagsForResourceRequest {
    #[serde(rename = "ResourceARN")]
    pub resource_arn: String,
}

#[derive(Debug, Serialize)]
pub struct ListTagsForResourceResponse {
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

// Used internally for storing events
#[derive(Debug, Clone)]
pub struct StoredEvent {
    pub event_id: String,
    pub event_bus_name: String,
    pub source: String,
    pub detail_type: String,
    pub detail: String,
    pub timestamp: f64,
}

// Used for structured storage per bus
pub type TagMap = HashMap<String, String>;
