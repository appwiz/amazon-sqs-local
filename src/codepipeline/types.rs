mod _types {
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct CreatePipelineRequest {
    #[serde(rename = "PipelineName")]
    pub pipeline_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct CreatePipelineResponse {
    #[serde(rename = "PipelineArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_arn: Option<String>,
    #[serde(rename = "PipelineName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_name: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DescribePipelineRequest {
    #[serde(rename = "PipelineName")]
    pub pipeline_name: Option<String>,
    #[serde(rename = "PipelineArn")]
    pub pipeline_arn: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
#[allow(dead_code)]
pub struct PipelineDetail {
    #[serde(rename = "PipelineName")]
    pub pipeline_name: String,
    #[serde(rename = "PipelineArn")]
    pub pipeline_arn: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct DescribePipelineResponse {
    #[serde(rename = "Pipeline")]
    pub pipeline: PipelineDetail,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct ListPipelinesRequest {
    #[serde(rename = "MaxResults")]
    pub max_results: Option<usize>,
    #[serde(rename = "NextToken")]
    pub next_token: Option<String>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct ListPipelinesResponse {
    #[serde(rename = "Pipelines")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipelines: Option<Vec<PipelineDetail>>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DeletePipelineRequest {
    #[serde(rename = "PipelineName")]
    pub pipeline_name: Option<String>,
    #[serde(rename = "PipelineArn")]
    pub pipeline_arn: Option<String>,
}

}
pub use _types::*;
