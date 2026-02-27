mod _types {
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct CreateContainerRequest {
    #[serde(rename = "ContainerName")]
    pub container_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct CreateContainerResponse {
    #[serde(rename = "ContainerArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_arn: Option<String>,
    #[serde(rename = "ContainerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_name: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DescribeContainerRequest {
    #[serde(rename = "ContainerName")]
    pub container_name: Option<String>,
    #[serde(rename = "ContainerArn")]
    pub container_arn: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
#[allow(dead_code)]
pub struct ContainerDetail {
    #[serde(rename = "ContainerName")]
    pub container_name: String,
    #[serde(rename = "ContainerArn")]
    pub container_arn: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct DescribeContainerResponse {
    #[serde(rename = "Container")]
    pub container: ContainerDetail,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct ListContainersRequest {
    #[serde(rename = "MaxResults")]
    pub max_results: Option<usize>,
    #[serde(rename = "NextToken")]
    pub next_token: Option<String>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct ListContainersResponse {
    #[serde(rename = "Containers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub containers: Option<Vec<ContainerDetail>>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DeleteContainerRequest {
    #[serde(rename = "ContainerName")]
    pub container_name: Option<String>,
    #[serde(rename = "ContainerArn")]
    pub container_arn: Option<String>,
}

}
pub use _types::*;
