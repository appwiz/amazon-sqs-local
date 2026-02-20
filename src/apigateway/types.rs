use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// --- RestApi types ---

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RestApiOutput {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub created_date: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RestApisOutput {
    #[serde(rename = "item")]
    pub items: Vec<RestApiOutput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
}

// --- CreateRestApi ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRestApiRequest {
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub version: Option<String>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

// --- UpdateRestApi ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRestApiRequest {
    #[serde(default)]
    pub patch_operations: Vec<PatchOperation>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PatchOperation {
    pub op: String,
    pub path: String,
    #[serde(default)]
    pub value: Option<String>,
}

// --- Resource types ---

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceOutput {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_part: Option<String>,
    pub path: String,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub resource_methods: HashMap<String, MethodOutput>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourcesOutput {
    #[serde(rename = "item")]
    pub items: Vec<ResourceOutput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
}

// --- CreateResource ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateResourceRequest {
    pub path_part: String,
}

// --- Method types ---

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MethodOutput {
    pub http_method: String,
    pub authorization_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_id: Option<String>,
    pub api_key_required: bool,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub request_parameters: HashMap<String, bool>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub request_models: HashMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method_integration: Option<IntegrationOutput>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub method_responses: HashMap<String, MethodResponseOutput>,
}

// --- PutMethod ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PutMethodRequest {
    pub authorization_type: String,
    #[serde(default)]
    pub authorizer_id: Option<String>,
    #[serde(default)]
    pub api_key_required: bool,
    #[serde(default)]
    pub request_parameters: HashMap<String, bool>,
    #[serde(default)]
    pub request_models: HashMap<String, String>,
}

// --- Integration types ---

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IntegrationOutput {
    #[serde(rename = "type")]
    pub integration_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_method: Option<String>,
    pub passthrough_behavior: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_handling: Option<String>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub request_parameters: HashMap<String, String>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub request_templates: HashMap<String, String>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub integration_responses: HashMap<String, IntegrationResponseOutput>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IntegrationResponseOutput {
    pub status_code: String,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub response_parameters: HashMap<String, String>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub response_templates: HashMap<String, String>,
}

// --- PutIntegration ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PutIntegrationRequest {
    #[serde(rename = "type")]
    pub integration_type: String,
    #[serde(default)]
    pub uri: Option<String>,
    #[serde(rename = "httpMethod")]
    #[serde(default)]
    pub http_method: Option<String>,
    #[serde(default)]
    pub passthrough_behavior: Option<String>,
    #[serde(default)]
    pub content_handling: Option<String>,
    #[serde(default)]
    pub request_parameters: HashMap<String, String>,
    #[serde(default)]
    pub request_templates: HashMap<String, String>,
}

// --- MethodResponse types ---

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MethodResponseOutput {
    pub status_code: String,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub response_parameters: HashMap<String, bool>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub response_models: HashMap<String, String>,
}

// --- PutMethodResponse ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PutMethodResponseRequest {
    #[serde(default)]
    pub response_parameters: HashMap<String, bool>,
    #[serde(default)]
    pub response_models: HashMap<String, String>,
}

// --- PutIntegrationResponse ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PutIntegrationResponseRequest {
    #[serde(default)]
    pub selection_pattern: Option<String>,
    #[serde(default)]
    pub response_parameters: HashMap<String, String>,
    #[serde(default)]
    pub response_templates: HashMap<String, String>,
    #[serde(default)]
    pub content_handling: Option<String>,
}

// --- Deployment types ---

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeploymentOutput {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub created_date: f64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeploymentsOutput {
    #[serde(rename = "item")]
    pub items: Vec<DeploymentOutput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
}

// --- CreateDeployment ---

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CreateDeploymentRequest {
    #[serde(default)]
    pub stage_name: Option<String>,
    #[serde(default)]
    pub stage_description: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub variables: std::collections::HashMap<String, String>,
}

// --- Stage types ---

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StageOutput {
    pub stage_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub created_date: f64,
    pub last_updated_date: f64,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub variables: HashMap<String, String>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StagesOutput {
    pub item: Vec<StageOutput>,
}

// --- CreateStage ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateStageRequest {
    pub stage_name: String,
    pub deployment_id: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub variables: HashMap<String, String>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

// --- UpdateStage ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateStageRequest {
    #[serde(default)]
    pub patch_operations: Vec<PatchOperation>,
}

// --- Tag operations ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TagResourceRequest {
    pub tags: HashMap<String, String>,
}
