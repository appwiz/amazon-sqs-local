mod _types {
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct CreateTerminologyRequest {
    #[serde(rename = "TerminologyName")]
    pub terminology_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct CreateTerminologyResponse {
    #[serde(rename = "TerminologyArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminology_arn: Option<String>,
    #[serde(rename = "TerminologyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminology_name: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DescribeTerminologyRequest {
    #[serde(rename = "TerminologyName")]
    pub terminology_name: Option<String>,
    #[serde(rename = "TerminologyArn")]
    pub terminology_arn: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
#[allow(dead_code)]
pub struct TerminologyDetail {
    #[serde(rename = "TerminologyName")]
    pub terminology_name: String,
    #[serde(rename = "TerminologyArn")]
    pub terminology_arn: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct DescribeTerminologyResponse {
    #[serde(rename = "Terminology")]
    pub terminology: TerminologyDetail,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct ListTerminologysRequest {
    #[serde(rename = "MaxResults")]
    pub max_results: Option<usize>,
    #[serde(rename = "NextToken")]
    pub next_token: Option<String>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct ListTerminologysResponse {
    #[serde(rename = "Terminologys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminologys: Option<Vec<TerminologyDetail>>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DeleteTerminologyRequest {
    #[serde(rename = "TerminologyName")]
    pub terminology_name: Option<String>,
    #[serde(rename = "TerminologyArn")]
    pub terminology_arn: Option<String>,
}

}
pub use _types::*;
