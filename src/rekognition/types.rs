mod _types {
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct CreateCollectionRequest {
    #[serde(rename = "CollectionName")]
    pub collection_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct CreateCollectionResponse {
    #[serde(rename = "CollectionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_arn: Option<String>,
    #[serde(rename = "CollectionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_name: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DescribeCollectionRequest {
    #[serde(rename = "CollectionName")]
    pub collection_name: Option<String>,
    #[serde(rename = "CollectionArn")]
    pub collection_arn: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
#[allow(dead_code)]
pub struct CollectionDetail {
    #[serde(rename = "CollectionName")]
    pub collection_name: String,
    #[serde(rename = "CollectionArn")]
    pub collection_arn: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct DescribeCollectionResponse {
    #[serde(rename = "Collection")]
    pub collection: CollectionDetail,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct ListCollectionsRequest {
    #[serde(rename = "MaxResults")]
    pub max_results: Option<usize>,
    #[serde(rename = "NextToken")]
    pub next_token: Option<String>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct ListCollectionsResponse {
    #[serde(rename = "Collections")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collections: Option<Vec<CollectionDetail>>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DeleteCollectionRequest {
    #[serde(rename = "CollectionName")]
    pub collection_name: Option<String>,
    #[serde(rename = "CollectionArn")]
    pub collection_arn: Option<String>,
}

}
pub use _types::*;
