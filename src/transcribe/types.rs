mod _types {
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct CreateTranscriptionJobRequest {
    #[serde(rename = "TranscriptionJobName")]
    pub transcription_job_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct CreateTranscriptionJobResponse {
    #[serde(rename = "TranscriptionJobArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcription_job_arn: Option<String>,
    #[serde(rename = "TranscriptionJobName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcription_job_name: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DescribeTranscriptionJobRequest {
    #[serde(rename = "TranscriptionJobName")]
    pub transcription_job_name: Option<String>,
    #[serde(rename = "TranscriptionJobArn")]
    pub transcription_job_arn: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
#[allow(dead_code)]
pub struct TranscriptionJobDetail {
    #[serde(rename = "TranscriptionJobName")]
    pub transcription_job_name: String,
    #[serde(rename = "TranscriptionJobArn")]
    pub transcription_job_arn: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct DescribeTranscriptionJobResponse {
    #[serde(rename = "TranscriptionJob")]
    pub transcription_job: TranscriptionJobDetail,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct ListTranscriptionJobsRequest {
    #[serde(rename = "MaxResults")]
    pub max_results: Option<usize>,
    #[serde(rename = "NextToken")]
    pub next_token: Option<String>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct ListTranscriptionJobsResponse {
    #[serde(rename = "TranscriptionJobs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcription_jobs: Option<Vec<TranscriptionJobDetail>>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DeleteTranscriptionJobRequest {
    #[serde(rename = "TranscriptionJobName")]
    pub transcription_job_name: Option<String>,
    #[serde(rename = "TranscriptionJobArn")]
    pub transcription_job_arn: Option<String>,
}

}
pub use _types::*;
