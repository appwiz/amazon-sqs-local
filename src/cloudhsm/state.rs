use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::CloudhsmError;
use super::types::*;

#[allow(dead_code)]
struct CloudhsmStateInner {
    clusters: HashMap<String, StoredCluster>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredCluster {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct CloudhsmState {
    inner: Arc<Mutex<CloudhsmStateInner>>,
}

impl CloudhsmState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        CloudhsmState {
            inner: Arc::new(Mutex::new(CloudhsmStateInner {
                clusters: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_cluster(&self, req: CreateClusterRequest) -> Result<CreateClusterResponse, CloudhsmError> {
        let mut state = self.inner.lock().await;
        let name = req.cluster_name.clone();
        if state.clusters.contains_key(&name) {
            return Err(CloudhsmError::ResourceAlreadyExistsException(format!("Cluster {} already exists", name)));
        }
        let arn = format!("arn:aws:cloudhsmv2:{}:{}:clusters/{}", state.region, state.account_id, name);
        state.clusters.insert(name.clone(), StoredCluster {
            name: name.clone(),
            arn: arn.clone(),
            tags: req.tags.unwrap_or_default(),
        });
        Ok(CreateClusterResponse {
            cluster_arn: Some(arn),
            cluster_name: Some(name),
        })
    }

    #[allow(dead_code)]
    pub async fn describe_cluster(&self, req: DescribeClusterRequest) -> Result<DescribeClusterResponse, CloudhsmError> {
        let state = self.inner.lock().await;
        let name = req.cluster_name.or_else(|| req.cluster_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| CloudhsmError::ValidationException("Name or ARN required".to_string()))?;
        let stored = state.clusters.get(&name)
            .ok_or_else(|| CloudhsmError::ResourceNotFoundException(format!("Cluster {} not found", name)))?;
        Ok(DescribeClusterResponse {
            cluster: ClusterDetail {
                cluster_name: stored.name.clone(),
                cluster_arn: stored.arn.clone(),
                status: "ACTIVE".to_string(),
                tags: Some(stored.tags.clone()),
            },
        })
    }

    #[allow(dead_code)]
    pub async fn list_clusters(&self, _req: ListClustersRequest) -> Result<ListClustersResponse, CloudhsmError> {
        let state = self.inner.lock().await;
        let items: Vec<ClusterDetail> = state.clusters.values().map(|s| ClusterDetail {
            cluster_name: s.name.clone(),
            cluster_arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListClustersResponse {
            clusters: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_cluster(&self, req: DeleteClusterRequest) -> Result<(), CloudhsmError> {
        let mut state = self.inner.lock().await;
        let name = req.cluster_name.or_else(|| req.cluster_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| CloudhsmError::ValidationException("Name or ARN required".to_string()))?;
        state.clusters.remove(&name)
            .ok_or_else(|| CloudhsmError::ResourceNotFoundException(format!("Cluster {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = CloudhsmState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_cluster() {
        let state = CloudhsmState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateClusterRequest::default();
        let result = state.create_cluster(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_describe_cluster_not_found() {
        let state = CloudhsmState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DescribeClusterRequest::default();
        let result = state.describe_cluster(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_clusters_empty() {
        let state = CloudhsmState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = ListClustersRequest::default();
        let result = state.list_clusters(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_cluster_not_found() {
        let state = CloudhsmState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DeleteClusterRequest::default();
        let result = state.delete_cluster(req).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_cluster_create_and_list() {
        let state = CloudhsmState::new("123456789012".to_string(), "us-east-1".to_string());
        let create_req = CreateClusterRequest::default();
        let _created = state.create_cluster(create_req).await.unwrap();
        let list_req = ListClustersRequest::default();
        let listed = state.list_clusters(list_req).await.unwrap();
        let _ = listed;
    }

    #[tokio::test]
    async fn test_cluster_full_crud() {
        let state = CloudhsmState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateClusterRequest::default();
        create_req.cluster_name = "test-crud-resource".to_string();
        let create_result = state.create_cluster(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let mut get_req = DescribeClusterRequest::default();
        get_req.cluster_name = Some("test-crud-resource".to_string());
        let get_result = state.describe_cluster(get_req).await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let mut del_req = DeleteClusterRequest::default();
        del_req.cluster_name = Some("test-crud-resource".to_string());
        let del_result = state.delete_cluster(del_req).await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
