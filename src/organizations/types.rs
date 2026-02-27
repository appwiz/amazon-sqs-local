mod _types {
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct CreateOrganizationRequest {
    #[serde(rename = "OrganizationName")]
    pub organization_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct CreateOrganizationResponse {
    #[serde(rename = "OrganizationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_arn: Option<String>,
    #[serde(rename = "OrganizationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_name: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DescribeOrganizationRequest {
    #[serde(rename = "OrganizationName")]
    pub organization_name: Option<String>,
    #[serde(rename = "OrganizationArn")]
    pub organization_arn: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
#[allow(dead_code)]
pub struct OrganizationDetail {
    #[serde(rename = "OrganizationName")]
    pub organization_name: String,
    #[serde(rename = "OrganizationArn")]
    pub organization_arn: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct DescribeOrganizationResponse {
    #[serde(rename = "Organization")]
    pub organization: OrganizationDetail,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct ListOrganizationsRequest {
    #[serde(rename = "MaxResults")]
    pub max_results: Option<usize>,
    #[serde(rename = "NextToken")]
    pub next_token: Option<String>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct ListOrganizationsResponse {
    #[serde(rename = "Organizations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organizations: Option<Vec<OrganizationDetail>>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DeleteOrganizationRequest {
    #[serde(rename = "OrganizationName")]
    pub organization_name: Option<String>,
    #[serde(rename = "OrganizationArn")]
    pub organization_arn: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct CreateAccountRequest {
    #[serde(rename = "AccountName")]
    pub account_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct CreateAccountResponse {
    #[serde(rename = "AccountArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_arn: Option<String>,
    #[serde(rename = "AccountName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DescribeAccountRequest {
    #[serde(rename = "AccountName")]
    pub account_name: Option<String>,
    #[serde(rename = "AccountArn")]
    pub account_arn: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
#[allow(dead_code)]
pub struct AccountDetail {
    #[serde(rename = "AccountName")]
    pub account_name: String,
    #[serde(rename = "AccountArn")]
    pub account_arn: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct DescribeAccountResponse {
    #[serde(rename = "Account")]
    pub account: AccountDetail,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct ListAccountsRequest {
    #[serde(rename = "MaxResults")]
    pub max_results: Option<usize>,
    #[serde(rename = "NextToken")]
    pub next_token: Option<String>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct ListAccountsResponse {
    #[serde(rename = "Accounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounts: Option<Vec<AccountDetail>>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DeleteAccountRequest {
    #[serde(rename = "AccountName")]
    pub account_name: Option<String>,
    #[serde(rename = "AccountArn")]
    pub account_arn: Option<String>,
}

}
pub use _types::*;
