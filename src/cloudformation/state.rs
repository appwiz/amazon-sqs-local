use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::CloudformationError;
use super::types::*;

#[allow(dead_code)]
struct CloudformationStateInner {
    stacks: HashMap<String, StackInfo>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
pub struct CloudformationState {
    inner: Arc<Mutex<CloudformationStateInner>>,
}

impl CloudformationState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        CloudformationState {
            inner: Arc::new(Mutex::new(CloudformationStateInner {
                stacks: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_stack(&self, name: String) -> Result<StackInfo, CloudformationError> {
        let mut state = self.inner.lock().await;
        if state.stacks.contains_key(&name) {
            return Err(CloudformationError::ResourceAlreadyExistsException(format!("Stack {} already exists", name)));
        }
        let arn = format!("arn:aws:cloudformation:{}:{}:stacks/{}", state.region, state.account_id, name);
        let info = StackInfo {
            stack_name: name.clone(),
            stack_arn: arn,
            status: "ACTIVE".to_string(),
        };
        state.stacks.insert(name, info.clone());
        Ok(info)
    }

    #[allow(dead_code)]
    pub async fn describe_stack(&self, name: &str) -> Result<StackInfo, CloudformationError> {
        let state = self.inner.lock().await;
        state.stacks.get(name).cloned()
            .ok_or_else(|| CloudformationError::ResourceNotFoundException(format!("Stack {} not found", name)))
    }

    #[allow(dead_code)]
    pub async fn list_stacks(&self) -> Result<Vec<StackInfo>, CloudformationError> {
        let state = self.inner.lock().await;
        Ok(state.stacks.values().cloned().collect())
    }

    #[allow(dead_code)]
    pub async fn delete_stack(&self, name: &str) -> Result<(), CloudformationError> {
        let mut state = self.inner.lock().await;
        state.stacks.remove(name)
            .ok_or_else(|| CloudformationError::ResourceNotFoundException(format!("Stack {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = CloudformationState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_stack() {
        let state = CloudformationState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.create_stack("test-resource".to_string()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_stack_duplicate() {
        let state = CloudformationState::new("123456789012".to_string(), "us-east-1".to_string());
        state.create_stack("dup".to_string()).await.unwrap();
        let result = state.create_stack("dup".to_string()).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_describe_stack_not_found() {
        let state = CloudformationState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.describe_stack("nonexistent").await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_stacks() {
        let state = CloudformationState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.list_stacks().await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_stack_not_found() {
        let state = CloudformationState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.delete_stack("nonexistent").await;
        assert!(result.is_err());
    }
}
