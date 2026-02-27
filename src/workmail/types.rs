mod _types {
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct CreateOrganizationRequest {
    #[serde(rename = "OrganizationName")]
    pub organization_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct CreateOrganizationResponse {
    #[serde(rename = "OrganizationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_arn: Option<String>,
    #[serde(rename = "OrganizationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_name: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DescribeOrganizationRequest {
    #[serde(rename = "OrganizationName")]
    pub organization_name: Option<String>,
    #[serde(rename = "OrganizationArn")]
    pub organization_arn: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
#[allow(dead_code)]
pub struct OrganizationDetail {
    #[serde(rename = "OrganizationName")]
    pub organization_name: String,
    #[serde(rename = "OrganizationArn")]
    pub organization_arn: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct DescribeOrganizationResponse {
    #[serde(rename = "Organization")]
    pub organization: OrganizationDetail,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct ListOrganizationsRequest {
    #[serde(rename = "MaxResults")]
    pub max_results: Option<usize>,
    #[serde(rename = "NextToken")]
    pub next_token: Option<String>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct ListOrganizationsResponse {
    #[serde(rename = "Organizations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organizations: Option<Vec<OrganizationDetail>>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DeleteOrganizationRequest {
    #[serde(rename = "OrganizationName")]
    pub organization_name: Option<String>,
    #[serde(rename = "OrganizationArn")]
    pub organization_arn: Option<String>,
}

}
pub use _types::*;
