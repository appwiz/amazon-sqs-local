mod _types {
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct CreateServerRequest {
    #[serde(rename = "ServerName")]
    pub server_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct CreateServerResponse {
    #[serde(rename = "ServerArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_arn: Option<String>,
    #[serde(rename = "ServerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DescribeServerRequest {
    #[serde(rename = "ServerName")]
    pub server_name: Option<String>,
    #[serde(rename = "ServerArn")]
    pub server_arn: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
#[allow(dead_code)]
pub struct ServerDetail {
    #[serde(rename = "ServerName")]
    pub server_name: String,
    #[serde(rename = "ServerArn")]
    pub server_arn: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct DescribeServerResponse {
    #[serde(rename = "Server")]
    pub server: ServerDetail,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct ListServersRequest {
    #[serde(rename = "MaxResults")]
    pub max_results: Option<usize>,
    #[serde(rename = "NextToken")]
    pub next_token: Option<String>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct ListServersResponse {
    #[serde(rename = "Servers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servers: Option<Vec<ServerDetail>>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DeleteServerRequest {
    #[serde(rename = "ServerName")]
    pub server_name: Option<String>,
    #[serde(rename = "ServerArn")]
    pub server_arn: Option<String>,
}

}
pub use _types::*;
