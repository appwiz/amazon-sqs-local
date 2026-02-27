mod _types {
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct CreateGatewayRequest {
    #[serde(rename = "GatewayName")]
    pub gateway_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct CreateGatewayResponse {
    #[serde(rename = "GatewayArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
    #[serde(rename = "GatewayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_name: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DescribeGatewayRequest {
    #[serde(rename = "GatewayName")]
    pub gateway_name: Option<String>,
    #[serde(rename = "GatewayArn")]
    pub gateway_arn: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
#[allow(dead_code)]
pub struct GatewayDetail {
    #[serde(rename = "GatewayName")]
    pub gateway_name: String,
    #[serde(rename = "GatewayArn")]
    pub gateway_arn: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct DescribeGatewayResponse {
    #[serde(rename = "Gateway")]
    pub gateway: GatewayDetail,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct ListGatewaysRequest {
    #[serde(rename = "MaxResults")]
    pub max_results: Option<usize>,
    #[serde(rename = "NextToken")]
    pub next_token: Option<String>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct ListGatewaysResponse {
    #[serde(rename = "Gateways")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateways: Option<Vec<GatewayDetail>>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DeleteGatewayRequest {
    #[serde(rename = "GatewayName")]
    pub gateway_name: Option<String>,
    #[serde(rename = "GatewayArn")]
    pub gateway_arn: Option<String>,
}

}
pub use _types::*;
