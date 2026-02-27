use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::WorkdocsError;
use super::types::*;

#[allow(dead_code)]
struct WorkdocsStateInner {
    folders: HashMap<String, StoredFolder>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredFolder {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct WorkdocsState {
    inner: Arc<Mutex<WorkdocsStateInner>>,
}

impl WorkdocsState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        WorkdocsState {
            inner: Arc::new(Mutex::new(WorkdocsStateInner {
                folders: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_folder(&self, req: CreateFolderRequest) -> Result<FolderDetail, WorkdocsError> {
        let mut state = self.inner.lock().await;
        let name = req.name.or(req.name_pascal).unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        if state.folders.contains_key(&name) {
            return Err(WorkdocsError::ResourceAlreadyExistsException(format!("Folder {} already exists", name)));
        }
        let arn = format!("arn:aws:workdocs:{}:{}:folders/{}", state.region, state.account_id, name);
        let detail = FolderDetail {
            name: name.clone(),
            arn: arn.clone(),
            status: "ACTIVE".to_string(),
            tags: req.tags.clone(),
        };
        state.folders.insert(name, StoredFolder {
            name: detail.name.clone(),
            arn,
            tags: req.tags.unwrap_or_default(),
        });
        Ok(detail)
    }

    #[allow(dead_code)]
    pub async fn get_folder(&self, name: &str) -> Result<FolderDetail, WorkdocsError> {
        let state = self.inner.lock().await;
        let stored = state.folders.get(name)
            .ok_or_else(|| WorkdocsError::ResourceNotFoundException(format!("Folder {} not found", name)))?;
        Ok(FolderDetail {
            name: stored.name.clone(),
            arn: stored.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(stored.tags.clone()),
        })
    }

    #[allow(dead_code)]
    pub async fn list_folders(&self) -> Result<ListFoldersResponse, WorkdocsError> {
        let state = self.inner.lock().await;
        let items: Vec<FolderDetail> = state.folders.values().map(|s| FolderDetail {
            name: s.name.clone(),
            arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListFoldersResponse {
            items: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_folder(&self, name: &str) -> Result<(), WorkdocsError> {
        let mut state = self.inner.lock().await;
        state.folders.remove(name)
            .ok_or_else(|| WorkdocsError::ResourceNotFoundException(format!("Folder {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = WorkdocsState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_folder() {
        let state = WorkdocsState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateFolderRequest::default();
        let result = state.create_folder(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_get_folder_not_found() {
        let state = WorkdocsState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.get_folder("nonexistent").await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_folders() {
        let state = WorkdocsState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.list_folders().await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_folder_not_found() {
        let state = WorkdocsState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.delete_folder("nonexistent").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_folder_full_crud() {
        let state = WorkdocsState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateFolderRequest::default();
        create_req.name = Some("test-crud-resource".to_string());
        let create_result = state.create_folder(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let get_result = state.get_folder("test-crud-resource").await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let del_result = state.delete_folder("test-crud-resource").await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
