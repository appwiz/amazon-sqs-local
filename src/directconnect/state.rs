use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::DirectconnectError;
use super::types::*;

#[allow(dead_code)]
struct DirectconnectStateInner {
    connections: HashMap<String, StoredConnection>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredConnection {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct DirectconnectState {
    inner: Arc<Mutex<DirectconnectStateInner>>,
}

impl DirectconnectState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        DirectconnectState {
            inner: Arc::new(Mutex::new(DirectconnectStateInner {
                connections: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_connection(&self, req: CreateConnectionRequest) -> Result<CreateConnectionResponse, DirectconnectError> {
        let mut state = self.inner.lock().await;
        let name = req.connection_name.clone();
        if state.connections.contains_key(&name) {
            return Err(DirectconnectError::ResourceAlreadyExistsException(format!("Connection {} already exists", name)));
        }
        let arn = format!("arn:aws:directconnect:{}:{}:connections/{}", state.region, state.account_id, name);
        state.connections.insert(name.clone(), StoredConnection {
            name: name.clone(),
            arn: arn.clone(),
            tags: req.tags.unwrap_or_default(),
        });
        Ok(CreateConnectionResponse {
            connection_arn: Some(arn),
            connection_name: Some(name),
        })
    }

    #[allow(dead_code)]
    pub async fn describe_connection(&self, req: DescribeConnectionRequest) -> Result<DescribeConnectionResponse, DirectconnectError> {
        let state = self.inner.lock().await;
        let name = req.connection_name.or_else(|| req.connection_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| DirectconnectError::ValidationException("Name or ARN required".to_string()))?;
        let stored = state.connections.get(&name)
            .ok_or_else(|| DirectconnectError::ResourceNotFoundException(format!("Connection {} not found", name)))?;
        Ok(DescribeConnectionResponse {
            connection: ConnectionDetail {
                connection_name: stored.name.clone(),
                connection_arn: stored.arn.clone(),
                status: "ACTIVE".to_string(),
                tags: Some(stored.tags.clone()),
            },
        })
    }

    #[allow(dead_code)]
    pub async fn list_connections(&self, _req: ListConnectionsRequest) -> Result<ListConnectionsResponse, DirectconnectError> {
        let state = self.inner.lock().await;
        let items: Vec<ConnectionDetail> = state.connections.values().map(|s| ConnectionDetail {
            connection_name: s.name.clone(),
            connection_arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListConnectionsResponse {
            connections: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_connection(&self, req: DeleteConnectionRequest) -> Result<(), DirectconnectError> {
        let mut state = self.inner.lock().await;
        let name = req.connection_name.or_else(|| req.connection_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| DirectconnectError::ValidationException("Name or ARN required".to_string()))?;
        state.connections.remove(&name)
            .ok_or_else(|| DirectconnectError::ResourceNotFoundException(format!("Connection {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = DirectconnectState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_connection() {
        let state = DirectconnectState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateConnectionRequest::default();
        let result = state.create_connection(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_describe_connection_not_found() {
        let state = DirectconnectState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DescribeConnectionRequest::default();
        let result = state.describe_connection(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_connections_empty() {
        let state = DirectconnectState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = ListConnectionsRequest::default();
        let result = state.list_connections(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_connection_not_found() {
        let state = DirectconnectState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DeleteConnectionRequest::default();
        let result = state.delete_connection(req).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_connection_create_and_list() {
        let state = DirectconnectState::new("123456789012".to_string(), "us-east-1".to_string());
        let create_req = CreateConnectionRequest::default();
        let _created = state.create_connection(create_req).await.unwrap();
        let list_req = ListConnectionsRequest::default();
        let listed = state.list_connections(list_req).await.unwrap();
        let _ = listed;
    }

    #[tokio::test]
    async fn test_connection_full_crud() {
        let state = DirectconnectState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateConnectionRequest::default();
        create_req.connection_name = "test-crud-resource".to_string();
        let create_result = state.create_connection(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let mut get_req = DescribeConnectionRequest::default();
        get_req.connection_name = Some("test-crud-resource".to_string());
        let get_result = state.describe_connection(get_req).await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let mut del_req = DeleteConnectionRequest::default();
        del_req.connection_name = Some("test-crud-resource".to_string());
        let del_result = state.delete_connection(del_req).await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
