use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::NetworkfirewallError;
use super::types::*;

#[allow(dead_code)]
struct NetworkfirewallStateInner {
    firewalls: HashMap<String, StoredFirewall>,
    firewall_policys: HashMap<String, StoredFirewallPolicy>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredFirewall {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
struct StoredFirewallPolicy {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct NetworkfirewallState {
    inner: Arc<Mutex<NetworkfirewallStateInner>>,
}

impl NetworkfirewallState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        NetworkfirewallState {
            inner: Arc::new(Mutex::new(NetworkfirewallStateInner {
                firewalls: HashMap::new(),
                firewall_policys: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_firewall(&self, req: CreateFirewallRequest) -> Result<CreateFirewallResponse, NetworkfirewallError> {
        let mut state = self.inner.lock().await;
        let name = req.firewall_name.clone();
        if state.firewalls.contains_key(&name) {
            return Err(NetworkfirewallError::ResourceAlreadyExistsException(format!("Firewall {} already exists", name)));
        }
        let arn = format!("arn:aws:network-firewall:{}:{}:firewalls/{}", state.region, state.account_id, name);
        state.firewalls.insert(name.clone(), StoredFirewall {
            name: name.clone(),
            arn: arn.clone(),
            tags: req.tags.unwrap_or_default(),
        });
        Ok(CreateFirewallResponse {
            firewall_arn: Some(arn),
            firewall_name: Some(name),
        })
    }

    #[allow(dead_code)]
    pub async fn describe_firewall(&self, req: DescribeFirewallRequest) -> Result<DescribeFirewallResponse, NetworkfirewallError> {
        let state = self.inner.lock().await;
        let name = req.firewall_name.or_else(|| req.firewall_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| NetworkfirewallError::ValidationException("Name or ARN required".to_string()))?;
        let stored = state.firewalls.get(&name)
            .ok_or_else(|| NetworkfirewallError::ResourceNotFoundException(format!("Firewall {} not found", name)))?;
        Ok(DescribeFirewallResponse {
            firewall: FirewallDetail {
                firewall_name: stored.name.clone(),
                firewall_arn: stored.arn.clone(),
                status: "ACTIVE".to_string(),
                tags: Some(stored.tags.clone()),
            },
        })
    }

    #[allow(dead_code)]
    pub async fn list_firewalls(&self, _req: ListFirewallsRequest) -> Result<ListFirewallsResponse, NetworkfirewallError> {
        let state = self.inner.lock().await;
        let items: Vec<FirewallDetail> = state.firewalls.values().map(|s| FirewallDetail {
            firewall_name: s.name.clone(),
            firewall_arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListFirewallsResponse {
            firewalls: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_firewall(&self, req: DeleteFirewallRequest) -> Result<(), NetworkfirewallError> {
        let mut state = self.inner.lock().await;
        let name = req.firewall_name.or_else(|| req.firewall_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| NetworkfirewallError::ValidationException("Name or ARN required".to_string()))?;
        state.firewalls.remove(&name)
            .ok_or_else(|| NetworkfirewallError::ResourceNotFoundException(format!("Firewall {} not found", name)))?;
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn create_firewall_policy(&self, req: CreateFirewallPolicyRequest) -> Result<CreateFirewallPolicyResponse, NetworkfirewallError> {
        let mut state = self.inner.lock().await;
        let name = req.firewall_policy_name.clone();
        if state.firewall_policys.contains_key(&name) {
            return Err(NetworkfirewallError::ResourceAlreadyExistsException(format!("FirewallPolicy {} already exists", name)));
        }
        let arn = format!("arn:aws:network-firewall:{}:{}:firewall-policies/{}", state.region, state.account_id, name);
        state.firewall_policys.insert(name.clone(), StoredFirewallPolicy {
            name: name.clone(),
            arn: arn.clone(),
            tags: req.tags.unwrap_or_default(),
        });
        Ok(CreateFirewallPolicyResponse {
            firewall_policy_arn: Some(arn),
            firewall_policy_name: Some(name),
        })
    }

    #[allow(dead_code)]
    pub async fn describe_firewall_policy(&self, req: DescribeFirewallPolicyRequest) -> Result<DescribeFirewallPolicyResponse, NetworkfirewallError> {
        let state = self.inner.lock().await;
        let name = req.firewall_policy_name.or_else(|| req.firewall_policy_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| NetworkfirewallError::ValidationException("Name or ARN required".to_string()))?;
        let stored = state.firewall_policys.get(&name)
            .ok_or_else(|| NetworkfirewallError::ResourceNotFoundException(format!("FirewallPolicy {} not found", name)))?;
        Ok(DescribeFirewallPolicyResponse {
            firewall_policy: FirewallPolicyDetail {
                firewall_policy_name: stored.name.clone(),
                firewall_policy_arn: stored.arn.clone(),
                status: "ACTIVE".to_string(),
                tags: Some(stored.tags.clone()),
            },
        })
    }

    #[allow(dead_code)]
    pub async fn list_firewall_policys(&self, _req: ListFirewallPolicysRequest) -> Result<ListFirewallPolicysResponse, NetworkfirewallError> {
        let state = self.inner.lock().await;
        let items: Vec<FirewallPolicyDetail> = state.firewall_policys.values().map(|s| FirewallPolicyDetail {
            firewall_policy_name: s.name.clone(),
            firewall_policy_arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListFirewallPolicysResponse {
            firewall_policys: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_firewall_policy(&self, req: DeleteFirewallPolicyRequest) -> Result<(), NetworkfirewallError> {
        let mut state = self.inner.lock().await;
        let name = req.firewall_policy_name.or_else(|| req.firewall_policy_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| NetworkfirewallError::ValidationException("Name or ARN required".to_string()))?;
        state.firewall_policys.remove(&name)
            .ok_or_else(|| NetworkfirewallError::ResourceNotFoundException(format!("FirewallPolicy {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = NetworkfirewallState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_firewall() {
        let state = NetworkfirewallState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateFirewallRequest::default();
        let result = state.create_firewall(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_describe_firewall_not_found() {
        let state = NetworkfirewallState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DescribeFirewallRequest::default();
        let result = state.describe_firewall(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_firewalls_empty() {
        let state = NetworkfirewallState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = ListFirewallsRequest::default();
        let result = state.list_firewalls(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_firewall_not_found() {
        let state = NetworkfirewallState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DeleteFirewallRequest::default();
        let result = state.delete_firewall(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_create_firewall_policy() {
        let state = NetworkfirewallState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateFirewallPolicyRequest::default();
        let result = state.create_firewall_policy(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_describe_firewall_policy_not_found() {
        let state = NetworkfirewallState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DescribeFirewallPolicyRequest::default();
        let result = state.describe_firewall_policy(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_firewall_policys_empty() {
        let state = NetworkfirewallState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = ListFirewallPolicysRequest::default();
        let result = state.list_firewall_policys(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_firewall_policy_not_found() {
        let state = NetworkfirewallState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DeleteFirewallPolicyRequest::default();
        let result = state.delete_firewall_policy(req).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_firewall_create_and_list() {
        let state = NetworkfirewallState::new("123456789012".to_string(), "us-east-1".to_string());
        let create_req = CreateFirewallRequest::default();
        let _created = state.create_firewall(create_req).await.unwrap();
        let list_req = ListFirewallsRequest::default();
        let listed = state.list_firewalls(list_req).await.unwrap();
        let _ = listed;
    }
    #[tokio::test]
    async fn test_firewall_policy_create_and_list() {
        let state = NetworkfirewallState::new("123456789012".to_string(), "us-east-1".to_string());
        let create_req = CreateFirewallPolicyRequest::default();
        let _created = state.create_firewall_policy(create_req).await.unwrap();
        let list_req = ListFirewallPolicysRequest::default();
        let listed = state.list_firewall_policys(list_req).await.unwrap();
        let _ = listed;
    }

    #[tokio::test]
    async fn test_firewall_full_crud() {
        let state = NetworkfirewallState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateFirewallRequest::default();
        create_req.firewall_name = "test-crud-resource".to_string();
        let create_result = state.create_firewall(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let mut get_req = DescribeFirewallRequest::default();
        get_req.firewall_name = Some("test-crud-resource".to_string());
        let get_result = state.describe_firewall(get_req).await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let mut del_req = DeleteFirewallRequest::default();
        del_req.firewall_name = Some("test-crud-resource".to_string());
        let del_result = state.delete_firewall(del_req).await;
        assert!(del_result.is_ok(), "delete should succeed");
    }

    #[tokio::test]
    async fn test_firewall_policy_full_crud() {
        let state = NetworkfirewallState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateFirewallPolicyRequest::default();
        create_req.firewall_policy_name = "test-crud-resource".to_string();
        let create_result = state.create_firewall_policy(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let mut get_req = DescribeFirewallPolicyRequest::default();
        get_req.firewall_policy_name = Some("test-crud-resource".to_string());
        let get_result = state.describe_firewall_policy(get_req).await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let mut del_req = DeleteFirewallPolicyRequest::default();
        del_req.firewall_policy_name = Some("test-crud-resource".to_string());
        let del_result = state.delete_firewall_policy(del_req).await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
