use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::B2biError;
use super::types::*;

#[allow(dead_code)]
struct B2biStateInner {
    profiles: HashMap<String, StoredProfile>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredProfile {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct B2biState {
    inner: Arc<Mutex<B2biStateInner>>,
}

impl B2biState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        B2biState {
            inner: Arc::new(Mutex::new(B2biStateInner {
                profiles: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_profile(&self, req: CreateProfileRequest) -> Result<ProfileDetail, B2biError> {
        let mut state = self.inner.lock().await;
        let name = req.name.or(req.name_pascal).unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        if state.profiles.contains_key(&name) {
            return Err(B2biError::ResourceAlreadyExistsException(format!("Profile {} already exists", name)));
        }
        let arn = format!("arn:aws:b2bi:{}:{}:profiles/{}", state.region, state.account_id, name);
        let detail = ProfileDetail {
            name: name.clone(),
            arn: arn.clone(),
            status: "ACTIVE".to_string(),
            tags: req.tags.clone(),
        };
        state.profiles.insert(name, StoredProfile {
            name: detail.name.clone(),
            arn,
            tags: req.tags.unwrap_or_default(),
        });
        Ok(detail)
    }

    #[allow(dead_code)]
    pub async fn get_profile(&self, name: &str) -> Result<ProfileDetail, B2biError> {
        let state = self.inner.lock().await;
        let stored = state.profiles.get(name)
            .ok_or_else(|| B2biError::ResourceNotFoundException(format!("Profile {} not found", name)))?;
        Ok(ProfileDetail {
            name: stored.name.clone(),
            arn: stored.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(stored.tags.clone()),
        })
    }

    #[allow(dead_code)]
    pub async fn list_profiles(&self) -> Result<ListProfilesResponse, B2biError> {
        let state = self.inner.lock().await;
        let items: Vec<ProfileDetail> = state.profiles.values().map(|s| ProfileDetail {
            name: s.name.clone(),
            arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListProfilesResponse {
            items: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_profile(&self, name: &str) -> Result<(), B2biError> {
        let mut state = self.inner.lock().await;
        state.profiles.remove(name)
            .ok_or_else(|| B2biError::ResourceNotFoundException(format!("Profile {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = B2biState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_profile() {
        let state = B2biState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateProfileRequest::default();
        let result = state.create_profile(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_get_profile_not_found() {
        let state = B2biState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.get_profile("nonexistent").await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_profiles() {
        let state = B2biState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.list_profiles().await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_profile_not_found() {
        let state = B2biState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.delete_profile("nonexistent").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_profile_full_crud() {
        let state = B2biState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateProfileRequest::default();
        create_req.name = Some("test-crud-resource".to_string());
        let create_result = state.create_profile(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let get_result = state.get_profile("test-crud-resource").await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let del_result = state.delete_profile("test-crud-resource").await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
