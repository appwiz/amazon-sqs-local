use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::RedshiftError;
use super::types::*;

#[allow(dead_code)]
struct RedshiftStateInner {
    clusters: HashMap<String, ClusterInfo>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
pub struct RedshiftState {
    inner: Arc<Mutex<RedshiftStateInner>>,
}

impl RedshiftState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        RedshiftState {
            inner: Arc::new(Mutex::new(RedshiftStateInner {
                clusters: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_cluster(&self, name: String) -> Result<ClusterInfo, RedshiftError> {
        let mut state = self.inner.lock().await;
        if state.clusters.contains_key(&name) {
            return Err(RedshiftError::ResourceAlreadyExistsException(format!("Cluster {} already exists", name)));
        }
        let arn = format!("arn:aws:redshift:{}:{}:clusters/{}", state.region, state.account_id, name);
        let info = ClusterInfo {
            cluster_name: name.clone(),
            cluster_arn: arn,
            status: "ACTIVE".to_string(),
        };
        state.clusters.insert(name, info.clone());
        Ok(info)
    }

    #[allow(dead_code)]
    pub async fn describe_cluster(&self, name: &str) -> Result<ClusterInfo, RedshiftError> {
        let state = self.inner.lock().await;
        state.clusters.get(name).cloned()
            .ok_or_else(|| RedshiftError::ResourceNotFoundException(format!("Cluster {} not found", name)))
    }

    #[allow(dead_code)]
    pub async fn list_clusters(&self) -> Result<Vec<ClusterInfo>, RedshiftError> {
        let state = self.inner.lock().await;
        Ok(state.clusters.values().cloned().collect())
    }

    #[allow(dead_code)]
    pub async fn delete_cluster(&self, name: &str) -> Result<(), RedshiftError> {
        let mut state = self.inner.lock().await;
        state.clusters.remove(name)
            .ok_or_else(|| RedshiftError::ResourceNotFoundException(format!("Cluster {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = RedshiftState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_cluster() {
        let state = RedshiftState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.create_cluster("test-resource".to_string()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_cluster_duplicate() {
        let state = RedshiftState::new("123456789012".to_string(), "us-east-1".to_string());
        state.create_cluster("dup".to_string()).await.unwrap();
        let result = state.create_cluster("dup".to_string()).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_describe_cluster_not_found() {
        let state = RedshiftState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.describe_cluster("nonexistent").await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_clusters() {
        let state = RedshiftState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.list_clusters().await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_cluster_not_found() {
        let state = RedshiftState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.delete_cluster("nonexistent").await;
        assert!(result.is_err());
    }
}
