use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::SwfError;
use super::types::*;

#[allow(dead_code)]
struct SwfStateInner {
    domains: HashMap<String, StoredDomain>,
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
pub struct SwfState {
    inner: Arc<Mutex<SwfStateInner>>,
}

impl SwfState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        SwfState {
            inner: Arc::new(Mutex::new(SwfStateInner {
                domains: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_domain(&self, req: CreateDomainRequest) -> Result<CreateDomainResponse, SwfError> {
        let mut state = self.inner.lock().await;
        let name = req.domain_name.clone();
        if state.domains.contains_key(&name) {
            return Err(SwfError::ResourceAlreadyExistsException(format!("Domain {} already exists", name)));
        }
        let arn = format!("arn:aws:swf:{}:{}:domains/{}", state.region, state.account_id, name);
        state.domains.insert(name.clone(), StoredDomain {
            name: name.clone(),
            arn: arn.clone(),
            tags: req.tags.unwrap_or_default(),
        });
        Ok(CreateDomainResponse {
            domain_arn: Some(arn),
            domain_name: Some(name),
        })
    }

    #[allow(dead_code)]
    pub async fn describe_domain(&self, req: DescribeDomainRequest) -> Result<DescribeDomainResponse, SwfError> {
        let state = self.inner.lock().await;
        let name = req.domain_name.or_else(|| req.domain_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| SwfError::ValidationException("Name or ARN required".to_string()))?;
        let stored = state.domains.get(&name)
            .ok_or_else(|| SwfError::ResourceNotFoundException(format!("Domain {} not found", name)))?;
        Ok(DescribeDomainResponse {
            domain: DomainDetail {
                domain_name: stored.name.clone(),
                domain_arn: stored.arn.clone(),
                status: "ACTIVE".to_string(),
                tags: Some(stored.tags.clone()),
            },
        })
    }

    #[allow(dead_code)]
    pub async fn list_domains(&self, _req: ListDomainsRequest) -> Result<ListDomainsResponse, SwfError> {
        let state = self.inner.lock().await;
        let items: Vec<DomainDetail> = state.domains.values().map(|s| DomainDetail {
            domain_name: s.name.clone(),
            domain_arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListDomainsResponse {
            domains: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_domain(&self, req: DeleteDomainRequest) -> Result<(), SwfError> {
        let mut state = self.inner.lock().await;
        let name = req.domain_name.or_else(|| req.domain_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| SwfError::ValidationException("Name or ARN required".to_string()))?;
        state.domains.remove(&name)
            .ok_or_else(|| SwfError::ResourceNotFoundException(format!("Domain {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = SwfState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_domain() {
        let state = SwfState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateDomainRequest::default();
        let result = state.create_domain(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_describe_domain_not_found() {
        let state = SwfState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DescribeDomainRequest::default();
        let result = state.describe_domain(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_domains_empty() {
        let state = SwfState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = ListDomainsRequest::default();
        let result = state.list_domains(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_domain_not_found() {
        let state = SwfState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DeleteDomainRequest::default();
        let result = state.delete_domain(req).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_domain_create_and_list() {
        let state = SwfState::new("123456789012".to_string(), "us-east-1".to_string());
        let create_req = CreateDomainRequest::default();
        let _created = state.create_domain(create_req).await.unwrap();
        let list_req = ListDomainsRequest::default();
        let listed = state.list_domains(list_req).await.unwrap();
        let _ = listed;
    }

    #[tokio::test]
    async fn test_domain_full_crud() {
        let state = SwfState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateDomainRequest::default();
        create_req.domain_name = "test-crud-resource".to_string();
        let create_result = state.create_domain(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let mut get_req = DescribeDomainRequest::default();
        get_req.domain_name = Some("test-crud-resource".to_string());
        let get_result = state.describe_domain(get_req).await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let mut del_req = DeleteDomainRequest::default();
        del_req.domain_name = Some("test-crud-resource".to_string());
        let del_result = state.delete_domain(del_req).await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
