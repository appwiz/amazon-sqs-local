use serde::{Deserialize, Serialize};

// --- Endpoint ---

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Endpoint {
    pub address: String,
    pub port: i32,
}

// --- SecurityGroupMembership ---

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SecurityGroupMembership {
    pub security_group_id: String,
    pub status: String,
}

// --- Node ---

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Node {
    pub name: String,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<Endpoint>,
}

// --- Shard ---

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Shard {
    pub name: String,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slots: Option<String>,
    pub number_of_nodes: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<Node>>,
}

// --- Cluster ---

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Cluster {
    pub name: String,
    #[serde(rename = "ARN")]
    pub arn: String,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub node_type: String,
    pub engine: String,
    pub engine_version: String,
    pub number_of_shards: i32,
    #[serde(rename = "ACLName")]
    pub acl_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_group_name: Option<String>,
    #[serde(rename = "TLSEnabled")]
    pub tls_enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic_arn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_window: Option<String>,
    pub parameter_group_name: String,
    pub parameter_group_status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<SecurityGroupMembership>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shards: Option<Vec<Shard>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_endpoint: Option<Endpoint>,
    pub auto_minor_version_upgrade: bool,
    pub snapshot_retention_limit: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_window: Option<String>,
}

// --- Subnet ---

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Subnet {
    pub identifier: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<AvailabilityZone>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AvailabilityZone {
    pub name: String,
}

// --- SubnetGroup ---

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SubnetGroup {
    pub name: String,
    #[serde(rename = "ARN")]
    pub arn: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "VpcId")]
    pub vpc_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnets: Option<Vec<Subnet>>,
}

// --- Authentication ---

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Authentication {
    #[serde(rename = "Type")]
    pub auth_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_count: Option<i32>,
}

// --- User ---

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct User {
    pub name: String,
    #[serde(rename = "ARN")]
    pub arn: String,
    pub status: String,
    pub access_string: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication: Option<Authentication>,
    #[serde(rename = "ACLNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_names: Option<Vec<String>>,
    pub minimum_engine_version: String,
}

// --- ACL ---

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Acl {
    pub name: String,
    #[serde(rename = "ARN")]
    pub arn: String,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_names: Option<Vec<String>>,
    pub minimum_engine_version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clusters: Option<Vec<String>>,
}

// --- Snapshot ---

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Snapshot {
    pub name: String,
    #[serde(rename = "ARN")]
    pub arn: String,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_configuration: Option<ClusterConfiguration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ClusterConfiguration {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub node_type: String,
    pub engine_version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_window: Option<String>,
    pub snapshot_retention_limit: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_group_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
    pub number_of_shards: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shards: Option<Vec<ShardDetail>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ShardDetail {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    pub snapshot_creation_time: String,
}

// --- Tag ---

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Tag {
    pub key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
