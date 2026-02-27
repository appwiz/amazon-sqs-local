mod _types {
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct InstanceInfo {
    pub instance_name: String,
    pub instance_arn: String,
    pub status: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct VpcInfo {
    pub vpc_name: String,
    pub vpc_arn: String,
    pub status: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct SecurityGroupInfo {
    pub security_group_name: String,
    pub security_group_arn: String,
    pub status: String,
}

}
pub use _types::*;
