mod _types {
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct CreateTaskRequest {
    #[serde(rename = "TaskName")]
    pub task_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct CreateTaskResponse {
    #[serde(rename = "TaskArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_arn: Option<String>,
    #[serde(rename = "TaskName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_name: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DescribeTaskRequest {
    #[serde(rename = "TaskName")]
    pub task_name: Option<String>,
    #[serde(rename = "TaskArn")]
    pub task_arn: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
#[allow(dead_code)]
pub struct TaskDetail {
    #[serde(rename = "TaskName")]
    pub task_name: String,
    #[serde(rename = "TaskArn")]
    pub task_arn: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct DescribeTaskResponse {
    #[serde(rename = "Task")]
    pub task: TaskDetail,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct ListTasksRequest {
    #[serde(rename = "MaxResults")]
    pub max_results: Option<usize>,
    #[serde(rename = "NextToken")]
    pub next_token: Option<String>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct ListTasksResponse {
    #[serde(rename = "Tasks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tasks: Option<Vec<TaskDetail>>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DeleteTaskRequest {
    #[serde(rename = "TaskName")]
    pub task_name: Option<String>,
    #[serde(rename = "TaskArn")]
    pub task_arn: Option<String>,
}

}
pub use _types::*;
