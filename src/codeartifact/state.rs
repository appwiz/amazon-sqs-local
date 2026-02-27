use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::CodeartifactError;
use super::types::*;

#[allow(dead_code)]
struct CodeartifactStateInner {
    domains: HashMap<String, StoredDomain>,
    repositorys: HashMap<String, StoredRepository>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredDomain {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
struct StoredRepository {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct CodeartifactState {
    inner: Arc<Mutex<CodeartifactStateInner>>,
}

impl CodeartifactState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        CodeartifactState {
            inner: Arc::new(Mutex::new(CodeartifactStateInner {
                domains: HashMap::new(),
                repositorys: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_domain(&self, req: CreateDomainRequest) -> Result<DomainDetail, CodeartifactError> {
        let mut state = self.inner.lock().await;
        let name = req.name.or(req.name_pascal).unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        if state.domains.contains_key(&name) {
            return Err(CodeartifactError::ResourceAlreadyExistsException(format!("Domain {} already exists", name)));
        }
        let arn = format!("arn:aws:codeartifact:{}:{}:domains/{}", state.region, state.account_id, name);
        let detail = DomainDetail {
            name: name.clone(),
            arn: arn.clone(),
            status: "ACTIVE".to_string(),
            tags: req.tags.clone(),
        };
        state.domains.insert(name, StoredDomain {
            name: detail.name.clone(),
            arn,
            tags: req.tags.unwrap_or_default(),
        });
        Ok(detail)
    }

    #[allow(dead_code)]
    pub async fn get_domain(&self, name: &str) -> Result<DomainDetail, CodeartifactError> {
        let state = self.inner.lock().await;
        let stored = state.domains.get(name)
            .ok_or_else(|| CodeartifactError::ResourceNotFoundException(format!("Domain {} not found", name)))?;
        Ok(DomainDetail {
            name: stored.name.clone(),
            arn: stored.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(stored.tags.clone()),
        })
    }

    #[allow(dead_code)]
    pub async fn list_domains(&self) -> Result<ListDomainsResponse, CodeartifactError> {
        let state = self.inner.lock().await;
        let items: Vec<DomainDetail> = state.domains.values().map(|s| DomainDetail {
            name: s.name.clone(),
            arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListDomainsResponse {
            items: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_domain(&self, name: &str) -> Result<(), CodeartifactError> {
        let mut state = self.inner.lock().await;
        state.domains.remove(name)
            .ok_or_else(|| CodeartifactError::ResourceNotFoundException(format!("Domain {} not found", name)))?;
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn create_repository(&self, req: CreateRepositoryRequest) -> Result<RepositoryDetail, CodeartifactError> {
        let mut state = self.inner.lock().await;
        let name = req.name.or(req.name_pascal).unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        if state.repositorys.contains_key(&name) {
            return Err(CodeartifactError::ResourceAlreadyExistsException(format!("Repository {} already exists", name)));
        }
        let arn = format!("arn:aws:codeartifact:{}:{}:repositories/{}", state.region, state.account_id, name);
        let detail = RepositoryDetail {
            name: name.clone(),
            arn: arn.clone(),
            status: "ACTIVE".to_string(),
            tags: req.tags.clone(),
        };
        state.repositorys.insert(name, StoredRepository {
            name: detail.name.clone(),
            arn,
            tags: req.tags.unwrap_or_default(),
        });
        Ok(detail)
    }

    #[allow(dead_code)]
    pub async fn get_repository(&self, name: &str) -> Result<RepositoryDetail, CodeartifactError> {
        let state = self.inner.lock().await;
        let stored = state.repositorys.get(name)
            .ok_or_else(|| CodeartifactError::ResourceNotFoundException(format!("Repository {} not found", name)))?;
        Ok(RepositoryDetail {
            name: stored.name.clone(),
            arn: stored.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(stored.tags.clone()),
        })
    }

    #[allow(dead_code)]
    pub async fn list_repositorys(&self) -> Result<ListRepositorysResponse, CodeartifactError> {
        let state = self.inner.lock().await;
        let items: Vec<RepositoryDetail> = state.repositorys.values().map(|s| RepositoryDetail {
            name: s.name.clone(),
            arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListRepositorysResponse {
            items: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_repository(&self, name: &str) -> Result<(), CodeartifactError> {
        let mut state = self.inner.lock().await;
        state.repositorys.remove(name)
            .ok_or_else(|| CodeartifactError::ResourceNotFoundException(format!("Repository {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = CodeartifactState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_domain() {
        let state = CodeartifactState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateDomainRequest::default();
        let result = state.create_domain(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_get_domain_not_found() {
        let state = CodeartifactState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.get_domain("nonexistent").await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_domains() {
        let state = CodeartifactState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.list_domains().await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_domain_not_found() {
        let state = CodeartifactState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.delete_domain("nonexistent").await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_create_repository() {
        let state = CodeartifactState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateRepositoryRequest::default();
        let result = state.create_repository(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_get_repository_not_found() {
        let state = CodeartifactState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.get_repository("nonexistent").await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_repositorys() {
        let state = CodeartifactState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.list_repositorys().await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_repository_not_found() {
        let state = CodeartifactState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.delete_repository("nonexistent").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_domain_full_crud() {
        let state = CodeartifactState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateDomainRequest::default();
        create_req.name = Some("test-crud-resource".to_string());
        let create_result = state.create_domain(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let get_result = state.get_domain("test-crud-resource").await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let del_result = state.delete_domain("test-crud-resource").await;
        assert!(del_result.is_ok(), "delete should succeed");
    }

    #[tokio::test]
    async fn test_repository_full_crud() {
        let state = CodeartifactState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateRepositoryRequest::default();
        create_req.name = Some("test-crud-resource".to_string());
        let create_result = state.create_repository(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let get_result = state.get_repository("test-crud-resource").await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let del_result = state.delete_repository("test-crud-resource").await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
