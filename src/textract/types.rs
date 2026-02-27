mod _types {
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct CreateAdapterRequest {
    #[serde(rename = "AdapterName")]
    pub adapter_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct CreateAdapterResponse {
    #[serde(rename = "AdapterArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adapter_arn: Option<String>,
    #[serde(rename = "AdapterName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adapter_name: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DescribeAdapterRequest {
    #[serde(rename = "AdapterName")]
    pub adapter_name: Option<String>,
    #[serde(rename = "AdapterArn")]
    pub adapter_arn: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
#[allow(dead_code)]
pub struct AdapterDetail {
    #[serde(rename = "AdapterName")]
    pub adapter_name: String,
    #[serde(rename = "AdapterArn")]
    pub adapter_arn: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct DescribeAdapterResponse {
    #[serde(rename = "Adapter")]
    pub adapter: AdapterDetail,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct ListAdaptersRequest {
    #[serde(rename = "MaxResults")]
    pub max_results: Option<usize>,
    #[serde(rename = "NextToken")]
    pub next_token: Option<String>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct ListAdaptersResponse {
    #[serde(rename = "Adapters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adapters: Option<Vec<AdapterDetail>>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DeleteAdapterRequest {
    #[serde(rename = "AdapterName")]
    pub adapter_name: Option<String>,
    #[serde(rename = "AdapterArn")]
    pub adapter_arn: Option<String>,
}

}
pub use _types::*;
