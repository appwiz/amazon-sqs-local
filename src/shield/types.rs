mod _types {
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct CreateProtectionRequest {
    #[serde(rename = "ProtectionName")]
    pub protection_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct CreateProtectionResponse {
    #[serde(rename = "ProtectionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protection_arn: Option<String>,
    #[serde(rename = "ProtectionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protection_name: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DescribeProtectionRequest {
    #[serde(rename = "ProtectionName")]
    pub protection_name: Option<String>,
    #[serde(rename = "ProtectionArn")]
    pub protection_arn: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
#[allow(dead_code)]
pub struct ProtectionDetail {
    #[serde(rename = "ProtectionName")]
    pub protection_name: String,
    #[serde(rename = "ProtectionArn")]
    pub protection_arn: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct DescribeProtectionResponse {
    #[serde(rename = "Protection")]
    pub protection: ProtectionDetail,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct ListProtectionsRequest {
    #[serde(rename = "MaxResults")]
    pub max_results: Option<usize>,
    #[serde(rename = "NextToken")]
    pub next_token: Option<String>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct ListProtectionsResponse {
    #[serde(rename = "Protections")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protections: Option<Vec<ProtectionDetail>>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DeleteProtectionRequest {
    #[serde(rename = "ProtectionName")]
    pub protection_name: Option<String>,
    #[serde(rename = "ProtectionArn")]
    pub protection_arn: Option<String>,
}

}
pub use _types::*;
