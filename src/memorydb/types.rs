use serde::{Deserialize, Serialize};

use super::cluster::*;

// --- CreateCluster ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateClusterRequest {
    pub cluster_name: String,
    pub node_type: String,
    #[serde(rename = "ACLName")]
    pub acl_name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub subnet_group_name: Option<String>,
    #[serde(default)]
    pub engine: Option<String>,
    #[serde(default)]
    pub engine_version: Option<String>,
    #[serde(default)]
    pub num_shards: Option<i32>,
    #[serde(default)]
    pub num_replicas_per_shard: Option<i32>,
    #[serde(rename = "TLSEnabled")]
    #[serde(default)]
    pub tls_enabled: Option<bool>,
    #[serde(default)]
    pub kms_key_id: Option<String>,
    #[serde(default)]
    pub sns_topic_arn: Option<String>,
    #[serde(default)]
    pub maintenance_window: Option<String>,
    #[serde(default)]
    pub parameter_group_name: Option<String>,
    #[serde(default)]
    pub snapshot_retention_limit: Option<i32>,
    #[serde(default)]
    pub snapshot_window: Option<String>,
    #[serde(default)]
    pub security_group_ids: Option<Vec<String>>,
    #[serde(default)]
    pub tags: Option<Vec<Tag>>,
    #[serde(default)]
    pub auto_minor_version_upgrade: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateClusterResponse {
    pub cluster: Cluster,
}

// --- DeleteCluster ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteClusterRequest {
    pub cluster_name: String,
    #[serde(default)]
    pub final_snapshot_name: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteClusterResponse {
    pub cluster: Cluster,
}

// --- DescribeClusters ---

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct DescribeClustersRequest {
    #[serde(default)]
    pub cluster_name: Option<String>,
    #[serde(default)]
    pub max_results: Option<i32>,
    #[serde(default)]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct DescribeClustersResponse {
    pub clusters: Vec<Cluster>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

// --- UpdateCluster ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UpdateClusterRequest {
    pub cluster_name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub node_type: Option<String>,
    #[serde(default)]
    pub engine_version: Option<String>,
    #[serde(rename = "ACLName")]
    #[serde(default)]
    pub acl_name: Option<String>,
    #[serde(default)]
    pub security_group_ids: Option<Vec<String>>,
    #[serde(default)]
    pub maintenance_window: Option<String>,
    #[serde(default)]
    pub sns_topic_arn: Option<String>,
    #[serde(default)]
    pub parameter_group_name: Option<String>,
    #[serde(default)]
    pub snapshot_retention_limit: Option<i32>,
    #[serde(default)]
    pub snapshot_window: Option<String>,
    #[serde(default)]
    pub num_shards: Option<i32>,
    #[serde(default)]
    pub num_replicas_per_shard: Option<i32>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct UpdateClusterResponse {
    pub cluster: Cluster,
}

// --- CreateSubnetGroup ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateSubnetGroupRequest {
    pub subnet_group_name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub subnet_ids: Option<Vec<String>>,
    #[serde(default)]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateSubnetGroupResponse {
    pub subnet_group: SubnetGroup,
}

// --- DeleteSubnetGroup ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteSubnetGroupRequest {
    pub subnet_group_name: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteSubnetGroupResponse {
    pub subnet_group: SubnetGroup,
}

// --- DescribeSubnetGroups ---

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct DescribeSubnetGroupsRequest {
    #[serde(default)]
    pub subnet_group_name: Option<String>,
    #[serde(default)]
    pub max_results: Option<i32>,
    #[serde(default)]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct DescribeSubnetGroupsResponse {
    pub subnet_groups: Vec<SubnetGroup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

// --- CreateUser ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AuthenticationMode {
    #[serde(rename = "Type")]
    pub auth_type: String,
    #[serde(default)]
    pub passwords: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateUserRequest {
    pub user_name: String,
    pub access_string: String,
    pub authentication_mode: AuthenticationMode,
    #[serde(default)]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateUserResponse {
    pub user: User,
}

// --- DeleteUser ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteUserRequest {
    pub user_name: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteUserResponse {
    pub user: User,
}

// --- DescribeUsers ---

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct DescribeUsersRequest {
    #[serde(default)]
    pub user_name: Option<String>,
    #[serde(default)]
    pub max_results: Option<i32>,
    #[serde(default)]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct DescribeUsersResponse {
    pub users: Vec<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

// --- UpdateUser ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UpdateUserRequest {
    pub user_name: String,
    #[serde(default)]
    pub access_string: Option<String>,
    #[serde(default)]
    pub authentication_mode: Option<AuthenticationMode>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct UpdateUserResponse {
    pub user: User,
}

// --- CreateACL ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateAclRequest {
    #[serde(rename = "ACLName")]
    pub acl_name: String,
    #[serde(default)]
    pub user_names: Option<Vec<String>>,
    #[serde(default)]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateAclResponse {
    #[serde(rename = "ACL")]
    pub acl: Acl,
}

// --- DeleteACL ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteAclRequest {
    #[serde(rename = "ACLName")]
    pub acl_name: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteAclResponse {
    #[serde(rename = "ACL")]
    pub acl: Acl,
}

// --- DescribeACLs ---

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct DescribeAclsRequest {
    #[serde(rename = "ACLName")]
    #[serde(default)]
    pub acl_name: Option<String>,
    #[serde(default)]
    pub max_results: Option<i32>,
    #[serde(default)]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct DescribeAclsResponse {
    #[serde(rename = "ACLs")]
    pub acls: Vec<Acl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

// --- UpdateACL ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UpdateAclRequest {
    #[serde(rename = "ACLName")]
    pub acl_name: String,
    #[serde(default)]
    pub user_names_to_add: Option<Vec<String>>,
    #[serde(default)]
    pub user_names_to_remove: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct UpdateAclResponse {
    #[serde(rename = "ACL")]
    pub acl: Acl,
}

// --- CreateSnapshot ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateSnapshotRequest {
    pub cluster_name: String,
    pub snapshot_name: String,
    #[serde(default)]
    pub kms_key_id: Option<String>,
    #[serde(default)]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateSnapshotResponse {
    pub snapshot: Snapshot,
}

// --- DeleteSnapshot ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteSnapshotRequest {
    pub snapshot_name: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteSnapshotResponse {
    pub snapshot: Snapshot,
}

// --- DescribeSnapshots ---

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct DescribeSnapshotsRequest {
    #[serde(default)]
    pub cluster_name: Option<String>,
    #[serde(default)]
    pub snapshot_name: Option<String>,
    #[serde(default)]
    pub max_results: Option<i32>,
    #[serde(default)]
    pub next_token: Option<String>,
    #[serde(default)]
    pub source: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct DescribeSnapshotsResponse {
    pub snapshots: Vec<Snapshot>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

// --- TagResource ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TagResourceRequest {
    pub resource_arn: String,
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct TagResourceResponse {
    pub tag_list: Vec<Tag>,
}

// --- UntagResource ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UntagResourceRequest {
    pub resource_arn: String,
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct UntagResourceResponse {
    pub tag_list: Vec<Tag>,
}

// --- ListTags ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListTagsRequest {
    pub resource_arn: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListTagsResponse {
    pub tag_list: Vec<Tag>,
}
