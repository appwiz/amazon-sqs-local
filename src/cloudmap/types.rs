mod _types {
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct CreateNamespaceRequest {
    #[serde(rename = "NamespaceName")]
    pub namespace_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct CreateNamespaceResponse {
    #[serde(rename = "NamespaceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_arn: Option<String>,
    #[serde(rename = "NamespaceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_name: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DescribeNamespaceRequest {
    #[serde(rename = "NamespaceName")]
    pub namespace_name: Option<String>,
    #[serde(rename = "NamespaceArn")]
    pub namespace_arn: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
#[allow(dead_code)]
pub struct NamespaceDetail {
    #[serde(rename = "NamespaceName")]
    pub namespace_name: String,
    #[serde(rename = "NamespaceArn")]
    pub namespace_arn: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct DescribeNamespaceResponse {
    #[serde(rename = "Namespace")]
    pub namespace: NamespaceDetail,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct ListNamespacesRequest {
    #[serde(rename = "MaxResults")]
    pub max_results: Option<usize>,
    #[serde(rename = "NextToken")]
    pub next_token: Option<String>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct ListNamespacesResponse {
    #[serde(rename = "Namespaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespaces: Option<Vec<NamespaceDetail>>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DeleteNamespaceRequest {
    #[serde(rename = "NamespaceName")]
    pub namespace_name: Option<String>,
    #[serde(rename = "NamespaceArn")]
    pub namespace_arn: Option<String>,
}

}
pub use _types::*;
