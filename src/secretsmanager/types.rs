use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct CreateSecretRequest {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "SecretString")]
    pub secret_string: Option<String>,
    #[serde(rename = "SecretBinary")]
    pub secret_binary: Option<String>,
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Serialize)]
pub struct CreateSecretResponse {
    #[serde(rename = "ARN")]
    pub arn: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "VersionId")]
    pub version_id: String,
}

#[derive(Debug, Deserialize)]
pub struct GetSecretValueRequest {
    #[serde(rename = "SecretId")]
    pub secret_id: String,
    #[serde(rename = "VersionId")]
    pub version_id: Option<String>,
    #[serde(rename = "VersionStage")]
    pub version_stage: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct GetSecretValueResponse {
    #[serde(rename = "ARN")]
    pub arn: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "VersionId")]
    pub version_id: String,
    #[serde(rename = "SecretString")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_string: Option<String>,
    #[serde(rename = "SecretBinary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_binary: Option<String>,
    #[serde(rename = "VersionStages")]
    pub version_stages: Vec<String>,
    #[serde(rename = "CreatedDate")]
    pub created_date: f64,
}

#[derive(Debug, Deserialize)]
pub struct PutSecretValueRequest {
    #[serde(rename = "SecretId")]
    pub secret_id: String,
    #[serde(rename = "SecretString")]
    pub secret_string: Option<String>,
    #[serde(rename = "SecretBinary")]
    pub secret_binary: Option<String>,
    #[serde(rename = "ClientRequestToken")]
    pub client_request_token: Option<String>,
    #[serde(rename = "VersionStages")]
    pub version_stages: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
pub struct PutSecretValueResponse {
    #[serde(rename = "ARN")]
    pub arn: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "VersionId")]
    pub version_id: String,
    #[serde(rename = "VersionStages")]
    pub version_stages: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct DescribeSecretRequest {
    #[serde(rename = "SecretId")]
    pub secret_id: String,
}

#[derive(Debug, Serialize)]
pub struct DescribeSecretResponse {
    #[serde(rename = "ARN")]
    pub arn: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "RotationEnabled")]
    pub rotation_enabled: bool,
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
    #[serde(rename = "CreatedDate")]
    pub created_date: f64,
    #[serde(rename = "LastChangedDate")]
    pub last_changed_date: f64,
    #[serde(rename = "VersionIdsToStages")]
    pub version_ids_to_stages: HashMap<String, Vec<String>>,
}

#[derive(Debug, Deserialize, Default)]
pub struct ListSecretsRequest {
    #[serde(rename = "MaxResults")]
    pub max_results: Option<usize>,
    #[serde(rename = "NextToken")]
    pub next_token: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ListSecretsResponse {
    #[serde(rename = "SecretList")]
    pub secret_list: Vec<SecretListEntry>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct SecretListEntry {
    #[serde(rename = "ARN")]
    pub arn: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "CreatedDate")]
    pub created_date: f64,
    #[serde(rename = "LastChangedDate")]
    pub last_changed_date: f64,
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateSecretRequest {
    #[serde(rename = "SecretId")]
    pub secret_id: String,
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "KmsKeyId")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "SecretString")]
    pub secret_string: Option<String>,
    #[serde(rename = "SecretBinary")]
    pub secret_binary: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UpdateSecretResponse {
    #[serde(rename = "ARN")]
    pub arn: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "VersionId")]
    pub version_id: String,
}

#[derive(Debug, Deserialize)]
pub struct DeleteSecretRequest {
    #[serde(rename = "SecretId")]
    pub secret_id: String,
    #[serde(rename = "RecoveryWindowInDays")]
    pub recovery_window_in_days: Option<u32>,
    #[serde(rename = "ForceDeleteWithoutRecovery")]
    pub force_delete_without_recovery: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct DeleteSecretResponse {
    #[serde(rename = "ARN")]
    pub arn: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "DeletionDate")]
    pub deletion_date: f64,
}

#[derive(Debug, Deserialize)]
pub struct RestoreSecretRequest {
    #[serde(rename = "SecretId")]
    pub secret_id: String,
}

#[derive(Debug, Serialize)]
pub struct RestoreSecretResponse {
    #[serde(rename = "ARN")]
    pub arn: String,
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct TagResourceRequest {
    #[serde(rename = "SecretId")]
    pub secret_id: String,
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

#[derive(Debug, Deserialize)]
pub struct UntagResourceRequest {
    #[serde(rename = "SecretId")]
    pub secret_id: String,
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct ListSecretVersionIdsRequest {
    #[serde(rename = "SecretId")]
    pub secret_id: String,
    #[serde(rename = "MaxResults")]
    pub max_results: Option<usize>,
}

#[derive(Debug, Serialize)]
pub struct ListSecretVersionIdsResponse {
    #[serde(rename = "ARN")]
    pub arn: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Versions")]
    pub versions: Vec<SecretVersionEntry>,
}

#[derive(Debug, Serialize, Clone)]
pub struct SecretVersionEntry {
    #[serde(rename = "VersionId")]
    pub version_id: String,
    #[serde(rename = "VersionStages")]
    pub version_stages: Vec<String>,
    #[serde(rename = "CreatedDate")]
    pub created_date: f64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,
}
