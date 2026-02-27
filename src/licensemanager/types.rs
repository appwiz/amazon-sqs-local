mod _types {
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct CreateLicenseRequest {
    #[serde(rename = "LicenseName")]
    pub license_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct CreateLicenseResponse {
    #[serde(rename = "LicenseArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_arn: Option<String>,
    #[serde(rename = "LicenseName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_name: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DescribeLicenseRequest {
    #[serde(rename = "LicenseName")]
    pub license_name: Option<String>,
    #[serde(rename = "LicenseArn")]
    pub license_arn: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
#[allow(dead_code)]
pub struct LicenseDetail {
    #[serde(rename = "LicenseName")]
    pub license_name: String,
    #[serde(rename = "LicenseArn")]
    pub license_arn: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct DescribeLicenseResponse {
    #[serde(rename = "License")]
    pub license: LicenseDetail,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct ListLicensesRequest {
    #[serde(rename = "MaxResults")]
    pub max_results: Option<usize>,
    #[serde(rename = "NextToken")]
    pub next_token: Option<String>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct ListLicensesResponse {
    #[serde(rename = "Licenses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub licenses: Option<Vec<LicenseDetail>>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DeleteLicenseRequest {
    #[serde(rename = "LicenseName")]
    pub license_name: Option<String>,
    #[serde(rename = "LicenseArn")]
    pub license_arn: Option<String>,
}

}
pub use _types::*;
