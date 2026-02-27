mod _types {
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct AlarmInfo {
    pub alarm_name: String,
    pub alarm_arn: String,
    pub status: String,
}

}
pub use _types::*;
