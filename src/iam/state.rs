use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::IAMError;
use super::types::*;

#[allow(dead_code)]
struct IAMStateInner {
    users: HashMap<String, UserInfo>,
    roles: HashMap<String, RoleInfo>,
    policys: HashMap<String, PolicyInfo>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
pub struct IAMState {
    inner: Arc<Mutex<IAMStateInner>>,
}

impl IAMState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        IAMState {
            inner: Arc::new(Mutex::new(IAMStateInner {
                users: HashMap::new(),
                roles: HashMap::new(),
                policys: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_user(&self, name: String) -> Result<UserInfo, IAMError> {
        let mut state = self.inner.lock().await;
        if state.users.contains_key(&name) {
            return Err(IAMError::ResourceAlreadyExistsException(format!("User {} already exists", name)));
        }
        let arn = format!("arn:aws:iam:{}:{}:users/{}", state.region, state.account_id, name);
        let info = UserInfo {
            user_name: name.clone(),
            user_arn: arn,
            status: "ACTIVE".to_string(),
        };
        state.users.insert(name, info.clone());
        Ok(info)
    }

    #[allow(dead_code)]
    pub async fn describe_user(&self, name: &str) -> Result<UserInfo, IAMError> {
        let state = self.inner.lock().await;
        state.users.get(name).cloned()
            .ok_or_else(|| IAMError::ResourceNotFoundException(format!("User {} not found", name)))
    }

    #[allow(dead_code)]
    pub async fn list_users(&self) -> Result<Vec<UserInfo>, IAMError> {
        let state = self.inner.lock().await;
        Ok(state.users.values().cloned().collect())
    }

    #[allow(dead_code)]
    pub async fn delete_user(&self, name: &str) -> Result<(), IAMError> {
        let mut state = self.inner.lock().await;
        state.users.remove(name)
            .ok_or_else(|| IAMError::ResourceNotFoundException(format!("User {} not found", name)))?;
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn create_role(&self, name: String) -> Result<RoleInfo, IAMError> {
        let mut state = self.inner.lock().await;
        if state.roles.contains_key(&name) {
            return Err(IAMError::ResourceAlreadyExistsException(format!("Role {} already exists", name)));
        }
        let arn = format!("arn:aws:iam:{}:{}:roles/{}", state.region, state.account_id, name);
        let info = RoleInfo {
            role_name: name.clone(),
            role_arn: arn,
            status: "ACTIVE".to_string(),
        };
        state.roles.insert(name, info.clone());
        Ok(info)
    }

    #[allow(dead_code)]
    pub async fn describe_role(&self, name: &str) -> Result<RoleInfo, IAMError> {
        let state = self.inner.lock().await;
        state.roles.get(name).cloned()
            .ok_or_else(|| IAMError::ResourceNotFoundException(format!("Role {} not found", name)))
    }

    #[allow(dead_code)]
    pub async fn list_roles(&self) -> Result<Vec<RoleInfo>, IAMError> {
        let state = self.inner.lock().await;
        Ok(state.roles.values().cloned().collect())
    }

    #[allow(dead_code)]
    pub async fn delete_role(&self, name: &str) -> Result<(), IAMError> {
        let mut state = self.inner.lock().await;
        state.roles.remove(name)
            .ok_or_else(|| IAMError::ResourceNotFoundException(format!("Role {} not found", name)))?;
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn create_policy(&self, name: String) -> Result<PolicyInfo, IAMError> {
        let mut state = self.inner.lock().await;
        if state.policys.contains_key(&name) {
            return Err(IAMError::ResourceAlreadyExistsException(format!("Policy {} already exists", name)));
        }
        let arn = format!("arn:aws:iam:{}:{}:policies/{}", state.region, state.account_id, name);
        let info = PolicyInfo {
            policy_name: name.clone(),
            policy_arn: arn,
            status: "ACTIVE".to_string(),
        };
        state.policys.insert(name, info.clone());
        Ok(info)
    }

    #[allow(dead_code)]
    pub async fn describe_policy(&self, name: &str) -> Result<PolicyInfo, IAMError> {
        let state = self.inner.lock().await;
        state.policys.get(name).cloned()
            .ok_or_else(|| IAMError::ResourceNotFoundException(format!("Policy {} not found", name)))
    }

    #[allow(dead_code)]
    pub async fn list_policys(&self) -> Result<Vec<PolicyInfo>, IAMError> {
        let state = self.inner.lock().await;
        Ok(state.policys.values().cloned().collect())
    }

    #[allow(dead_code)]
    pub async fn delete_policy(&self, name: &str) -> Result<(), IAMError> {
        let mut state = self.inner.lock().await;
        state.policys.remove(name)
            .ok_or_else(|| IAMError::ResourceNotFoundException(format!("Policy {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = IAMState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_user() {
        let state = IAMState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.create_user("test-resource".to_string()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_user_duplicate() {
        let state = IAMState::new("123456789012".to_string(), "us-east-1".to_string());
        state.create_user("dup".to_string()).await.unwrap();
        let result = state.create_user("dup".to_string()).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_describe_user_not_found() {
        let state = IAMState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.describe_user("nonexistent").await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_users() {
        let state = IAMState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.list_users().await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_user_not_found() {
        let state = IAMState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.delete_user("nonexistent").await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_create_role() {
        let state = IAMState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.create_role("test-resource".to_string()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_role_duplicate() {
        let state = IAMState::new("123456789012".to_string(), "us-east-1".to_string());
        state.create_role("dup".to_string()).await.unwrap();
        let result = state.create_role("dup".to_string()).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_describe_role_not_found() {
        let state = IAMState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.describe_role("nonexistent").await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_roles() {
        let state = IAMState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.list_roles().await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_role_not_found() {
        let state = IAMState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.delete_role("nonexistent").await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_create_policy() {
        let state = IAMState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.create_policy("test-resource".to_string()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_policy_duplicate() {
        let state = IAMState::new("123456789012".to_string(), "us-east-1".to_string());
        state.create_policy("dup".to_string()).await.unwrap();
        let result = state.create_policy("dup".to_string()).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_describe_policy_not_found() {
        let state = IAMState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.describe_policy("nonexistent").await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_policys() {
        let state = IAMState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.list_policys().await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_policy_not_found() {
        let state = IAMState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.delete_policy("nonexistent").await;
        assert!(result.is_err());
    }
}
