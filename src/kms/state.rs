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
