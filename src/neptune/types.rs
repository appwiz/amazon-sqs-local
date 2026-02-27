mod _types {
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct DBClusterInfo {
    pub d_b_cluster_name: String,
    pub d_b_cluster_arn: String,
    pub status: String,
}

}
pub use _types::*;
