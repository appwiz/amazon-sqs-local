mod _types {
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct CreateDirectoryRequest {
    #[serde(rename = "DirectoryName")]
    pub directory_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct CreateDirectoryResponse {
    #[serde(rename = "DirectoryArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_arn: Option<String>,
    #[serde(rename = "DirectoryName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_name: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DescribeDirectoryRequest {
    #[serde(rename = "DirectoryName")]
    pub directory_name: Option<String>,
    #[serde(rename = "DirectoryArn")]
    pub directory_arn: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
#[allow(dead_code)]
pub struct DirectoryDetail {
    #[serde(rename = "DirectoryName")]
    pub directory_name: String,
    #[serde(rename = "DirectoryArn")]
    pub directory_arn: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct DescribeDirectoryResponse {
    #[serde(rename = "Directory")]
    pub directory: DirectoryDetail,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct ListDirectorysRequest {
    #[serde(rename = "MaxResults")]
    pub max_results: Option<usize>,
    #[serde(rename = "NextToken")]
    pub next_token: Option<String>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct ListDirectorysResponse {
    #[serde(rename = "Directorys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directorys: Option<Vec<DirectoryDetail>>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DeleteDirectoryRequest {
    #[serde(rename = "DirectoryName")]
    pub directory_name: Option<String>,
    #[serde(rename = "DirectoryArn")]
    pub directory_arn: Option<String>,
}

}
pub use _types::*;
