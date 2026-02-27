mod _types {
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct CreateResourceRequest {
    #[serde(rename = "ResourceName")]
    pub resource_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct CreateResourceResponse {
    #[serde(rename = "ResourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(rename = "ResourceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DescribeResourceRequest {
    #[serde(rename = "ResourceName")]
    pub resource_name: Option<String>,
    #[serde(rename = "ResourceArn")]
    pub resource_arn: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
#[allow(dead_code)]
pub struct ResourceDetail {
    #[serde(rename = "ResourceName")]
    pub resource_name: String,
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct DescribeResourceResponse {
    #[serde(rename = "Resource")]
    pub resource: ResourceDetail,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct ListResourcesRequest {
    #[serde(rename = "MaxResults")]
    pub max_results: Option<usize>,
    #[serde(rename = "NextToken")]
    pub next_token: Option<String>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct ListResourcesResponse {
    #[serde(rename = "Resources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<ResourceDetail>>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DeleteResourceRequest {
    #[serde(rename = "ResourceName")]
    pub resource_name: Option<String>,
    #[serde(rename = "ResourceArn")]
    pub resource_arn: Option<String>,
}

}
pub use _types::*;
