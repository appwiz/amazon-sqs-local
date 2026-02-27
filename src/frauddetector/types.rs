mod _types {
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct CreateDetectorRequest {
    #[serde(rename = "DetectorName")]
    pub detector_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct CreateDetectorResponse {
    #[serde(rename = "DetectorArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detector_arn: Option<String>,
    #[serde(rename = "DetectorName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detector_name: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DescribeDetectorRequest {
    #[serde(rename = "DetectorName")]
    pub detector_name: Option<String>,
    #[serde(rename = "DetectorArn")]
    pub detector_arn: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
#[allow(dead_code)]
pub struct DetectorDetail {
    #[serde(rename = "DetectorName")]
    pub detector_name: String,
    #[serde(rename = "DetectorArn")]
    pub detector_arn: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct DescribeDetectorResponse {
    #[serde(rename = "Detector")]
    pub detector: DetectorDetail,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct ListDetectorsRequest {
    #[serde(rename = "MaxResults")]
    pub max_results: Option<usize>,
    #[serde(rename = "NextToken")]
    pub next_token: Option<String>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct ListDetectorsResponse {
    #[serde(rename = "Detectors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detectors: Option<Vec<DetectorDetail>>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DeleteDetectorRequest {
    #[serde(rename = "DetectorName")]
    pub detector_name: Option<String>,
    #[serde(rename = "DetectorArn")]
    pub detector_arn: Option<String>,
}

}
pub use _types::*;
