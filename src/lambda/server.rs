use std::sync::Arc;

use axum::body::Bytes;
use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::{delete, get, post, put};
use axum::{Json, Router};

use super::error::LambdaError;
use super::state::LambdaState;
use super::types::*;

fn json_response<T: serde::Serialize>(status: StatusCode, value: &T) -> Response {
    (status, Json(serde_json::to_value(value).unwrap())).into_response()
}

// --- Function handlers ---

async fn create_function_handler(
    State(state): State<Arc<LambdaState>>,
    Json(req): Json<CreateFunctionRequest>,
) -> Result<Response, LambdaError> {
    let config = state.create_function(req).await?;
    Ok(json_response(StatusCode::CREATED, &config))
}

async fn list_functions_handler(
    State(state): State<Arc<LambdaState>>,
) -> Result<Response, LambdaError> {
    let resp = state.list_functions().await?;
    Ok(json_response(StatusCode::OK, &resp))
}

async fn get_function_handler(
    State(state): State<Arc<LambdaState>>,
    Path(function_name): Path<String>,
) -> Result<Response, LambdaError> {
    let resp = state.get_function(&function_name).await?;
    Ok(json_response(StatusCode::OK, &resp))
}

async fn delete_function_handler(
    State(state): State<Arc<LambdaState>>,
    Path(function_name): Path<String>,
) -> Result<Response, LambdaError> {
    state.delete_function(&function_name).await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}

async fn update_function_code_handler(
    State(state): State<Arc<LambdaState>>,
    Path(function_name): Path<String>,
    Json(req): Json<UpdateFunctionCodeRequest>,
) -> Result<Response, LambdaError> {
    let config = state.update_function_code(&function_name, req).await?;
    Ok(json_response(StatusCode::OK, &config))
}

async fn update_function_configuration_handler(
    State(state): State<Arc<LambdaState>>,
    Path(function_name): Path<String>,
    Json(req): Json<UpdateFunctionConfigurationRequest>,
) -> Result<Response, LambdaError> {
    let config = state
        .update_function_configuration(&function_name, req)
        .await?;
    Ok(json_response(StatusCode::OK, &config))
}

// --- Invoke ---

async fn invoke_handler(
    State(state): State<Arc<LambdaState>>,
    Path(function_name): Path<String>,
    headers: HeaderMap,
    _body: Bytes,
) -> Result<Response, LambdaError> {
    let invocation_type = headers
        .get("X-Amz-Invocation-Type")
        .and_then(|v| v.to_str().ok());

    let (status, body) = state.invoke(&function_name, invocation_type).await?;

    Ok((
        status,
        [("X-Amz-Executed-Version", "$LATEST")],
        body,
    )
        .into_response())
}

// --- Permission handlers ---

async fn add_permission_handler(
    State(state): State<Arc<LambdaState>>,
    Path(function_name): Path<String>,
    Json(req): Json<AddPermissionRequest>,
) -> Result<Response, LambdaError> {
    let resp = state.add_permission(&function_name, req).await?;
    Ok(json_response(StatusCode::CREATED, &resp))
}

async fn remove_permission_handler(
    State(state): State<Arc<LambdaState>>,
    Path((function_name, statement_id)): Path<(String, String)>,
) -> Result<Response, LambdaError> {
    state
        .remove_permission(&function_name, &statement_id)
        .await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}

async fn get_policy_handler(
    State(state): State<Arc<LambdaState>>,
    Path(function_name): Path<String>,
) -> Result<Response, LambdaError> {
    let resp = state.get_policy(&function_name).await?;
    Ok(json_response(StatusCode::OK, &resp))
}

// --- Version handlers ---

async fn publish_version_handler(
    State(state): State<Arc<LambdaState>>,
    Path(function_name): Path<String>,
    body: Bytes,
) -> Result<Response, LambdaError> {
    let req: PublishVersionRequest = if body.is_empty() {
        PublishVersionRequest {
            description: None,
            code_sha256: None,
        }
    } else {
        serde_json::from_slice(&body)
            .map_err(|e| LambdaError::InvalidParameterValueException(e.to_string()))?
    };
    let config = state.publish_version(&function_name, req).await?;
    Ok(json_response(StatusCode::CREATED, &config))
}

async fn list_versions_handler(
    State(state): State<Arc<LambdaState>>,
    Path(function_name): Path<String>,
) -> Result<Response, LambdaError> {
    let resp = state.list_versions_by_function(&function_name).await?;
    Ok(json_response(StatusCode::OK, &resp))
}

// --- Alias handlers ---

async fn create_alias_handler(
    State(state): State<Arc<LambdaState>>,
    Path(function_name): Path<String>,
    Json(req): Json<CreateAliasRequest>,
) -> Result<Response, LambdaError> {
    let resp = state.create_alias(&function_name, req).await?;
    Ok(json_response(StatusCode::CREATED, &resp))
}

