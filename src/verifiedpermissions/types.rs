mod _types {
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct CreatePolicyStoreRequest {
    #[serde(rename = "PolicyStoreName")]
    pub policy_store_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct CreatePolicyStoreResponse {
    #[serde(rename = "PolicyStoreArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_store_arn: Option<String>,
    #[serde(rename = "PolicyStoreName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_store_name: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DescribePolicyStoreRequest {
    #[serde(rename = "PolicyStoreName")]
    pub policy_store_name: Option<String>,
    #[serde(rename = "PolicyStoreArn")]
    pub policy_store_arn: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
#[allow(dead_code)]
pub struct PolicyStoreDetail {
    #[serde(rename = "PolicyStoreName")]
    pub policy_store_name: String,
    #[serde(rename = "PolicyStoreArn")]
    pub policy_store_arn: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct DescribePolicyStoreResponse {
    #[serde(rename = "PolicyStore")]
    pub policy_store: PolicyStoreDetail,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct ListPolicyStoresRequest {
    #[serde(rename = "MaxResults")]
    pub max_results: Option<usize>,
    #[serde(rename = "NextToken")]
    pub next_token: Option<String>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct ListPolicyStoresResponse {
    #[serde(rename = "PolicyStores")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_stores: Option<Vec<PolicyStoreDetail>>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DeletePolicyStoreRequest {
    #[serde(rename = "PolicyStoreName")]
    pub policy_store_name: Option<String>,
    #[serde(rename = "PolicyStoreArn")]
    pub policy_store_arn: Option<String>,
}

}
pub use _types::*;
