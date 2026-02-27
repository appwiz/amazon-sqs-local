use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::KeyspacesError;
use super::types::*;

#[allow(dead_code)]
struct KeyspacesStateInner {
    keyspaces: HashMap<String, StoredKeyspace>,
    tables: HashMap<String, StoredTable>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredKeyspace {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
struct StoredTable {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct KeyspacesState {
    inner: Arc<Mutex<KeyspacesStateInner>>,
}

impl KeyspacesState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        KeyspacesState {
            inner: Arc::new(Mutex::new(KeyspacesStateInner {
                keyspaces: HashMap::new(),
                tables: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_keyspace(&self, req: CreateKeyspaceRequest) -> Result<CreateKeyspaceResponse, KeyspacesError> {
        let mut state = self.inner.lock().await;
        let name = req.keyspace_name.clone();
        if state.keyspaces.contains_key(&name) {
            return Err(KeyspacesError::ResourceAlreadyExistsException(format!("Keyspace {} already exists", name)));
        }
        let arn = format!("arn:aws:cassandra:{}:{}:keyspaces/{}", state.region, state.account_id, name);
        state.keyspaces.insert(name.clone(), StoredKeyspace {
            name: name.clone(),
            arn: arn.clone(),
            tags: req.tags.unwrap_or_default(),
        });
        Ok(CreateKeyspaceResponse {
            keyspace_arn: Some(arn),
            keyspace_name: Some(name),
        })
    }

    #[allow(dead_code)]
    pub async fn describe_keyspace(&self, req: DescribeKeyspaceRequest) -> Result<DescribeKeyspaceResponse, KeyspacesError> {
        let state = self.inner.lock().await;
        let name = req.keyspace_name.or_else(|| req.keyspace_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| KeyspacesError::ValidationException("Name or ARN required".to_string()))?;
        let stored = state.keyspaces.get(&name)
            .ok_or_else(|| KeyspacesError::ResourceNotFoundException(format!("Keyspace {} not found", name)))?;
        Ok(DescribeKeyspaceResponse {
            keyspace: KeyspaceDetail {
                keyspace_name: stored.name.clone(),
                keyspace_arn: stored.arn.clone(),
                status: "ACTIVE".to_string(),
                tags: Some(stored.tags.clone()),
            },
        })
    }

    #[allow(dead_code)]
    pub async fn list_keyspaces(&self, _req: ListKeyspacesRequest) -> Result<ListKeyspacesResponse, KeyspacesError> {
        let state = self.inner.lock().await;
        let items: Vec<KeyspaceDetail> = state.keyspaces.values().map(|s| KeyspaceDetail {
            keyspace_name: s.name.clone(),
            keyspace_arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListKeyspacesResponse {
            keyspaces: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_keyspace(&self, req: DeleteKeyspaceRequest) -> Result<(), KeyspacesError> {
        let mut state = self.inner.lock().await;
        let name = req.keyspace_name.or_else(|| req.keyspace_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| KeyspacesError::ValidationException("Name or ARN required".to_string()))?;
        state.keyspaces.remove(&name)
            .ok_or_else(|| KeyspacesError::ResourceNotFoundException(format!("Keyspace {} not found", name)))?;
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn create_table(&self, req: CreateTableRequest) -> Result<CreateTableResponse, KeyspacesError> {
        let mut state = self.inner.lock().await;
        let name = req.table_name.clone();
        if state.tables.contains_key(&name) {
            return Err(KeyspacesError::ResourceAlreadyExistsException(format!("Table {} already exists", name)));
        }
        let arn = format!("arn:aws:cassandra:{}:{}:tables/{}", state.region, state.account_id, name);
        state.tables.insert(name.clone(), StoredTable {
            name: name.clone(),
            arn: arn.clone(),
            tags: req.tags.unwrap_or_default(),
        });
        Ok(CreateTableResponse {
            table_arn: Some(arn),
            table_name: Some(name),
        })
    }

    #[allow(dead_code)]
    pub async fn describe_table(&self, req: DescribeTableRequest) -> Result<DescribeTableResponse, KeyspacesError> {
        let state = self.inner.lock().await;
        let name = req.table_name.or_else(|| req.table_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| KeyspacesError::ValidationException("Name or ARN required".to_string()))?;
        let stored = state.tables.get(&name)
            .ok_or_else(|| KeyspacesError::ResourceNotFoundException(format!("Table {} not found", name)))?;
        Ok(DescribeTableResponse {
            table: TableDetail {
                table_name: stored.name.clone(),
                table_arn: stored.arn.clone(),
                status: "ACTIVE".to_string(),
                tags: Some(stored.tags.clone()),
            },
        })
    }

    #[allow(dead_code)]
    pub async fn list_tables(&self, _req: ListTablesRequest) -> Result<ListTablesResponse, KeyspacesError> {
        let state = self.inner.lock().await;
        let items: Vec<TableDetail> = state.tables.values().map(|s| TableDetail {
            table_name: s.name.clone(),
            table_arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListTablesResponse {
            tables: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_table(&self, req: DeleteTableRequest) -> Result<(), KeyspacesError> {
        let mut state = self.inner.lock().await;
        let name = req.table_name.or_else(|| req.table_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| KeyspacesError::ValidationException("Name or ARN required".to_string()))?;
        state.tables.remove(&name)
            .ok_or_else(|| KeyspacesError::ResourceNotFoundException(format!("Table {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = KeyspacesState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_keyspace() {
        let state = KeyspacesState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateKeyspaceRequest::default();
        let result = state.create_keyspace(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_describe_keyspace_not_found() {
        let state = KeyspacesState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DescribeKeyspaceRequest::default();
        let result = state.describe_keyspace(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_keyspaces_empty() {
        let state = KeyspacesState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = ListKeyspacesRequest::default();
        let result = state.list_keyspaces(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_keyspace_not_found() {
        let state = KeyspacesState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DeleteKeyspaceRequest::default();
        let result = state.delete_keyspace(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_create_table() {
        let state = KeyspacesState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateTableRequest::default();
        let result = state.create_table(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_describe_table_not_found() {
        let state = KeyspacesState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DescribeTableRequest::default();
        let result = state.describe_table(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_tables_empty() {
        let state = KeyspacesState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = ListTablesRequest::default();
        let result = state.list_tables(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_table_not_found() {
        let state = KeyspacesState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DeleteTableRequest::default();
        let result = state.delete_table(req).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_keyspace_create_and_list() {
        let state = KeyspacesState::new("123456789012".to_string(), "us-east-1".to_string());
        let create_req = CreateKeyspaceRequest::default();
        let _created = state.create_keyspace(create_req).await.unwrap();
        let list_req = ListKeyspacesRequest::default();
        let listed = state.list_keyspaces(list_req).await.unwrap();
        let _ = listed;
    }
    #[tokio::test]
    async fn test_table_create_and_list() {
        let state = KeyspacesState::new("123456789012".to_string(), "us-east-1".to_string());
        let create_req = CreateTableRequest::default();
        let _created = state.create_table(create_req).await.unwrap();
        let list_req = ListTablesRequest::default();
        let listed = state.list_tables(list_req).await.unwrap();
        let _ = listed;
    }

    #[tokio::test]
    async fn test_keyspace_full_crud() {
        let state = KeyspacesState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateKeyspaceRequest::default();
        create_req.keyspace_name = "test-crud-resource".to_string();
        let create_result = state.create_keyspace(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let mut get_req = DescribeKeyspaceRequest::default();
        get_req.keyspace_name = Some("test-crud-resource".to_string());
        let get_result = state.describe_keyspace(get_req).await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let mut del_req = DeleteKeyspaceRequest::default();
        del_req.keyspace_name = Some("test-crud-resource".to_string());
        let del_result = state.delete_keyspace(del_req).await;
        assert!(del_result.is_ok(), "delete should succeed");
    }

    #[tokio::test]
    async fn test_table_full_crud() {
        let state = KeyspacesState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateTableRequest::default();
        create_req.table_name = "test-crud-resource".to_string();
        let create_result = state.create_table(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let mut get_req = DescribeTableRequest::default();
        get_req.table_name = Some("test-crud-resource".to_string());
        let get_result = state.describe_table(get_req).await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let mut del_req = DeleteTableRequest::default();
        del_req.table_name = Some("test-crud-resource".to_string());
        let del_result = state.delete_table(del_req).await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
