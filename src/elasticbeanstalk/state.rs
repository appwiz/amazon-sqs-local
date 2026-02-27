use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::ElasticbeanstalkError;
use super::types::*;

#[allow(dead_code)]
struct ElasticbeanstalkStateInner {
    applications: HashMap<String, ApplicationInfo>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
pub struct ElasticbeanstalkState {
    inner: Arc<Mutex<ElasticbeanstalkStateInner>>,
}

impl ElasticbeanstalkState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        ElasticbeanstalkState {
            inner: Arc::new(Mutex::new(ElasticbeanstalkStateInner {
                applications: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_application(&self, name: String) -> Result<ApplicationInfo, ElasticbeanstalkError> {
        let mut state = self.inner.lock().await;
        if state.applications.contains_key(&name) {
            return Err(ElasticbeanstalkError::ResourceAlreadyExistsException(format!("Application {} already exists", name)));
        }
        let arn = format!("arn:aws:elasticbeanstalk:{}:{}:applications/{}", state.region, state.account_id, name);
        let info = ApplicationInfo {
            application_name: name.clone(),
            application_arn: arn,
            status: "ACTIVE".to_string(),
        };
        state.applications.insert(name, info.clone());
        Ok(info)
    }

    #[allow(dead_code)]
    pub async fn describe_application(&self, name: &str) -> Result<ApplicationInfo, ElasticbeanstalkError> {
        let state = self.inner.lock().await;
        state.applications.get(name).cloned()
            .ok_or_else(|| ElasticbeanstalkError::ResourceNotFoundException(format!("Application {} not found", name)))
    }

    #[allow(dead_code)]
    pub async fn list_applications(&self) -> Result<Vec<ApplicationInfo>, ElasticbeanstalkError> {
        let state = self.inner.lock().await;
        Ok(state.applications.values().cloned().collect())
    }

    #[allow(dead_code)]
    pub async fn delete_application(&self, name: &str) -> Result<(), ElasticbeanstalkError> {
        let mut state = self.inner.lock().await;
        state.applications.remove(name)
            .ok_or_else(|| ElasticbeanstalkError::ResourceNotFoundException(format!("Application {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = ElasticbeanstalkState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_application() {
        let state = ElasticbeanstalkState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.create_application("test-resource".to_string()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_application_duplicate() {
        let state = ElasticbeanstalkState::new("123456789012".to_string(), "us-east-1".to_string());
        state.create_application("dup".to_string()).await.unwrap();
        let result = state.create_application("dup".to_string()).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_describe_application_not_found() {
        let state = ElasticbeanstalkState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.describe_application("nonexistent").await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_applications() {
        let state = ElasticbeanstalkState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.list_applications().await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_application_not_found() {
        let state = ElasticbeanstalkState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.delete_application("nonexistent").await;
        assert!(result.is_err());
    }
}
