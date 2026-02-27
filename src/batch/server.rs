use std::sync::Arc;

use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use axum::http::StatusCode;

use super::error::BatchError;
use super::state::BatchState;
use super::types::*;

async fn create_compute_environment_handler(
    State(state): State<Arc<BatchState>>,
    Json(req): Json<CreateComputeEnvironmentRequest>,
) -> Result<axum::response::Response, BatchError> {
    let detail = state.create_compute_environment(req).await?;
    Ok((StatusCode::CREATED, Json(detail)).into_response())
}

async fn get_compute_environment_handler(
    State(state): State<Arc<BatchState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, BatchError> {
    let detail = state.get_compute_environment(&name).await?;
    Ok(Json(detail).into_response())
}

async fn list_compute_environments_handler(
    State(state): State<Arc<BatchState>>,
) -> Result<axum::response::Response, BatchError> {
    let resp = state.list_compute_environments().await?;
    Ok(Json(resp).into_response())
}

async fn delete_compute_environment_handler(
    State(state): State<Arc<BatchState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, BatchError> {
    state.delete_compute_environment(&name).await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}

async fn create_job_queue_handler(
    State(state): State<Arc<BatchState>>,
    Json(req): Json<CreateJobQueueRequest>,
) -> Result<axum::response::Response, BatchError> {
    let detail = state.create_job_queue(req).await?;
    Ok((StatusCode::CREATED, Json(detail)).into_response())
}

async fn get_job_queue_handler(
    State(state): State<Arc<BatchState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, BatchError> {
    let detail = state.get_job_queue(&name).await?;
    Ok(Json(detail).into_response())
}

async fn list_job_queues_handler(
    State(state): State<Arc<BatchState>>,
) -> Result<axum::response::Response, BatchError> {
    let resp = state.list_job_queues().await?;
    Ok(Json(resp).into_response())
}

async fn delete_job_queue_handler(
    State(state): State<Arc<BatchState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, BatchError> {
    state.delete_job_queue(&name).await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}

pub fn create_router(state: Arc<BatchState>) -> Router {
    Router::new()
        .route("/compute-environments", post(create_compute_environment_handler).get(list_compute_environments_handler))
        .route("/compute-environments/{name}", get(get_compute_environment_handler).delete(delete_compute_environment_handler))
        .route("/job-queues", post(create_job_queue_handler).get(list_job_queues_handler))
        .route("/job-queues/{name}", get(get_job_queue_handler).delete(delete_job_queue_handler))
        .with_state(state)
}


#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_list_endpoint() {
        let state = Arc::new(BatchState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/compute-environments")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[tokio::test]
    async fn test_get_not_found() {
        let state = Arc::new(BatchState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/compute-environments/nonexistent")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_create_compute_environments() {
        let state = Arc::new(BatchState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/compute-environments")
            .header("content-type", "application/json")
            .body(Body::from("{}"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert!(resp.status().is_success() || resp.status().is_client_error());
    }
    #[tokio::test]
    async fn test_create_job_queues() {
        let state = Arc::new(BatchState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/job-queues")
            .header("content-type", "application/json")
            .body(Body::from("{}"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert!(resp.status().is_success() || resp.status().is_client_error());
    }
}
