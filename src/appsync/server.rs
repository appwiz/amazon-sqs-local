use std::sync::Arc;

use axum::body::Bytes;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{delete, get, post};
use axum::{Json, Router};
use serde::Deserialize;

use super::error::AppSyncError;
use super::state::AppSyncState;
use super::types::*;

// --- GraphQL API handlers ---

// POST /v1/apis
async fn create_graphql_api(
    State(state): State<Arc<AppSyncState>>,
    body: Bytes,
) -> Result<axum::response::Response, AppSyncError> {
    let req: CreateGraphqlApiRequest =
        serde_json::from_slice(&body).map_err(|e| AppSyncError::BadRequestException(e.to_string()))?;
    let api = state.create_graphql_api(req).await?;
    let resp = GraphqlApiResponse { graphql_api: api };
    Ok((StatusCode::OK, Json(serde_json::to_value(resp).unwrap())).into_response())
}

// GET /v1/apis/{apiId}
async fn get_graphql_api(
    State(state): State<Arc<AppSyncState>>,
    Path(api_id): Path<String>,
) -> Result<axum::response::Response, AppSyncError> {
    let api = state.get_graphql_api(&api_id).await?;
    let resp = GraphqlApiResponse { graphql_api: api };
    Ok((StatusCode::OK, Json(serde_json::to_value(resp).unwrap())).into_response())
}

// GET /v1/apis
async fn list_graphql_apis(
    State(state): State<Arc<AppSyncState>>,
) -> Result<axum::response::Response, AppSyncError> {
    let apis = state.list_graphql_apis().await?;
    let resp = ListGraphqlApisResponse { graphql_apis: apis };
    Ok((StatusCode::OK, Json(serde_json::to_value(resp).unwrap())).into_response())
}

// POST /v1/apis/{apiId}
async fn update_graphql_api(
    State(state): State<Arc<AppSyncState>>,
    Path(api_id): Path<String>,
    body: Bytes,
) -> Result<axum::response::Response, AppSyncError> {
    let req: UpdateGraphqlApiRequest =
        serde_json::from_slice(&body).map_err(|e| AppSyncError::BadRequestException(e.to_string()))?;
    let api = state.update_graphql_api(&api_id, req).await?;
    let resp = GraphqlApiResponse { graphql_api: api };
    Ok((StatusCode::OK, Json(serde_json::to_value(resp).unwrap())).into_response())
}

// DELETE /v1/apis/{apiId}
async fn delete_graphql_api(
    State(state): State<Arc<AppSyncState>>,
    Path(api_id): Path<String>,
) -> Result<axum::response::Response, AppSyncError> {
    state.delete_graphql_api(&api_id).await?;
    Ok(StatusCode::OK.into_response())
}

// --- API Key handlers ---

// POST /v1/apis/{apiId}/apikeys
async fn create_api_key(
    State(state): State<Arc<AppSyncState>>,
    Path(api_id): Path<String>,
    body: Bytes,
) -> Result<axum::response::Response, AppSyncError> {
    let req: CreateApiKeyRequest = if body.is_empty() {
        CreateApiKeyRequest {
            description: None,
            expires: None,
        }
    } else {
        serde_json::from_slice(&body)
            .map_err(|e| AppSyncError::BadRequestException(e.to_string()))?
    };
    let key = state.create_api_key(&api_id, req).await?;
    let resp = ApiKeyResponse { api_key: key };
    Ok((StatusCode::OK, Json(serde_json::to_value(resp).unwrap())).into_response())
}

// GET /v1/apis/{apiId}/apikeys
async fn list_api_keys(
    State(state): State<Arc<AppSyncState>>,
    Path(api_id): Path<String>,
) -> Result<axum::response::Response, AppSyncError> {
    let keys = state.list_api_keys(&api_id).await?;
    let resp = ListApiKeysResponse { api_keys: keys };
    Ok((StatusCode::OK, Json(serde_json::to_value(resp).unwrap())).into_response())
}

