mod _types {
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct CreateDocumentClassifierRequest {
    #[serde(rename = "DocumentClassifierName")]
    pub document_classifier_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct CreateDocumentClassifierResponse {
    #[serde(rename = "DocumentClassifierArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_classifier_arn: Option<String>,
    #[serde(rename = "DocumentClassifierName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_classifier_name: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DescribeDocumentClassifierRequest {
    #[serde(rename = "DocumentClassifierName")]
    pub document_classifier_name: Option<String>,
    #[serde(rename = "DocumentClassifierArn")]
    pub document_classifier_arn: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
#[allow(dead_code)]
pub struct DocumentClassifierDetail {
    #[serde(rename = "DocumentClassifierName")]
    pub document_classifier_name: String,
    #[serde(rename = "DocumentClassifierArn")]
    pub document_classifier_arn: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct DescribeDocumentClassifierResponse {
    #[serde(rename = "DocumentClassifier")]
    pub document_classifier: DocumentClassifierDetail,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct ListDocumentClassifiersRequest {
    #[serde(rename = "MaxResults")]
    pub max_results: Option<usize>,
    #[serde(rename = "NextToken")]
    pub next_token: Option<String>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct ListDocumentClassifiersResponse {
    #[serde(rename = "DocumentClassifiers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_classifiers: Option<Vec<DocumentClassifierDetail>>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DeleteDocumentClassifierRequest {
    #[serde(rename = "DocumentClassifierName")]
    pub document_classifier_name: Option<String>,
    #[serde(rename = "DocumentClassifierArn")]
    pub document_classifier_arn: Option<String>,
}

}
pub use _types::*;
