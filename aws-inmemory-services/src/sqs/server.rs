use std::sync::Arc;

use axum::body::Bytes;
use axum::extract::State;
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};

use super::error::SqsError;
use super::state::SqsState;
use super::types::*;

macro_rules! dispatch {
    ($state:expr, $body:expr, $req_type:ty, $method:ident) => {{
        let req: $req_type = serde_json::from_slice(&$body)
            .map_err(|e| SqsError::InvalidParameterValue(e.to_string()))?;
        let resp = $state.$method(req).await?;
        Ok(Json(serde_json::to_value(resp).unwrap()).into_response())
    }};
}

macro_rules! dispatch_empty {
    ($state:expr, $body:expr, $req_type:ty, $method:ident) => {{
        let req: $req_type = serde_json::from_slice(&$body)
            .map_err(|e| SqsError::InvalidParameterValue(e.to_string()))?;
        $state.$method(req).await?;
        Ok(Json(serde_json::json!({})).into_response())
    }};
}

async fn handle_request(
    State(state): State<Arc<SqsState>>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<axum::response::Response, SqsError> {
    let target = headers
        .get("x-amz-target")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| SqsError::InvalidAction("Missing X-Amz-Target header".into()))?;

    let action = target.strip_prefix("AmazonSQS.").ok_or_else(|| {
        SqsError::InvalidAction(format!("Invalid target: {target}"))
    })?;

    match action {
        "CreateQueue" => dispatch!(state, body, CreateQueueRequest, create_queue),
        "DeleteQueue" => dispatch_empty!(state, body, DeleteQueueRequest, delete_queue),
        "GetQueueUrl" => dispatch!(state, body, GetQueueUrlRequest, get_queue_url),
        "ListQueues" => dispatch!(state, body, ListQueuesRequest, list_queues),
        "GetQueueAttributes" => {
            dispatch!(state, body, GetQueueAttributesRequest, get_queue_attributes)
        }
        "SetQueueAttributes" => {
            dispatch_empty!(state, body, SetQueueAttributesRequest, set_queue_attributes)
        }
        "PurgeQueue" => dispatch_empty!(state, body, PurgeQueueRequest, purge_queue),
        "SendMessage" => dispatch!(state, body, SendMessageRequest, send_message),
        "SendMessageBatch" => {
            dispatch!(state, body, SendMessageBatchRequest, send_message_batch)
        }
        "ReceiveMessage" => {
            dispatch!(state, body, ReceiveMessageRequest, receive_message)
        }
        "DeleteMessage" => {
            dispatch_empty!(state, body, DeleteMessageRequest, delete_message)
        }
        "DeleteMessageBatch" => {
            dispatch!(state, body, DeleteMessageBatchRequest, delete_message_batch)
        }
        "ChangeMessageVisibility" => {
            dispatch_empty!(
                state,
                body,
                ChangeMessageVisibilityRequest,
                change_message_visibility
            )
        }
        "ChangeMessageVisibilityBatch" => {
            dispatch!(
                state,
                body,
                ChangeMessageVisibilityBatchRequest,
                change_message_visibility_batch
            )
        }
        "TagQueue" => dispatch_empty!(state, body, TagQueueRequest, tag_queue),
        "UntagQueue" => dispatch_empty!(state, body, UntagQueueRequest, untag_queue),
        "ListQueueTags" => dispatch!(state, body, ListQueueTagsRequest, list_queue_tags),
        "AddPermission" => {
            dispatch_empty!(state, body, AddPermissionRequest, add_permission)
        }
        "RemovePermission" => {
            dispatch_empty!(state, body, RemovePermissionRequest, remove_permission)
        }
        "ListDeadLetterSourceQueues" => {
            dispatch!(
                state,
                body,
                ListDeadLetterSourceQueuesRequest,
                list_dead_letter_source_queues
            )
        }
        "StartMessageMoveTask" => {
            dispatch!(
                state,
                body,
                StartMessageMoveTaskRequest,
                start_message_move_task
            )
        }
        "CancelMessageMoveTask" => {
            dispatch!(
                state,
                body,
                CancelMessageMoveTaskRequest,
                cancel_message_move_task
            )
        }
        "ListMessageMoveTasks" => {
            dispatch!(
                state,
                body,
                ListMessageMoveTasksRequest,
                list_message_move_tasks
            )
        }
        _ => Err(SqsError::InvalidAction(format!(
            "Unknown action: {action}"
        ))),
    }
}

pub fn create_router(state: Arc<SqsState>) -> Router {
    Router::new()
        .route("/", post(handle_request))
        .with_state(state)
}