// POST /v1/apis/{apiId}/apikeys/{id}
async fn update_api_key(
    State(state): State<Arc<AppSyncState>>,
    Path((api_id, key_id)): Path<(String, String)>,
    body: Bytes,
) -> Result<axum::response::Response, AppSyncError> {
    let req: UpdateApiKeyRequest =
        serde_json::from_slice(&body).map_err(|e| AppSyncError::BadRequestException(e.to_string()))?;
    let key = state.update_api_key(&api_id, &key_id, req).await?;
    let resp = ApiKeyResponse { api_key: key };
    Ok((StatusCode::OK, Json(serde_json::to_value(resp).unwrap())).into_response())
}

// DELETE /v1/apis/{apiId}/apikeys/{id}
async fn delete_api_key(
    State(state): State<Arc<AppSyncState>>,
    Path((api_id, key_id)): Path<(String, String)>,
) -> Result<axum::response::Response, AppSyncError> {
    state.delete_api_key(&api_id, &key_id).await?;
    Ok(StatusCode::OK.into_response())
}

// --- Data Source handlers ---

// POST /v1/apis/{apiId}/datasources
async fn create_data_source(
    State(state): State<Arc<AppSyncState>>,
    Path(api_id): Path<String>,
    body: Bytes,
) -> Result<axum::response::Response, AppSyncError> {
    let req: CreateDataSourceRequest =
        serde_json::from_slice(&body).map_err(|e| AppSyncError::BadRequestException(e.to_string()))?;
    let ds = state.create_data_source(&api_id, req).await?;
    let resp = DataSourceResponse { data_source: ds };
    Ok((StatusCode::OK, Json(serde_json::to_value(resp).unwrap())).into_response())
}

// GET /v1/apis/{apiId}/datasources/{name}
async fn get_data_source(
    State(state): State<Arc<AppSyncState>>,
    Path((api_id, name)): Path<(String, String)>,
) -> Result<axum::response::Response, AppSyncError> {
    let ds = state.get_data_source(&api_id, &name).await?;
    let resp = DataSourceResponse { data_source: ds };
    Ok((StatusCode::OK, Json(serde_json::to_value(resp).unwrap())).into_response())
}

// GET /v1/apis/{apiId}/datasources
async fn list_data_sources(
    State(state): State<Arc<AppSyncState>>,
    Path(api_id): Path<String>,
) -> Result<axum::response::Response, AppSyncError> {
    let sources = state.list_data_sources(&api_id).await?;
    let resp = ListDataSourcesResponse {
        data_sources: sources,
    };
    Ok((StatusCode::OK, Json(serde_json::to_value(resp).unwrap())).into_response())
}

// POST /v1/apis/{apiId}/datasources/{name}
async fn update_data_source(
    State(state): State<Arc<AppSyncState>>,
    Path((api_id, name)): Path<(String, String)>,
    body: Bytes,
) -> Result<axum::response::Response, AppSyncError> {
    let req: UpdateDataSourceRequest =
        serde_json::from_slice(&body).map_err(|e| AppSyncError::BadRequestException(e.to_string()))?;
    let ds = state.update_data_source(&api_id, &name, req).await?;
    let resp = DataSourceResponse { data_source: ds };
    Ok((StatusCode::OK, Json(serde_json::to_value(resp).unwrap())).into_response())
}

// DELETE /v1/apis/{apiId}/datasources/{name}
async fn delete_data_source(
    State(state): State<Arc<AppSyncState>>,
    Path((api_id, name)): Path<(String, String)>,
) -> Result<axum::response::Response, AppSyncError> {
    state.delete_data_source(&api_id, &name).await?;
    Ok(StatusCode::OK.into_response())
}

// --- Schema handlers ---

// POST /v1/apis/{apiId}/schemacreation
async fn start_schema_creation(
    State(state): State<Arc<AppSyncState>>,
    Path(api_id): Path<String>,
    body: Bytes,
) -> Result<axum::response::Response, AppSyncError> {
    let req: StartSchemaCreationRequest =
        serde_json::from_slice(&body).map_err(|e| AppSyncError::BadRequestException(e.to_string()))?;
    let info = state.start_schema_creation(&api_id, req).await?;
    let resp = SchemaCreationResponse {
        status: info.status,
    };
    Ok((StatusCode::OK, Json(serde_json::to_value(resp).unwrap())).into_response())
}

