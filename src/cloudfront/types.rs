mod _types {
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct CreateDistributionRequest {
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(rename = "Name")]
    pub name_pascal: Option<String>,
    #[serde(rename = "tags")]
    #[serde(default)]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize, Clone)]
#[allow(dead_code)]
pub struct DistributionDetail {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "arn")]
    pub arn: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct ListDistributionsResponse {
    #[serde(rename = "distributions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<DistributionDetail>>,
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

}
pub use _types::*;
