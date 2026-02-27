use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::ECRError;
use super::types::*;

#[allow(dead_code)]
struct ECRStateInner {
    repositorys: HashMap<String, StoredRepository>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredRepository {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct ECRState {
    inner: Arc<Mutex<ECRStateInner>>,
}

impl ECRState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        ECRState {
            inner: Arc::new(Mutex::new(ECRStateInner {
                repositorys: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_repository(&self, req: CreateRepositoryRequest) -> Result<CreateRepositoryResponse, ECRError> {
        let mut state = self.inner.lock().await;
        let name = req.repository_name.clone();
        if state.repositorys.contains_key(&name) {
            return Err(ECRError::ResourceAlreadyExistsException(format!("Repository {} already exists", name)));
        }
        let arn = format!("arn:aws:ecr:{}:{}:repositories/{}", state.region, state.account_id, name);
        state.repositorys.insert(name.clone(), StoredRepository {
            name: name.clone(),
            arn: arn.clone(),
            tags: req.tags.unwrap_or_default(),
        });
        Ok(CreateRepositoryResponse {
            repository_arn: Some(arn),
            repository_name: Some(name),
        })
    }

    #[allow(dead_code)]
    pub async fn describe_repository(&self, req: DescribeRepositoryRequest) -> Result<DescribeRepositoryResponse, ECRError> {
        let state = self.inner.lock().await;
        let name = req.repository_name.or_else(|| req.repository_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| ECRError::ValidationException("Name or ARN required".to_string()))?;
        let stored = state.repositorys.get(&name)
            .ok_or_else(|| ECRError::ResourceNotFoundException(format!("Repository {} not found", name)))?;
        Ok(DescribeRepositoryResponse {
            repository: RepositoryDetail {
                repository_name: stored.name.clone(),
                repository_arn: stored.arn.clone(),
                status: "ACTIVE".to_string(),
                tags: Some(stored.tags.clone()),
            },
        })
    }

    #[allow(dead_code)]
    pub async fn list_repositorys(&self, _req: ListRepositorysRequest) -> Result<ListRepositorysResponse, ECRError> {
        let state = self.inner.lock().await;
        let items: Vec<RepositoryDetail> = state.repositorys.values().map(|s| RepositoryDetail {
            repository_name: s.name.clone(),
            repository_arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListRepositorysResponse {
            repositorys: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_repository(&self, req: DeleteRepositoryRequest) -> Result<(), ECRError> {
        let mut state = self.inner.lock().await;
        let name = req.repository_name.or_else(|| req.repository_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| ECRError::ValidationException("Name or ARN required".to_string()))?;
        state.repositorys.remove(&name)
            .ok_or_else(|| ECRError::ResourceNotFoundException(format!("Repository {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = ECRState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_repository() {
        let state = ECRState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateRepositoryRequest::default();
        let result = state.create_repository(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_describe_repository_not_found() {
        let state = ECRState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DescribeRepositoryRequest::default();
        let result = state.describe_repository(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_repositorys_empty() {
        let state = ECRState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = ListRepositorysRequest::default();
        let result = state.list_repositorys(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_repository_not_found() {
        let state = ECRState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DeleteRepositoryRequest::default();
        let result = state.delete_repository(req).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_repository_create_and_list() {
        let state = ECRState::new("123456789012".to_string(), "us-east-1".to_string());
        let create_req = CreateRepositoryRequest::default();
        let _created = state.create_repository(create_req).await.unwrap();
        let list_req = ListRepositorysRequest::default();
        let listed = state.list_repositorys(list_req).await.unwrap();
        let _ = listed;
    }

    #[tokio::test]
    async fn test_repository_full_crud() {
        let state = ECRState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateRepositoryRequest::default();
        create_req.repository_name = "test-crud-resource".to_string();
        let create_result = state.create_repository(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let mut get_req = DescribeRepositoryRequest::default();
        get_req.repository_name = Some("test-crud-resource".to_string());
        let get_result = state.describe_repository(get_req).await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let mut del_req = DeleteRepositoryRequest::default();
        del_req.repository_name = Some("test-crud-resource".to_string());
        let del_result = state.delete_repository(del_req).await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
