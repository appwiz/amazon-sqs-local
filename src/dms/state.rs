use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::DMSError;
use super::types::*;

#[allow(dead_code)]
struct DMSStateInner {
    replication_instances: HashMap<String, StoredReplicationInstance>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredReplicationInstance {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct DMSState {
    inner: Arc<Mutex<DMSStateInner>>,
}

impl DMSState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        DMSState {
            inner: Arc::new(Mutex::new(DMSStateInner {
                replication_instances: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_replication_instance(&self, req: CreateReplicationInstanceRequest) -> Result<CreateReplicationInstanceResponse, DMSError> {
        let mut state = self.inner.lock().await;
        let name = req.replication_instance_name.clone();
        if state.replication_instances.contains_key(&name) {
            return Err(DMSError::ResourceAlreadyExistsException(format!("ReplicationInstance {} already exists", name)));
        }
        let arn = format!("arn:aws:dms:{}:{}:replication-instances/{}", state.region, state.account_id, name);
        state.replication_instances.insert(name.clone(), StoredReplicationInstance {
            name: name.clone(),
            arn: arn.clone(),
            tags: req.tags.unwrap_or_default(),
        });
        Ok(CreateReplicationInstanceResponse {
            replication_instance_arn: Some(arn),
            replication_instance_name: Some(name),
        })
    }

    #[allow(dead_code)]
    pub async fn describe_replication_instance(&self, req: DescribeReplicationInstanceRequest) -> Result<DescribeReplicationInstanceResponse, DMSError> {
        let state = self.inner.lock().await;
        let name = req.replication_instance_name.or_else(|| req.replication_instance_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| DMSError::ValidationException("Name or ARN required".to_string()))?;
        let stored = state.replication_instances.get(&name)
            .ok_or_else(|| DMSError::ResourceNotFoundException(format!("ReplicationInstance {} not found", name)))?;
        Ok(DescribeReplicationInstanceResponse {
            replication_instance: ReplicationInstanceDetail {
                replication_instance_name: stored.name.clone(),
                replication_instance_arn: stored.arn.clone(),
                status: "ACTIVE".to_string(),
                tags: Some(stored.tags.clone()),
            },
        })
    }

    #[allow(dead_code)]
    pub async fn list_replication_instances(&self, _req: ListReplicationInstancesRequest) -> Result<ListReplicationInstancesResponse, DMSError> {
        let state = self.inner.lock().await;
        let items: Vec<ReplicationInstanceDetail> = state.replication_instances.values().map(|s| ReplicationInstanceDetail {
            replication_instance_name: s.name.clone(),
            replication_instance_arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListReplicationInstancesResponse {
            replication_instances: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_replication_instance(&self, req: DeleteReplicationInstanceRequest) -> Result<(), DMSError> {
        let mut state = self.inner.lock().await;
        let name = req.replication_instance_name.or_else(|| req.replication_instance_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| DMSError::ValidationException("Name or ARN required".to_string()))?;
        state.replication_instances.remove(&name)
            .ok_or_else(|| DMSError::ResourceNotFoundException(format!("ReplicationInstance {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = DMSState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_replication_instance() {
        let state = DMSState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateReplicationInstanceRequest::default();
        let result = state.create_replication_instance(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_describe_replication_instance_not_found() {
        let state = DMSState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DescribeReplicationInstanceRequest::default();
        let result = state.describe_replication_instance(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_replication_instances_empty() {
        let state = DMSState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = ListReplicationInstancesRequest::default();
        let result = state.list_replication_instances(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_replication_instance_not_found() {
        let state = DMSState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DeleteReplicationInstanceRequest::default();
        let result = state.delete_replication_instance(req).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_replication_instance_create_and_list() {
        let state = DMSState::new("123456789012".to_string(), "us-east-1".to_string());
        let create_req = CreateReplicationInstanceRequest::default();
        let _created = state.create_replication_instance(create_req).await.unwrap();
        let list_req = ListReplicationInstancesRequest::default();
        let listed = state.list_replication_instances(list_req).await.unwrap();
        let _ = listed;
    }

    #[tokio::test]
    async fn test_replication_instance_full_crud() {
        let state = DMSState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateReplicationInstanceRequest::default();
        create_req.replication_instance_name = "test-crud-resource".to_string();
        let create_result = state.create_replication_instance(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let mut get_req = DescribeReplicationInstanceRequest::default();
        get_req.replication_instance_name = Some("test-crud-resource".to_string());
        let get_result = state.describe_replication_instance(get_req).await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let mut del_req = DeleteReplicationInstanceRequest::default();
        del_req.replication_instance_name = Some("test-crud-resource".to_string());
        let del_result = state.delete_replication_instance(del_req).await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