async fn delete_alias_handler(
    State(state): State<Arc<LambdaState>>,
    Path((function_name, alias_name)): Path<(String, String)>,
) -> Result<Response, LambdaError> {
    state.delete_alias(&function_name, &alias_name).await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}

async fn get_alias_handler(
    State(state): State<Arc<LambdaState>>,
    Path((function_name, alias_name)): Path<(String, String)>,
) -> Result<Response, LambdaError> {
    let resp = state.get_alias(&function_name, &alias_name).await?;
    Ok(json_response(StatusCode::OK, &resp))
}

async fn list_aliases_handler(
    State(state): State<Arc<LambdaState>>,
    Path(function_name): Path<String>,
) -> Result<Response, LambdaError> {
    let resp = state.list_aliases(&function_name).await?;
    Ok(json_response(StatusCode::OK, &resp))
}

// --- EventSourceMapping handlers ---

async fn create_event_source_mapping_handler(
    State(state): State<Arc<LambdaState>>,
    Json(req): Json<CreateEventSourceMappingRequest>,
) -> Result<Response, LambdaError> {
    let resp = state.create_event_source_mapping(req).await?;
    Ok(json_response(StatusCode::ACCEPTED, &resp))
}

async fn delete_event_source_mapping_handler(
    State(state): State<Arc<LambdaState>>,
    Path(uuid): Path<String>,
) -> Result<Response, LambdaError> {
    let resp = state.delete_event_source_mapping(&uuid).await?;
    Ok(json_response(StatusCode::ACCEPTED, &resp))
}

async fn list_event_source_mappings_handler(
    State(state): State<Arc<LambdaState>>,
) -> Result<Response, LambdaError> {
    let resp = state.list_event_source_mappings().await?;
    Ok(json_response(StatusCode::OK, &resp))
}

// --- Tag handlers ---

async fn tag_resource_handler(
    State(state): State<Arc<LambdaState>>,
    Path(arn): Path<String>,
    Json(req): Json<TagResourceRequest>,
) -> Result<Response, LambdaError> {
    state.tag_resource(&arn, req.tags).await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}

async fn untag_resource_handler(
    State(state): State<Arc<LambdaState>>,
    Path(arn): Path<String>,
    raw_query: axum::extract::RawQuery,
) -> Result<Response, LambdaError> {
    let tag_keys: Vec<String> = raw_query
        .0
        .map(|q| {
            form_urlencoded::parse(q.as_bytes())
                .filter(|(k, _)| k == "tagKeys")
                .map(|(_, v)| v.into_owned())
                .collect()
        })
        .unwrap_or_default();

    state.untag_resource(&arn, &tag_keys).await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}

async fn list_tags_handler(
    State(state): State<Arc<LambdaState>>,
    Path(arn): Path<String>,
) -> Result<Response, LambdaError> {
    let resp = state.list_tags(&arn).await?;
    Ok(json_response(StatusCode::OK, &resp))
}

pub fn create_router(state: Arc<LambdaState>) -> Router {
    Router::new()
        // Functions
        .route(
            "/2015-03-31/functions",
            post(create_function_handler).get(list_functions_handler),
        )
        .route(
            "/2015-03-31/functions/{name}",
            get(get_function_handler).delete(delete_function_handler),
        )
        .route(
            "/2015-03-31/functions/{name}/code",
            put(update_function_code_handler),
        )
        .route(
            "/2015-03-31/functions/{name}/configuration",
            put(update_function_configuration_handler),
        )
        // Invoke
        .route(
            "/2015-03-31/functions/{name}/invocations",
            post(invoke_handler),
        )
        // Policy
        .route(
            "/2015-03-31/functions/{name}/policy",
            post(add_permission_handler).get(get_policy_handler),
        )
        .route(
            "/2015-03-31/functions/{name}/policy/{sid}",
            delete(remove_permission_handler),
        )
        // Versions
        .route(
            "/2015-03-31/functions/{name}/versions",
            post(publish_version_handler).get(list_versions_handler),
        )
        // Aliases
        .route(
            "/2015-03-31/functions/{name}/aliases",
            post(create_alias_handler).get(list_aliases_handler),
        )
        .route(
            "/2015-03-31/functions/{name}/aliases/{alias_name}",
            get(get_alias_handler).delete(delete_alias_handler),
        )
        // Event Source Mappings
        .route(
            "/2015-03-31/event-source-mappings",
            post(create_event_source_mapping_handler).get(list_event_source_mappings_handler),
        )
        .route(
            "/2015-03-31/event-source-mappings/{uuid}",
            delete(delete_event_source_mapping_handler),
        )
        // Tags - using wildcard path to capture full ARN
        // AWS CLI uses /2017-03-31/tags/ for tag operations
        .route(
            "/2015-03-31/tags/{*arn}",
            post(tag_resource_handler)
                .delete(untag_resource_handler)
                .get(list_tags_handler),
        )
        .route(
            "/2017-03-31/tags/{*arn}",
            post(tag_resource_handler)
                .delete(untag_resource_handler)
                .get(list_tags_handler),
        )
        .with_state(state)
}
