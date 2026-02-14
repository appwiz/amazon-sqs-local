use std::sync::Arc;

use axum::body::Bytes;
use axum::extract::State;
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};

use super::error::FirehoseError;
use super::state::FirehoseState;
use super::types::*;

macro_rules! dispatch {
    ($state:expr, $body:expr, $req_type:ty, $method:ident) => {{
        let req: $req_type = serde_json::from_slice(&$body)
            .map_err(|e| FirehoseError::InvalidArgumentException(e.to_string()))?;
        let resp = $state.$method(req).await?;
        Ok(Json(serde_json::to_value(resp).unwrap()).into_response())
    }};
}

macro_rules! dispatch_empty {
    ($state:expr, $body:expr, $req_type:ty, $method:ident) => {{
        let req: $req_type = serde_json::from_slice(&$body)
            .map_err(|e| FirehoseError::InvalidArgumentException(e.to_string()))?;
        $state.$method(req).await?;
        Ok(Json(serde_json::json!({})).into_response())
    }};
}

async fn handle_request(
    State(state): State<Arc<FirehoseState>>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<axum::response::Response, FirehoseError> {
    let target = headers
        .get("x-amz-target")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| FirehoseError::InvalidAction("Missing X-Amz-Target header".into()))?;

    let action = target
        .strip_prefix("Firehose_20150804.")
        .ok_or_else(|| FirehoseError::InvalidAction(format!("Invalid target: {target}")))?;

    match action {
        "CreateDeliveryStream" => {
            dispatch!(state, body, CreateDeliveryStreamRequest, create_delivery_stream)
        }
        "DeleteDeliveryStream" => {
            dispatch_empty!(state, body, DeleteDeliveryStreamRequest, delete_delivery_stream)
        }
        "DescribeDeliveryStream" => {
            dispatch!(state, body, DescribeDeliveryStreamRequest, describe_delivery_stream)
        }
        "ListDeliveryStreams" => {
            dispatch!(state, body, ListDeliveryStreamsRequest, list_delivery_streams)
        }
        "UpdateDestination" => {
            dispatch_empty!(state, body, UpdateDestinationRequest, update_destination)
        }
        "PutRecord" => dispatch!(state, body, PutRecordRequest, put_record),
        "PutRecordBatch" => {
            dispatch!(state, body, PutRecordBatchRequest, put_record_batch)
        }
        "TagDeliveryStream" => {
            dispatch_empty!(state, body, TagDeliveryStreamRequest, tag_delivery_stream)
        }
        "UntagDeliveryStream" => {
            dispatch_empty!(state, body, UntagDeliveryStreamRequest, untag_delivery_stream)
        }
        "ListTagsForDeliveryStream" => {
            dispatch!(
                state,
                body,
                ListTagsForDeliveryStreamRequest,
                list_tags_for_delivery_stream
            )
        }
        _ => Err(FirehoseError::InvalidAction(format!(
            "Unknown action: {action}"
        ))),
    }
}

pub fn create_router(state: Arc<FirehoseState>) -> Router {
    Router::new()
        .route("/", post(handle_request))
        .with_state(state)
}
