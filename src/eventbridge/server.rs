use std::sync::Arc;

use axum::body::Bytes;
use axum::extract::State;
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};

use super::error::EventBridgeError;
use super::state::EventBridgeState;
use super::types::*;

macro_rules! dispatch {
    ($state:expr, $body:expr, $req_type:ty, $method:ident) => {{
        let req: $req_type = serde_json::from_slice(&$body)
            .map_err(|e| EventBridgeError::InvalidAction(e.to_string()))?;
        let resp = $state.$method(req).await?;
        Ok(Json(serde_json::to_value(resp).unwrap()).into_response())
    }};
}

macro_rules! dispatch_empty {
    ($state:expr, $body:expr, $req_type:ty, $method:ident) => {{
        let req: $req_type = serde_json::from_slice(&$body)
            .map_err(|e| EventBridgeError::InvalidAction(e.to_string()))?;
        $state.$method(req).await?;
        Ok(Json(serde_json::json!({})).into_response())
    }};
}

async fn handle_request(
    State(state): State<Arc<EventBridgeState>>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<axum::response::Response, EventBridgeError> {
    let target = headers
        .get("x-amz-target")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| EventBridgeError::InvalidAction("Missing X-Amz-Target header".into()))?;

    let action = target
        .strip_prefix("AmazonEventBridge.")
        .ok_or_else(|| EventBridgeError::InvalidAction(format!("Invalid target: {target}")))?;

    match action {
        "CreateEventBus" => dispatch!(state, body, CreateEventBusRequest, create_event_bus),
        "DeleteEventBus" => dispatch_empty!(state, body, DeleteEventBusRequest, delete_event_bus),
        "DescribeEventBus" => dispatch!(state, body, DescribeEventBusRequest, describe_event_bus),
        "ListEventBuses" => dispatch!(state, body, ListEventBusesRequest, list_event_buses),
        "PutEvents" => dispatch!(state, body, PutEventsRequest, put_events),
        "PutRule" => dispatch!(state, body, PutRuleRequest, put_rule),
        "DeleteRule" => dispatch_empty!(state, body, DeleteRuleRequest, delete_rule),
        "DescribeRule" => dispatch!(state, body, DescribeRuleRequest, describe_rule),
        "ListRules" => dispatch!(state, body, ListRulesRequest, list_rules),
        "PutTargets" => dispatch!(state, body, PutTargetsRequest, put_targets),
        "RemoveTargets" => dispatch!(state, body, RemoveTargetsRequest, remove_targets),
        "ListTargetsByRule" => {
            dispatch!(state, body, ListTargetsByRuleRequest, list_targets_by_rule)
        }
        "TagResource" => dispatch_empty!(state, body, TagResourceRequest, tag_resource),
        "UntagResource" => dispatch_empty!(state, body, UntagResourceRequest, untag_resource),
        "ListTagsForResource" => {
            dispatch!(state, body, ListTagsForResourceRequest, list_tags_for_resource)
        }
        _ => Err(EventBridgeError::InvalidAction(format!("Unknown action: {action}"))),
    }
}

pub fn create_router(state: Arc<EventBridgeState>) -> Router {
    Router::new()
        .route("/", post(handle_request))
        .with_state(state)
}
