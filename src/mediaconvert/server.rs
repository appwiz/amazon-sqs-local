use std::sync::Arc;

use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use axum::http::StatusCode;

use super::error::MediaconvertError;
use super::state::MediaconvertState;
use super::types::*;

async fn create_job_handler(
    State(state): State<Arc<MediaconvertState>>,
    Json(req): Json<CreateJobRequest>,
) -> Result<axum::response::Response, MediaconvertError> {
    let detail = state.create_job(req).await?;
    Ok((StatusCode::CREATED, Json(detail)).into_response())
}

async fn get_job_handler(
    State(state): State<Arc<MediaconvertState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, MediaconvertError> {
    let detail = state.get_job(&name).await?;
    Ok(Json(detail).into_response())
}

async fn list_jobs_handler(
    State(state): State<Arc<MediaconvertState>>,
) -> Result<axum::response::Response, MediaconvertError> {
    let resp = state.list_jobs().await?;
    Ok(Json(resp).into_response())
}

async fn delete_job_handler(
    State(state): State<Arc<MediaconvertState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, MediaconvertError> {
    state.delete_job(&name).await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}

pub fn create_router(state: Arc<MediaconvertState>) -> Router {
    Router::new()
        .route("/jobs", post(create_job_handler).get(list_jobs_handler))
        .route("/jobs/{name}", get(get_job_handler).delete(delete_job_handler))
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
        let state = Arc::new(MediaconvertState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/jobs")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[tokio::test]
    async fn test_get_not_found() {
        let state = Arc::new(MediaconvertState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/jobs/nonexistent")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_create_jobs() {
        let state = Arc::new(MediaconvertState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/jobs")
            .header("content-type", "application/json")
            .body(Body::from("{}"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert!(resp.status().is_success() || resp.status().is_client_error());
    }
}
