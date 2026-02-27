mod _types {
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct CreateDomainRequest {
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct CreateDomainResponse {
    #[serde(rename = "DomainArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_arn: Option<String>,
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DescribeDomainRequest {
    #[serde(rename = "DomainName")]
    pub domain_name: Option<String>,
    #[serde(rename = "DomainArn")]
    pub domain_arn: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
#[allow(dead_code)]
pub struct DomainDetail {
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    #[serde(rename = "DomainArn")]
    pub domain_arn: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct DescribeDomainResponse {
    #[serde(rename = "Domain")]
    pub domain: DomainDetail,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct ListDomainsRequest {
    #[serde(rename = "MaxResults")]
    pub max_results: Option<usize>,
    #[serde(rename = "NextToken")]
    pub next_token: Option<String>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct ListDomainsResponse {
    #[serde(rename = "Domains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domains: Option<Vec<DomainDetail>>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DeleteDomainRequest {
    #[serde(rename = "DomainName")]
    pub domain_name: Option<String>,
    #[serde(rename = "DomainArn")]
    pub domain_arn: Option<String>,
}

}
pub use _types::*;
