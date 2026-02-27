mod _types {
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct CreateClusterRequest {
    #[serde(rename = "ClusterName")]
    pub cluster_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct CreateClusterResponse {
    #[serde(rename = "ClusterArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arn: Option<String>,
    #[serde(rename = "ClusterName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DescribeClusterRequest {
    #[serde(rename = "ClusterName")]
    pub cluster_name: Option<String>,
    #[serde(rename = "ClusterArn")]
    pub cluster_arn: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
#[allow(dead_code)]
pub struct ClusterDetail {
    #[serde(rename = "ClusterName")]
    pub cluster_name: String,
    #[serde(rename = "ClusterArn")]
    pub cluster_arn: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct DescribeClusterResponse {
    #[serde(rename = "Cluster")]
    pub cluster: ClusterDetail,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct ListClustersRequest {
    #[serde(rename = "MaxResults")]
    pub max_results: Option<usize>,
    #[serde(rename = "NextToken")]
    pub next_token: Option<String>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct ListClustersResponse {
    #[serde(rename = "Clusters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clusters: Option<Vec<ClusterDetail>>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DeleteClusterRequest {
    #[serde(rename = "ClusterName")]
    pub cluster_name: Option<String>,
    #[serde(rename = "ClusterArn")]
    pub cluster_arn: Option<String>,
}

}
pub use _types::*;
