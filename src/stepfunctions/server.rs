use std::sync::Arc;

use axum::body::Bytes;
use axum::extract::State;
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};

use super::error::SfnError;
use super::state::SfnState;
use super::types::*;

macro_rules! dispatch {
    ($state:expr, $body:expr, $req_type:ty, $method:ident) => {{
        let req: $req_type = serde_json::from_slice(&$body)
            .map_err(|e| SfnError::InvalidAction(e.to_string()))?;
        let resp = $state.$method(req).await?;
        Ok(Json(serde_json::to_value(resp).unwrap()).into_response())
    }};
}

macro_rules! dispatch_empty {
    ($state:expr, $body:expr, $req_type:ty, $method:ident) => {{
        let req: $req_type = serde_json::from_slice(&$body)
            .map_err(|e| SfnError::InvalidAction(e.to_string()))?;
        $state.$method(req).await?;
        Ok(Json(serde_json::json!({})).into_response())
    }};
}

async fn handle_request(
    State(state): State<Arc<SfnState>>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<axum::response::Response, SfnError> {
    let target = headers
        .get("x-amz-target")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| SfnError::InvalidAction("Missing X-Amz-Target header".into()))?;

    let action = target
        .strip_prefix("AmazonStates.")
        .ok_or_else(|| SfnError::InvalidAction(format!("Invalid target: {target}")))?;

    match action {
        "CreateStateMachine" => {
            dispatch!(state, body, CreateStateMachineRequest, create_state_machine)
        }
        "DeleteStateMachine" => {
            dispatch_empty!(state, body, DeleteStateMachineRequest, delete_state_machine)
        }
        "DescribeStateMachine" => {
            dispatch!(state, body, DescribeStateMachineRequest, describe_state_machine)
        }
        "ListStateMachines" => {
            dispatch!(state, body, ListStateMachinesRequest, list_state_machines)
        }
        "StartExecution" => dispatch!(state, body, StartExecutionRequest, start_execution),
        "StopExecution" => dispatch!(state, body, StopExecutionRequest, stop_execution),
        "DescribeExecution" => {
            dispatch!(state, body, DescribeExecutionRequest, describe_execution)
        }
        "ListExecutions" => dispatch!(state, body, ListExecutionsRequest, list_executions),
        "GetExecutionHistory" => {
            dispatch!(state, body, GetExecutionHistoryRequest, get_execution_history)
        }
        "SendTaskSuccess" => dispatch_empty!(state, body, SendTaskSuccessRequest, send_task_success),
        "SendTaskFailure" => dispatch_empty!(state, body, SendTaskFailureRequest, send_task_failure),
        "SendTaskHeartbeat" => {
            dispatch_empty!(state, body, SendTaskHeartbeatRequest, send_task_heartbeat)
        }
        "TagResource" => dispatch_empty!(state, body, TagResourceRequest, tag_resource),
        "UntagResource" => dispatch_empty!(state, body, UntagResourceRequest, untag_resource),
        "ListTagsForResource" => {
            dispatch!(state, body, ListTagsForResourceRequest, list_tags_for_resource)
        }
        _ => Err(SfnError::InvalidAction(format!("Unknown action: {action}"))),
    }
}

pub fn create_router(state: Arc<SfnState>) -> Router {
    Router::new()
        .route("/", post(handle_request))
        .with_state(state)
}
