use std::sync::Arc;

use axum::body::Bytes;
use axum::extract::State;
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};

use super::error::SsmError;
use super::state::SsmState;
use super::types::*;

macro_rules! dispatch {
    ($state:expr, $body:expr, $req_type:ty, $method:ident) => {{
        let req: $req_type = serde_json::from_slice(&$body)
            .map_err(|e| SsmError::InvalidAction(e.to_string()))?;
        let resp = $state.$method(req).await?;
        Ok(Json(serde_json::to_value(resp).unwrap()).into_response())
    }};
}

macro_rules! dispatch_empty {
    ($state:expr, $body:expr, $req_type:ty, $method:ident) => {{
        let req: $req_type = serde_json::from_slice(&$body)
            .map_err(|e| SsmError::InvalidAction(e.to_string()))?;
        $state.$method(req).await?;
        Ok(Json(serde_json::json!({})).into_response())
    }};
}

async fn handle_request(
    State(state): State<Arc<SsmState>>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<axum::response::Response, SsmError> {
    let target = headers
        .get("x-amz-target")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| SsmError::InvalidAction("Missing X-Amz-Target header".into()))?;

    let action = target
        .strip_prefix("AmazonSSM.")
        .ok_or_else(|| SsmError::InvalidAction(format!("Invalid target: {target}")))?;

    match action {
        "PutParameter" => dispatch!(state, body, PutParameterRequest, put_parameter),
        "GetParameter" => dispatch!(state, body, GetParameterRequest, get_parameter),
        "GetParameters" => dispatch!(state, body, GetParametersRequest, get_parameters),
        "GetParametersByPath" => {
            dispatch!(state, body, GetParametersByPathRequest, get_parameters_by_path)
        }
        "DeleteParameter" => dispatch_empty!(state, body, DeleteParameterRequest, delete_parameter),
        "DeleteParameters" => {
            dispatch!(state, body, DeleteParametersRequest, delete_parameters)
        }
        "DescribeParameters" => {
            dispatch!(state, body, DescribeParametersRequest, describe_parameters)
        }
        "AddTagsToResource" => {
            dispatch_empty!(state, body, AddTagsToResourceRequest, add_tags_to_resource)
        }
        "RemoveTagsFromResource" => {
            dispatch_empty!(state, body, RemoveTagsFromResourceRequest, remove_tags_from_resource)
        }
        "ListTagsForResource" => {
            dispatch!(state, body, ListTagsForResourceRequest, list_tags_for_resource)
        }
        _ => Err(SsmError::InvalidAction(format!("Unknown action: {action}"))),
    }
}

pub fn create_router(state: Arc<SsmState>) -> Router {
    Router::new()
        .route("/", post(handle_request))
        .with_state(state)
}
