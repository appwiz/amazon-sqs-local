use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::AutoscalingError;
use super::types::*;

#[allow(dead_code)]
struct AutoscalingStateInner {
    auto_scaling_groups: HashMap<String, AutoScalingGroupInfo>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
pub struct AutoscalingState {
    inner: Arc<Mutex<AutoscalingStateInner>>,
}

impl AutoscalingState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        AutoscalingState {
            inner: Arc::new(Mutex::new(AutoscalingStateInner {
                auto_scaling_groups: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_auto_scaling_group(&self, name: String) -> Result<AutoScalingGroupInfo, AutoscalingError> {
        let mut state = self.inner.lock().await;
        if state.auto_scaling_groups.contains_key(&name) {
            return Err(AutoscalingError::ResourceAlreadyExistsException(format!("AutoScalingGroup {} already exists", name)));
        }
        let arn = format!("arn:aws:autoscaling:{}:{}:auto-scaling-groups/{}", state.region, state.account_id, name);
        let info = AutoScalingGroupInfo {
            auto_scaling_group_name: name.clone(),
            auto_scaling_group_arn: arn,
            status: "ACTIVE".to_string(),
        };
        state.auto_scaling_groups.insert(name, info.clone());
        Ok(info)
    }

    #[allow(dead_code)]
    pub async fn describe_auto_scaling_group(&self, name: &str) -> Result<AutoScalingGroupInfo, AutoscalingError> {
        let state = self.inner.lock().await;
        state.auto_scaling_groups.get(name).cloned()
            .ok_or_else(|| AutoscalingError::ResourceNotFoundException(format!("AutoScalingGroup {} not found", name)))
    }

    #[allow(dead_code)]
    pub async fn list_auto_scaling_groups(&self) -> Result<Vec<AutoScalingGroupInfo>, AutoscalingError> {
        let state = self.inner.lock().await;
        Ok(state.auto_scaling_groups.values().cloned().collect())
    }

    #[allow(dead_code)]
    pub async fn delete_auto_scaling_group(&self, name: &str) -> Result<(), AutoscalingError> {
        let mut state = self.inner.lock().await;
        state.auto_scaling_groups.remove(name)
            .ok_or_else(|| AutoscalingError::ResourceNotFoundException(format!("AutoScalingGroup {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = AutoscalingState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_auto_scaling_group() {
        let state = AutoscalingState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.create_auto_scaling_group("test-resource".to_string()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_auto_scaling_group_duplicate() {
        let state = AutoscalingState::new("123456789012".to_string(), "us-east-1".to_string());
        state.create_auto_scaling_group("dup".to_string()).await.unwrap();
        let result = state.create_auto_scaling_group("dup".to_string()).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_describe_auto_scaling_group_not_found() {
        let state = AutoscalingState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.describe_auto_scaling_group("nonexistent").await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_auto_scaling_groups() {
        let state = AutoscalingState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.list_auto_scaling_groups().await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_auto_scaling_group_not_found() {
        let state = AutoscalingState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.delete_auto_scaling_group("nonexistent").await;
        assert!(result.is_err());
    }
}
