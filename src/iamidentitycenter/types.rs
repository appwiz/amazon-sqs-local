mod _types {
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct CreatePermissionSetRequest {
    #[serde(rename = "PermissionSetName")]
    pub permission_set_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct CreatePermissionSetResponse {
    #[serde(rename = "PermissionSetArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_set_arn: Option<String>,
    #[serde(rename = "PermissionSetName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_set_name: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DescribePermissionSetRequest {
    #[serde(rename = "PermissionSetName")]
    pub permission_set_name: Option<String>,
    #[serde(rename = "PermissionSetArn")]
    pub permission_set_arn: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
#[allow(dead_code)]
pub struct PermissionSetDetail {
    #[serde(rename = "PermissionSetName")]
    pub permission_set_name: String,
    #[serde(rename = "PermissionSetArn")]
    pub permission_set_arn: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct DescribePermissionSetResponse {
    #[serde(rename = "PermissionSet")]
    pub permission_set: PermissionSetDetail,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct ListPermissionSetsRequest {
    #[serde(rename = "MaxResults")]
    pub max_results: Option<usize>,
    #[serde(rename = "NextToken")]
    pub next_token: Option<String>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct ListPermissionSetsResponse {
    #[serde(rename = "PermissionSets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_sets: Option<Vec<PermissionSetDetail>>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DeletePermissionSetRequest {
    #[serde(rename = "PermissionSetName")]
    pub permission_set_name: Option<String>,
    #[serde(rename = "PermissionSetArn")]
    pub permission_set_arn: Option<String>,
}

}
pub use _types::*;
