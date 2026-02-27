mod _types {
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct CreateServiceRequest {
    #[serde(rename = "ServiceName")]
    pub service_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct CreateServiceResponse {
    #[serde(rename = "ServiceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_arn: Option<String>,
    #[serde(rename = "ServiceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DescribeServiceRequest {
    #[serde(rename = "ServiceName")]
    pub service_name: Option<String>,
    #[serde(rename = "ServiceArn")]
    pub service_arn: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
#[allow(dead_code)]
pub struct ServiceDetail {
    #[serde(rename = "ServiceName")]
    pub service_name: String,
    #[serde(rename = "ServiceArn")]
    pub service_arn: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct DescribeServiceResponse {
    #[serde(rename = "Service")]
    pub service: ServiceDetail,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct ListServicesRequest {
    #[serde(rename = "MaxResults")]
    pub max_results: Option<usize>,
    #[serde(rename = "NextToken")]
    pub next_token: Option<String>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct ListServicesResponse {
    #[serde(rename = "Services")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<ServiceDetail>>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DeleteServiceRequest {
    #[serde(rename = "ServiceName")]
    pub service_name: Option<String>,
    #[serde(rename = "ServiceArn")]
    pub service_arn: Option<String>,
}

}
pub use _types::*;
