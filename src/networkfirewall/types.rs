mod _types {
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct CreateFirewallRequest {
    #[serde(rename = "FirewallName")]
    pub firewall_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct CreateFirewallResponse {
    #[serde(rename = "FirewallArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_arn: Option<String>,
    #[serde(rename = "FirewallName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_name: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DescribeFirewallRequest {
    #[serde(rename = "FirewallName")]
    pub firewall_name: Option<String>,
    #[serde(rename = "FirewallArn")]
    pub firewall_arn: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
#[allow(dead_code)]
pub struct FirewallDetail {
    #[serde(rename = "FirewallName")]
    pub firewall_name: String,
    #[serde(rename = "FirewallArn")]
    pub firewall_arn: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct DescribeFirewallResponse {
    #[serde(rename = "Firewall")]
    pub firewall: FirewallDetail,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct ListFirewallsRequest {
    #[serde(rename = "MaxResults")]
    pub max_results: Option<usize>,
    #[serde(rename = "NextToken")]
    pub next_token: Option<String>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct ListFirewallsResponse {
    #[serde(rename = "Firewalls")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewalls: Option<Vec<FirewallDetail>>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DeleteFirewallRequest {
    #[serde(rename = "FirewallName")]
    pub firewall_name: Option<String>,
    #[serde(rename = "FirewallArn")]
    pub firewall_arn: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct CreateFirewallPolicyRequest {
    #[serde(rename = "FirewallPolicyName")]
    pub firewall_policy_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct CreateFirewallPolicyResponse {
    #[serde(rename = "FirewallPolicyArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_policy_arn: Option<String>,
    #[serde(rename = "FirewallPolicyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_policy_name: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DescribeFirewallPolicyRequest {
    #[serde(rename = "FirewallPolicyName")]
    pub firewall_policy_name: Option<String>,
    #[serde(rename = "FirewallPolicyArn")]
    pub firewall_policy_arn: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
#[allow(dead_code)]
pub struct FirewallPolicyDetail {
    #[serde(rename = "FirewallPolicyName")]
    pub firewall_policy_name: String,
    #[serde(rename = "FirewallPolicyArn")]
    pub firewall_policy_arn: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct DescribeFirewallPolicyResponse {
    #[serde(rename = "FirewallPolicy")]
    pub firewall_policy: FirewallPolicyDetail,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct ListFirewallPolicysRequest {
    #[serde(rename = "MaxResults")]
    pub max_results: Option<usize>,
    #[serde(rename = "NextToken")]
    pub next_token: Option<String>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct ListFirewallPolicysResponse {
    #[serde(rename = "FirewallPolicys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_policys: Option<Vec<FirewallPolicyDetail>>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DeleteFirewallPolicyRequest {
    #[serde(rename = "FirewallPolicyName")]
    pub firewall_policy_name: Option<String>,
    #[serde(rename = "FirewallPolicyArn")]
    pub firewall_policy_arn: Option<String>,
}

}
pub use _types::*;
