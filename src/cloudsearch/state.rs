use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::CloudsearchError;
use super::types::*;

#[allow(dead_code)]
struct CloudsearchStateInner {
    domains: HashMap<String, DomainInfo>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
pub struct CloudsearchState {
    inner: Arc<Mutex<CloudsearchStateInner>>,
}

impl CloudsearchState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        CloudsearchState {
            inner: Arc::new(Mutex::new(CloudsearchStateInner {
                domains: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_domain(&self, name: String) -> Result<DomainInfo, CloudsearchError> {
        let mut state = self.inner.lock().await;
        if state.domains.contains_key(&name) {
            return Err(CloudsearchError::ResourceAlreadyExistsException(format!("Domain {} already exists", name)));
        }
        let arn = format!("arn:aws:cloudsearch:{}:{}:domains/{}", state.region, state.account_id, name);
        let info = DomainInfo {
            domain_name: name.clone(),
            domain_arn: arn,
            status: "ACTIVE".to_string(),
        };
        state.domains.insert(name, info.clone());
        Ok(info)
    }

    #[allow(dead_code)]
    pub async fn describe_domain(&self, name: &str) -> Result<DomainInfo, CloudsearchError> {
        let state = self.inner.lock().await;
        state.domains.get(name).cloned()
            .ok_or_else(|| CloudsearchError::ResourceNotFoundException(format!("Domain {} not found", name)))
    }

    #[allow(dead_code)]
    pub async fn list_domains(&self) -> Result<Vec<DomainInfo>, CloudsearchError> {
        let state = self.inner.lock().await;
        Ok(state.domains.values().cloned().collect())
    }

    #[allow(dead_code)]
    pub async fn delete_domain(&self, name: &str) -> Result<(), CloudsearchError> {
        let mut state = self.inner.lock().await;
        state.domains.remove(name)
            .ok_or_else(|| CloudsearchError::ResourceNotFoundException(format!("Domain {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = CloudsearchState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_domain() {
        let state = CloudsearchState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.create_domain("test-resource".to_string()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_domain_duplicate() {
        let state = CloudsearchState::new("123456789012".to_string(), "us-east-1".to_string());
        state.create_domain("dup".to_string()).await.unwrap();
        let result = state.create_domain("dup".to_string()).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_describe_domain_not_found() {
        let state = CloudsearchState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.describe_domain("nonexistent").await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_domains() {
        let state = CloudsearchState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.list_domains().await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_domain_not_found() {
        let state = CloudsearchState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.delete_domain("nonexistent").await;
        assert!(result.is_err());
    }
}
