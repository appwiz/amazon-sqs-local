use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::MediastoreError;
use super::types::*;

#[allow(dead_code)]
struct MediastoreStateInner {
    containers: HashMap<String, StoredContainer>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredContainer {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct MediastoreState {
    inner: Arc<Mutex<MediastoreStateInner>>,
}

impl MediastoreState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        MediastoreState {
            inner: Arc::new(Mutex::new(MediastoreStateInner {
                containers: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_container(&self, req: CreateContainerRequest) -> Result<CreateContainerResponse, MediastoreError> {
        let mut state = self.inner.lock().await;
        let name = req.container_name.clone();
        if state.containers.contains_key(&name) {
            return Err(MediastoreError::ResourceAlreadyExistsException(format!("Container {} already exists", name)));
        }
        let arn = format!("arn:aws:mediastore:{}:{}:containers/{}", state.region, state.account_id, name);
        state.containers.insert(name.clone(), StoredContainer {
            name: name.clone(),
            arn: arn.clone(),
            tags: req.tags.unwrap_or_default(),
        });
        Ok(CreateContainerResponse {
            container_arn: Some(arn),
            container_name: Some(name),
        })
    }

    #[allow(dead_code)]
    pub async fn describe_container(&self, req: DescribeContainerRequest) -> Result<DescribeContainerResponse, MediastoreError> {
        let state = self.inner.lock().await;
        let name = req.container_name.or_else(|| req.container_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| MediastoreError::ValidationException("Name or ARN required".to_string()))?;
        let stored = state.containers.get(&name)
            .ok_or_else(|| MediastoreError::ResourceNotFoundException(format!("Container {} not found", name)))?;
        Ok(DescribeContainerResponse {
            container: ContainerDetail {
                container_name: stored.name.clone(),
                container_arn: stored.arn.clone(),
                status: "ACTIVE".to_string(),
                tags: Some(stored.tags.clone()),
            },
        })
    }

    #[allow(dead_code)]
    pub async fn list_containers(&self, _req: ListContainersRequest) -> Result<ListContainersResponse, MediastoreError> {
        let state = self.inner.lock().await;
        let items: Vec<ContainerDetail> = state.containers.values().map(|s| ContainerDetail {
            container_name: s.name.clone(),
            container_arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListContainersResponse {
            containers: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_container(&self, req: DeleteContainerRequest) -> Result<(), MediastoreError> {
        let mut state = self.inner.lock().await;
        let name = req.container_name.or_else(|| req.container_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| MediastoreError::ValidationException("Name or ARN required".to_string()))?;
        state.containers.remove(&name)
            .ok_or_else(|| MediastoreError::ResourceNotFoundException(format!("Container {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = MediastoreState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_container() {
        let state = MediastoreState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateContainerRequest::default();
        let result = state.create_container(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_describe_container_not_found() {
        let state = MediastoreState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DescribeContainerRequest::default();
        let result = state.describe_container(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_containers_empty() {
        let state = MediastoreState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = ListContainersRequest::default();
        let result = state.list_containers(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_container_not_found() {
        let state = MediastoreState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DeleteContainerRequest::default();
        let result = state.delete_container(req).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_container_create_and_list() {
        let state = MediastoreState::new("123456789012".to_string(), "us-east-1".to_string());
        let create_req = CreateContainerRequest::default();
        let _created = state.create_container(create_req).await.unwrap();
        let list_req = ListContainersRequest::default();
        let listed = state.list_containers(list_req).await.unwrap();
        let _ = listed;
    }

    #[tokio::test]
    async fn test_container_full_crud() {
        let state = MediastoreState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateContainerRequest::default();
        create_req.container_name = "test-crud-resource".to_string();
        let create_result = state.create_container(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let mut get_req = DescribeContainerRequest::default();
        get_req.container_name = Some("test-crud-resource".to_string());
        let get_result = state.describe_container(get_req).await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let mut del_req = DeleteContainerRequest::default();
        del_req.container_name = Some("test-crud-resource".to_string());
        let del_result = state.delete_container(del_req).await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
