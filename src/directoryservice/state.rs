use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::DirectoryserviceError;
use super::types::*;

#[allow(dead_code)]
struct DirectoryserviceStateInner {
    directorys: HashMap<String, StoredDirectory>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredDirectory {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct DirectoryserviceState {
    inner: Arc<Mutex<DirectoryserviceStateInner>>,
}

impl DirectoryserviceState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        DirectoryserviceState {
            inner: Arc::new(Mutex::new(DirectoryserviceStateInner {
                directorys: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_directory(&self, req: CreateDirectoryRequest) -> Result<CreateDirectoryResponse, DirectoryserviceError> {
        let mut state = self.inner.lock().await;
        let name = req.directory_name.clone();
        if state.directorys.contains_key(&name) {
            return Err(DirectoryserviceError::ResourceAlreadyExistsException(format!("Directory {} already exists", name)));
        }
        let arn = format!("arn:aws:ds:{}:{}:directories/{}", state.region, state.account_id, name);
        state.directorys.insert(name.clone(), StoredDirectory {
            name: name.clone(),
            arn: arn.clone(),
            tags: req.tags.unwrap_or_default(),
        });
        Ok(CreateDirectoryResponse {
            directory_arn: Some(arn),
            directory_name: Some(name),
        })
    }

    #[allow(dead_code)]
    pub async fn describe_directory(&self, req: DescribeDirectoryRequest) -> Result<DescribeDirectoryResponse, DirectoryserviceError> {
        let state = self.inner.lock().await;
        let name = req.directory_name.or_else(|| req.directory_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| DirectoryserviceError::ValidationException("Name or ARN required".to_string()))?;
        let stored = state.directorys.get(&name)
            .ok_or_else(|| DirectoryserviceError::ResourceNotFoundException(format!("Directory {} not found", name)))?;
        Ok(DescribeDirectoryResponse {
            directory: DirectoryDetail {
                directory_name: stored.name.clone(),
                directory_arn: stored.arn.clone(),
                status: "ACTIVE".to_string(),
                tags: Some(stored.tags.clone()),
            },
        })
    }

    #[allow(dead_code)]
    pub async fn list_directorys(&self, _req: ListDirectorysRequest) -> Result<ListDirectorysResponse, DirectoryserviceError> {
        let state = self.inner.lock().await;
        let items: Vec<DirectoryDetail> = state.directorys.values().map(|s| DirectoryDetail {
            directory_name: s.name.clone(),
            directory_arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListDirectorysResponse {
            directorys: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_directory(&self, req: DeleteDirectoryRequest) -> Result<(), DirectoryserviceError> {
        let mut state = self.inner.lock().await;
        let name = req.directory_name.or_else(|| req.directory_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| DirectoryserviceError::ValidationException("Name or ARN required".to_string()))?;
        state.directorys.remove(&name)
            .ok_or_else(|| DirectoryserviceError::ResourceNotFoundException(format!("Directory {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = DirectoryserviceState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_directory() {
        let state = DirectoryserviceState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateDirectoryRequest::default();
        let result = state.create_directory(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_describe_directory_not_found() {
        let state = DirectoryserviceState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DescribeDirectoryRequest::default();
        let result = state.describe_directory(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_directorys_empty() {
        let state = DirectoryserviceState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = ListDirectorysRequest::default();
        let result = state.list_directorys(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_directory_not_found() {
        let state = DirectoryserviceState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DeleteDirectoryRequest::default();
        let result = state.delete_directory(req).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_directory_create_and_list() {
        let state = DirectoryserviceState::new("123456789012".to_string(), "us-east-1".to_string());
        let create_req = CreateDirectoryRequest::default();
        let _created = state.create_directory(create_req).await.unwrap();
        let list_req = ListDirectorysRequest::default();
        let listed = state.list_directorys(list_req).await.unwrap();
        let _ = listed;
    }

    #[tokio::test]
    async fn test_directory_full_crud() {
        let state = DirectoryserviceState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateDirectoryRequest::default();
        create_req.directory_name = "test-crud-resource".to_string();
        let create_result = state.create_directory(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let mut get_req = DescribeDirectoryRequest::default();
        get_req.directory_name = Some("test-crud-resource".to_string());
        let get_result = state.describe_directory(get_req).await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let mut del_req = DeleteDirectoryRequest::default();
        del_req.directory_name = Some("test-crud-resource".to_string());
        let del_result = state.delete_directory(del_req).await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
