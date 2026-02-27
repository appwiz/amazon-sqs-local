use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::ShieldError;
use super::types::*;

#[allow(dead_code)]
struct ShieldStateInner {
    protections: HashMap<String, StoredProtection>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredProtection {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct ShieldState {
    inner: Arc<Mutex<ShieldStateInner>>,
}

impl ShieldState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        ShieldState {
            inner: Arc::new(Mutex::new(ShieldStateInner {
                protections: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_protection(&self, req: CreateProtectionRequest) -> Result<CreateProtectionResponse, ShieldError> {
        let mut state = self.inner.lock().await;
        let name = req.protection_name.clone();
        if state.protections.contains_key(&name) {
            return Err(ShieldError::ResourceAlreadyExistsException(format!("Protection {} already exists", name)));
        }
        let arn = format!("arn:aws:shield:{}:{}:protections/{}", state.region, state.account_id, name);
        state.protections.insert(name.clone(), StoredProtection {
            name: name.clone(),
            arn: arn.clone(),
            tags: req.tags.unwrap_or_default(),
        });
        Ok(CreateProtectionResponse {
            protection_arn: Some(arn),
            protection_name: Some(name),
        })
    }

    #[allow(dead_code)]
    pub async fn describe_protection(&self, req: DescribeProtectionRequest) -> Result<DescribeProtectionResponse, ShieldError> {
        let state = self.inner.lock().await;
        let name = req.protection_name.or_else(|| req.protection_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| ShieldError::ValidationException("Name or ARN required".to_string()))?;
        let stored = state.protections.get(&name)
            .ok_or_else(|| ShieldError::ResourceNotFoundException(format!("Protection {} not found", name)))?;
        Ok(DescribeProtectionResponse {
            protection: ProtectionDetail {
                protection_name: stored.name.clone(),
                protection_arn: stored.arn.clone(),
                status: "ACTIVE".to_string(),
                tags: Some(stored.tags.clone()),
            },
        })
    }

    #[allow(dead_code)]
    pub async fn list_protections(&self, _req: ListProtectionsRequest) -> Result<ListProtectionsResponse, ShieldError> {
        let state = self.inner.lock().await;
        let items: Vec<ProtectionDetail> = state.protections.values().map(|s| ProtectionDetail {
            protection_name: s.name.clone(),
            protection_arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListProtectionsResponse {
            protections: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_protection(&self, req: DeleteProtectionRequest) -> Result<(), ShieldError> {
        let mut state = self.inner.lock().await;
        let name = req.protection_name.or_else(|| req.protection_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| ShieldError::ValidationException("Name or ARN required".to_string()))?;
        state.protections.remove(&name)
            .ok_or_else(|| ShieldError::ResourceNotFoundException(format!("Protection {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = ShieldState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_protection() {
        let state = ShieldState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateProtectionRequest::default();
        let result = state.create_protection(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_describe_protection_not_found() {
        let state = ShieldState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DescribeProtectionRequest::default();
        let result = state.describe_protection(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_protections_empty() {
        let state = ShieldState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = ListProtectionsRequest::default();
        let result = state.list_protections(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_protection_not_found() {
        let state = ShieldState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DeleteProtectionRequest::default();
        let result = state.delete_protection(req).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_protection_create_and_list() {
        let state = ShieldState::new("123456789012".to_string(), "us-east-1".to_string());
        let create_req = CreateProtectionRequest::default();
        let _created = state.create_protection(create_req).await.unwrap();
        let list_req = ListProtectionsRequest::default();
        let listed = state.list_protections(list_req).await.unwrap();
        let _ = listed;
    }

    #[tokio::test]
    async fn test_protection_full_crud() {
        let state = ShieldState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateProtectionRequest::default();
        create_req.protection_name = "test-crud-resource".to_string();
        let create_result = state.create_protection(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let mut get_req = DescribeProtectionRequest::default();
        get_req.protection_name = Some("test-crud-resource".to_string());
        let get_result = state.describe_protection(get_req).await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let mut del_req = DeleteProtectionRequest::default();
        del_req.protection_name = Some("test-crud-resource".to_string());
        let del_result = state.delete_protection(del_req).await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
