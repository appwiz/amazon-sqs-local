use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::DocumentdbError;
use super::types::*;

#[allow(dead_code)]
struct DocumentdbStateInner {
    d_b_clusters: HashMap<String, DBClusterInfo>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
pub struct DocumentdbState {
    inner: Arc<Mutex<DocumentdbStateInner>>,
}

impl DocumentdbState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        DocumentdbState {
            inner: Arc::new(Mutex::new(DocumentdbStateInner {
                d_b_clusters: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_d_b_cluster(&self, name: String) -> Result<DBClusterInfo, DocumentdbError> {
        let mut state = self.inner.lock().await;
        if state.d_b_clusters.contains_key(&name) {
            return Err(DocumentdbError::ResourceAlreadyExistsException(format!("DBCluster {} already exists", name)));
        }
        let arn = format!("arn:aws:rds:{}:{}:db-clusters/{}", state.region, state.account_id, name);
        let info = DBClusterInfo {
            d_b_cluster_name: name.clone(),
            d_b_cluster_arn: arn,
            status: "ACTIVE".to_string(),
        };
        state.d_b_clusters.insert(name, info.clone());
        Ok(info)
    }

    #[allow(dead_code)]
    pub async fn describe_d_b_cluster(&self, name: &str) -> Result<DBClusterInfo, DocumentdbError> {
        let state = self.inner.lock().await;
        state.d_b_clusters.get(name).cloned()
            .ok_or_else(|| DocumentdbError::ResourceNotFoundException(format!("DBCluster {} not found", name)))
    }

    #[allow(dead_code)]
    pub async fn list_d_b_clusters(&self) -> Result<Vec<DBClusterInfo>, DocumentdbError> {
        let state = self.inner.lock().await;
        Ok(state.d_b_clusters.values().cloned().collect())
    }

    #[allow(dead_code)]
    pub async fn delete_d_b_cluster(&self, name: &str) -> Result<(), DocumentdbError> {
        let mut state = self.inner.lock().await;
        state.d_b_clusters.remove(name)
            .ok_or_else(|| DocumentdbError::ResourceNotFoundException(format!("DBCluster {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = DocumentdbState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_d_b_cluster() {
        let state = DocumentdbState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.create_d_b_cluster("test-resource".to_string()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_d_b_cluster_duplicate() {
        let state = DocumentdbState::new("123456789012".to_string(), "us-east-1".to_string());
        state.create_d_b_cluster("dup".to_string()).await.unwrap();
        let result = state.create_d_b_cluster("dup".to_string()).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_describe_d_b_cluster_not_found() {
        let state = DocumentdbState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.describe_d_b_cluster("nonexistent").await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_d_b_clusters() {
        let state = DocumentdbState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.list_d_b_clusters().await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_d_b_cluster_not_found() {
        let state = DocumentdbState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.delete_d_b_cluster("nonexistent").await;
        assert!(result.is_err());
    }
}
