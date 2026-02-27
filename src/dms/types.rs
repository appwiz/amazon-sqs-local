mod _types {
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct CreateReplicationInstanceRequest {
    #[serde(rename = "ReplicationInstanceName")]
    pub replication_instance_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct CreateReplicationInstanceResponse {
    #[serde(rename = "ReplicationInstanceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_instance_arn: Option<String>,
    #[serde(rename = "ReplicationInstanceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_instance_name: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DescribeReplicationInstanceRequest {
    #[serde(rename = "ReplicationInstanceName")]
    pub replication_instance_name: Option<String>,
    #[serde(rename = "ReplicationInstanceArn")]
    pub replication_instance_arn: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
#[allow(dead_code)]
pub struct ReplicationInstanceDetail {
    #[serde(rename = "ReplicationInstanceName")]
    pub replication_instance_name: String,
    #[serde(rename = "ReplicationInstanceArn")]
    pub replication_instance_arn: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct DescribeReplicationInstanceResponse {
    #[serde(rename = "ReplicationInstance")]
    pub replication_instance: ReplicationInstanceDetail,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct ListReplicationInstancesRequest {
    #[serde(rename = "MaxResults")]
    pub max_results: Option<usize>,
    #[serde(rename = "NextToken")]
    pub next_token: Option<String>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct ListReplicationInstancesResponse {
    #[serde(rename = "ReplicationInstances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_instances: Option<Vec<ReplicationInstanceDetail>>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DeleteReplicationInstanceRequest {
    #[serde(rename = "ReplicationInstanceName")]
    pub replication_instance_name: Option<String>,
    #[serde(rename = "ReplicationInstanceArn")]
    pub replication_instance_arn: Option<String>,
}

}
pub use _types::*;
