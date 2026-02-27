use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::ElasticacheError;
use super::types::*;

#[allow(dead_code)]
struct ElasticacheStateInner {
    cache_clusters: HashMap<String, CacheClusterInfo>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
pub struct ElasticacheState {
    inner: Arc<Mutex<ElasticacheStateInner>>,
}

impl ElasticacheState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        ElasticacheState {
            inner: Arc::new(Mutex::new(ElasticacheStateInner {
                cache_clusters: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_cache_cluster(&self, name: String) -> Result<CacheClusterInfo, ElasticacheError> {
        let mut state = self.inner.lock().await;
        if state.cache_clusters.contains_key(&name) {
            return Err(ElasticacheError::ResourceAlreadyExistsException(format!("CacheCluster {} already exists", name)));
        }
        let arn = format!("arn:aws:elasticache:{}:{}:cache-clusters/{}", state.region, state.account_id, name);
        let info = CacheClusterInfo {
            cache_cluster_name: name.clone(),
            cache_cluster_arn: arn,
            status: "ACTIVE".to_string(),
        };
        state.cache_clusters.insert(name, info.clone());
        Ok(info)
    }

    #[allow(dead_code)]
    pub async fn describe_cache_cluster(&self, name: &str) -> Result<CacheClusterInfo, ElasticacheError> {
        let state = self.inner.lock().await;
        state.cache_clusters.get(name).cloned()
            .ok_or_else(|| ElasticacheError::ResourceNotFoundException(format!("CacheCluster {} not found", name)))
    }

    #[allow(dead_code)]
    pub async fn list_cache_clusters(&self) -> Result<Vec<CacheClusterInfo>, ElasticacheError> {
        let state = self.inner.lock().await;
        Ok(state.cache_clusters.values().cloned().collect())
    }

    #[allow(dead_code)]
    pub async fn delete_cache_cluster(&self, name: &str) -> Result<(), ElasticacheError> {
        let mut state = self.inner.lock().await;
        state.cache_clusters.remove(name)
            .ok_or_else(|| ElasticacheError::ResourceNotFoundException(format!("CacheCluster {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = ElasticacheState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_cache_cluster() {
        let state = ElasticacheState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.create_cache_cluster("test-resource".to_string()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_cache_cluster_duplicate() {
        let state = ElasticacheState::new("123456789012".to_string(), "us-east-1".to_string());
        state.create_cache_cluster("dup".to_string()).await.unwrap();
        let result = state.create_cache_cluster("dup".to_string()).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_describe_cache_cluster_not_found() {
        let state = ElasticacheState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.describe_cache_cluster("nonexistent").await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_cache_clusters() {
        let state = ElasticacheState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.list_cache_clusters().await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_cache_cluster_not_found() {
        let state = ElasticacheState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.delete_cache_cluster("nonexistent").await;
        assert!(result.is_err());
    }
}
