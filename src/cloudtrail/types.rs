mod _types {
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct CreateTrailRequest {
    #[serde(rename = "TrailName")]
    pub trail_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct CreateTrailResponse {
    #[serde(rename = "TrailArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trail_arn: Option<String>,
    #[serde(rename = "TrailName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trail_name: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DescribeTrailRequest {
    #[serde(rename = "TrailName")]
    pub trail_name: Option<String>,
    #[serde(rename = "TrailArn")]
    pub trail_arn: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
#[allow(dead_code)]
pub struct TrailDetail {
    #[serde(rename = "TrailName")]
    pub trail_name: String,
    #[serde(rename = "TrailArn")]
    pub trail_arn: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct DescribeTrailResponse {
    #[serde(rename = "Trail")]
    pub trail: TrailDetail,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct ListTrailsRequest {
    #[serde(rename = "MaxResults")]
    pub max_results: Option<usize>,
    #[serde(rename = "NextToken")]
    pub next_token: Option<String>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct ListTrailsResponse {
    #[serde(rename = "Trails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trails: Option<Vec<TrailDetail>>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DeleteTrailRequest {
    #[serde(rename = "TrailName")]
    pub trail_name: Option<String>,
    #[serde(rename = "TrailArn")]
    pub trail_arn: Option<String>,
}

}
pub use _types::*;
