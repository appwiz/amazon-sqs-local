mod _types {
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct CreateCertificateRequest {
    #[serde(rename = "CertificateName")]
    pub certificate_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct CreateCertificateResponse {
    #[serde(rename = "CertificateArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    #[serde(rename = "CertificateName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_name: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DescribeCertificateRequest {
    #[serde(rename = "CertificateName")]
    pub certificate_name: Option<String>,
    #[serde(rename = "CertificateArn")]
    pub certificate_arn: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
#[allow(dead_code)]
pub struct CertificateDetail {
    #[serde(rename = "CertificateName")]
    pub certificate_name: String,
    #[serde(rename = "CertificateArn")]
    pub certificate_arn: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct DescribeCertificateResponse {
    #[serde(rename = "Certificate")]
    pub certificate: CertificateDetail,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct ListCertificatesRequest {
    #[serde(rename = "MaxResults")]
    pub max_results: Option<usize>,
    #[serde(rename = "NextToken")]
    pub next_token: Option<String>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct ListCertificatesResponse {
    #[serde(rename = "Certificates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificates: Option<Vec<CertificateDetail>>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DeleteCertificateRequest {
    #[serde(rename = "CertificateName")]
    pub certificate_name: Option<String>,
    #[serde(rename = "CertificateArn")]
    pub certificate_arn: Option<String>,
}

}
pub use _types::*;
