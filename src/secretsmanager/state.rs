use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;
use uuid::Uuid;

use super::error::SecretsManagerError;
use super::types::*;

struct SecretVersion {
    version_id: String,
    secret_string: Option<String>,
    secret_binary: Option<String>,
    version_stages: Vec<String>,
    created_date: f64,
}

struct Secret {
    name: String,
    arn: String,
    description: Option<String>,
    kms_key_id: Option<String>,
    tags: HashMap<String, String>,
    versions: Vec<SecretVersion>,
    current_version_id: String,
    created_date: f64,
    last_changed_date: f64,
    deleted: bool,
}

struct SecretsManagerStateInner {
    secrets: HashMap<String, Secret>,
    account_id: String,
    region: String,
}

pub struct SecretsManagerState {
    inner: Arc<Mutex<SecretsManagerStateInner>>,
}

impl SecretsManagerState {
    pub fn new(account_id: String, region: String) -> Self {
        SecretsManagerState {
            inner: Arc::new(Mutex::new(SecretsManagerStateInner {
                secrets: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    fn now() -> f64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64()
    }

    fn resolve<'a>(state: &'a SecretsManagerStateInner, secret_id: &'a str) -> Option<&'a str> {
        // By name
        if state.secrets.contains_key(secret_id) {
            return Some(secret_id);
        }
        // By ARN
        for (name, s) in &state.secrets {
            if s.arn == secret_id {
                return Some(name.as_str());
            }
        }
        None
    }

    pub async fn create_secret(
        &self,
        req: CreateSecretRequest,
    ) -> Result<CreateSecretResponse, SecretsManagerError> {
        let mut state = self.inner.lock().await;
        if state.secrets.contains_key(&req.name) {
            return Err(SecretsManagerError::ResourceExistsException(format!(
                "Secret {} already exists",
                req.name
            )));
        }
        let arn = format!(
            "arn:aws:secretsmanager:{}:{}:secret:{}-{}",
            state.region,
            state.account_id,
            req.name,
            &Uuid::new_v4().to_string()[..6]
        );
        let version_id = Uuid::new_v4().to_string();
        let now = Self::now();
        let mut tags = HashMap::new();
        if let Some(t) = req.tags {
            for tag in t {
                tags.insert(tag.key, tag.value);
            }
        }
        let version = SecretVersion {
            version_id: version_id.clone(),
            secret_string: req.secret_string,
            secret_binary: req.secret_binary,
            version_stages: vec!["AWSCURRENT".to_string()],
            created_date: now,
        };
        state.secrets.insert(req.name.clone(), Secret {
            name: req.name.clone(),
            arn: arn.clone(),
            description: req.description,
            kms_key_id: req.kms_key_id,
            tags,
            versions: vec![version],
            current_version_id: version_id.clone(),
            created_date: now,
            last_changed_date: now,
            deleted: false,
        });
        Ok(CreateSecretResponse {
            arn,
            name: req.name,
            version_id,
        })
    }

    pub async fn get_secret_value(
        &self,
        req: GetSecretValueRequest,
    ) -> Result<GetSecretValueResponse, SecretsManagerError> {
        let state = self.inner.lock().await;
        let resolved = Self::resolve(&state, &req.secret_id)
            .ok_or_else(|| SecretsManagerError::ResourceNotFoundException(format!(
                "Secrets Manager can't find the specified secret: {} for account {}", req.secret_id, state.account_id
            )))?
            .to_string();
        let secret = &state.secrets[&resolved];
        if secret.deleted {
            return Err(SecretsManagerError::InvalidRequestException(
                "Secret is scheduled for deletion".to_string(),
            ));
        }
        let version = if let Some(ref vid) = req.version_id {
            secret.versions.iter().find(|v| &v.version_id == vid)
                .ok_or_else(|| SecretsManagerError::ResourceNotFoundException("Version not found".to_string()))?
        } else if let Some(ref stage) = req.version_stage {
            secret.versions.iter().find(|v| v.version_stages.contains(stage))
                .ok_or_else(|| SecretsManagerError::ResourceNotFoundException("Version stage not found".to_string()))?
        } else {
            secret.versions.iter().find(|v| v.version_stages.contains(&"AWSCURRENT".to_string()))
                .ok_or_else(|| SecretsManagerError::ResourceNotFoundException("No current version found".to_string()))?
        };
        Ok(GetSecretValueResponse {
            arn: secret.arn.clone(),
            name: secret.name.clone(),
            version_id: version.version_id.clone(),
            secret_string: version.secret_string.clone(),
            secret_binary: version.secret_binary.clone(),
            version_stages: version.version_stages.clone(),
            created_date: version.created_date,
        })
    }

    pub async fn put_secret_value(
        &self,
        req: PutSecretValueRequest,
    ) -> Result<PutSecretValueResponse, SecretsManagerError> {
        let mut state = self.inner.lock().await;
        let resolved = Self::resolve(&state, &req.secret_id)
            .ok_or_else(|| SecretsManagerError::ResourceNotFoundException(format!(
                "Secrets Manager can't find the specified secret: {}", req.secret_id
            )))?
            .to_string();
        let now = Self::now();
        let version_id = req.client_request_token.unwrap_or_else(|| Uuid::new_v4().to_string());
        let stages = req.version_stages.unwrap_or_else(|| vec!["AWSCURRENT".to_string()]);
        let secret = state.secrets.get_mut(&resolved).unwrap();
        // Remove AWSCURRENT from previous version
        for v in secret.versions.iter_mut() {
            v.version_stages.retain(|s| s != "AWSCURRENT");
            if v.version_stages.is_empty() {
                v.version_stages.push("AWSPREVIOUS".to_string());
            }
        }
        let arn = secret.arn.clone();
        let name = secret.name.clone();
        secret.versions.push(SecretVersion {
            version_id: version_id.clone(),
            secret_string: req.secret_string,
            secret_binary: req.secret_binary,
            version_stages: stages.clone(),
            created_date: now,
        });
        secret.current_version_id = version_id.clone();
        secret.last_changed_date = now;
        Ok(PutSecretValueResponse {
            arn,
            name,
            version_id,
            version_stages: stages,
        })
    }

    pub async fn describe_secret(
        &self,
        req: DescribeSecretRequest,
    ) -> Result<DescribeSecretResponse, SecretsManagerError> {
        let state = self.inner.lock().await;
        let resolved = Self::resolve(&state, &req.secret_id)
            .ok_or_else(|| SecretsManagerError::ResourceNotFoundException(format!(
                "Secrets Manager can't find the specified secret: {}", req.secret_id
            )))?
            .to_string();
        let secret = &state.secrets[&resolved];
        let tags: Vec<Tag> = secret.tags.iter().map(|(k, v)| Tag {
            key: k.clone(),
            value: v.clone(),
        }).collect();
        let mut version_ids_to_stages: HashMap<String, Vec<String>> = HashMap::new();
        for v in &secret.versions {
            version_ids_to_stages.insert(v.version_id.clone(), v.version_stages.clone());
        }
        Ok(DescribeSecretResponse {
            arn: secret.arn.clone(),
            name: secret.name.clone(),
            description: secret.description.clone(),
            kms_key_id: secret.kms_key_id.clone(),
            rotation_enabled: false,
            tags,
            created_date: secret.created_date,
            last_changed_date: secret.last_changed_date,
            version_ids_to_stages,
        })
    }

    pub async fn list_secrets(
        &self,
        req: ListSecretsRequest,
    ) -> Result<ListSecretsResponse, SecretsManagerError> {
        let state = self.inner.lock().await;
        let mut secrets: Vec<SecretListEntry> = state.secrets.values()
            .filter(|s| !s.deleted)
            .map(|s| SecretListEntry {
                arn: s.arn.clone(),
                name: s.name.clone(),
                description: s.description.clone(),
                created_date: s.created_date,
                last_changed_date: s.last_changed_date,
                tags: s.tags.iter().map(|(k, v)| Tag { key: k.clone(), value: v.clone() }).collect(),
            })
            .collect();
        secrets.sort_by(|a, b| a.name.cmp(&b.name));
        let limit = req.max_results.unwrap_or(100);
        let truncated = secrets.len() > limit;
        secrets.truncate(limit);
        Ok(ListSecretsResponse {
            secret_list: secrets,
            next_token: if truncated { Some("next".to_string()) } else { None },
        })
    }

    pub async fn update_secret(
        &self,
        req: UpdateSecretRequest,
    ) -> Result<UpdateSecretResponse, SecretsManagerError> {
        let mut state = self.inner.lock().await;
        let resolved = Self::resolve(&state, &req.secret_id)
            .ok_or_else(|| SecretsManagerError::ResourceNotFoundException(format!(
                "Secrets Manager can't find the specified secret: {}", req.secret_id
            )))?
            .to_string();
        let now = Self::now();
        let secret = state.secrets.get_mut(&resolved).unwrap();
        if let Some(d) = req.description { secret.description = Some(d); }
        if let Some(k) = req.kms_key_id { secret.kms_key_id = Some(k); }
        let mut version_id = secret.current_version_id.clone();
        if req.secret_string.is_some() || req.secret_binary.is_some() {
            version_id = Uuid::new_v4().to_string();
            for v in secret.versions.iter_mut() {
                v.version_stages.retain(|s| s != "AWSCURRENT");
                if v.version_stages.is_empty() {
                    v.version_stages.push("AWSPREVIOUS".to_string());
                }
            }
            secret.versions.push(SecretVersion {
                version_id: version_id.clone(),
                secret_string: req.secret_string,
                secret_binary: req.secret_binary,
                version_stages: vec!["AWSCURRENT".to_string()],
                created_date: now,
            });
            secret.current_version_id = version_id.clone();
        }
        secret.last_changed_date = now;
        let arn = secret.arn.clone();
        let name = secret.name.clone();
        Ok(UpdateSecretResponse { arn, name, version_id })
    }

    pub async fn delete_secret(
        &self,
        req: DeleteSecretRequest,
    ) -> Result<DeleteSecretResponse, SecretsManagerError> {
        let mut state = self.inner.lock().await;
        let resolved = Self::resolve(&state, &req.secret_id)
            .ok_or_else(|| SecretsManagerError::ResourceNotFoundException(format!(
                "Secrets Manager can't find the specified secret: {}", req.secret_id
            )))?
            .to_string();
        let now = Self::now();
        let force = req.force_delete_without_recovery.unwrap_or(false);
        let days = req.recovery_window_in_days.unwrap_or(30);
        let deletion_date = now + (days as f64 * 86400.0);

        let secret = state.secrets.get_mut(&resolved).unwrap();
        let arn = secret.arn.clone();
        let name = secret.name.clone();
        if force {
            state.secrets.remove(&resolved);
        } else {
            secret.deleted = true;
        }
        Ok(DeleteSecretResponse {
            arn,
            name,
            deletion_date,
        })
    }

    pub async fn restore_secret(
        &self,
        req: RestoreSecretRequest,
    ) -> Result<RestoreSecretResponse, SecretsManagerError> {
        let mut state = self.inner.lock().await;
        let resolved = Self::resolve(&state, &req.secret_id)
            .ok_or_else(|| SecretsManagerError::ResourceNotFoundException(format!(
                "Secrets Manager can't find the specified secret: {}", req.secret_id
            )))?
            .to_string();
        let secret = state.secrets.get_mut(&resolved).unwrap();
        secret.deleted = false;
        Ok(RestoreSecretResponse {
            arn: secret.arn.clone(),
            name: secret.name.clone(),
        })
    }

    pub async fn tag_resource(&self, req: TagResourceRequest) -> Result<(), SecretsManagerError> {
        let mut state = self.inner.lock().await;
        let resolved = Self::resolve(&state, &req.secret_id)
            .ok_or_else(|| SecretsManagerError::ResourceNotFoundException(format!(
                "Secrets Manager can't find the specified secret: {}", req.secret_id
            )))?
            .to_string();
        let secret = state.secrets.get_mut(&resolved).unwrap();
        for tag in req.tags {
            secret.tags.insert(tag.key, tag.value);
        }
        Ok(())
    }

    pub async fn untag_resource(&self, req: UntagResourceRequest) -> Result<(), SecretsManagerError> {
        let mut state = self.inner.lock().await;
        let resolved = Self::resolve(&state, &req.secret_id)
            .ok_or_else(|| SecretsManagerError::ResourceNotFoundException(format!(
                "Secrets Manager can't find the specified secret: {}", req.secret_id
            )))?
            .to_string();
        let secret = state.secrets.get_mut(&resolved).unwrap();
        for key in &req.tag_keys {
            secret.tags.remove(key);
        }
        Ok(())
    }

    pub async fn list_secret_version_ids(
        &self,
        req: ListSecretVersionIdsRequest,
    ) -> Result<ListSecretVersionIdsResponse, SecretsManagerError> {
        let state = self.inner.lock().await;
        let resolved = Self::resolve(&state, &req.secret_id)
            .ok_or_else(|| SecretsManagerError::ResourceNotFoundException(format!(
                "Secrets Manager can't find the specified secret: {}", req.secret_id
            )))?
            .to_string();
        let secret = &state.secrets[&resolved];
        let versions: Vec<SecretVersionEntry> = secret.versions.iter().map(|v| SecretVersionEntry {
            version_id: v.version_id.clone(),
            version_stages: v.version_stages.clone(),
            created_date: v.created_date,
        }).collect();
        Ok(ListSecretVersionIdsResponse {
            arn: secret.arn.clone(),
            name: secret.name.clone(),
            versions,
        })
    }
}
