mod _types {
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct CreateIndexRequest {
    #[serde(rename = "IndexName")]
    pub index_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct CreateIndexResponse {
    #[serde(rename = "IndexArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_arn: Option<String>,
    #[serde(rename = "IndexName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DescribeIndexRequest {
    #[serde(rename = "IndexName")]
    pub index_name: Option<String>,
    #[serde(rename = "IndexArn")]
    pub index_arn: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
#[allow(dead_code)]
pub struct IndexDetail {
    #[serde(rename = "IndexName")]
    pub index_name: String,
    #[serde(rename = "IndexArn")]
    pub index_arn: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct DescribeIndexResponse {
    #[serde(rename = "Index")]
    pub index: IndexDetail,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct ListIndexsRequest {
    #[serde(rename = "MaxResults")]
    pub max_results: Option<usize>,
    #[serde(rename = "NextToken")]
    pub next_token: Option<String>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct ListIndexsResponse {
    #[serde(rename = "Indexs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indexs: Option<Vec<IndexDetail>>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DeleteIndexRequest {
    #[serde(rename = "IndexName")]
    pub index_name: Option<String>,
    #[serde(rename = "IndexArn")]
    pub index_arn: Option<String>,
}

}
pub use _types::*;
