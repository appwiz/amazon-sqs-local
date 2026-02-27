mod _types {
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct CreateRecommendationRequest {
    #[serde(rename = "RecommendationName")]
    pub recommendation_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct CreateRecommendationResponse {
    #[serde(rename = "RecommendationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_arn: Option<String>,
    #[serde(rename = "RecommendationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_name: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DescribeRecommendationRequest {
    #[serde(rename = "RecommendationName")]
    pub recommendation_name: Option<String>,
    #[serde(rename = "RecommendationArn")]
    pub recommendation_arn: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
#[allow(dead_code)]
pub struct RecommendationDetail {
    #[serde(rename = "RecommendationName")]
    pub recommendation_name: String,
    #[serde(rename = "RecommendationArn")]
    pub recommendation_arn: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct DescribeRecommendationResponse {
    #[serde(rename = "Recommendation")]
    pub recommendation: RecommendationDetail,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct ListRecommendationsRequest {
    #[serde(rename = "MaxResults")]
    pub max_results: Option<usize>,
    #[serde(rename = "NextToken")]
    pub next_token: Option<String>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct ListRecommendationsResponse {
    #[serde(rename = "Recommendations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendations: Option<Vec<RecommendationDetail>>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DeleteRecommendationRequest {
    #[serde(rename = "RecommendationName")]
    pub recommendation_name: Option<String>,
    #[serde(rename = "RecommendationArn")]
    pub recommendation_arn: Option<String>,
}

}
pub use _types::*;
