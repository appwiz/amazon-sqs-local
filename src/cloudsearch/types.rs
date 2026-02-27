mod _types {
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct DomainInfo {
    pub domain_name: String,
    pub domain_arn: String,
    pub status: String,
}

}
pub use _types::*;
