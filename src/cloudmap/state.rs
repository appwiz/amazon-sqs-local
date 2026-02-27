use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::CloudmapError;
use super::types::*;

#[allow(dead_code)]
struct CloudmapStateInner {
    namespaces: HashMap<String, StoredNamespace>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredNamespace {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct CloudmapState {
    inner: Arc<Mutex<CloudmapStateInner>>,
}

impl CloudmapState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        CloudmapState {
            inner: Arc::new(Mutex::new(CloudmapStateInner {
                namespaces: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_namespace(&self, req: CreateNamespaceRequest) -> Result<CreateNamespaceResponse, CloudmapError> {
        let mut state = self.inner.lock().await;
        let name = req.namespace_name.clone();
        if state.namespaces.contains_key(&name) {
            return Err(CloudmapError::ResourceAlreadyExistsException(format!("Namespace {} already exists", name)));
        }
        let arn = format!("arn:aws:servicediscovery:{}:{}:namespaces/{}", state.region, state.account_id, name);
        state.namespaces.insert(name.clone(), StoredNamespace {
            name: name.clone(),
            arn: arn.clone(),
            tags: req.tags.unwrap_or_default(),
        });
        Ok(CreateNamespaceResponse {
            namespace_arn: Some(arn),
            namespace_name: Some(name),
        })
    }

    #[allow(dead_code)]
    pub async fn describe_namespace(&self, req: DescribeNamespaceRequest) -> Result<DescribeNamespaceResponse, CloudmapError> {
        let state = self.inner.lock().await;
        let name = req.namespace_name.or_else(|| req.namespace_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| CloudmapError::ValidationException("Name or ARN required".to_string()))?;
        let stored = state.namespaces.get(&name)
            .ok_or_else(|| CloudmapError::ResourceNotFoundException(format!("Namespace {} not found", name)))?;
        Ok(DescribeNamespaceResponse {
            namespace: NamespaceDetail {
                namespace_name: stored.name.clone(),
                namespace_arn: stored.arn.clone(),
                status: "ACTIVE".to_string(),
                tags: Some(stored.tags.clone()),
            },
        })
    }

    #[allow(dead_code)]
    pub async fn list_namespaces(&self, _req: ListNamespacesRequest) -> Result<ListNamespacesResponse, CloudmapError> {
        let state = self.inner.lock().await;
        let items: Vec<NamespaceDetail> = state.namespaces.values().map(|s| NamespaceDetail {
            namespace_name: s.name.clone(),
            namespace_arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListNamespacesResponse {
            namespaces: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_namespace(&self, req: DeleteNamespaceRequest) -> Result<(), CloudmapError> {
        let mut state = self.inner.lock().await;
        let name = req.namespace_name.or_else(|| req.namespace_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| CloudmapError::ValidationException("Name or ARN required".to_string()))?;
        state.namespaces.remove(&name)
            .ok_or_else(|| CloudmapError::ResourceNotFoundException(format!("Namespace {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = CloudmapState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_namespace() {
        let state = CloudmapState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateNamespaceRequest::default();
        let result = state.create_namespace(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_describe_namespace_not_found() {
        let state = CloudmapState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DescribeNamespaceRequest::default();
        let result = state.describe_namespace(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_namespaces_empty() {
        let state = CloudmapState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = ListNamespacesRequest::default();
        let result = state.list_namespaces(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_namespace_not_found() {
        let state = CloudmapState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DeleteNamespaceRequest::default();
        let result = state.delete_namespace(req).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_namespace_create_and_list() {
        let state = CloudmapState::new("123456789012".to_string(), "us-east-1".to_string());
        let create_req = CreateNamespaceRequest::default();
        let _created = state.create_namespace(create_req).await.unwrap();
        let list_req = ListNamespacesRequest::default();
        let listed = state.list_namespaces(list_req).await.unwrap();
        let _ = listed;
    }

    #[tokio::test]
    async fn test_namespace_full_crud() {
        let state = CloudmapState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateNamespaceRequest::default();
        create_req.namespace_name = "test-crud-resource".to_string();
        let create_result = state.create_namespace(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let mut get_req = DescribeNamespaceRequest::default();
        get_req.namespace_name = Some("test-crud-resource".to_string());
        let get_result = state.describe_namespace(get_req).await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let mut del_req = DeleteNamespaceRequest::default();
        del_req.namespace_name = Some("test-crud-resource".to_string());
        let del_result = state.delete_namespace(del_req).await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
