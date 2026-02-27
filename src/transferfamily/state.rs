use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::TransferfamilyError;
use super::types::*;

#[allow(dead_code)]
struct TransferfamilyStateInner {
    servers: HashMap<String, StoredServer>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredServer {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct TransferfamilyState {
    inner: Arc<Mutex<TransferfamilyStateInner>>,
}

impl TransferfamilyState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        TransferfamilyState {
            inner: Arc::new(Mutex::new(TransferfamilyStateInner {
                servers: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_server(&self, req: CreateServerRequest) -> Result<CreateServerResponse, TransferfamilyError> {
        let mut state = self.inner.lock().await;
        let name = req.server_name.clone();
        if state.servers.contains_key(&name) {
            return Err(TransferfamilyError::ResourceAlreadyExistsException(format!("Server {} already exists", name)));
        }
        let arn = format!("arn:aws:transfer:{}:{}:servers/{}", state.region, state.account_id, name);
        state.servers.insert(name.clone(), StoredServer {
            name: name.clone(),
            arn: arn.clone(),
            tags: req.tags.unwrap_or_default(),
        });
        Ok(CreateServerResponse {
            server_arn: Some(arn),
            server_name: Some(name),
        })
    }

    #[allow(dead_code)]
    pub async fn describe_server(&self, req: DescribeServerRequest) -> Result<DescribeServerResponse, TransferfamilyError> {
        let state = self.inner.lock().await;
        let name = req.server_name.or_else(|| req.server_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| TransferfamilyError::ValidationException("Name or ARN required".to_string()))?;
        let stored = state.servers.get(&name)
            .ok_or_else(|| TransferfamilyError::ResourceNotFoundException(format!("Server {} not found", name)))?;
        Ok(DescribeServerResponse {
            server: ServerDetail {
                server_name: stored.name.clone(),
                server_arn: stored.arn.clone(),
                status: "ACTIVE".to_string(),
                tags: Some(stored.tags.clone()),
            },
        })
    }

    #[allow(dead_code)]
    pub async fn list_servers(&self, _req: ListServersRequest) -> Result<ListServersResponse, TransferfamilyError> {
        let state = self.inner.lock().await;
        let items: Vec<ServerDetail> = state.servers.values().map(|s| ServerDetail {
            server_name: s.name.clone(),
            server_arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListServersResponse {
            servers: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_server(&self, req: DeleteServerRequest) -> Result<(), TransferfamilyError> {
        let mut state = self.inner.lock().await;
        let name = req.server_name.or_else(|| req.server_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| TransferfamilyError::ValidationException("Name or ARN required".to_string()))?;
        state.servers.remove(&name)
            .ok_or_else(|| TransferfamilyError::ResourceNotFoundException(format!("Server {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = TransferfamilyState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_server() {
        let state = TransferfamilyState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateServerRequest::default();
        let result = state.create_server(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_describe_server_not_found() {
        let state = TransferfamilyState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DescribeServerRequest::default();
        let result = state.describe_server(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_servers_empty() {
        let state = TransferfamilyState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = ListServersRequest::default();
        let result = state.list_servers(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_server_not_found() {
        let state = TransferfamilyState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DeleteServerRequest::default();
        let result = state.delete_server(req).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_server_create_and_list() {
        let state = TransferfamilyState::new("123456789012".to_string(), "us-east-1".to_string());
        let create_req = CreateServerRequest::default();
        let _created = state.create_server(create_req).await.unwrap();
        let list_req = ListServersRequest::default();
        let listed = state.list_servers(list_req).await.unwrap();
        let _ = listed;
    }

    #[tokio::test]
    async fn test_server_full_crud() {
        let state = TransferfamilyState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateServerRequest::default();
        create_req.server_name = "test-crud-resource".to_string();
        let create_result = state.create_server(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let mut get_req = DescribeServerRequest::default();
        get_req.server_name = Some("test-crud-resource".to_string());
        let get_result = state.describe_server(get_req).await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let mut del_req = DeleteServerRequest::default();
        del_req.server_name = Some("test-crud-resource".to_string());
        let del_result = state.delete_server(del_req).await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
