use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::BackupError;
use super::types::*;

#[allow(dead_code)]
struct BackupStateInner {
    backup_vaults: HashMap<String, StoredBackupVault>,
    backup_plans: HashMap<String, StoredBackupPlan>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredBackupVault {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
struct StoredBackupPlan {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct BackupState {
    inner: Arc<Mutex<BackupStateInner>>,
}

impl BackupState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        BackupState {
            inner: Arc::new(Mutex::new(BackupStateInner {
                backup_vaults: HashMap::new(),
                backup_plans: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_backup_vault(&self, req: CreateBackupVaultRequest) -> Result<BackupVaultDetail, BackupError> {
        let mut state = self.inner.lock().await;
        let name = req.name.or(req.name_pascal).unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        if state.backup_vaults.contains_key(&name) {
            return Err(BackupError::ResourceAlreadyExistsException(format!("BackupVault {} already exists", name)));
        }
        let arn = format!("arn:aws:backup:{}:{}:backup-vaults/{}", state.region, state.account_id, name);
        let detail = BackupVaultDetail {
            name: name.clone(),
            arn: arn.clone(),
            status: "ACTIVE".to_string(),
            tags: req.tags.clone(),
        };
        state.backup_vaults.insert(name, StoredBackupVault {
            name: detail.name.clone(),
            arn,
            tags: req.tags.unwrap_or_default(),
        });
        Ok(detail)
    }

    #[allow(dead_code)]
    pub async fn get_backup_vault(&self, name: &str) -> Result<BackupVaultDetail, BackupError> {
        let state = self.inner.lock().await;
        let stored = state.backup_vaults.get(name)
            .ok_or_else(|| BackupError::ResourceNotFoundException(format!("BackupVault {} not found", name)))?;
        Ok(BackupVaultDetail {
            name: stored.name.clone(),
            arn: stored.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(stored.tags.clone()),
        })
    }

    #[allow(dead_code)]
    pub async fn list_backup_vaults(&self) -> Result<ListBackupVaultsResponse, BackupError> {
        let state = self.inner.lock().await;
        let items: Vec<BackupVaultDetail> = state.backup_vaults.values().map(|s| BackupVaultDetail {
            name: s.name.clone(),
            arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListBackupVaultsResponse {
            items: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_backup_vault(&self, name: &str) -> Result<(), BackupError> {
        let mut state = self.inner.lock().await;
        state.backup_vaults.remove(name)
            .ok_or_else(|| BackupError::ResourceNotFoundException(format!("BackupVault {} not found", name)))?;
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn create_backup_plan(&self, req: CreateBackupPlanRequest) -> Result<BackupPlanDetail, BackupError> {
        let mut state = self.inner.lock().await;
        let name = req.name.or(req.name_pascal).unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        if state.backup_plans.contains_key(&name) {
            return Err(BackupError::ResourceAlreadyExistsException(format!("BackupPlan {} already exists", name)));
        }
        let arn = format!("arn:aws:backup:{}:{}:backup-plans/{}", state.region, state.account_id, name);
        let detail = BackupPlanDetail {
            name: name.clone(),
            arn: arn.clone(),
            status: "ACTIVE".to_string(),
            tags: req.tags.clone(),
        };
        state.backup_plans.insert(name, StoredBackupPlan {
            name: detail.name.clone(),
            arn,
            tags: req.tags.unwrap_or_default(),
        });
        Ok(detail)
    }

    #[allow(dead_code)]
    pub async fn get_backup_plan(&self, name: &str) -> Result<BackupPlanDetail, BackupError> {
        let state = self.inner.lock().await;
        let stored = state.backup_plans.get(name)
            .ok_or_else(|| BackupError::ResourceNotFoundException(format!("BackupPlan {} not found", name)))?;
        Ok(BackupPlanDetail {
            name: stored.name.clone(),
            arn: stored.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(stored.tags.clone()),
        })
    }

    #[allow(dead_code)]
    pub async fn list_backup_plans(&self) -> Result<ListBackupPlansResponse, BackupError> {
        let state = self.inner.lock().await;
        let items: Vec<BackupPlanDetail> = state.backup_plans.values().map(|s| BackupPlanDetail {
            name: s.name.clone(),
            arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListBackupPlansResponse {
            items: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_backup_plan(&self, name: &str) -> Result<(), BackupError> {
        let mut state = self.inner.lock().await;
        state.backup_plans.remove(name)
            .ok_or_else(|| BackupError::ResourceNotFoundException(format!("BackupPlan {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = BackupState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_backup_vault() {
        let state = BackupState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateBackupVaultRequest::default();
        let result = state.create_backup_vault(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_get_backup_vault_not_found() {
        let state = BackupState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.get_backup_vault("nonexistent").await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_backup_vaults() {
        let state = BackupState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.list_backup_vaults().await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_backup_vault_not_found() {
        let state = BackupState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.delete_backup_vault("nonexistent").await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_create_backup_plan() {
        let state = BackupState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateBackupPlanRequest::default();
        let result = state.create_backup_plan(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_get_backup_plan_not_found() {
        let state = BackupState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.get_backup_plan("nonexistent").await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_backup_plans() {
        let state = BackupState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.list_backup_plans().await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_backup_plan_not_found() {
        let state = BackupState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.delete_backup_plan("nonexistent").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_backup_vault_full_crud() {
        let state = BackupState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateBackupVaultRequest::default();
        create_req.name = Some("test-crud-resource".to_string());
        let create_result = state.create_backup_vault(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let get_result = state.get_backup_vault("test-crud-resource").await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let del_result = state.delete_backup_vault("test-crud-resource").await;
        assert!(del_result.is_ok(), "delete should succeed");
    }

    #[tokio::test]
    async fn test_backup_plan_full_crud() {
        let state = BackupState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateBackupPlanRequest::default();
        create_req.name = Some("test-crud-resource".to_string());
        let create_result = state.create_backup_plan(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let get_result = state.get_backup_plan("test-crud-resource").await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let del_result = state.delete_backup_plan("test-crud-resource").await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
