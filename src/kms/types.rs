use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Default)]
pub struct CreateKeyRequest {
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "KeyUsage")]
    pub key_usage: Option<String>,
    #[serde(rename = "KeySpec")]
    pub key_spec: Option<String>,
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Serialize)]
pub struct CreateKeyResponse {
    #[serde(rename = "KeyMetadata")]
    pub key_metadata: KeyMetadata,
}

#[derive(Debug, Serialize, Clone)]
pub struct KeyMetadata {
    #[serde(rename = "KeyId")]
    pub key_id: String,
    #[serde(rename = "Arn")]
    pub arn: String,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "KeyUsage")]
    pub key_usage: String,
    #[serde(rename = "KeySpec")]
    pub key_spec: String,
    #[serde(rename = "KeyState")]
    pub key_state: String,
    #[serde(rename = "Enabled")]
    pub enabled: bool,
    #[serde(rename = "CreationDate")]
    pub creation_date: f64,
    #[serde(rename = "KeyManager")]
    pub key_manager: String,
    #[serde(rename = "MultiRegion")]
    pub multi_region: bool,
}

#[derive(Debug, Deserialize)]
pub struct DescribeKeyRequest {
    #[serde(rename = "KeyId")]
    pub key_id: String,
}

#[derive(Debug, Serialize)]
pub struct DescribeKeyResponse {
    #[serde(rename = "KeyMetadata")]
    pub key_metadata: KeyMetadata,
}

