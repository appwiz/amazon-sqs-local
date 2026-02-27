mod _types {
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct CreateWorkspaceRequest {
    #[serde(rename = "WorkspaceName")]
    pub workspace_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct CreateWorkspaceResponse {
    #[serde(rename = "WorkspaceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_arn: Option<String>,
    #[serde(rename = "WorkspaceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_name: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DescribeWorkspaceRequest {
    #[serde(rename = "WorkspaceName")]
    pub workspace_name: Option<String>,
    #[serde(rename = "WorkspaceArn")]
    pub workspace_arn: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
#[allow(dead_code)]
pub struct WorkspaceDetail {
    #[serde(rename = "WorkspaceName")]
    pub workspace_name: String,
    #[serde(rename = "WorkspaceArn")]
    pub workspace_arn: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct DescribeWorkspaceResponse {
    #[serde(rename = "Workspace")]
    pub workspace: WorkspaceDetail,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct ListWorkspacesRequest {
    #[serde(rename = "MaxResults")]
    pub max_results: Option<usize>,
    #[serde(rename = "NextToken")]
    pub next_token: Option<String>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct ListWorkspacesResponse {
    #[serde(rename = "Workspaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspaces: Option<Vec<WorkspaceDetail>>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DeleteWorkspaceRequest {
    #[serde(rename = "WorkspaceName")]
    pub workspace_name: Option<String>,
    #[serde(rename = "WorkspaceArn")]
    pub workspace_arn: Option<String>,
}

}
pub use _types::*;
