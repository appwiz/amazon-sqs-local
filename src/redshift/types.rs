mod _types {
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ClusterInfo {
    pub cluster_name: String,
    pub cluster_arn: String,
    pub status: String,
}

}
pub use _types::*;
