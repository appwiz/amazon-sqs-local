use std::sync::Arc;

use axum::body::Bytes;
use axum::extract::State;
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};

use super::error::SecretsManagerError;
use super::state::SecretsManagerState;
use super::types::*;

macro_rules! dispatch {
    ($state:expr, $body:expr, $req_type:ty, $method:ident) => {{
        let req: $req_type = serde_json::from_slice(&$body)
            .map_err(|e| SecretsManagerError::InvalidParameterException(e.to_string()))?;
        let resp = $state.$method(req).await?;
        Ok(Json(serde_json::to_value(resp).unwrap()).into_response())
    }};
}

macro_rules! dispatch_empty {
    ($state:expr, $body:expr, $req_type:ty, $method:ident) => {{
        let req: $req_type = serde_json::from_slice(&$body)
            .map_err(|e| SecretsManagerError::InvalidParameterException(e.to_string()))?;
        $state.$method(req).await?;
        Ok(Json(serde_json::json!({})).into_response())
    }};
}

async fn handle_request(
    State(state): State<Arc<SecretsManagerState>>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<axum::response::Response, SecretsManagerError> {
    let target = headers
        .get("x-amz-target")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| SecretsManagerError::InvalidAction("Missing X-Amz-Target header".into()))?;

    let action = target
        .strip_prefix("secretsmanager.")
        .ok_or_else(|| SecretsManagerError::InvalidAction(format!("Invalid target: {target}")))?;

    match action {
        "CreateSecret" => dispatch!(state, body, CreateSecretRequest, create_secret),
        "GetSecretValue" => dispatch!(state, body, GetSecretValueRequest, get_secret_value),
        "PutSecretValue" => dispatch!(state, body, PutSecretValueRequest, put_secret_value),
        "DescribeSecret" => dispatch!(state, body, DescribeSecretRequest, describe_secret),
        "ListSecrets" => dispatch!(state, body, ListSecretsRequest, list_secrets),
        "UpdateSecret" => dispatch!(state, body, UpdateSecretRequest, update_secret),
        "DeleteSecret" => dispatch!(state, body, DeleteSecretRequest, delete_secret),
        "RestoreSecret" => dispatch!(state, body, RestoreSecretRequest, restore_secret),
        "TagResource" => dispatch_empty!(state, body, TagResourceRequest, tag_resource),
        "UntagResource" => dispatch_empty!(state, body, UntagResourceRequest, untag_resource),
        "ListSecretVersionIds" => {
            dispatch!(state, body, ListSecretVersionIdsRequest, list_secret_version_ids)
        }
        _ => Err(SecretsManagerError::InvalidAction(format!(
            "Unknown action: {action}"
        ))),
    }
}

pub fn create_router(state: Arc<SecretsManagerState>) -> Router {
    Router::new()
        .route("/", post(handle_request))
        .with_state(state)
}
