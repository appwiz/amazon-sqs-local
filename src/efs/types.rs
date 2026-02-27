use serde::{Deserialize, Serialize};

// --- Tag ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,
}

// --- SizeInBytes ---

#[derive(Debug, Clone, Serialize)]
pub struct SizeInBytes {
    #[serde(rename = "Value")]
    pub value: i64,
    #[serde(rename = "Timestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<f64>,
    #[serde(rename = "ValueInIA")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_in_ia: Option<i64>,
    #[serde(rename = "ValueInStandard")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_in_standard: Option<i64>,
}

// --- LifecyclePolicy ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifecyclePolicy {
    #[serde(rename = "TransitionToIA")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transition_to_ia: Option<String>,
    #[serde(rename = "TransitionToPrimaryStorageClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transition_to_primary_storage_class: Option<String>,
    #[serde(rename = "TransitionToArchive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transition_to_archive: Option<String>,
}

// --- FileSystem ---

#[derive(Debug, Clone, Serialize)]
pub struct FileSystemDescription {
    #[serde(rename = "OwnerId")]
    pub owner_id: String,
    #[serde(rename = "CreationToken")]
    pub creation_token: String,
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    #[serde(rename = "FileSystemArn")]
    pub file_system_arn: String,
    #[serde(rename = "CreationTime")]
    pub creation_time: f64,
    #[serde(rename = "LifeCycleState")]
    pub life_cycle_state: String,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "NumberOfMountTargets")]
    pub number_of_mount_targets: i32,
    #[serde(rename = "SizeInBytes")]
    pub size_in_bytes: SizeInBytes,
    #[serde(rename = "PerformanceMode")]
    pub performance_mode: String,
    #[serde(rename = "Encrypted")]
    pub encrypted: bool,
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "ThroughputMode")]
    pub throughput_mode: String,
    #[serde(rename = "ProvisionedThroughputInMibps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_throughput_in_mibps: Option<f64>,
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

// --- MountTarget ---

#[derive(Debug, Clone, Serialize)]
pub struct MountTargetDescription {
    #[serde(rename = "OwnerId")]
    pub owner_id: String,
    #[serde(rename = "MountTargetId")]
    pub mount_target_id: String,
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    #[serde(rename = "SubnetId")]
    pub subnet_id: String,
    #[serde(rename = "LifeCycleState")]
    pub life_cycle_state: String,
    #[serde(rename = "IpAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(rename = "NetworkInterfaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interface_id: Option<String>,
    #[serde(rename = "AvailabilityZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone_id: Option<String>,
    #[serde(rename = "AvailabilityZoneName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone_name: Option<String>,
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

// --- PosixUser ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PosixUser {
    #[serde(rename = "Uid")]
    pub uid: i64,
    #[serde(rename = "Gid")]
    pub gid: i64,
    #[serde(rename = "SecondaryGids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_gids: Option<Vec<i64>>,
}

// --- RootDirectory ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RootDirectory {
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "CreationInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_info: Option<CreationInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreationInfo {
    #[serde(rename = "OwnerUid")]
    pub owner_uid: i64,
    #[serde(rename = "OwnerGid")]
    pub owner_gid: i64,
    #[serde(rename = "Permissions")]
    pub permissions: String,
}

// --- AccessPoint ---

#[derive(Debug, Clone, Serialize)]
pub struct AccessPointDescription {
    #[serde(rename = "ClientToken")]
    pub client_token: String,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
    #[serde(rename = "AccessPointId")]
    pub access_point_id: String,
    #[serde(rename = "AccessPointArn")]
    pub access_point_arn: String,
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    #[serde(rename = "PosixUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub posix_user: Option<PosixUser>,
    #[serde(rename = "RootDirectory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_directory: Option<RootDirectory>,
    #[serde(rename = "OwnerId")]
    pub owner_id: String,
    #[serde(rename = "LifeCycleState")]
    pub life_cycle_state: String,
}

// --- Request types ---

#[derive(Debug, Deserialize, Clone, Default)]
pub struct CreateFileSystemRequest {
    #[serde(rename = "CreationToken")]
    pub creation_token: String,
    #[serde(rename = "PerformanceMode")]
    pub performance_mode: Option<String>,
    #[serde(rename = "Encrypted")]
    pub encrypted: Option<bool>,
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "ThroughputMode")]
    pub throughput_mode: Option<String>,
    #[serde(rename = "ProvisionedThroughputInMibps")]
    pub provisioned_throughput_in_mibps: Option<f64>,
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct UpdateFileSystemRequest {
    #[serde(rename = "ThroughputMode")]
    pub throughput_mode: Option<String>,
    #[serde(rename = "ProvisionedThroughputInMibps")]
    pub provisioned_throughput_in_mibps: Option<f64>,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct CreateMountTargetRequest {
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    #[serde(rename = "SubnetId")]
    pub subnet_id: String,
    #[serde(rename = "IpAddress")]
    pub ip_address: Option<String>,
    #[serde(rename = "SecurityGroups")]
    pub _security_groups: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct CreateAccessPointRequest {
    #[serde(rename = "ClientToken")]
    pub client_token: String,
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    #[serde(rename = "PosixUser")]
    pub posix_user: Option<PosixUser>,
    #[serde(rename = "RootDirectory")]
    pub root_directory: Option<RootDirectory>,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct TagResourceRequest {
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct UntagResourceRequest {
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct PutLifecycleConfigurationRequest {
    #[serde(rename = "LifecyclePolicies")]
    pub lifecycle_policies: Vec<LifecyclePolicy>,
}

// --- Response types ---

#[derive(Debug, Serialize)]
pub struct DescribeFileSystemsResponse {
    #[serde(rename = "FileSystems")]
    pub file_systems: Vec<FileSystemDescription>,
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct DescribeMountTargetsResponse {
    #[serde(rename = "MountTargets")]
    pub mount_targets: Vec<MountTargetDescription>,
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct DescribeAccessPointsResponse {
    #[serde(rename = "AccessPoints")]
    pub access_points: Vec<AccessPointDescription>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ListTagsForResourceResponse {
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct LifecycleConfigurationDescription {
    #[serde(rename = "LifecyclePolicies")]
    pub lifecycle_policies: Vec<LifecyclePolicy>,
}
