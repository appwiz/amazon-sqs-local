use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::FSXError;
use super::types::*;

#[allow(dead_code)]
struct FSXStateInner {
    file_systems: HashMap<String, StoredFileSystem>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredFileSystem {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct FSXState {
    inner: Arc<Mutex<FSXStateInner>>,
}

impl FSXState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        FSXState {
            inner: Arc::new(Mutex::new(FSXStateInner {
                file_systems: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_file_system(&self, req: CreateFileSystemRequest) -> Result<CreateFileSystemResponse, FSXError> {
        let mut state = self.inner.lock().await;
        let name = req.file_system_name.clone();
        if state.file_systems.contains_key(&name) {
            return Err(FSXError::ResourceAlreadyExistsException(format!("FileSystem {} already exists", name)));
        }
        let arn = format!("arn:aws:fsx:{}:{}:file-systems/{}", state.region, state.account_id, name);
        state.file_systems.insert(name.clone(), StoredFileSystem {
            name: name.clone(),
            arn: arn.clone(),
            tags: req.tags.unwrap_or_default(),
        });
        Ok(CreateFileSystemResponse {
            file_system_arn: Some(arn),
            file_system_name: Some(name),
        })
    }

    #[allow(dead_code)]
    pub async fn describe_file_system(&self, req: DescribeFileSystemRequest) -> Result<DescribeFileSystemResponse, FSXError> {
        let state = self.inner.lock().await;
        let name = req.file_system_name.or_else(|| req.file_system_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| FSXError::ValidationException("Name or ARN required".to_string()))?;
        let stored = state.file_systems.get(&name)
            .ok_or_else(|| FSXError::ResourceNotFoundException(format!("FileSystem {} not found", name)))?;
        Ok(DescribeFileSystemResponse {
            file_system: FileSystemDetail {
                file_system_name: stored.name.clone(),
                file_system_arn: stored.arn.clone(),
                status: "ACTIVE".to_string(),
                tags: Some(stored.tags.clone()),
            },
        })
    }

    #[allow(dead_code)]
    pub async fn list_file_systems(&self, _req: ListFileSystemsRequest) -> Result<ListFileSystemsResponse, FSXError> {
        let state = self.inner.lock().await;
        let items: Vec<FileSystemDetail> = state.file_systems.values().map(|s| FileSystemDetail {
            file_system_name: s.name.clone(),
            file_system_arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListFileSystemsResponse {
            file_systems: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_file_system(&self, req: DeleteFileSystemRequest) -> Result<(), FSXError> {
        let mut state = self.inner.lock().await;
        let name = req.file_system_name.or_else(|| req.file_system_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| FSXError::ValidationException("Name or ARN required".to_string()))?;
        state.file_systems.remove(&name)
            .ok_or_else(|| FSXError::ResourceNotFoundException(format!("FileSystem {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = FSXState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_file_system() {
        let state = FSXState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateFileSystemRequest::default();
        let result = state.create_file_system(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_describe_file_system_not_found() {
        let state = FSXState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DescribeFileSystemRequest::default();
        let result = state.describe_file_system(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_file_systems_empty() {
        let state = FSXState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = ListFileSystemsRequest::default();
        let result = state.list_file_systems(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_file_system_not_found() {
        let state = FSXState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DeleteFileSystemRequest::default();
        let result = state.delete_file_system(req).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_file_system_create_and_list() {
        let state = FSXState::new("123456789012".to_string(), "us-east-1".to_string());
        let create_req = CreateFileSystemRequest::default();
        let _created = state.create_file_system(create_req).await.unwrap();
        let list_req = ListFileSystemsRequest::default();
        let listed = state.list_file_systems(list_req).await.unwrap();
        let _ = listed;
    }

    #[tokio::test]
    async fn test_file_system_full_crud() {
        let state = FSXState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateFileSystemRequest::default();
        create_req.file_system_name = "test-crud-resource".to_string();
        let create_result = state.create_file_system(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let mut get_req = DescribeFileSystemRequest::default();
        get_req.file_system_name = Some("test-crud-resource".to_string());
        let get_result = state.describe_file_system(get_req).await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let mut del_req = DeleteFileSystemRequest::default();
        del_req.file_system_name = Some("test-crud-resource".to_string());
        let del_result = state.delete_file_system(del_req).await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
