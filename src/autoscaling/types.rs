mod _types {
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct AutoScalingGroupInfo {
    pub auto_scaling_group_name: String,
    pub auto_scaling_group_arn: String,
    pub status: String,
}

}
pub use _types::*;