// GET /v1/apis/{apiId}/schemacreation
async fn get_schema_creation_status(
    State(state): State<Arc<AppSyncState>>,
    Path(api_id): Path<String>,
) -> Result<axum::response::Response, AppSyncError> {
    let info = state.get_schema_creation_status(&api_id).await?;
    let resp = SchemaCreationStatusResponse {
        status: info.status,
        details: info.details,
    };
    Ok((StatusCode::OK, Json(serde_json::to_value(resp).unwrap())).into_response())
}

// --- Tag handlers ---

// POST /v1/tags/{resourceArn}
async fn tag_resource(
    State(state): State<Arc<AppSyncState>>,
    Path(resource_arn): Path<String>,
    body: Bytes,
) -> Result<axum::response::Response, AppSyncError> {
    let req: TagResourceRequest =
        serde_json::from_slice(&body).map_err(|e| AppSyncError::BadRequestException(e.to_string()))?;
    state.tag_resource(&resource_arn, req).await?;
    Ok(StatusCode::OK.into_response())
}

#[derive(Deserialize)]
struct UntagQuery {
    #[serde(rename = "tagKeys")]
    tag_keys: Option<String>,
}

// DELETE /v1/tags/{resourceArn}
async fn untag_resource(
    State(state): State<Arc<AppSyncState>>,
    Path(resource_arn): Path<String>,
    Query(query): Query<UntagQuery>,
) -> Result<axum::response::Response, AppSyncError> {
    let tag_keys: Vec<String> = query
        .tag_keys
        .map(|s| s.split(',').map(|k| k.to_string()).collect())
        .unwrap_or_default();
    state.untag_resource(&resource_arn, tag_keys).await?;
    Ok(StatusCode::OK.into_response())
}

// GET /v1/tags/{resourceArn}
async fn list_tags_for_resource(
    State(state): State<Arc<AppSyncState>>,
    Path(resource_arn): Path<String>,
) -> Result<axum::response::Response, AppSyncError> {
    let tags = state.list_tags_for_resource(&resource_arn).await?;
    let resp = TagsResponse { tags };
    Ok((StatusCode::OK, Json(serde_json::to_value(resp).unwrap())).into_response())
}

pub fn create_router(state: Arc<AppSyncState>) -> Router {
    Router::new()
        // GraphQL API routes
        .route("/v1/apis", post(create_graphql_api))
        .route("/v1/apis", get(list_graphql_apis))
        .route("/v1/apis/{apiId}", get(get_graphql_api))
        .route("/v1/apis/{apiId}", post(update_graphql_api))
        .route("/v1/apis/{apiId}", delete(delete_graphql_api))
        // API Key routes
        .route("/v1/apis/{apiId}/apikeys", post(create_api_key))
        .route("/v1/apis/{apiId}/apikeys", get(list_api_keys))
        .route("/v1/apis/{apiId}/apikeys/{keyId}", post(update_api_key))
        .route("/v1/apis/{apiId}/apikeys/{keyId}", delete(delete_api_key))
        // Data Source routes
        .route("/v1/apis/{apiId}/datasources", post(create_data_source))
        .route("/v1/apis/{apiId}/datasources", get(list_data_sources))
        .route("/v1/apis/{apiId}/datasources/{name}", get(get_data_source))
        .route("/v1/apis/{apiId}/datasources/{name}", post(update_data_source))
        .route("/v1/apis/{apiId}/datasources/{name}", delete(delete_data_source))
        // Schema routes
        .route("/v1/apis/{apiId}/schemacreation", post(start_schema_creation))
        .route("/v1/apis/{apiId}/schemacreation", get(get_schema_creation_status))
        // Tag routes
        .route("/v1/tags/{resourceArn}", post(tag_resource))
        .route("/v1/tags/{resourceArn}", delete(untag_resource))
        .route("/v1/tags/{resourceArn}", get(list_tags_for_resource))
        .with_state(state)
}
