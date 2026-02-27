use std::sync::Arc;

use axum::body::Bytes;
use axum::extract::State;
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};

use super::error::KinesisError;
use super::state::KinesisState;
use super::types::*;

macro_rules! dispatch {
    ($state:expr, $body:expr, $req_type:ty, $method:ident) => {{
        let req: $req_type = serde_json::from_slice(&$body)
            .map_err(|e| KinesisError::InvalidArgumentException(e.to_string()))?;
        let resp = $state.$method(req).await?;
        Ok(Json(resp).into_response())
    }};
}

macro_rules! dispatch_empty {
    ($state:expr, $body:expr, $req_type:ty, $method:ident) => {{
        let req: $req_type = serde_json::from_slice(&$body)
            .map_err(|e| KinesisError::InvalidArgumentException(e.to_string()))?;
        $state.$method(req).await?;
        Ok(Json(serde_json::json!({})).into_response())
    }};
}

async fn handle_request(
    State(state): State<Arc<KinesisState>>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<axum::response::Response, KinesisError> {
    let target = headers
        .get("x-amz-target")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| KinesisError::InvalidAction("Missing X-Amz-Target header".into()))?;

    let action = target
        .strip_prefix("Kinesis_20131202.")
        .ok_or_else(|| KinesisError::InvalidAction(format!("Invalid target: {target}")))?;

    match action {
        "CreateStream" => dispatch_empty!(state, body, CreateStreamRequest, create_stream),
        "DeleteStream" => dispatch_empty!(state, body, DeleteStreamRequest, delete_stream),
        "DescribeStream" => dispatch!(state, body, DescribeStreamRequest, describe_stream),
        "DescribeStreamSummary" => {
            dispatch!(state, body, DescribeStreamSummaryRequest, describe_stream_summary)
        }
        "ListStreams" => dispatch!(state, body, ListStreamsRequest, list_streams),
        "PutRecord" => dispatch!(state, body, PutRecordRequest, put_record),
        "PutRecords" => dispatch!(state, body, PutRecordsRequest, put_records),
        "GetShardIterator" => dispatch!(state, body, GetShardIteratorRequest, get_shard_iterator),
        "GetRecords" => dispatch!(state, body, GetRecordsRequest, get_records),
        "ListShards" => dispatch!(state, body, ListShardsRequest, list_shards),
        "AddTagsToStream" => {
            dispatch_empty!(state, body, AddTagsToStreamRequest, add_tags_to_stream)
        }
        "RemoveTagsFromStream" => {
            dispatch_empty!(state, body, RemoveTagsFromStreamRequest, remove_tags_from_stream)
        }
        "ListTagsForStream" => {
            dispatch!(state, body, ListTagsForStreamRequest, list_tags_for_stream)
        }
        "IncreaseStreamRetentionPeriod" => {
            dispatch_empty!(state, body, IncreaseStreamRetentionPeriodRequest, increase_stream_retention_period)
        }
        "DecreaseStreamRetentionPeriod" => {
            dispatch_empty!(state, body, DecreaseStreamRetentionPeriodRequest, decrease_stream_retention_period)
        }
        _ => Err(KinesisError::InvalidAction(format!("Unknown action: {action}"))),
    }
}

pub fn create_router(state: Arc<KinesisState>) -> Router {
    Router::new()
        .route("/", post(handle_request))
        .with_state(state)
}


#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_missing_target_header() {
        let state = Arc::new(KinesisState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/")
            .header("content-type", "application/x-amz-json-1.1")
            .body(Body::from("{}"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_unknown_action() {
        let state = Arc::new(KinesisState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/")
            .header("content-type", "application/x-amz-json-1.1")
            .header("x-amz-target", "Kinesis_20131202.FakeAction")
            .body(Body::from("{}"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }
    #[tokio::test]
    async fn test_describestream_requires_params() {
        let state = Arc::new(KinesisState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/")
            .header("content-type", "application/x-amz-json-1.1")
            .header("x-amz-target", "Kinesis_20131202.DescribeStream")
            .body(Body::from("{}"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_ne!(resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_createstream_action() {
        let state = Arc::new(KinesisState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/")
            .header("content-type", "application/x-amz-json-1.1")
            .header("x-amz-target", "Kinesis_20131202.CreateStream")
            .body(Body::from("{}"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert!(resp.status().is_success() || resp.status().is_client_error());
    }
    #[tokio::test]
    async fn test_liststreams_action() {
        let state = Arc::new(KinesisState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/")
            .header("content-type", "application/x-amz-json-1.1")
            .header("x-amz-target", "Kinesis_20131202.ListStreams")
            .body(Body::from("{}"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[tokio::test]
    async fn test_listshards_action() {
        let state = Arc::new(KinesisState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/")
            .header("content-type", "application/x-amz-json-1.1")
            .header("x-amz-target", "Kinesis_20131202.ListShards")
            .body(Body::from("{}"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert!(resp.status().is_client_error());
    }
}
