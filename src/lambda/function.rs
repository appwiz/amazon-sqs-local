use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct LambdaFunction {
    pub function_name: String,
    pub function_arn: String,
    pub runtime: Option<String>,
    pub role: String,
    pub handler: Option<String>,
    pub code_size: i64,
    pub code_sha256: String,
    pub description: String,
    pub timeout: i32,
    pub memory_size: i32,
    pub last_modified: String,
    pub version: String,
    pub state: String,
    pub package_type: String,
    pub environment: HashMap<String, String>,
    pub tags: HashMap<String, String>,
    pub code: Vec<u8>,
    pub architectures: Vec<String>,
    pub versions: Vec<PublishedVersion>,
    pub aliases: HashMap<String, Alias>,
    pub policy_statements: Vec<PolicyStatement>,
}

#[derive(Debug, Clone)]
pub struct PublishedVersion {
    pub version: String,
    pub code_sha256: String,
    pub code_size: i64,
    pub description: String,
    pub runtime: Option<String>,
    pub role: String,
    pub handler: Option<String>,
    pub timeout: i32,
    pub memory_size: i32,
    pub last_modified: String,
    pub environment: HashMap<String, String>,
    pub architectures: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Alias {
    pub name: String,
    pub function_version: String,
    pub description: String,
    pub routing_config: Option<AliasRoutingConfig>,
}

#[derive(Debug, Clone)]
pub struct AliasRoutingConfig {
    pub additional_version_weights: HashMap<String, f64>,
}

#[derive(Debug, Clone)]
pub struct PolicyStatement {
    pub sid: String,
    pub effect: String,
    pub principal: serde_json::Value,
    pub action: String,
    pub resource: String,
}

#[derive(Debug, Clone)]
pub struct EventSourceMapping {
    pub uuid: String,
    pub event_source_arn: String,
    pub function_arn: String,
    pub state: String,
    pub batch_size: i32,
    pub last_modified: String,
}