#[derive(Debug, Deserialize, Default)]
pub struct ListKeysRequest {
    #[serde(rename = "Limit")]
    pub limit: Option<usize>,
    #[serde(rename = "Marker")]
    pub marker: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ListKeysResponse {
    #[serde(rename = "Keys")]
    pub keys: Vec<KeyListEntry>,
    #[serde(rename = "Truncated")]
    pub truncated: bool,
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct KeyListEntry {
    #[serde(rename = "KeyId")]
    pub key_id: String,
    #[serde(rename = "KeyArn")]
    pub key_arn: String,
}

#[derive(Debug, Deserialize)]
pub struct ScheduleKeyDeletionRequest {
    #[serde(rename = "KeyId")]
    pub key_id: String,
    #[serde(rename = "PendingWindowInDays")]
    pub pending_window_in_days: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct ScheduleKeyDeletionResponse {
    #[serde(rename = "KeyId")]
    pub key_id: String,
    #[serde(rename = "DeletionDate")]
    pub deletion_date: f64,
    #[serde(rename = "KeyState")]
    pub key_state: String,
    #[serde(rename = "PendingWindowInDays")]
    pub pending_window_in_days: u32,
}

#[derive(Debug, Deserialize)]
pub struct CancelKeyDeletionRequest {
    #[serde(rename = "KeyId")]
    pub key_id: String,
}

#[derive(Debug, Serialize)]
pub struct CancelKeyDeletionResponse {
    #[serde(rename = "KeyId")]
    pub key_id: String,
}

#[derive(Debug, Deserialize)]
pub struct EnableKeyRequest {
    #[serde(rename = "KeyId")]
    pub key_id: String,
}

#[derive(Debug, Deserialize)]
pub struct DisableKeyRequest {
    #[serde(rename = "KeyId")]
    pub key_id: String,
}

#[derive(Debug, Deserialize)]
pub struct EncryptRequest {
    #[serde(rename = "KeyId")]
    pub key_id: String,
    #[serde(rename = "Plaintext")]
    pub plaintext: String, // base64
    #[serde(rename = "EncryptionContext")]
    pub encryption_context: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "EncryptionAlgorithm")]
    pub encryption_algorithm: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct EncryptResponse {
    #[serde(rename = "KeyId")]
    pub key_id: String,
    #[serde(rename = "CiphertextBlob")]
    pub ciphertext_blob: String, // base64
    #[serde(rename = "EncryptionAlgorithm")]
    pub encryption_algorithm: String,
}

#[derive(Debug, Deserialize)]
pub struct DecryptRequest {
    #[serde(rename = "CiphertextBlob")]
    pub ciphertext_blob: String, // base64
    #[serde(rename = "KeyId")]
    pub key_id: Option<String>,
    #[serde(rename = "EncryptionContext")]
    pub encryption_context: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "EncryptionAlgorithm")]
    pub encryption_algorithm: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct DecryptResponse {
    #[serde(rename = "KeyId")]
    pub key_id: String,
    #[serde(rename = "Plaintext")]
    pub plaintext: String, // base64
    #[serde(rename = "EncryptionAlgorithm")]
    pub encryption_algorithm: String,
}

#[derive(Debug, Deserialize)]
pub struct GenerateDataKeyRequest {
    #[serde(rename = "KeyId")]
    pub key_id: String,
    #[serde(rename = "KeySpec")]
    pub key_spec: Option<String>,
    #[serde(rename = "NumberOfBytes")]
    pub number_of_bytes: Option<usize>,
    #[serde(rename = "EncryptionContext")]
    pub encryption_context: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
pub struct GenerateDataKeyResponse {
    #[serde(rename = "KeyId")]
    pub key_id: String,
    #[serde(rename = "Plaintext")]
    pub plaintext: String, // base64
    #[serde(rename = "CiphertextBlob")]
    pub ciphertext_blob: String, // base64
}

#[derive(Debug, Deserialize)]
pub struct GenerateDataKeyWithoutPlaintextRequest {
    #[serde(rename = "KeyId")]
    pub key_id: String,
    #[serde(rename = "KeySpec")]
    pub key_spec: Option<String>,
    #[serde(rename = "NumberOfBytes")]
    pub number_of_bytes: Option<usize>,
}

#[derive(Debug, Serialize)]
pub struct GenerateDataKeyWithoutPlaintextResponse {
    #[serde(rename = "KeyId")]
    pub key_id: String,
    #[serde(rename = "CiphertextBlob")]
    pub ciphertext_blob: String, // base64
}

#[derive(Debug, Deserialize)]
pub struct GenerateRandomRequest {
    #[serde(rename = "NumberOfBytes")]
    pub number_of_bytes: Option<usize>,
}

#[derive(Debug, Serialize)]
pub struct GenerateRandomResponse {
    #[serde(rename = "Plaintext")]
    pub plaintext: String, // base64
}

#[derive(Debug, Deserialize)]
pub struct SignRequest {
    #[serde(rename = "KeyId")]
    pub key_id: String,
    #[serde(rename = "Message")]
    pub message: String, // base64
    #[serde(rename = "MessageType")]
    pub message_type: Option<String>,
    #[serde(rename = "SigningAlgorithm")]
    pub signing_algorithm: String,
}

#[derive(Debug, Serialize)]
pub struct SignResponse {
    #[serde(rename = "KeyId")]
    pub key_id: String,
    #[serde(rename = "Signature")]
    pub signature: String, // base64
    #[serde(rename = "SigningAlgorithm")]
    pub signing_algorithm: String,
}

#[derive(Debug, Deserialize)]
pub struct VerifyRequest {
    #[serde(rename = "KeyId")]
    pub key_id: String,
    #[serde(rename = "Message")]
    pub message: String, // base64
    #[serde(rename = "Signature")]
    pub signature: String, // base64
    #[serde(rename = "SigningAlgorithm")]
    pub signing_algorithm: String,
    #[serde(rename = "MessageType")]
    pub message_type: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct VerifyResponse {
    #[serde(rename = "KeyId")]
    pub key_id: String,
    #[serde(rename = "SignatureValid")]
    pub signature_valid: bool,
    #[serde(rename = "SigningAlgorithm")]
    pub signing_algorithm: String,
}

#[derive(Debug, Deserialize)]
pub struct TagResourceRequest {
    #[serde(rename = "KeyId")]
    pub key_id: String,
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

#[derive(Debug, Deserialize)]
pub struct UntagResourceRequest {
    #[serde(rename = "KeyId")]
    pub key_id: String,
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct ListResourceTagsRequest {
    #[serde(rename = "KeyId")]
    pub key_id: String,
    #[serde(rename = "Limit")]
    pub limit: Option<usize>,
    #[serde(rename = "Marker")]
    pub marker: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ListResourceTagsResponse {
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
    #[serde(rename = "Truncated")]
    pub truncated: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tag {
    #[serde(rename = "TagKey")]
    pub tag_key: String,
    #[serde(rename = "TagValue")]
    pub tag_value: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateAliasRequest {
    #[serde(rename = "AliasName")]
    pub alias_name: String,
    #[serde(rename = "TargetKeyId")]
    pub target_key_id: String,
}

#[derive(Debug, Deserialize)]
pub struct DeleteAliasRequest {
    #[serde(rename = "AliasName")]
    pub alias_name: String,
}

#[derive(Debug, Deserialize, Default)]
pub struct ListAliasesRequest {
    #[serde(rename = "KeyId")]
    pub key_id: Option<String>,
    #[serde(rename = "Limit")]
    pub limit: Option<usize>,
    #[serde(rename = "Marker")]
    pub marker: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ListAliasesResponse {
    #[serde(rename = "Aliases")]
    pub aliases: Vec<AliasListEntry>,
    #[serde(rename = "Truncated")]
    pub truncated: bool,
}

#[derive(Debug, Serialize, Clone)]
pub struct AliasListEntry {
    #[serde(rename = "AliasName")]
    pub alias_name: String,
    #[serde(rename = "AliasArn")]
    pub alias_arn: String,
    #[serde(rename = "TargetKeyId")]
    pub target_key_id: String,
}

#[derive(Debug, Deserialize)]
pub struct GetKeyPolicyRequest {
    #[serde(rename = "KeyId")]
    pub key_id: String,
    #[serde(rename = "PolicyName")]
    pub policy_name: String,
}

#[derive(Debug, Serialize)]
pub struct GetKeyPolicyResponse {
    #[serde(rename = "Policy")]
    pub policy: String,
}

#[derive(Debug, Deserialize)]
pub struct PutKeyPolicyRequest {
    #[serde(rename = "KeyId")]
    pub key_id: String,
    #[serde(rename = "PolicyName")]
    pub policy_name: String,
    #[serde(rename = "Policy")]
    pub policy: String,
}
