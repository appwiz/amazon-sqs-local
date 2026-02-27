mod _types {
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct CreateInstanceRequest {
    #[serde(rename = "InstanceName")]
    pub instance_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct CreateInstanceResponse {
    #[serde(rename = "InstanceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_arn: Option<String>,
    #[serde(rename = "InstanceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DescribeInstanceRequest {
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,
    #[serde(rename = "InstanceArn")]
    pub instance_arn: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
#[allow(dead_code)]
pub struct InstanceDetail {
    #[serde(rename = "InstanceName")]
    pub instance_name: String,
    #[serde(rename = "InstanceArn")]
    pub instance_arn: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct DescribeInstanceResponse {
    #[serde(rename = "Instance")]
    pub instance: InstanceDetail,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct ListInstancesRequest {
    #[serde(rename = "MaxResults")]
    pub max_results: Option<usize>,
    #[serde(rename = "NextToken")]
    pub next_token: Option<String>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct ListInstancesResponse {
    #[serde(rename = "Instances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances: Option<Vec<InstanceDetail>>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DeleteInstanceRequest {
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,
    #[serde(rename = "InstanceArn")]
    pub instance_arn: Option<String>,
}

}
pub use _types::*;
