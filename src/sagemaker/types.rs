mod _types {
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct CreateNotebookInstanceRequest {
    #[serde(rename = "NotebookInstanceName")]
    pub notebook_instance_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct CreateNotebookInstanceResponse {
    #[serde(rename = "NotebookInstanceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notebook_instance_arn: Option<String>,
    #[serde(rename = "NotebookInstanceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notebook_instance_name: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DescribeNotebookInstanceRequest {
    #[serde(rename = "NotebookInstanceName")]
    pub notebook_instance_name: Option<String>,
    #[serde(rename = "NotebookInstanceArn")]
    pub notebook_instance_arn: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
#[allow(dead_code)]
pub struct NotebookInstanceDetail {
    #[serde(rename = "NotebookInstanceName")]
    pub notebook_instance_name: String,
    #[serde(rename = "NotebookInstanceArn")]
    pub notebook_instance_arn: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct DescribeNotebookInstanceResponse {
    #[serde(rename = "NotebookInstance")]
    pub notebook_instance: NotebookInstanceDetail,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct ListNotebookInstancesRequest {
    #[serde(rename = "MaxResults")]
    pub max_results: Option<usize>,
    #[serde(rename = "NextToken")]
    pub next_token: Option<String>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct ListNotebookInstancesResponse {
    #[serde(rename = "NotebookInstances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notebook_instances: Option<Vec<NotebookInstanceDetail>>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DeleteNotebookInstanceRequest {
    #[serde(rename = "NotebookInstanceName")]
    pub notebook_instance_name: Option<String>,
    #[serde(rename = "NotebookInstanceArn")]
    pub notebook_instance_arn: Option<String>,
}

}
pub use _types::*;
