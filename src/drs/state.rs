use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::DRSError;
use super::types::*;

#[allow(dead_code)]
struct DRSStateInner {
    source_servers: HashMap<String, StoredSourceServer>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredSourceServer {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct DRSState {
    inner: Arc<Mutex<DRSStateInner>>,
}

impl DRSState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        DRSState {
            inner: Arc::new(Mutex::new(DRSStateInner {
                source_servers: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_source_server(&self, req: CreateSourceServerRequest) -> Result<SourceServerDetail, DRSError> {
        let mut state = self.inner.lock().await;
        let name = req.name.or(req.name_pascal).unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        if state.source_servers.contains_key(&name) {
            return Err(DRSError::ResourceAlreadyExistsException(format!("SourceServer {} already exists", name)));
        }
        let arn = format!("arn:aws:drs:{}:{}:source-servers/{}", state.region, state.account_id, name);
        let detail = SourceServerDetail {
            name: name.clone(),
            arn: arn.clone(),
            status: "ACTIVE".to_string(),
            tags: req.tags.clone(),
        };
        state.source_servers.insert(name, StoredSourceServer {
            name: detail.name.clone(),
            arn,
            tags: req.tags.unwrap_or_default(),
        });
        Ok(detail)
    }

    #[allow(dead_code)]
    pub async fn get_source_server(&self, name: &str) -> Result<SourceServerDetail, DRSError> {
        let state = self.inner.lock().await;
        let stored = state.source_servers.get(name)
            .ok_or_else(|| DRSError::ResourceNotFoundException(format!("SourceServer {} not found", name)))?;
        Ok(SourceServerDetail {
            name: stored.name.clone(),
            arn: stored.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(stored.tags.clone()),
        })
    }

    #[allow(dead_code)]
    pub async fn list_source_servers(&self) -> Result<ListSourceServersResponse, DRSError> {
        let state = self.inner.lock().await;
        let items: Vec<SourceServerDetail> = state.source_servers.values().map(|s| SourceServerDetail {
            name: s.name.clone(),
            arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListSourceServersResponse {
            items: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_source_server(&self, name: &str) -> Result<(), DRSError> {
        let mut state = self.inner.lock().await;
        state.source_servers.remove(name)
            .ok_or_else(|| DRSError::ResourceNotFoundException(format!("SourceServer {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = DRSState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_source_server() {
        let state = DRSState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateSourceServerRequest::default();
        let result = state.create_source_server(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_get_source_server_not_found() {
        let state = DRSState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.get_source_server("nonexistent").await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_source_servers() {
        let state = DRSState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.list_source_servers().await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_source_server_not_found() {
        let state = DRSState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.delete_source_server("nonexistent").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_source_server_full_crud() {
        let state = DRSState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateSourceServerRequest::default();
        create_req.name = Some("test-crud-resource".to_string());
        let create_result = state.create_source_server(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let get_result = state.get_source_server("test-crud-resource").await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let del_result = state.delete_source_server("test-crud-resource").await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
