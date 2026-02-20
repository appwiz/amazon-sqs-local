use std::sync::Arc;

use axum::body::Bytes;
use axum::extract::State;
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};

use super::error::CwlError;
use super::state::CwlState;
use super::types::*;

macro_rules! dispatch {
    ($state:expr, $body:expr, $req_type:ty, $method:ident) => {{
        let req: $req_type = serde_json::from_slice(&$body)
            .map_err(|e| CwlError::InvalidParameterException(e.to_string()))?;
        let resp = $state.$method(req).await?;
        Ok(Json(serde_json::to_value(resp).unwrap()).into_response())
    }};
}

macro_rules! dispatch_empty {
    ($state:expr, $body:expr, $req_type:ty, $method:ident) => {{
        let req: $req_type = serde_json::from_slice(&$body)
            .map_err(|e| CwlError::InvalidParameterException(e.to_string()))?;
        $state.$method(req).await?;
        Ok(Json(serde_json::json!({})).into_response())
    }};
}

async fn handle_request(
    State(state): State<Arc<CwlState>>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<axum::response::Response, CwlError> {
    let target = headers
        .get("x-amz-target")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| CwlError::InvalidAction("Missing X-Amz-Target header".into()))?;

    let action = target
        .strip_prefix("Logs_20140328.")
        .ok_or_else(|| CwlError::InvalidAction(format!("Invalid target: {target}")))?;

    match action {
        "CreateLogGroup" => dispatch_empty!(state, body, CreateLogGroupRequest, create_log_group),
        "DeleteLogGroup" => dispatch_empty!(state, body, DeleteLogGroupRequest, delete_log_group),
        "DescribeLogGroups" => {
            dispatch!(state, body, DescribeLogGroupsRequest, describe_log_groups)
        }
        "CreateLogStream" => {
            dispatch_empty!(state, body, CreateLogStreamRequest, create_log_stream)
        }
        "DeleteLogStream" => {
            dispatch_empty!(state, body, DeleteLogStreamRequest, delete_log_stream)
        }
        "DescribeLogStreams" => {
            dispatch!(state, body, DescribeLogStreamsRequest, describe_log_streams)
        }
        "PutLogEvents" => dispatch!(state, body, PutLogEventsRequest, put_log_events),
        "GetLogEvents" => dispatch!(state, body, GetLogEventsRequest, get_log_events),
        "FilterLogEvents" => dispatch!(state, body, FilterLogEventsRequest, filter_log_events),
        "PutRetentionPolicy" => {
            dispatch_empty!(state, body, PutRetentionPolicyRequest, put_retention_policy)
        }
        "DeleteRetentionPolicy" => {
            dispatch_empty!(state, body, DeleteRetentionPolicyRequest, delete_retention_policy)
        }
        "TagLogGroup" => dispatch_empty!(state, body, TagLogGroupRequest, tag_log_group),
        "UntagLogGroup" => dispatch_empty!(state, body, UntagLogGroupRequest, untag_log_group),
        "ListTagsLogGroup" => {
            dispatch!(state, body, ListTagsLogGroupRequest, list_tags_log_group)
        }
        "TagResource" => dispatch_empty!(state, body, TagResourceRequest, tag_resource),
        "UntagResource" => dispatch_empty!(state, body, UntagResourceRequest, untag_resource),
        "ListTagsForResource" => {
            dispatch!(state, body, ListTagsForResourceRequest, list_tags_for_resource)
        }
        _ => Err(CwlError::InvalidAction(format!("Unknown action: {action}"))),
    }
}

pub fn create_router(state: Arc<CwlState>) -> Router {
    Router::new()
        .route("/", post(handle_request))
        .with_state(state)
}
