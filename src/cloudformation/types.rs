mod _types {
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct StackInfo {
    pub stack_name: String,
    pub stack_arn: String,
    pub status: String,
}

}
pub use _types::*;
