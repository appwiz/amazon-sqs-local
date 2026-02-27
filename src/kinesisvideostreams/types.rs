mod _types {
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct CreateStreamRequest {
    #[serde(rename = "StreamName")]
    pub stream_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct CreateStreamResponse {
    #[serde(rename = "StreamArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_arn: Option<String>,
    #[serde(rename = "StreamName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DescribeStreamRequest {
    #[serde(rename = "StreamName")]
    pub stream_name: Option<String>,
    #[serde(rename = "StreamArn")]
    pub stream_arn: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
#[allow(dead_code)]
pub struct StreamDetail {
    #[serde(rename = "StreamName")]
    pub stream_name: String,
    #[serde(rename = "StreamArn")]
    pub stream_arn: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct DescribeStreamResponse {
    #[serde(rename = "Stream")]
    pub stream: StreamDetail,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct ListStreamsRequest {
    #[serde(rename = "MaxResults")]
    pub max_results: Option<usize>,
    #[serde(rename = "NextToken")]
    pub next_token: Option<String>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct ListStreamsResponse {
    #[serde(rename = "Streams")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub streams: Option<Vec<StreamDetail>>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DeleteStreamRequest {
    #[serde(rename = "StreamName")]
    pub stream_name: Option<String>,
    #[serde(rename = "StreamArn")]
    pub stream_arn: Option<String>,
}

}
pub use _types::*;
