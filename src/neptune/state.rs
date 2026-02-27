use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::NeptuneError;
use super::types::*;

#[allow(dead_code)]
struct NeptuneStateInner {
    d_b_clusters: HashMap<String, DBClusterInfo>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
pub struct NeptuneState {
    inner: Arc<Mutex<NeptuneStateInner>>,
}

impl NeptuneState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        NeptuneState {
            inner: Arc::new(Mutex::new(NeptuneStateInner {
                d_b_clusters: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_d_b_cluster(&self, name: String) -> Result<DBClusterInfo, NeptuneError> {
        let mut state = self.inner.lock().await;
        if state.d_b_clusters.contains_key(&name) {
            return Err(NeptuneError::ResourceAlreadyExistsException(format!("DBCluster {} already exists", name)));
        }
        let arn = format!("arn:aws:neptune:{}:{}:db-clusters/{}", state.region, state.account_id, name);
        let info = DBClusterInfo {
            d_b_cluster_name: name.clone(),
            d_b_cluster_arn: arn,
            status: "ACTIVE".to_string(),
        };
        state.d_b_clusters.insert(name, info.clone());
        Ok(info)
    }

    #[allow(dead_code)]
    pub async fn describe_d_b_cluster(&self, name: &str) -> Result<DBClusterInfo, NeptuneError> {
        let state = self.inner.lock().await;
        state.d_b_clusters.get(name).cloned()
            .ok_or_else(|| NeptuneError::ResourceNotFoundException(format!("DBCluster {} not found", name)))
    }

    #[allow(dead_code)]
    pub async fn list_d_b_clusters(&self) -> Result<Vec<DBClusterInfo>, NeptuneError> {
        let state = self.inner.lock().await;
        Ok(state.d_b_clusters.values().cloned().collect())
    }

    #[allow(dead_code)]
    pub async fn delete_d_b_cluster(&self, name: &str) -> Result<(), NeptuneError> {
        let mut state = self.inner.lock().await;
        state.d_b_clusters.remove(name)
            .ok_or_else(|| NeptuneError::ResourceNotFoundException(format!("DBCluster {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = NeptuneState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_d_b_cluster() {
        let state = NeptuneState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.create_d_b_cluster("test-resource".to_string()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_d_b_cluster_duplicate() {
        let state = NeptuneState::new("123456789012".to_string(), "us-east-1".to_string());
        state.create_d_b_cluster("dup".to_string()).await.unwrap();
        let result = state.create_d_b_cluster("dup".to_string()).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_describe_d_b_cluster_not_found() {
        let state = NeptuneState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.describe_d_b_cluster("nonexistent").await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_d_b_clusters() {
        let state = NeptuneState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.list_d_b_clusters().await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_d_b_cluster_not_found() {
        let state = NeptuneState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.delete_d_b_cluster("nonexistent").await;
        assert!(result.is_err());
    }
}
