mod _types {
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct CreateConnectionRequest {
    #[serde(rename = "ConnectionName")]
    pub connection_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct CreateConnectionResponse {
    #[serde(rename = "ConnectionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_arn: Option<String>,
    #[serde(rename = "ConnectionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_name: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DescribeConnectionRequest {
    #[serde(rename = "ConnectionName")]
    pub connection_name: Option<String>,
    #[serde(rename = "ConnectionArn")]
    pub connection_arn: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
#[allow(dead_code)]
pub struct ConnectionDetail {
    #[serde(rename = "ConnectionName")]
    pub connection_name: String,
    #[serde(rename = "ConnectionArn")]
    pub connection_arn: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct DescribeConnectionResponse {
    #[serde(rename = "Connection")]
    pub connection: ConnectionDetail,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct ListConnectionsRequest {
    #[serde(rename = "MaxResults")]
    pub max_results: Option<usize>,
    #[serde(rename = "NextToken")]
    pub next_token: Option<String>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct ListConnectionsResponse {
    #[serde(rename = "Connections")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connections: Option<Vec<ConnectionDetail>>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DeleteConnectionRequest {
    #[serde(rename = "ConnectionName")]
    pub connection_name: Option<String>,
    #[serde(rename = "ConnectionArn")]
    pub connection_arn: Option<String>,
}

}
pub use _types::*;
