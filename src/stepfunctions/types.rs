use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateStateMachineRequest {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "definition")]
    pub definition: String,
    #[serde(rename = "roleArn")]
    pub role_arn: String,
    #[serde(rename = "type")]
    pub machine_type: Option<String>,
    #[serde(rename = "loggingConfiguration")]
    pub logging_configuration: Option<serde_json::Value>,
    #[serde(rename = "tracingConfiguration")]
    pub tracing_configuration: Option<serde_json::Value>,
    #[serde(rename = "tags")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Serialize)]
pub struct CreateStateMachineResponse {
    #[serde(rename = "stateMachineArn")]
    pub state_machine_arn: String,
    #[serde(rename = "creationDate")]
    pub creation_date: f64,
}

#[derive(Debug, Deserialize)]
pub struct DeleteStateMachineRequest {
    #[serde(rename = "stateMachineArn")]
    pub state_machine_arn: String,
}

#[derive(Debug, Deserialize)]
pub struct DescribeStateMachineRequest {
    #[serde(rename = "stateMachineArn")]
    pub state_machine_arn: String,
}

#[derive(Debug, Serialize)]
pub struct DescribeStateMachineResponse {
    #[serde(rename = "stateMachineArn")]
    pub state_machine_arn: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "definition")]
    pub definition: String,
    #[serde(rename = "roleArn")]
    pub role_arn: String,
    #[serde(rename = "type")]
    pub machine_type: String,
    #[serde(rename = "creationDate")]
    pub creation_date: f64,
}

#[derive(Debug, Deserialize, Default)]
pub struct ListStateMachinesRequest {
    #[serde(rename = "maxResults")]
    pub max_results: Option<usize>,
    #[serde(rename = "nextToken")]
    pub next_token: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ListStateMachinesResponse {
    #[serde(rename = "stateMachines")]
    pub state_machines: Vec<StateMachineListItem>,
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct StateMachineListItem {
    #[serde(rename = "stateMachineArn")]
    pub state_machine_arn: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "type")]
    pub machine_type: String,
    #[serde(rename = "creationDate")]
    pub creation_date: f64,
}

#[derive(Debug, Deserialize)]
pub struct StartExecutionRequest {
    #[serde(rename = "stateMachineArn")]
    pub state_machine_arn: String,
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(rename = "input")]
    pub input: Option<String>,
    #[serde(rename = "traceHeader")]
    pub trace_header: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct StartExecutionResponse {
    #[serde(rename = "executionArn")]
    pub execution_arn: String,
    #[serde(rename = "startDate")]
    pub start_date: f64,
}

#[derive(Debug, Deserialize)]
pub struct StopExecutionRequest {
    #[serde(rename = "executionArn")]
    pub execution_arn: String,
    #[serde(rename = "error")]
    pub error: Option<String>,
    #[serde(rename = "cause")]
    pub cause: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct StopExecutionResponse {
    #[serde(rename = "stopDate")]
    pub stop_date: f64,
}

#[derive(Debug, Deserialize)]
pub struct DescribeExecutionRequest {
    #[serde(rename = "executionArn")]
    pub execution_arn: String,
}

#[derive(Debug, Serialize)]
pub struct DescribeExecutionResponse {
    #[serde(rename = "executionArn")]
    pub execution_arn: String,
    #[serde(rename = "stateMachineArn")]
    pub state_machine_arn: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "startDate")]
    pub start_date: f64,
    #[serde(rename = "stopDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_date: Option<f64>,
    #[serde(rename = "input")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    #[serde(rename = "output")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
pub struct ListExecutionsRequest {
    #[serde(rename = "stateMachineArn")]
    pub state_machine_arn: Option<String>,
    #[serde(rename = "statusFilter")]
    pub status_filter: Option<String>,
    #[serde(rename = "maxResults")]
    pub max_results: Option<usize>,
    #[serde(rename = "nextToken")]
    pub next_token: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ListExecutionsResponse {
    #[serde(rename = "executions")]
    pub executions: Vec<ExecutionListItem>,
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct ExecutionListItem {
    #[serde(rename = "executionArn")]
    pub execution_arn: String,
    #[serde(rename = "stateMachineArn")]
    pub state_machine_arn: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "startDate")]
    pub start_date: f64,
    #[serde(rename = "stopDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_date: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct GetExecutionHistoryRequest {
    #[serde(rename = "executionArn")]
    pub execution_arn: String,
    #[serde(rename = "maxResults")]
    pub max_results: Option<usize>,
    #[serde(rename = "reverseOrder")]
    pub reverse_order: Option<bool>,
    #[serde(rename = "nextToken")]
    pub next_token: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct GetExecutionHistoryResponse {
    #[serde(rename = "events")]
    pub events: Vec<HistoryEvent>,
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct HistoryEvent {
    #[serde(rename = "id")]
    pub id: u64,
    #[serde(rename = "type")]
    pub event_type: String,
    #[serde(rename = "timestamp")]
    pub timestamp: f64,
    #[serde(rename = "previousEventId")]
    pub previous_event_id: u64,
    #[serde(rename = "executionStartedEventDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_started_event_details: Option<serde_json::Value>,
    #[serde(rename = "executionSucceededEventDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_succeeded_event_details: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct SendTaskSuccessRequest {
    #[serde(rename = "taskToken")]
    pub task_token: String,
    #[serde(rename = "output")]
    pub output: String,
}

#[derive(Debug, Deserialize)]
pub struct SendTaskFailureRequest {
    #[serde(rename = "taskToken")]
    pub task_token: String,
    #[serde(rename = "error")]
    pub error: Option<String>,
    #[serde(rename = "cause")]
    pub cause: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SendTaskHeartbeatRequest {
    #[serde(rename = "taskToken")]
    pub task_token: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Tag {
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "value")]
    pub value: String,
}

#[derive(Debug, Deserialize)]
pub struct TagResourceRequest {
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    #[serde(rename = "tags")]
    pub tags: Vec<Tag>,
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
    pub tags: Vec<Tag>,
}
