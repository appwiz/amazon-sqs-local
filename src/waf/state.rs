use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::WAFError;
use super::types::*;

#[allow(dead_code)]
struct WAFStateInner {
    web_a_c_ls: HashMap<String, StoredWebACL>,
    i_p_sets: HashMap<String, StoredIPSet>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredWebACL {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
struct StoredIPSet {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct WAFState {
    inner: Arc<Mutex<WAFStateInner>>,
}

impl WAFState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        WAFState {
            inner: Arc::new(Mutex::new(WAFStateInner {
                web_a_c_ls: HashMap::new(),
                i_p_sets: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_web_a_c_l(&self, req: CreateWebACLRequest) -> Result<CreateWebACLResponse, WAFError> {
        let mut state = self.inner.lock().await;
        let name = req.web_a_c_l_name.clone();
        if state.web_a_c_ls.contains_key(&name) {
            return Err(WAFError::ResourceAlreadyExistsException(format!("WebACL {} already exists", name)));
        }
        let arn = format!("arn:aws:wafv2:{}:{}:web-acls/{}", state.region, state.account_id, name);
        state.web_a_c_ls.insert(name.clone(), StoredWebACL {
            name: name.clone(),
            arn: arn.clone(),
            tags: req.tags.unwrap_or_default(),
        });
        Ok(CreateWebACLResponse {
            web_a_c_l_arn: Some(arn),
            web_a_c_l_name: Some(name),
        })
    }

    #[allow(dead_code)]
    pub async fn describe_web_a_c_l(&self, req: DescribeWebACLRequest) -> Result<DescribeWebACLResponse, WAFError> {
        let state = self.inner.lock().await;
        let name = req.web_a_c_l_name.or_else(|| req.web_a_c_l_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| WAFError::ValidationException("Name or ARN required".to_string()))?;
        let stored = state.web_a_c_ls.get(&name)
            .ok_or_else(|| WAFError::ResourceNotFoundException(format!("WebACL {} not found", name)))?;
        Ok(DescribeWebACLResponse {
            web_a_c_l: WebACLDetail {
                web_a_c_l_name: stored.name.clone(),
                web_a_c_l_arn: stored.arn.clone(),
                status: "ACTIVE".to_string(),
                tags: Some(stored.tags.clone()),
            },
        })
    }

    #[allow(dead_code)]
    pub async fn list_web_a_c_ls(&self, _req: ListWebACLsRequest) -> Result<ListWebACLsResponse, WAFError> {
        let state = self.inner.lock().await;
        let items: Vec<WebACLDetail> = state.web_a_c_ls.values().map(|s| WebACLDetail {
            web_a_c_l_name: s.name.clone(),
            web_a_c_l_arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListWebACLsResponse {
            web_a_c_ls: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_web_a_c_l(&self, req: DeleteWebACLRequest) -> Result<(), WAFError> {
        let mut state = self.inner.lock().await;
        let name = req.web_a_c_l_name.or_else(|| req.web_a_c_l_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| WAFError::ValidationException("Name or ARN required".to_string()))?;
        state.web_a_c_ls.remove(&name)
            .ok_or_else(|| WAFError::ResourceNotFoundException(format!("WebACL {} not found", name)))?;
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn create_i_p_set(&self, req: CreateIPSetRequest) -> Result<CreateIPSetResponse, WAFError> {
        let mut state = self.inner.lock().await;
        let name = req.i_p_set_name.clone();
        if state.i_p_sets.contains_key(&name) {
            return Err(WAFError::ResourceAlreadyExistsException(format!("IPSet {} already exists", name)));
        }
        let arn = format!("arn:aws:wafv2:{}:{}:ip-sets/{}", state.region, state.account_id, name);
        state.i_p_sets.insert(name.clone(), StoredIPSet {
            name: name.clone(),
            arn: arn.clone(),
            tags: req.tags.unwrap_or_default(),
        });
        Ok(CreateIPSetResponse {
            i_p_set_arn: Some(arn),
            i_p_set_name: Some(name),
        })
    }

    #[allow(dead_code)]
    pub async fn describe_i_p_set(&self, req: DescribeIPSetRequest) -> Result<DescribeIPSetResponse, WAFError> {
        let state = self.inner.lock().await;
        let name = req.i_p_set_name.or_else(|| req.i_p_set_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| WAFError::ValidationException("Name or ARN required".to_string()))?;
        let stored = state.i_p_sets.get(&name)
            .ok_or_else(|| WAFError::ResourceNotFoundException(format!("IPSet {} not found", name)))?;
        Ok(DescribeIPSetResponse {
            i_p_set: IPSetDetail {
                i_p_set_name: stored.name.clone(),
                i_p_set_arn: stored.arn.clone(),
                status: "ACTIVE".to_string(),
                tags: Some(stored.tags.clone()),
            },
        })
    }

    #[allow(dead_code)]
    pub async fn list_i_p_sets(&self, _req: ListIPSetsRequest) -> Result<ListIPSetsResponse, WAFError> {
        let state = self.inner.lock().await;
        let items: Vec<IPSetDetail> = state.i_p_sets.values().map(|s| IPSetDetail {
            i_p_set_name: s.name.clone(),
            i_p_set_arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListIPSetsResponse {
            i_p_sets: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_i_p_set(&self, req: DeleteIPSetRequest) -> Result<(), WAFError> {
        let mut state = self.inner.lock().await;
        let name = req.i_p_set_name.or_else(|| req.i_p_set_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| WAFError::ValidationException("Name or ARN required".to_string()))?;
        state.i_p_sets.remove(&name)
            .ok_or_else(|| WAFError::ResourceNotFoundException(format!("IPSet {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = WAFState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_web_a_c_l() {
        let state = WAFState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateWebACLRequest::default();
        let result = state.create_web_a_c_l(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_describe_web_a_c_l_not_found() {
        let state = WAFState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DescribeWebACLRequest::default();
        let result = state.describe_web_a_c_l(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_web_a_c_ls_empty() {
        let state = WAFState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = ListWebACLsRequest::default();
        let result = state.list_web_a_c_ls(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_web_a_c_l_not_found() {
        let state = WAFState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DeleteWebACLRequest::default();
        let result = state.delete_web_a_c_l(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_create_i_p_set() {
        let state = WAFState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateIPSetRequest::default();
        let result = state.create_i_p_set(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_describe_i_p_set_not_found() {
        let state = WAFState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DescribeIPSetRequest::default();
        let result = state.describe_i_p_set(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_i_p_sets_empty() {
        let state = WAFState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = ListIPSetsRequest::default();
        let result = state.list_i_p_sets(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_i_p_set_not_found() {
        let state = WAFState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DeleteIPSetRequest::default();
        let result = state.delete_i_p_set(req).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_web_a_c_l_create_and_list() {
        let state = WAFState::new("123456789012".to_string(), "us-east-1".to_string());
        let create_req = CreateWebACLRequest::default();
        let _created = state.create_web_a_c_l(create_req).await.unwrap();
        let list_req = ListWebACLsRequest::default();
        let listed = state.list_web_a_c_ls(list_req).await.unwrap();
        let _ = listed;
    }
    #[tokio::test]
    async fn test_i_p_set_create_and_list() {
        let state = WAFState::new("123456789012".to_string(), "us-east-1".to_string());
        let create_req = CreateIPSetRequest::default();
        let _created = state.create_i_p_set(create_req).await.unwrap();
        let list_req = ListIPSetsRequest::default();
        let listed = state.list_i_p_sets(list_req).await.unwrap();
        let _ = listed;
    }

    #[tokio::test]
    async fn test_web_a_c_l_full_crud() {
        let state = WAFState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateWebACLRequest::default();
        create_req.web_a_c_l_name = "test-crud-resource".to_string();
        let create_result = state.create_web_a_c_l(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let mut get_req = DescribeWebACLRequest::default();
        get_req.web_a_c_l_name = Some("test-crud-resource".to_string());
        let get_result = state.describe_web_a_c_l(get_req).await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let mut del_req = DeleteWebACLRequest::default();
        del_req.web_a_c_l_name = Some("test-crud-resource".to_string());
        let del_result = state.delete_web_a_c_l(del_req).await;
        assert!(del_result.is_ok(), "delete should succeed");
    }

    #[tokio::test]
    async fn test_i_p_set_full_crud() {
        let state = WAFState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateIPSetRequest::default();
        create_req.i_p_set_name = "test-crud-resource".to_string();
        let create_result = state.create_i_p_set(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let mut get_req = DescribeIPSetRequest::default();
        get_req.i_p_set_name = Some("test-crud-resource".to_string());
        let get_result = state.describe_i_p_set(get_req).await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let mut del_req = DeleteIPSetRequest::default();
        del_req.i_p_set_name = Some("test-crud-resource".to_string());
        let del_result = state.delete_i_p_set(del_req).await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
