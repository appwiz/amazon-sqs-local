use serde::{Deserialize, Serialize};

// --- GraphQL API ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphqlApi {
    #[serde(rename = "apiId")]
    pub api_id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "authenticationType")]
    pub authentication_type: String,
    #[serde(rename = "arn")]
    pub arn: String,
    #[serde(rename = "uris")]
    pub uris: std::collections::HashMap<String, String>,
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "xrayEnabled")]
    pub xray_enabled: bool,
    #[serde(rename = "apiType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_type: Option<String>,
}

// --- API Key ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKey {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "expires")]
    pub expires: i64,
    #[serde(rename = "deletes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletes: Option<i64>,
}

// --- Data Source ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSource {
    #[serde(rename = "dataSourceArn")]
    pub data_source_arn: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "type")]
    pub ds_type: String,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "serviceRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role_arn: Option<String>,
}

// --- Schema ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaInfo {
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "details")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}

// --- Request types ---

#[derive(Debug, Deserialize)]
pub struct CreateGraphqlApiRequest {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "authenticationType")]
    pub authentication_type: Option<String>,
    #[serde(rename = "tags")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "xrayEnabled")]
    pub xray_enabled: Option<bool>,
    #[serde(rename = "apiType")]
    pub api_type: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateGraphqlApiRequest {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "authenticationType")]
    pub authentication_type: Option<String>,
    #[serde(rename = "xrayEnabled")]
    pub xray_enabled: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct CreateApiKeyRequest {
    #[serde(rename = "description")]
    pub description: Option<String>,
    #[serde(rename = "expires")]
    pub expires: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateApiKeyRequest {
    #[serde(rename = "description")]
    pub description: Option<String>,
    #[serde(rename = "expires")]
    pub expires: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct CreateDataSourceRequest {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "type")]
    pub ds_type: String,
    #[serde(rename = "description")]
    pub description: Option<String>,
    #[serde(rename = "serviceRoleArn")]
    pub service_role_arn: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateDataSourceRequest {
    #[serde(rename = "type")]
    pub ds_type: String,
    #[serde(rename = "description")]
    pub description: Option<String>,
    #[serde(rename = "serviceRoleArn")]
    pub service_role_arn: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct StartSchemaCreationRequest {
    #[serde(rename = "definition")]
    pub _definition: String,
}

#[derive(Debug, Deserialize)]
pub struct TagResourceRequest {
    #[serde(rename = "tags")]
    pub tags: std::collections::HashMap<String, String>,
}

// --- Response types ---

#[derive(Debug, Serialize)]
pub struct GraphqlApiResponse {
    #[serde(rename = "graphqlApi")]
    pub graphql_api: GraphqlApi,
}

#[derive(Debug, Serialize)]
pub struct ListGraphqlApisResponse {
    #[serde(rename = "graphqlApis")]
    pub graphql_apis: Vec<GraphqlApi>,
}

#[derive(Debug, Serialize)]
pub struct ApiKeyResponse {
    #[serde(rename = "apiKey")]
    pub api_key: ApiKey,
}

#[derive(Debug, Serialize)]
pub struct ListApiKeysResponse {
    #[serde(rename = "apiKeys")]
    pub api_keys: Vec<ApiKey>,
}

#[derive(Debug, Serialize)]
pub struct DataSourceResponse {
    #[serde(rename = "dataSource")]
    pub data_source: DataSource,
}

#[derive(Debug, Serialize)]
pub struct ListDataSourcesResponse {
    #[serde(rename = "dataSources")]
    pub data_sources: Vec<DataSource>,
}

#[derive(Debug, Serialize)]
pub struct SchemaCreationResponse {
    #[serde(rename = "status")]
    pub status: String,
}

#[derive(Debug, Serialize)]
pub struct SchemaCreationStatusResponse {
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "details")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TagsResponse {
    #[serde(rename = "tags")]
    pub tags: std::collections::HashMap<String, String>,
}
