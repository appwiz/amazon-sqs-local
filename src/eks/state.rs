use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::EKSError;
use super::types::*;

#[allow(dead_code)]
struct EKSStateInner {
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
pub struct EKSState {
    inner: Arc<Mutex<EKSStateInner>>,
}

impl EKSState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        EKSState {
            inner: Arc::new(Mutex::new(EKSStateInner {
                clusters: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_cluster(&self, req: CreateClusterRequest) -> Result<ClusterDetail, EKSError> {
        let mut state = self.inner.lock().await;
        let name = req.name.or(req.name_pascal).unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        if state.clusters.contains_key(&name) {
            return Err(EKSError::ResourceAlreadyExistsException(format!("Cluster {} already exists", name)));
        }
        let arn = format!("arn:aws:eks:{}:{}:clusters/{}", state.region, state.account_id, name);
        let detail = ClusterDetail {
            name: name.clone(),
            arn: arn.clone(),
            status: "ACTIVE".to_string(),
            tags: req.tags.clone(),
        };
        state.clusters.insert(name, StoredCluster {
            name: detail.name.clone(),
            arn,
            tags: req.tags.unwrap_or_default(),
        });
        Ok(detail)
    }

    #[allow(dead_code)]
    pub async fn get_cluster(&self, name: &str) -> Result<ClusterDetail, EKSError> {
        let state = self.inner.lock().await;
        let stored = state.clusters.get(name)
            .ok_or_else(|| EKSError::ResourceNotFoundException(format!("Cluster {} not found", name)))?;
        Ok(ClusterDetail {
            name: stored.name.clone(),
            arn: stored.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(stored.tags.clone()),
        })
    }

    #[allow(dead_code)]
    pub async fn list_clusters(&self) -> Result<ListClustersResponse, EKSError> {
        let state = self.inner.lock().await;
        let items: Vec<ClusterDetail> = state.clusters.values().map(|s| ClusterDetail {
            name: s.name.clone(),
            arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListClustersResponse {
            items: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_cluster(&self, name: &str) -> Result<(), EKSError> {
        let mut state = self.inner.lock().await;
        state.clusters.remove(name)
            .ok_or_else(|| EKSError::ResourceNotFoundException(format!("Cluster {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = EKSState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_cluster() {
        let state = EKSState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateClusterRequest::default();
        let result = state.create_cluster(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_get_cluster_not_found() {
        let state = EKSState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.get_cluster("nonexistent").await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_clusters() {
        let state = EKSState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.list_clusters().await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_cluster_not_found() {
        let state = EKSState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.delete_cluster("nonexistent").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_cluster_full_crud() {
        let state = EKSState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateClusterRequest::default();
        create_req.name = Some("test-crud-resource".to_string());
        let create_result = state.create_cluster(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let get_result = state.get_cluster("test-crud-resource").await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let del_result = state.delete_cluster("test-crud-resource").await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
