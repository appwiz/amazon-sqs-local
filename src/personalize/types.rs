mod _types {
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct CreateDatasetRequest {
    #[serde(rename = "DatasetName")]
    pub dataset_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct CreateDatasetResponse {
    #[serde(rename = "DatasetArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_arn: Option<String>,
    #[serde(rename = "DatasetName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_name: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DescribeDatasetRequest {
    #[serde(rename = "DatasetName")]
    pub dataset_name: Option<String>,
    #[serde(rename = "DatasetArn")]
    pub dataset_arn: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
#[allow(dead_code)]
pub struct DatasetDetail {
    #[serde(rename = "DatasetName")]
    pub dataset_name: String,
    #[serde(rename = "DatasetArn")]
    pub dataset_arn: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct DescribeDatasetResponse {
    #[serde(rename = "Dataset")]
    pub dataset: DatasetDetail,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct ListDatasetsRequest {
    #[serde(rename = "MaxResults")]
    pub max_results: Option<usize>,
    #[serde(rename = "NextToken")]
    pub next_token: Option<String>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct ListDatasetsResponse {
    #[serde(rename = "Datasets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datasets: Option<Vec<DatasetDetail>>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DeleteDatasetRequest {
    #[serde(rename = "DatasetName")]
    pub dataset_name: Option<String>,
    #[serde(rename = "DatasetArn")]
    pub dataset_arn: Option<String>,
}

}
pub use _types::*;
