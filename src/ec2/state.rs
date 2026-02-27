use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::EC2Error;
use super::types::*;

#[allow(dead_code)]
struct EC2StateInner {
    instances: HashMap<String, InstanceInfo>,
    vpcs: HashMap<String, VpcInfo>,
    security_groups: HashMap<String, SecurityGroupInfo>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
pub struct EC2State {
    inner: Arc<Mutex<EC2StateInner>>,
}

impl EC2State {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        EC2State {
            inner: Arc::new(Mutex::new(EC2StateInner {
                instances: HashMap::new(),
                vpcs: HashMap::new(),
                security_groups: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_instance(&self, name: String) -> Result<InstanceInfo, EC2Error> {
        let mut state = self.inner.lock().await;
        if state.instances.contains_key(&name) {
            return Err(EC2Error::ResourceAlreadyExistsException(format!("Instance {} already exists", name)));
        }
        let arn = format!("arn:aws:ec2:{}:{}:instances/{}", state.region, state.account_id, name);
        let info = InstanceInfo {
            instance_name: name.clone(),
            instance_arn: arn,
            status: "ACTIVE".to_string(),
        };
        state.instances.insert(name, info.clone());
        Ok(info)
    }

    #[allow(dead_code)]
    pub async fn describe_instance(&self, name: &str) -> Result<InstanceInfo, EC2Error> {
        let state = self.inner.lock().await;
        state.instances.get(name).cloned()
            .ok_or_else(|| EC2Error::ResourceNotFoundException(format!("Instance {} not found", name)))
    }

    #[allow(dead_code)]
    pub async fn list_instances(&self) -> Result<Vec<InstanceInfo>, EC2Error> {
        let state = self.inner.lock().await;
        Ok(state.instances.values().cloned().collect())
    }

    #[allow(dead_code)]
    pub async fn delete_instance(&self, name: &str) -> Result<(), EC2Error> {
        let mut state = self.inner.lock().await;
        state.instances.remove(name)
            .ok_or_else(|| EC2Error::ResourceNotFoundException(format!("Instance {} not found", name)))?;
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn create_vpc(&self, name: String) -> Result<VpcInfo, EC2Error> {
        let mut state = self.inner.lock().await;
        if state.vpcs.contains_key(&name) {
            return Err(EC2Error::ResourceAlreadyExistsException(format!("Vpc {} already exists", name)));
        }
        let arn = format!("arn:aws:ec2:{}:{}:vpcs/{}", state.region, state.account_id, name);
        let info = VpcInfo {
            vpc_name: name.clone(),
            vpc_arn: arn,
            status: "ACTIVE".to_string(),
        };
        state.vpcs.insert(name, info.clone());
        Ok(info)
    }

    #[allow(dead_code)]
    pub async fn describe_vpc(&self, name: &str) -> Result<VpcInfo, EC2Error> {
        let state = self.inner.lock().await;
        state.vpcs.get(name).cloned()
            .ok_or_else(|| EC2Error::ResourceNotFoundException(format!("Vpc {} not found", name)))
    }

    #[allow(dead_code)]
    pub async fn list_vpcs(&self) -> Result<Vec<VpcInfo>, EC2Error> {
        let state = self.inner.lock().await;
        Ok(state.vpcs.values().cloned().collect())
    }

    #[allow(dead_code)]
    pub async fn delete_vpc(&self, name: &str) -> Result<(), EC2Error> {
        let mut state = self.inner.lock().await;
        state.vpcs.remove(name)
            .ok_or_else(|| EC2Error::ResourceNotFoundException(format!("Vpc {} not found", name)))?;
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn create_security_group(&self, name: String) -> Result<SecurityGroupInfo, EC2Error> {
        let mut state = self.inner.lock().await;
        if state.security_groups.contains_key(&name) {
            return Err(EC2Error::ResourceAlreadyExistsException(format!("SecurityGroup {} already exists", name)));
        }
        let arn = format!("arn:aws:ec2:{}:{}:security-groups/{}", state.region, state.account_id, name);
        let info = SecurityGroupInfo {
            security_group_name: name.clone(),
            security_group_arn: arn,
            status: "ACTIVE".to_string(),
        };
        state.security_groups.insert(name, info.clone());
        Ok(info)
    }

    #[allow(dead_code)]
    pub async fn describe_security_group(&self, name: &str) -> Result<SecurityGroupInfo, EC2Error> {
        let state = self.inner.lock().await;
        state.security_groups.get(name).cloned()
            .ok_or_else(|| EC2Error::ResourceNotFoundException(format!("SecurityGroup {} not found", name)))
    }

    #[allow(dead_code)]
    pub async fn list_security_groups(&self) -> Result<Vec<SecurityGroupInfo>, EC2Error> {
        let state = self.inner.lock().await;
        Ok(state.security_groups.values().cloned().collect())
    }

    #[allow(dead_code)]
    pub async fn delete_security_group(&self, name: &str) -> Result<(), EC2Error> {
        let mut state = self.inner.lock().await;
        state.security_groups.remove(name)
            .ok_or_else(|| EC2Error::ResourceNotFoundException(format!("SecurityGroup {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = EC2State::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_instance() {
        let state = EC2State::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.create_instance("test-resource".to_string()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_instance_duplicate() {
        let state = EC2State::new("123456789012".to_string(), "us-east-1".to_string());
        state.create_instance("dup".to_string()).await.unwrap();
        let result = state.create_instance("dup".to_string()).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_describe_instance_not_found() {
        let state = EC2State::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.describe_instance("nonexistent").await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_instances() {
        let state = EC2State::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.list_instances().await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_instance_not_found() {
        let state = EC2State::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.delete_instance("nonexistent").await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_create_vpc() {
        let state = EC2State::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.create_vpc("test-resource".to_string()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_vpc_duplicate() {
        let state = EC2State::new("123456789012".to_string(), "us-east-1".to_string());
        state.create_vpc("dup".to_string()).await.unwrap();
        let result = state.create_vpc("dup".to_string()).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_describe_vpc_not_found() {
        let state = EC2State::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.describe_vpc("nonexistent").await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_vpcs() {
        let state = EC2State::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.list_vpcs().await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_vpc_not_found() {
        let state = EC2State::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.delete_vpc("nonexistent").await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_create_security_group() {
        let state = EC2State::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.create_security_group("test-resource".to_string()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_security_group_duplicate() {
        let state = EC2State::new("123456789012".to_string(), "us-east-1".to_string());
        state.create_security_group("dup".to_string()).await.unwrap();
        let result = state.create_security_group("dup".to_string()).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_describe_security_group_not_found() {
        let state = EC2State::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.describe_security_group("nonexistent").await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_security_groups() {
        let state = EC2State::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.list_security_groups().await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_security_group_not_found() {
        let state = EC2State::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.delete_security_group("nonexistent").await;
        assert!(result.is_err());
    }
}
