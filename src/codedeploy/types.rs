mod _types {
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct CreateApplicationRequest {
    #[serde(rename = "ApplicationName")]
    pub application_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct CreateApplicationResponse {
    #[serde(rename = "ApplicationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_arn: Option<String>,
    #[serde(rename = "ApplicationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_name: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DescribeApplicationRequest {
    #[serde(rename = "ApplicationName")]
    pub application_name: Option<String>,
    #[serde(rename = "ApplicationArn")]
    pub application_arn: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
#[allow(dead_code)]
pub struct ApplicationDetail {
    #[serde(rename = "ApplicationName")]
    pub application_name: String,
    #[serde(rename = "ApplicationArn")]
    pub application_arn: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct DescribeApplicationResponse {
    #[serde(rename = "Application")]
    pub application: ApplicationDetail,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct ListApplicationsRequest {
    #[serde(rename = "MaxResults")]
    pub max_results: Option<usize>,
    #[serde(rename = "NextToken")]
    pub next_token: Option<String>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct ListApplicationsResponse {
    #[serde(rename = "Applications")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applications: Option<Vec<ApplicationDetail>>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DeleteApplicationRequest {
    #[serde(rename = "ApplicationName")]
    pub application_name: Option<String>,
    #[serde(rename = "ApplicationArn")]
    pub application_arn: Option<String>,
}

}
pub use _types::*;
