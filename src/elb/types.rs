mod _types {
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct LoadBalancerInfo {
    pub load_balancer_name: String,
    pub load_balancer_arn: String,
    pub status: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct TargetGroupInfo {
    pub target_group_name: String,
    pub target_group_arn: String,
    pub status: String,
}

}
pub use _types::*;
