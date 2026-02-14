use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// --- CreateFunction ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateFunctionRequest {
    pub function_name: String,
    #[serde(default)]
    pub runtime: Option<String>,
    pub role: String,
    #[serde(default)]
    pub handler: Option<String>,
    pub code: FunctionCode,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub timeout: Option<i32>,
    #[serde(default)]
    pub memory_size: Option<i32>,
    #[serde(default)]
    pub environment: Option<Environment>,
    #[serde(default)]
    pub tags: Option<HashMap<String, String>>,
    #[serde(default)]
    pub package_type: Option<String>,
    #[serde(default)]
    pub architectures: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FunctionCode {
    #[serde(default)]
    pub zip_file: Option<String>,
    #[serde(default)]
    pub s3_bucket: Option<String>,
    #[serde(default)]
    pub s3_key: Option<String>,
    #[serde(default)]
    pub s3_object_version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Environment {
    #[serde(default)]
    pub variables: Option<HashMap<String, String>>,
}

// --- FunctionConfiguration (shared response type) ---

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct FunctionConfiguration {
    pub function_name: String,
    pub function_arn: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime: Option<String>,
    pub role: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handler: Option<String>,
    pub code_size: i64,
    pub description: String,
    pub timeout: i32,
    pub memory_size: i32,
    pub last_modified: String,
    pub code_sha256: String,
    pub version: String,
    pub state: String,
    pub package_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<Environment>,
    pub architectures: Vec<String>,
}

// --- GetFunction ---

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetFunctionResponse {
    pub configuration: FunctionConfiguration,
    pub code: FunctionCodeLocation,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct FunctionCodeLocation {
    pub location: String,
    pub repository_type: String,
}

// --- ListFunctions ---

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListFunctionsResponse {
    pub functions: Vec<FunctionConfiguration>,
}

// --- UpdateFunctionCode ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UpdateFunctionCodeRequest {
    #[serde(default)]
    pub zip_file: Option<String>,
    #[serde(default)]
    pub s3_bucket: Option<String>,
    #[serde(default)]
    pub s3_key: Option<String>,
    #[serde(default)]
    pub s3_object_version: Option<String>,
}

// --- UpdateFunctionConfiguration ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UpdateFunctionConfigurationRequest {
    #[serde(default)]
    pub runtime: Option<String>,
    #[serde(default)]
    pub role: Option<String>,
    #[serde(default)]
    pub handler: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub timeout: Option<i32>,
    #[serde(default)]
    pub memory_size: Option<i32>,
    #[serde(default)]
    pub environment: Option<Environment>,
}

// --- Invoke ---

#[derive(Debug, Clone, Serialize)]
pub struct InvokeResponse {
    #[serde(rename = "StatusCode")]
    pub status_code: i32,
}

// --- AddPermission ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AddPermissionRequest {
    pub statement_id: String,
    pub action: String,
    pub principal: String,
    #[serde(default)]
    pub source_arn: Option<String>,
    #[serde(default)]
    pub source_account: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct AddPermissionResponse {
    pub statement: String,
}

// --- GetPolicy ---

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetPolicyResponse {
    pub policy: String,
    pub revision_id: String,
}

// --- PublishVersion ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PublishVersionRequest {
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub code_sha256: Option<String>,
}

// --- CreateAlias ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateAliasRequest {
    pub name: String,
    pub function_version: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub routing_config: Option<AliasRoutingConfigRequest>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AliasRoutingConfigRequest {
    #[serde(default)]
    pub additional_version_weights: Option<HashMap<String, f64>>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct AliasResponse {
    pub alias_arn: String,
    pub name: String,
    pub function_version: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_config: Option<AliasRoutingConfigResponse>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct AliasRoutingConfigResponse {
    pub additional_version_weights: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListAliasesResponse {
    pub aliases: Vec<AliasResponse>,
}

// --- ListVersionsByFunction ---

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListVersionsResponse {
    pub versions: Vec<FunctionConfiguration>,
}

// --- EventSourceMapping ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateEventSourceMappingRequest {
    pub event_source_arn: String,
    pub function_name: String,
    #[serde(default)]
    pub batch_size: Option<i32>,
    #[serde(default)]
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct EventSourceMappingResponse {
    #[serde(rename = "UUID")]
    pub uuid: String,
    pub event_source_arn: String,
    pub function_arn: String,
    pub state: String,
    pub batch_size: i32,
    pub last_modified: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListEventSourceMappingsResponse {
    pub event_source_mappings: Vec<EventSourceMappingResponse>,
}

// --- TagResource ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TagResourceRequest {
    pub tags: HashMap<String, String>,
}

// --- ListTags ---

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListTagsResponse {
    pub tags: HashMap<String, String>,
}
