mod _types {
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct CreateAcceleratorRequest {
    #[serde(rename = "AcceleratorName")]
    pub accelerator_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct CreateAcceleratorResponse {
    #[serde(rename = "AcceleratorArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accelerator_arn: Option<String>,
    #[serde(rename = "AcceleratorName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accelerator_name: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DescribeAcceleratorRequest {
    #[serde(rename = "AcceleratorName")]
    pub accelerator_name: Option<String>,
    #[serde(rename = "AcceleratorArn")]
    pub accelerator_arn: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
#[allow(dead_code)]
pub struct AcceleratorDetail {
    #[serde(rename = "AcceleratorName")]
    pub accelerator_name: String,
    #[serde(rename = "AcceleratorArn")]
    pub accelerator_arn: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct DescribeAcceleratorResponse {
    #[serde(rename = "Accelerator")]
    pub accelerator: AcceleratorDetail,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct ListAcceleratorsRequest {
    #[serde(rename = "MaxResults")]
    pub max_results: Option<usize>,
    #[serde(rename = "NextToken")]
    pub next_token: Option<String>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct ListAcceleratorsResponse {
    #[serde(rename = "Accelerators")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accelerators: Option<Vec<AcceleratorDetail>>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DeleteAcceleratorRequest {
    #[serde(rename = "AcceleratorName")]
    pub accelerator_name: Option<String>,
    #[serde(rename = "AcceleratorArn")]
    pub accelerator_arn: Option<String>,
}

}
pub use _types::*;
