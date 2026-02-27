mod _types {
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct CreateWorkGroupRequest {
    #[serde(rename = "WorkGroupName")]
    pub work_group_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct CreateWorkGroupResponse {
    #[serde(rename = "WorkGroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_group_arn: Option<String>,
    #[serde(rename = "WorkGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_group_name: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DescribeWorkGroupRequest {
    #[serde(rename = "WorkGroupName")]
    pub work_group_name: Option<String>,
    #[serde(rename = "WorkGroupArn")]
    pub work_group_arn: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
#[allow(dead_code)]
pub struct WorkGroupDetail {
    #[serde(rename = "WorkGroupName")]
    pub work_group_name: String,
    #[serde(rename = "WorkGroupArn")]
    pub work_group_arn: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct DescribeWorkGroupResponse {
    #[serde(rename = "WorkGroup")]
    pub work_group: WorkGroupDetail,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct ListWorkGroupsRequest {
    #[serde(rename = "MaxResults")]
    pub max_results: Option<usize>,
    #[serde(rename = "NextToken")]
    pub next_token: Option<String>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct ListWorkGroupsResponse {
    #[serde(rename = "WorkGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_groups: Option<Vec<WorkGroupDetail>>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DeleteWorkGroupRequest {
    #[serde(rename = "WorkGroupName")]
    pub work_group_name: Option<String>,
    #[serde(rename = "WorkGroupArn")]
    pub work_group_arn: Option<String>,
}

}
pub use _types::*;
