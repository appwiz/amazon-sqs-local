mod _types {
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct CreateProgressUpdateStreamRequest {
    #[serde(rename = "ProgressUpdateStreamName")]
    pub progress_update_stream_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct CreateProgressUpdateStreamResponse {
    #[serde(rename = "ProgressUpdateStreamArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_update_stream_arn: Option<String>,
    #[serde(rename = "ProgressUpdateStreamName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_update_stream_name: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DescribeProgressUpdateStreamRequest {
    #[serde(rename = "ProgressUpdateStreamName")]
    pub progress_update_stream_name: Option<String>,
    #[serde(rename = "ProgressUpdateStreamArn")]
    pub progress_update_stream_arn: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
#[allow(dead_code)]
pub struct ProgressUpdateStreamDetail {
    #[serde(rename = "ProgressUpdateStreamName")]
    pub progress_update_stream_name: String,
    #[serde(rename = "ProgressUpdateStreamArn")]
    pub progress_update_stream_arn: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct DescribeProgressUpdateStreamResponse {
    #[serde(rename = "ProgressUpdateStream")]
    pub progress_update_stream: ProgressUpdateStreamDetail,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct ListProgressUpdateStreamsRequest {
    #[serde(rename = "MaxResults")]
    pub max_results: Option<usize>,
    #[serde(rename = "NextToken")]
    pub next_token: Option<String>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct ListProgressUpdateStreamsResponse {
    #[serde(rename = "ProgressUpdateStreams")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_update_streams: Option<Vec<ProgressUpdateStreamDetail>>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DeleteProgressUpdateStreamRequest {
    #[serde(rename = "ProgressUpdateStreamName")]
    pub progress_update_stream_name: Option<String>,
    #[serde(rename = "ProgressUpdateStreamArn")]
    pub progress_update_stream_arn: Option<String>,
}

}
pub use _types::*;
