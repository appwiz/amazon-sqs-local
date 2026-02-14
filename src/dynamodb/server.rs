use std::sync::Arc;

use axum::body::Bytes;
use axum::extract::State;
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};

use super::error::DynamoDbError;
use super::state::DynamoDbState;
use super::types::*;

macro_rules! dispatch {
    ($state:expr, $body:expr, $req_type:ty, $method:ident) => {{
        let req: $req_type = serde_json::from_slice(&$body)
            .map_err(|e| DynamoDbError::SerializationException(e.to_string()))?;
        let resp = $state.$method(req).await?;
        Ok(Json(serde_json::to_value(resp).unwrap()).into_response())
    }};
}

macro_rules! dispatch_empty {
    ($state:expr, $body:expr, $req_type:ty, $method:ident) => {{
        let req: $req_type = serde_json::from_slice(&$body)
            .map_err(|e| DynamoDbError::SerializationException(e.to_string()))?;
        $state.$method(req).await?;
        Ok(Json(serde_json::json!({})).into_response())
    }};
}

async fn handle_request(
    State(state): State<Arc<DynamoDbState>>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<axum::response::Response, DynamoDbError> {
    let target = headers
        .get("x-amz-target")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| {
            DynamoDbError::SerializationException("Missing X-Amz-Target header".into())
        })?;

    let action = target
        .strip_prefix("DynamoDB_20120810.")
        .ok_or_else(|| {
            DynamoDbError::SerializationException(format!("Invalid target: {target}"))
        })?;

    match action {
        "CreateTable" => dispatch!(state, body, CreateTableRequest, create_table),
        "DeleteTable" => dispatch!(state, body, DeleteTableRequest, delete_table),
        "DescribeTable" => dispatch!(state, body, DescribeTableRequest, describe_table),
        "ListTables" => dispatch!(state, body, ListTablesRequest, list_tables),
        "UpdateTable" => dispatch!(state, body, UpdateTableRequest, update_table),
        "PutItem" => dispatch!(state, body, PutItemRequest, put_item),
        "GetItem" => dispatch!(state, body, GetItemRequest, get_item),
        "DeleteItem" => dispatch!(state, body, DeleteItemRequest, delete_item),
        "UpdateItem" => dispatch!(state, body, UpdateItemRequest, update_item),
        "Query" => dispatch!(state, body, QueryRequest, query),
        "Scan" => dispatch!(state, body, ScanRequest, scan),
        "BatchGetItem" => dispatch!(state, body, BatchGetItemRequest, batch_get_item),
        "BatchWriteItem" => dispatch!(state, body, BatchWriteItemRequest, batch_write_item),
        "TagResource" => dispatch_empty!(state, body, TagResourceRequest, tag_resource),
        "UntagResource" => dispatch_empty!(state, body, UntagResourceRequest, untag_resource),
        "ListTagsOfResource" => {
            dispatch!(state, body, ListTagsOfResourceRequest, list_tags_of_resource)
        }
        _ => Err(DynamoDbError::SerializationException(format!(
            "Unknown action: {action}"
        ))),
    }
}

pub fn create_router(state: Arc<DynamoDbState>) -> Router {
    Router::new()
        .route("/", post(handle_request))
        .with_state(state)
}
