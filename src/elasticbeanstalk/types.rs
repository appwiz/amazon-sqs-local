mod _types {
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ApplicationInfo {
    pub application_name: String,
    pub application_arn: String,
    pub status: String,
}

}
pub use _types::*;
