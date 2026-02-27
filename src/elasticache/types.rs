mod _types {
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CacheClusterInfo {
    pub cache_cluster_name: String,
    pub cache_cluster_arn: String,
    pub status: String,
}

}
pub use _types::*;
