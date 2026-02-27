mod _types {
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct CreateProjectRequest {
    #[serde(rename = "ProjectName")]
    pub project_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct CreateProjectResponse {
    #[serde(rename = "ProjectArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_arn: Option<String>,
    #[serde(rename = "ProjectName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_name: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DescribeProjectRequest {
    #[serde(rename = "ProjectName")]
    pub project_name: Option<String>,
    #[serde(rename = "ProjectArn")]
    pub project_arn: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
#[allow(dead_code)]
pub struct ProjectDetail {
    #[serde(rename = "ProjectName")]
    pub project_name: String,
    #[serde(rename = "ProjectArn")]
    pub project_arn: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct DescribeProjectResponse {
    #[serde(rename = "Project")]
    pub project: ProjectDetail,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct ListProjectsRequest {
    #[serde(rename = "MaxResults")]
    pub max_results: Option<usize>,
    #[serde(rename = "NextToken")]
    pub next_token: Option<String>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct ListProjectsResponse {
    #[serde(rename = "Projects")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projects: Option<Vec<ProjectDetail>>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DeleteProjectRequest {
    #[serde(rename = "ProjectName")]
    pub project_name: Option<String>,
    #[serde(rename = "ProjectArn")]
    pub project_arn: Option<String>,
}

}
pub use _types::*;
