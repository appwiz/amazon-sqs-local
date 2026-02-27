use std::collections::HashMap;
use std::sync::Arc;

use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use tokio::sync::Mutex;
use uuid::Uuid;

use super::error::KmsError;
use super::types::*;

struct KmsKey {
    metadata: KeyMetadata,
    tags: HashMap<String, String>,
    policy: String,
    // For encrypt/decrypt simulation: just store a marker prefix
}

struct KmsStateInner {
    keys: HashMap<String, KmsKey>,
    aliases: HashMap<String, AliasListEntry>,
    account_id: String,
    region: String,
}

pub struct KmsState {
    inner: Arc<Mutex<KmsStateInner>>,
}

impl KmsState {
    pub fn new(account_id: String, region: String) -> Self {
        KmsState {
            inner: Arc::new(Mutex::new(KmsStateInner {
                keys: HashMap::new(),
                aliases: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    fn now_secs() -> f64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64()
    }

    // Resolve key_id which may be an alias or ARN
    fn resolve_key_id<'a>(state: &'a KmsStateInner, key_id: &'a str) -> Option<&'a str> {
        // Direct key ID
        if state.keys.contains_key(key_id) {
            return Some(key_id);
        }
        // ARN format: arn:aws:kms:region:account:key/key-id
        if key_id.starts_with("arn:") {
            let parts: Vec<&str> = key_id.splitn(6, ':').collect();
            if parts.len() >= 6 {
                let resource = parts[5]; // "key/uuid"
                let id = resource.strip_prefix("key/").unwrap_or(resource);
                if state.keys.contains_key(id) {
                    return state.keys.get_key_value(id).map(|(k, _)| k.as_str());
                }
            }
        }
        // Alias
        if let Some(alias) = state.aliases.get(key_id) {
            let target = alias.target_key_id.clone();
            return state.keys.get_key_value(&target).map(|(k, _)| k.as_str());
        }
        // alias/ prefix
        if key_id.starts_with("alias/") {
            if let Some(alias) = state.aliases.get(key_id) {
                let target = alias.target_key_id.clone();
                return state.keys.get_key_value(&target).map(|(k, _)| k.as_str());
            }
        }
        None
    }

    pub async fn create_key(
        &self,
        req: CreateKeyRequest,
    ) -> Result<CreateKeyResponse, KmsError> {
        let mut state = self.inner.lock().await;
        let key_id = Uuid::new_v4().to_string();
        let arn = format!(
            "arn:aws:kms:{}:{}:key/{}",
            state.region, state.account_id, key_id
        );
        let key_usage = req.key_usage.unwrap_or_else(|| "ENCRYPT_DECRYPT".to_string());
        let key_spec = req.key_spec.unwrap_or_else(|| "SYMMETRIC_DEFAULT".to_string());
        let metadata = KeyMetadata {
            key_id: key_id.clone(),
            arn: arn.clone(),
            description: req.description.unwrap_or_default(),
            key_usage: key_usage.clone(),
            key_spec: key_spec.clone(),
            key_state: "Enabled".to_string(),
            enabled: true,
            creation_date: Self::now_secs(),
            key_manager: "CUSTOMER".to_string(),
            multi_region: false,
        };
        let mut tags = HashMap::new();
        if let Some(t) = req.tags {
            for tag in t {
                tags.insert(tag.tag_key, tag.tag_value);
            }
        }
        state.keys.insert(key_id, KmsKey {
            metadata: metadata.clone(),
            tags,
            policy: r#"{"Version":"2012-10-17","Statement":[{"Effect":"Allow","Principal":{"AWS":"*"},"Action":"kms:*","Resource":"*"}]}"#.to_string(),
        });
        Ok(CreateKeyResponse { key_metadata: metadata })
    }

    pub async fn describe_key(
        &self,
        req: DescribeKeyRequest,
    ) -> Result<DescribeKeyResponse, KmsError> {
        let state = self.inner.lock().await;
        let key_id_str = req.key_id.clone();
        let resolved = Self::resolve_key_id(&state, &key_id_str)
            .ok_or_else(|| KmsError::NotFoundException(format!("Invalid keyId {}", key_id_str)))?;
        let key = &state.keys[resolved];
        Ok(DescribeKeyResponse { key_metadata: key.metadata.clone() })
    }

    pub async fn list_keys(
        &self,
        req: ListKeysRequest,
    ) -> Result<ListKeysResponse, KmsError> {
        let state = self.inner.lock().await;
        let mut entries: Vec<KeyListEntry> = state.keys.values().map(|k| KeyListEntry {
            key_id: k.metadata.key_id.clone(),
            key_arn: k.metadata.arn.clone(),
        }).collect();
        entries.sort_by(|a, b| a.key_id.cmp(&b.key_id));

        let limit = req.limit.unwrap_or(1000);
        let truncated = entries.len() > limit;
        entries.truncate(limit);

        Ok(ListKeysResponse { keys: entries, truncated, next_marker: None })
    }

    pub async fn schedule_key_deletion(
        &self,
        req: ScheduleKeyDeletionRequest,
    ) -> Result<ScheduleKeyDeletionResponse, KmsError> {
        let mut state = self.inner.lock().await;
        let key_id_str = req.key_id.clone();
        let resolved = Self::resolve_key_id(&state, &key_id_str)
            .ok_or_else(|| KmsError::NotFoundException(format!("Invalid keyId {}", key_id_str)))?
            .to_string();
        let key = state.keys.get_mut(&resolved).unwrap();
        let days = req.pending_window_in_days.unwrap_or(30);
        key.metadata.key_state = "PendingDeletion".to_string();
        key.metadata.enabled = false;
        let deletion_date = Self::now_secs() + (days as f64 * 86400.0);
        Ok(ScheduleKeyDeletionResponse {
            key_id: resolved,
            deletion_date,
            key_state: "PendingDeletion".to_string(),
            pending_window_in_days: days,
        })
    }

    pub async fn cancel_key_deletion(
        &self,
        req: CancelKeyDeletionRequest,
    ) -> Result<CancelKeyDeletionResponse, KmsError> {
        let mut state = self.inner.lock().await;
        let key_id_str = req.key_id.clone();
        let resolved = Self::resolve_key_id(&state, &key_id_str)
            .ok_or_else(|| KmsError::NotFoundException(format!("Invalid keyId {}", key_id_str)))?
            .to_string();
        let key = state.keys.get_mut(&resolved).unwrap();
        key.metadata.key_state = "Enabled".to_string();
        key.metadata.enabled = true;
        Ok(CancelKeyDeletionResponse { key_id: resolved })
    }

    pub async fn enable_key(&self, req: EnableKeyRequest) -> Result<(), KmsError> {
        let mut state = self.inner.lock().await;
        let key_id_str = req.key_id.clone();
        let resolved = Self::resolve_key_id(&state, &key_id_str)
            .ok_or_else(|| KmsError::NotFoundException(format!("Invalid keyId {}", key_id_str)))?
            .to_string();
        let key = state.keys.get_mut(&resolved).unwrap();
        key.metadata.enabled = true;
        key.metadata.key_state = "Enabled".to_string();
        Ok(())
    }

    pub async fn disable_key(&self, req: DisableKeyRequest) -> Result<(), KmsError> {
        let mut state = self.inner.lock().await;
        let key_id_str = req.key_id.clone();
        let resolved = Self::resolve_key_id(&state, &key_id_str)
            .ok_or_else(|| KmsError::NotFoundException(format!("Invalid keyId {}", key_id_str)))?
            .to_string();
        let key = state.keys.get_mut(&resolved).unwrap();
        key.metadata.enabled = false;
        key.metadata.key_state = "Disabled".to_string();
        Ok(())
    }

    pub async fn encrypt(&self, req: EncryptRequest) -> Result<EncryptResponse, KmsError> {
        let state = self.inner.lock().await;
        let key_id_str = req.key_id.clone();
        let resolved = Self::resolve_key_id(&state, &key_id_str)
            .ok_or_else(|| KmsError::NotFoundException(format!("Invalid keyId {}", key_id_str)))?
            .to_string();
        let key = &state.keys[&resolved];
        if !key.metadata.enabled {
            return Err(KmsError::DisabledException(format!("KMS key {} is disabled", resolved)));
        }
        // Simulate encryption: prefix key_id + ":" + original base64
        let plaintext_b64 = req.plaintext;
        let simulated = format!("{}:{}", resolved, plaintext_b64);
        let ciphertext_blob = BASE64.encode(simulated.as_bytes());
        Ok(EncryptResponse {
            key_id: key.metadata.arn.clone(),
            ciphertext_blob,
            encryption_algorithm: req.encryption_algorithm.unwrap_or_else(|| "SYMMETRIC_DEFAULT".to_string()),
        })
    }

    pub async fn decrypt(&self, req: DecryptRequest) -> Result<DecryptResponse, KmsError> {
        let state = self.inner.lock().await;
        let decoded = BASE64.decode(&req.ciphertext_blob)
            .map_err(|_| KmsError::InvalidCiphertextException("Invalid ciphertext".to_string()))?;
        let decoded_str = String::from_utf8(decoded)
            .map_err(|_| KmsError::InvalidCiphertextException("Invalid ciphertext encoding".to_string()))?;
        let (key_id_from_ct, plaintext_b64) = decoded_str.split_once(':')
            .ok_or_else(|| KmsError::InvalidCiphertextException("Malformed ciphertext".to_string()))?;

        // Verify key exists
        let resolved = if let Some(explicit) = &req.key_id {
            let r = Self::resolve_key_id(&state, explicit)
                .ok_or_else(|| KmsError::NotFoundException(format!("Invalid keyId {}", explicit)))?
                .to_string();
            r
        } else {
            key_id_from_ct.to_string()
        };

        let key = state.keys.get(&resolved)
            .ok_or_else(|| KmsError::NotFoundException(format!("Invalid keyId {}", resolved)))?;
        if !key.metadata.enabled {
            return Err(KmsError::DisabledException(format!("KMS key {} is disabled", resolved)));
        }

        Ok(DecryptResponse {
            key_id: key.metadata.arn.clone(),
            plaintext: plaintext_b64.to_string(),
            encryption_algorithm: req.encryption_algorithm.unwrap_or_else(|| "SYMMETRIC_DEFAULT".to_string()),
        })
    }

    pub async fn generate_data_key(
        &self,
        req: GenerateDataKeyRequest,
    ) -> Result<GenerateDataKeyResponse, KmsError> {
        let state = self.inner.lock().await;
        let key_id_str = req.key_id.clone();
        let resolved = Self::resolve_key_id(&state, &key_id_str)
            .ok_or_else(|| KmsError::NotFoundException(format!("Invalid keyId {}", key_id_str)))?
            .to_string();
        let key = &state.keys[&resolved];
        if !key.metadata.enabled {
            return Err(KmsError::DisabledException(format!("KMS key {} is disabled", resolved)));
        }
        let num_bytes = match req.key_spec.as_deref() {
            Some("AES_256") => 32,
            Some("AES_128") => 16,
            _ => req.number_of_bytes.unwrap_or(32),
        };
        // Generate pseudo-random bytes
        let plaintext: Vec<u8> = (0..num_bytes).map(|i| (i as u8).wrapping_add(42)).collect();
        let plaintext_b64 = BASE64.encode(&plaintext);
        let simulated = format!("{}:{}", resolved, plaintext_b64);
        let ciphertext_blob = BASE64.encode(simulated.as_bytes());
        Ok(GenerateDataKeyResponse {
            key_id: key.metadata.arn.clone(),
            plaintext: plaintext_b64,
            ciphertext_blob,
        })
    }

    pub async fn generate_data_key_without_plaintext(
        &self,
        req: GenerateDataKeyWithoutPlaintextRequest,
    ) -> Result<GenerateDataKeyWithoutPlaintextResponse, KmsError> {
        let state = self.inner.lock().await;
        let key_id_str = req.key_id.clone();
        let resolved = Self::resolve_key_id(&state, &key_id_str)
            .ok_or_else(|| KmsError::NotFoundException(format!("Invalid keyId {}", key_id_str)))?
            .to_string();
        let key = &state.keys[&resolved];
        if !key.metadata.enabled {
            return Err(KmsError::DisabledException(format!("KMS key {} is disabled", resolved)));
        }
        let num_bytes = match req.key_spec.as_deref() {
            Some("AES_256") => 32,
            Some("AES_128") => 16,
            _ => req.number_of_bytes.unwrap_or(32),
        };
        let plaintext: Vec<u8> = (0..num_bytes).map(|i| (i as u8).wrapping_add(42)).collect();
        let plaintext_b64 = BASE64.encode(&plaintext);
        let simulated = format!("{}:{}", resolved, plaintext_b64);
        let ciphertext_blob = BASE64.encode(simulated.as_bytes());
        Ok(GenerateDataKeyWithoutPlaintextResponse {
            key_id: key.metadata.arn.clone(),
            ciphertext_blob,
        })
    }

    pub async fn generate_random(
        &self,
        req: GenerateRandomRequest,
    ) -> Result<GenerateRandomResponse, KmsError> {
        let num_bytes = req.number_of_bytes.unwrap_or(32).min(1024);
        let random_bytes: Vec<u8> = (0..num_bytes).map(|i| i as u8).collect();
        Ok(GenerateRandomResponse {
            plaintext: BASE64.encode(&random_bytes),
        })
    }

    pub async fn sign(&self, req: SignRequest) -> Result<SignResponse, KmsError> {
        let state = self.inner.lock().await;
        let key_id_str = req.key_id.clone();
        let resolved = Self::resolve_key_id(&state, &key_id_str)
            .ok_or_else(|| KmsError::NotFoundException(format!("Invalid keyId {}", key_id_str)))?
            .to_string();
        let key = &state.keys[&resolved];
        if !key.metadata.enabled {
            return Err(KmsError::DisabledException(format!("KMS key {} is disabled", resolved)));
        }
        // Simulate: signature = base64(key_id + ":" + message_b64)
        let simulated = format!("{}:{}", resolved, req.message);
        let signature = BASE64.encode(simulated.as_bytes());
        Ok(SignResponse {
            key_id: key.metadata.arn.clone(),
            signature,
            signing_algorithm: req.signing_algorithm,
        })
    }

    pub async fn verify(&self, req: VerifyRequest) -> Result<VerifyResponse, KmsError> {
        let state = self.inner.lock().await;
        let key_id_str = req.key_id.clone();
        let resolved = Self::resolve_key_id(&state, &key_id_str)
            .ok_or_else(|| KmsError::NotFoundException(format!("Invalid keyId {}", key_id_str)))?
            .to_string();
        let key = &state.keys[&resolved];
        if !key.metadata.enabled {
            return Err(KmsError::DisabledException(format!("KMS key {} is disabled", resolved)));
        }
        // Verify: decode signature and check it matches our simulation format
        let decoded = BASE64.decode(&req.signature).unwrap_or_default();
        let decoded_str = String::from_utf8(decoded).unwrap_or_default();
        let expected = format!("{}:{}", resolved, req.message);
        let valid = decoded_str == expected;
        Ok(VerifyResponse {
            key_id: key.metadata.arn.clone(),
            signature_valid: valid,
            signing_algorithm: req.signing_algorithm,
        })
    }

    pub async fn tag_resource(&self, req: TagResourceRequest) -> Result<(), KmsError> {
        let mut state = self.inner.lock().await;
        let key_id_str = req.key_id.clone();
        let resolved = Self::resolve_key_id(&state, &key_id_str)
            .ok_or_else(|| KmsError::NotFoundException(format!("Invalid keyId {}", key_id_str)))?
            .to_string();
        let key = state.keys.get_mut(&resolved).unwrap();
        for tag in req.tags {
            key.tags.insert(tag.tag_key, tag.tag_value);
        }
        Ok(())
    }

    pub async fn untag_resource(&self, req: UntagResourceRequest) -> Result<(), KmsError> {
        let mut state = self.inner.lock().await;
        let key_id_str = req.key_id.clone();
        let resolved = Self::resolve_key_id(&state, &key_id_str)
            .ok_or_else(|| KmsError::NotFoundException(format!("Invalid keyId {}", key_id_str)))?
            .to_string();
        let key = state.keys.get_mut(&resolved).unwrap();
        for k in &req.tag_keys {
            key.tags.remove(k);
        }
        Ok(())
    }

    pub async fn list_resource_tags(
        &self,
        req: ListResourceTagsRequest,
    ) -> Result<ListResourceTagsResponse, KmsError> {
        let state = self.inner.lock().await;
        let key_id_str = req.key_id.clone();
        let resolved = Self::resolve_key_id(&state, &key_id_str)
            .ok_or_else(|| KmsError::NotFoundException(format!("Invalid keyId {}", key_id_str)))?
            .to_string();
        let key = &state.keys[&resolved];
        let mut tags: Vec<Tag> = key.tags.iter().map(|(k, v)| Tag {
            tag_key: k.clone(),
            tag_value: v.clone(),
        }).collect();
        tags.sort_by(|a, b| a.tag_key.cmp(&b.tag_key));
        let limit = req.limit.unwrap_or(50);
        let truncated = tags.len() > limit;
        tags.truncate(limit);
        Ok(ListResourceTagsResponse { tags, truncated })
    }

    pub async fn create_alias(&self, req: CreateAliasRequest) -> Result<(), KmsError> {
        let mut state = self.inner.lock().await;
        let key_id_str = req.target_key_id.clone();
        let resolved = {
            // Can't borrow state mutably and immutably, resolve first
            let r = state.keys.contains_key(&key_id_str);
            if !r {
                return Err(KmsError::NotFoundException(format!("Invalid keyId {}", key_id_str)));
            }
            key_id_str.clone()
        };
        let alias_arn = format!(
            "arn:aws:kms:{}:{}:{}",
            state.region, state.account_id, req.alias_name
        );
        state.aliases.insert(req.alias_name.clone(), AliasListEntry {
            alias_name: req.alias_name,
            alias_arn,
            target_key_id: resolved,
        });
        Ok(())
    }

    pub async fn delete_alias(&self, req: DeleteAliasRequest) -> Result<(), KmsError> {
        let mut state = self.inner.lock().await;
        if state.aliases.remove(&req.alias_name).is_none() {
            return Err(KmsError::NotFoundException(format!("Alias {} not found", req.alias_name)));
        }
        Ok(())
    }

    pub async fn list_aliases(
        &self,
        req: ListAliasesRequest,
    ) -> Result<ListAliasesResponse, KmsError> {
        let state = self.inner.lock().await;
        let mut aliases: Vec<AliasListEntry> = state.aliases.values().cloned().collect();
        if let Some(ref key_id) = req.key_id {
            aliases.retain(|a| &a.target_key_id == key_id);
        }
        aliases.sort_by(|a, b| a.alias_name.cmp(&b.alias_name));
        let limit = req.limit.unwrap_or(100);
        let truncated = aliases.len() > limit;
        aliases.truncate(limit);
        Ok(ListAliasesResponse { aliases, truncated })
    }

    pub async fn get_key_policy(
        &self,
        req: GetKeyPolicyRequest,
    ) -> Result<GetKeyPolicyResponse, KmsError> {
        let state = self.inner.lock().await;
        let key_id_str = req.key_id.clone();
        let resolved = Self::resolve_key_id(&state, &key_id_str)
            .ok_or_else(|| KmsError::NotFoundException(format!("Invalid keyId {}", key_id_str)))?
            .to_string();
        let key = &state.keys[&resolved];
        Ok(GetKeyPolicyResponse { policy: key.policy.clone() })
    }

    pub async fn put_key_policy(&self, req: PutKeyPolicyRequest) -> Result<(), KmsError> {
        let mut state = self.inner.lock().await;
        let key_id_str = req.key_id.clone();
        let resolved = Self::resolve_key_id(&state, &key_id_str)
            .ok_or_else(|| KmsError::NotFoundException(format!("Invalid keyId {}", key_id_str)))?
            .to_string();
        let key = state.keys.get_mut(&resolved).unwrap();
        key.policy = req.policy;
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = KmsState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_enable_key() {
        let state = KmsState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = EnableKeyRequest::default();
        let _ = state.enable_key(req).await;
    }
    #[tokio::test]
    async fn test_disable_key() {
        let state = KmsState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DisableKeyRequest::default();
        let _ = state.disable_key(req).await;
    }
    #[tokio::test]
    async fn test_encrypt() {
        let state = KmsState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = EncryptRequest::default();
        let _ = state.encrypt(req).await;
    }
    #[tokio::test]
    async fn test_decrypt() {
        let state = KmsState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DecryptRequest::default();
        let _ = state.decrypt(req).await;
    }
    #[tokio::test]
    async fn test_sign() {
        let state = KmsState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = SignRequest::default();
        let _ = state.sign(req).await;
    }
    #[tokio::test]
    async fn test_verify() {
        let state = KmsState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = VerifyRequest::default();
        let _ = state.verify(req).await;
    }
    #[tokio::test]
    async fn test_tag_resource() {
        let state = KmsState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = TagResourceRequest::default();
        let _ = state.tag_resource(req).await;
    }
    #[tokio::test]
    async fn test_untag_resource() {
        let state = KmsState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = UntagResourceRequest::default();
        let _ = state.untag_resource(req).await;
    }
    #[tokio::test]
    async fn test_create_alias() {
        let state = KmsState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateAliasRequest::default();
        let result = state.create_alias(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_delete_alias_not_found() {
        let state = KmsState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DeleteAliasRequest::default();
        let result = state.delete_alias(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_put_key_policy() {
        let state = KmsState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = PutKeyPolicyRequest::default();
        let _ = state.put_key_policy(req).await;
    }

    // --- Comprehensive additional tests ---

    fn make_state() -> KmsState {
        KmsState::new("123456789012".to_string(), "us-east-1".to_string())
    }

    async fn create_key(state: &KmsState) -> String {
        let resp = state.create_key(CreateKeyRequest::default()).await.unwrap();
        resp.key_metadata.key_id
    }

    #[tokio::test]
    async fn test_create_key_success() {
        let state = make_state();
        let resp = state.create_key(CreateKeyRequest {
            description: Some("test key".to_string()),
            key_usage: Some("ENCRYPT_DECRYPT".to_string()),
            key_spec: Some("SYMMETRIC_DEFAULT".to_string()),
            ..Default::default()
        }).await.unwrap();
        assert!(!resp.key_metadata.key_id.is_empty());
        assert!(resp.key_metadata.arn.contains("key/"));
        assert_eq!(resp.key_metadata.key_state, "Enabled");
        assert!(resp.key_metadata.enabled);
        assert_eq!(resp.key_metadata.key_usage, "ENCRYPT_DECRYPT");
    }

    #[tokio::test]
    async fn test_create_key_with_tags() {
        let state = make_state();
        let resp = state.create_key(CreateKeyRequest {
            tags: Some(vec![
                Tag { tag_key: "env".to_string(), tag_value: "test".to_string() },
            ]),
            ..Default::default()
        }).await.unwrap();
        let key_id = resp.key_metadata.key_id;

        let tags = state.list_resource_tags(ListResourceTagsRequest {
            key_id,
            ..Default::default()
        }).await.unwrap();
        assert_eq!(tags.tags.len(), 1);
        assert_eq!(tags.tags[0].tag_key, "env");
    }

    #[tokio::test]
    async fn test_describe_key_success() {
        let state = make_state();
        let key_id = create_key(&state).await;
        let resp = state.describe_key(DescribeKeyRequest { key_id: key_id.clone() }).await.unwrap();
        assert_eq!(resp.key_metadata.key_id, key_id);
    }

    #[tokio::test]
    async fn test_describe_key_not_found() {
        let state = make_state();
        let result = state.describe_key(DescribeKeyRequest {
            key_id: "nonexistent-key".to_string(),
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_keys() {
        let state = make_state();
        create_key(&state).await;
        create_key(&state).await;
        let result = state.list_keys(ListKeysRequest::default()).await.unwrap();
        assert_eq!(result.keys.len(), 2);
        assert!(!result.truncated);
    }

    #[tokio::test]
    async fn test_list_keys_empty() {
        let state = make_state();
        let result = state.list_keys(ListKeysRequest::default()).await.unwrap();
        assert!(result.keys.is_empty());
    }

    #[tokio::test]
    async fn test_enable_disable_key_lifecycle() {
        let state = make_state();
        let key_id = create_key(&state).await;

        // Disable
        state.disable_key(DisableKeyRequest { key_id: key_id.clone() }).await.unwrap();
        let desc = state.describe_key(DescribeKeyRequest { key_id: key_id.clone() }).await.unwrap();
        assert!(!desc.key_metadata.enabled);
        assert_eq!(desc.key_metadata.key_state, "Disabled");

        // Enable
        state.enable_key(EnableKeyRequest { key_id: key_id.clone() }).await.unwrap();
        let desc = state.describe_key(DescribeKeyRequest { key_id }).await.unwrap();
        assert!(desc.key_metadata.enabled);
        assert_eq!(desc.key_metadata.key_state, "Enabled");
    }

    #[tokio::test]
    async fn test_enable_key_not_found() {
        let state = make_state();
        let result = state.enable_key(EnableKeyRequest { key_id: "nope".to_string() }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_disable_key_not_found() {
        let state = make_state();
        let result = state.disable_key(DisableKeyRequest { key_id: "nope".to_string() }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_encrypt_decrypt_roundtrip() {
        let state = make_state();
        let key_id = create_key(&state).await;

        let plaintext = "SGVsbG8gV29ybGQ=".to_string(); // base64("Hello World")
        let enc = state.encrypt(EncryptRequest {
            key_id: key_id.clone(),
            plaintext: plaintext.clone(),
            ..Default::default()
        }).await.unwrap();
        assert!(!enc.ciphertext_blob.is_empty());
        assert_eq!(enc.encryption_algorithm, "SYMMETRIC_DEFAULT");

        let dec = state.decrypt(DecryptRequest {
            ciphertext_blob: enc.ciphertext_blob,
            key_id: Some(key_id),
            ..Default::default()
        }).await.unwrap();
        assert_eq!(dec.plaintext, plaintext);
    }

    #[tokio::test]
    async fn test_encrypt_disabled_key() {
        let state = make_state();
        let key_id = create_key(&state).await;
        state.disable_key(DisableKeyRequest { key_id: key_id.clone() }).await.unwrap();

        let result = state.encrypt(EncryptRequest {
            key_id,
            plaintext: "data".to_string(),
            ..Default::default()
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_encrypt_key_not_found() {
        let state = make_state();
        let result = state.encrypt(EncryptRequest {
            key_id: "nope".to_string(),
            plaintext: "data".to_string(),
            ..Default::default()
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_decrypt_disabled_key() {
        let state = make_state();
        let key_id = create_key(&state).await;
        let enc = state.encrypt(EncryptRequest {
            key_id: key_id.clone(),
            plaintext: "data".to_string(),
            ..Default::default()
        }).await.unwrap();

        state.disable_key(DisableKeyRequest { key_id: key_id.clone() }).await.unwrap();
        let result = state.decrypt(DecryptRequest {
            ciphertext_blob: enc.ciphertext_blob,
            key_id: Some(key_id),
            ..Default::default()
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_decrypt_invalid_ciphertext() {
        let state = make_state();
        let result = state.decrypt(DecryptRequest {
            ciphertext_blob: "not-valid-base64!!!".to_string(),
            ..Default::default()
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_sign_and_verify_roundtrip() {
        let state = make_state();
        let key_id = create_key(&state).await;

        let message = "dGVzdCBtZXNzYWdl".to_string();
        let sign_resp = state.sign(SignRequest {
            key_id: key_id.clone(),
            message: message.clone(),
            signing_algorithm: "RSASSA_PSS_SHA_256".to_string(),
        }).await.unwrap();
        assert!(!sign_resp.signature.is_empty());

        let verify_resp = state.verify(VerifyRequest {
            key_id: key_id.clone(),
            message: message.clone(),
            signature: sign_resp.signature,
            signing_algorithm: "RSASSA_PSS_SHA_256".to_string(),
        }).await.unwrap();
        assert!(verify_resp.signature_valid);
    }

    #[tokio::test]
    async fn test_verify_invalid_signature() {
        let state = make_state();
        let key_id = create_key(&state).await;

        let verify_resp = state.verify(VerifyRequest {
            key_id,
            message: "msg".to_string(),
            signature: "aW52YWxpZA==".to_string(), // base64("invalid")
            signing_algorithm: "RSASSA_PSS_SHA_256".to_string(),
        }).await.unwrap();
        assert!(!verify_resp.signature_valid);
    }

    #[tokio::test]
    async fn test_sign_disabled_key() {
        let state = make_state();
        let key_id = create_key(&state).await;
        state.disable_key(DisableKeyRequest { key_id: key_id.clone() }).await.unwrap();

        let result = state.sign(SignRequest {
            key_id,
            message: "msg".to_string(),
            signing_algorithm: "RSASSA_PSS_SHA_256".to_string(),
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_verify_disabled_key() {
        let state = make_state();
        let key_id = create_key(&state).await;
        state.disable_key(DisableKeyRequest { key_id: key_id.clone() }).await.unwrap();

        let result = state.verify(VerifyRequest {
            key_id,
            message: "msg".to_string(),
            signature: "sig".to_string(),
            signing_algorithm: "RSASSA_PSS_SHA_256".to_string(),
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_tag_untag_list_resource_tags() {
        let state = make_state();
        let key_id = create_key(&state).await;

        state.tag_resource(TagResourceRequest {
            key_id: key_id.clone(),
            tags: vec![
                Tag { tag_key: "env".to_string(), tag_value: "prod".to_string() },
                Tag { tag_key: "team".to_string(), tag_value: "infra".to_string() },
            ],
        }).await.unwrap();

        let tags = state.list_resource_tags(ListResourceTagsRequest {
            key_id: key_id.clone(),
            ..Default::default()
        }).await.unwrap();
        assert_eq!(tags.tags.len(), 2);

        state.untag_resource(UntagResourceRequest {
            key_id: key_id.clone(),
            tag_keys: vec!["env".to_string()],
        }).await.unwrap();

        let tags = state.list_resource_tags(ListResourceTagsRequest {
            key_id,
            ..Default::default()
        }).await.unwrap();
        assert_eq!(tags.tags.len(), 1);
        assert_eq!(tags.tags[0].tag_key, "team");
    }

    #[tokio::test]
    async fn test_tag_resource_key_not_found() {
        let state = make_state();
        let result = state.tag_resource(TagResourceRequest {
            key_id: "nope".to_string(),
            tags: vec![Tag { tag_key: "k".to_string(), tag_value: "v".to_string() }],
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_and_delete_alias() {
        let state = make_state();
        let key_id = create_key(&state).await;

        state.create_alias(CreateAliasRequest {
            alias_name: "alias/my-key".to_string(),
            target_key_id: key_id.clone(),
        }).await.unwrap();

        let aliases = state.list_aliases(ListAliasesRequest::default()).await.unwrap();
        assert_eq!(aliases.aliases.len(), 1);
        assert_eq!(aliases.aliases[0].alias_name, "alias/my-key");
        assert_eq!(aliases.aliases[0].target_key_id, key_id);

        state.delete_alias(DeleteAliasRequest {
            alias_name: "alias/my-key".to_string(),
        }).await.unwrap();

        let aliases = state.list_aliases(ListAliasesRequest::default()).await.unwrap();
        assert!(aliases.aliases.is_empty());
    }

    #[tokio::test]
    async fn test_list_aliases_filter_by_key() {
        let state = make_state();
        let key1 = create_key(&state).await;
        let key2 = create_key(&state).await;

        state.create_alias(CreateAliasRequest {
            alias_name: "alias/k1".to_string(),
            target_key_id: key1.clone(),
        }).await.unwrap();
        state.create_alias(CreateAliasRequest {
            alias_name: "alias/k2".to_string(),
            target_key_id: key2.clone(),
        }).await.unwrap();

        let aliases = state.list_aliases(ListAliasesRequest {
            key_id: Some(key1),
            ..Default::default()
        }).await.unwrap();
        assert_eq!(aliases.aliases.len(), 1);
        assert_eq!(aliases.aliases[0].alias_name, "alias/k1");
    }

    #[tokio::test]
    async fn test_get_and_put_key_policy() {
        let state = make_state();
        let key_id = create_key(&state).await;

        let policy = state.get_key_policy(GetKeyPolicyRequest { key_id: key_id.clone() }).await.unwrap();
        assert!(!policy.policy.is_empty());

        let new_policy = r#"{"Version":"2012-10-17","Statement":[]}"#.to_string();
        state.put_key_policy(PutKeyPolicyRequest {
            key_id: key_id.clone(),
            policy: new_policy.clone(),
        }).await.unwrap();

        let updated = state.get_key_policy(GetKeyPolicyRequest { key_id }).await.unwrap();
        assert_eq!(updated.policy, new_policy);
    }

    #[tokio::test]
    async fn test_get_key_policy_not_found() {
        let state = make_state();
        let result = state.get_key_policy(GetKeyPolicyRequest { key_id: "nope".to_string() }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_put_key_policy_not_found() {
        let state = make_state();
        let result = state.put_key_policy(PutKeyPolicyRequest {
            key_id: "nope".to_string(),
            policy: "{}".to_string(),
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_schedule_and_cancel_key_deletion() {
        let state = make_state();
        let key_id = create_key(&state).await;

        let sched = state.schedule_key_deletion(ScheduleKeyDeletionRequest {
            key_id: key_id.clone(),
            pending_window_in_days: Some(7),
        }).await.unwrap();
        assert_eq!(sched.key_state, "PendingDeletion");
        assert_eq!(sched.pending_window_in_days, 7);

        let desc = state.describe_key(DescribeKeyRequest { key_id: key_id.clone() }).await.unwrap();
        assert!(!desc.key_metadata.enabled);

        state.cancel_key_deletion(CancelKeyDeletionRequest { key_id: key_id.clone() }).await.unwrap();
        let desc = state.describe_key(DescribeKeyRequest { key_id }).await.unwrap();
        assert!(desc.key_metadata.enabled);
        assert_eq!(desc.key_metadata.key_state, "Enabled");
    }

    #[tokio::test]
    async fn test_schedule_key_deletion_not_found() {
        let state = make_state();
        let result = state.schedule_key_deletion(ScheduleKeyDeletionRequest {
            key_id: "nope".to_string(),
            ..Default::default()
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_cancel_key_deletion_not_found() {
        let state = make_state();
        let result = state.cancel_key_deletion(CancelKeyDeletionRequest {
            key_id: "nope".to_string(),
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_generate_data_key() {
        let state = make_state();
        let key_id = create_key(&state).await;

        let resp = state.generate_data_key(GenerateDataKeyRequest {
            key_id: key_id.clone(),
            key_spec: Some("AES_256".to_string()),
            ..Default::default()
        }).await.unwrap();
        assert!(!resp.plaintext.is_empty());
        assert!(!resp.ciphertext_blob.is_empty());
        assert!(resp.key_id.contains("key/"));
    }

    #[tokio::test]
    async fn test_generate_data_key_disabled() {
        let state = make_state();
        let key_id = create_key(&state).await;
        state.disable_key(DisableKeyRequest { key_id: key_id.clone() }).await.unwrap();

        let result = state.generate_data_key(GenerateDataKeyRequest {
            key_id,
            ..Default::default()
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_generate_data_key_without_plaintext() {
        let state = make_state();
        let key_id = create_key(&state).await;

        let resp = state.generate_data_key_without_plaintext(GenerateDataKeyWithoutPlaintextRequest {
            key_id,
            key_spec: Some("AES_128".to_string()),
            ..Default::default()
        }).await.unwrap();
        assert!(!resp.ciphertext_blob.is_empty());
    }

    #[tokio::test]
    async fn test_generate_random() {
        let state = make_state();
        let resp = state.generate_random(GenerateRandomRequest {
            number_of_bytes: Some(16),
        }).await.unwrap();
        assert!(!resp.plaintext.is_empty());
    }

    #[tokio::test]
    async fn test_generate_random_default_size() {
        let state = make_state();
        let resp = state.generate_random(GenerateRandomRequest::default()).await.unwrap();
        assert!(!resp.plaintext.is_empty());
    }

    #[tokio::test]
    async fn test_decrypt_without_explicit_key_id() {
        let state = make_state();
        let key_id = create_key(&state).await;

        let enc = state.encrypt(EncryptRequest {
            key_id: key_id.clone(),
            plaintext: "secret".to_string(),
            ..Default::default()
        }).await.unwrap();

        // Decrypt without specifying key_id -- key is embedded in ciphertext
        let dec = state.decrypt(DecryptRequest {
            ciphertext_blob: enc.ciphertext_blob,
            key_id: None,
            ..Default::default()
        }).await.unwrap();
        assert_eq!(dec.plaintext, "secret");
    }
}
