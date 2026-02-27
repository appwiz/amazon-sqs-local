mod _types {
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct CreatePolicyRequest {
    #[serde(rename = "PolicyName")]
    pub policy_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct CreatePolicyResponse {
    #[serde(rename = "PolicyArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_arn: Option<String>,
    #[serde(rename = "PolicyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DescribePolicyRequest {
    #[serde(rename = "PolicyName")]
    pub policy_name: Option<String>,
    #[serde(rename = "PolicyArn")]
    pub policy_arn: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
#[allow(dead_code)]
pub struct PolicyDetail {
    #[serde(rename = "PolicyName")]
    pub policy_name: String,
    #[serde(rename = "PolicyArn")]
    pub policy_arn: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct DescribePolicyResponse {
    #[serde(rename = "Policy")]
    pub policy: PolicyDetail,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct ListPolicysRequest {
    #[serde(rename = "MaxResults")]
    pub max_results: Option<usize>,
    #[serde(rename = "NextToken")]
    pub next_token: Option<String>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct ListPolicysResponse {
    #[serde(rename = "Policys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policys: Option<Vec<PolicyDetail>>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DeletePolicyRequest {
    #[serde(rename = "PolicyName")]
    pub policy_name: Option<String>,
    #[serde(rename = "PolicyArn")]
    pub policy_arn: Option<String>,
}

}
pub use _types::*;
