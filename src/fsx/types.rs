mod _types {
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct CreateFileSystemRequest {
    #[serde(rename = "FileSystemName")]
    pub file_system_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct CreateFileSystemResponse {
    #[serde(rename = "FileSystemArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_arn: Option<String>,
    #[serde(rename = "FileSystemName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_name: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DescribeFileSystemRequest {
    #[serde(rename = "FileSystemName")]
    pub file_system_name: Option<String>,
    #[serde(rename = "FileSystemArn")]
    pub file_system_arn: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
#[allow(dead_code)]
pub struct FileSystemDetail {
    #[serde(rename = "FileSystemName")]
    pub file_system_name: String,
    #[serde(rename = "FileSystemArn")]
    pub file_system_arn: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct DescribeFileSystemResponse {
    #[serde(rename = "FileSystem")]
    pub file_system: FileSystemDetail,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct ListFileSystemsRequest {
    #[serde(rename = "MaxResults")]
    pub max_results: Option<usize>,
    #[serde(rename = "NextToken")]
    pub next_token: Option<String>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct ListFileSystemsResponse {
    #[serde(rename = "FileSystems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_systems: Option<Vec<FileSystemDetail>>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DeleteFileSystemRequest {
    #[serde(rename = "FileSystemName")]
    pub file_system_name: Option<String>,
    #[serde(rename = "FileSystemArn")]
    pub file_system_arn: Option<String>,
}

}
pub use _types::*;
