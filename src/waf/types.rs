mod _types {
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct CreateWebACLRequest {
    #[serde(rename = "WebACLName")]
    pub web_a_c_l_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct CreateWebACLResponse {
    #[serde(rename = "WebACLArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_a_c_l_arn: Option<String>,
    #[serde(rename = "WebACLName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_a_c_l_name: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DescribeWebACLRequest {
    #[serde(rename = "WebACLName")]
    pub web_a_c_l_name: Option<String>,
    #[serde(rename = "WebACLArn")]
    pub web_a_c_l_arn: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
#[allow(dead_code)]
pub struct WebACLDetail {
    #[serde(rename = "WebACLName")]
    pub web_a_c_l_name: String,
    #[serde(rename = "WebACLArn")]
    pub web_a_c_l_arn: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct DescribeWebACLResponse {
    #[serde(rename = "WebACL")]
    pub web_a_c_l: WebACLDetail,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct ListWebACLsRequest {
    #[serde(rename = "MaxResults")]
    pub max_results: Option<usize>,
    #[serde(rename = "NextToken")]
    pub next_token: Option<String>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct ListWebACLsResponse {
    #[serde(rename = "WebACLs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_a_c_ls: Option<Vec<WebACLDetail>>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DeleteWebACLRequest {
    #[serde(rename = "WebACLName")]
    pub web_a_c_l_name: Option<String>,
    #[serde(rename = "WebACLArn")]
    pub web_a_c_l_arn: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct CreateIPSetRequest {
    #[serde(rename = "IPSetName")]
    pub i_p_set_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct CreateIPSetResponse {
    #[serde(rename = "IPSetArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_p_set_arn: Option<String>,
    #[serde(rename = "IPSetName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_p_set_name: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DescribeIPSetRequest {
    #[serde(rename = "IPSetName")]
    pub i_p_set_name: Option<String>,
    #[serde(rename = "IPSetArn")]
    pub i_p_set_arn: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
#[allow(dead_code)]
pub struct IPSetDetail {
    #[serde(rename = "IPSetName")]
    pub i_p_set_name: String,
    #[serde(rename = "IPSetArn")]
    pub i_p_set_arn: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct DescribeIPSetResponse {
    #[serde(rename = "IPSet")]
    pub i_p_set: IPSetDetail,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct ListIPSetsRequest {
    #[serde(rename = "MaxResults")]
    pub max_results: Option<usize>,
    #[serde(rename = "NextToken")]
    pub next_token: Option<String>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct ListIPSetsResponse {
    #[serde(rename = "IPSets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_p_sets: Option<Vec<IPSetDetail>>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DeleteIPSetRequest {
    #[serde(rename = "IPSetName")]
    pub i_p_set_name: Option<String>,
    #[serde(rename = "IPSetArn")]
    pub i_p_set_arn: Option<String>,
}

}
pub use _types::*;
