mod _types {
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct UserInfo {
    pub user_name: String,
    pub user_arn: String,
    pub status: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct RoleInfo {
    pub role_name: String,
    pub role_arn: String,
    pub status: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct PolicyInfo {
    pub policy_name: String,
    pub policy_arn: String,
    pub status: String,
}

}
pub use _types::*;
