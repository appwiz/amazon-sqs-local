use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::ELBError;
use super::types::*;

#[allow(dead_code)]
struct ELBStateInner {
    load_balancers: HashMap<String, LoadBalancerInfo>,
    target_groups: HashMap<String, TargetGroupInfo>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
pub struct ELBState {
    inner: Arc<Mutex<ELBStateInner>>,
}

impl ELBState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        ELBState {
            inner: Arc::new(Mutex::new(ELBStateInner {
                load_balancers: HashMap::new(),
                target_groups: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_load_balancer(&self, name: String) -> Result<LoadBalancerInfo, ELBError> {
        let mut state = self.inner.lock().await;
        if state.load_balancers.contains_key(&name) {
            return Err(ELBError::ResourceAlreadyExistsException(format!("LoadBalancer {} already exists", name)));
        }
        let arn = format!("arn:aws:elasticloadbalancing:{}:{}:load-balancers/{}", state.region, state.account_id, name);
        let info = LoadBalancerInfo {
            load_balancer_name: name.clone(),
            load_balancer_arn: arn,
            status: "ACTIVE".to_string(),
        };
        state.load_balancers.insert(name, info.clone());
        Ok(info)
    }

    #[allow(dead_code)]
    pub async fn describe_load_balancer(&self, name: &str) -> Result<LoadBalancerInfo, ELBError> {
        let state = self.inner.lock().await;
        state.load_balancers.get(name).cloned()
            .ok_or_else(|| ELBError::ResourceNotFoundException(format!("LoadBalancer {} not found", name)))
    }

    #[allow(dead_code)]
    pub async fn list_load_balancers(&self) -> Result<Vec<LoadBalancerInfo>, ELBError> {
        let state = self.inner.lock().await;
        Ok(state.load_balancers.values().cloned().collect())
    }

    #[allow(dead_code)]
    pub async fn delete_load_balancer(&self, name: &str) -> Result<(), ELBError> {
        let mut state = self.inner.lock().await;
        state.load_balancers.remove(name)
            .ok_or_else(|| ELBError::ResourceNotFoundException(format!("LoadBalancer {} not found", name)))?;
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn create_target_group(&self, name: String) -> Result<TargetGroupInfo, ELBError> {
        let mut state = self.inner.lock().await;
        if state.target_groups.contains_key(&name) {
            return Err(ELBError::ResourceAlreadyExistsException(format!("TargetGroup {} already exists", name)));
        }
        let arn = format!("arn:aws:elasticloadbalancing:{}:{}:target-groups/{}", state.region, state.account_id, name);
        let info = TargetGroupInfo {
            target_group_name: name.clone(),
            target_group_arn: arn,
            status: "ACTIVE".to_string(),
        };
        state.target_groups.insert(name, info.clone());
        Ok(info)
    }

    #[allow(dead_code)]
    pub async fn describe_target_group(&self, name: &str) -> Result<TargetGroupInfo, ELBError> {
        let state = self.inner.lock().await;
        state.target_groups.get(name).cloned()
            .ok_or_else(|| ELBError::ResourceNotFoundException(format!("TargetGroup {} not found", name)))
    }

    #[allow(dead_code)]
    pub async fn list_target_groups(&self) -> Result<Vec<TargetGroupInfo>, ELBError> {
        let state = self.inner.lock().await;
        Ok(state.target_groups.values().cloned().collect())
    }

    #[allow(dead_code)]
    pub async fn delete_target_group(&self, name: &str) -> Result<(), ELBError> {
        let mut state = self.inner.lock().await;
        state.target_groups.remove(name)
            .ok_or_else(|| ELBError::ResourceNotFoundException(format!("TargetGroup {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = ELBState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_load_balancer() {
        let state = ELBState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.create_load_balancer("test-resource".to_string()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_load_balancer_duplicate() {
        let state = ELBState::new("123456789012".to_string(), "us-east-1".to_string());
        state.create_load_balancer("dup".to_string()).await.unwrap();
        let result = state.create_load_balancer("dup".to_string()).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_describe_load_balancer_not_found() {
        let state = ELBState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.describe_load_balancer("nonexistent").await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_load_balancers() {
        let state = ELBState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.list_load_balancers().await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_load_balancer_not_found() {
        let state = ELBState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.delete_load_balancer("nonexistent").await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_create_target_group() {
        let state = ELBState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.create_target_group("test-resource".to_string()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_target_group_duplicate() {
        let state = ELBState::new("123456789012".to_string(), "us-east-1".to_string());
        state.create_target_group("dup".to_string()).await.unwrap();
        let result = state.create_target_group("dup".to_string()).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_describe_target_group_not_found() {
        let state = ELBState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.describe_target_group("nonexistent").await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_target_groups() {
        let state = ELBState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.list_target_groups().await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_target_group_not_found() {
        let state = ELBState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.delete_target_group("nonexistent").await;
        assert!(result.is_err());
    }
}
