use std::sync::Arc;

use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use axum::http::StatusCode;

use super::error::MwaaError;
use super::state::MwaaState;
use super::types::*;

async fn create_environment_handler(
    State(state): State<Arc<MwaaState>>,
    Json(req): Json<CreateEnvironmentRequest>,
) -> Result<axum::response::Response, MwaaError> {
    let detail = state.create_environment(req).await?;
    Ok((StatusCode::CREATED, Json(detail)).into_response())
}

async fn get_environment_handler(
    State(state): State<Arc<MwaaState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, MwaaError> {
    let detail = state.get_environment(&name).await?;
    Ok(Json(detail).into_response())
}

async fn list_environments_handler(
    State(state): State<Arc<MwaaState>>,
) -> Result<axum::response::Response, MwaaError> {
    let resp = state.list_environments().await?;
    Ok(Json(resp).into_response())
}

async fn delete_environment_handler(
    State(state): State<Arc<MwaaState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, MwaaError> {
    state.delete_environment(&name).await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}

pub fn create_router(state: Arc<MwaaState>) -> Router {
    Router::new()
        .route("/environments", post(create_environment_handler).get(list_environments_handler))
        .route("/environments/{name}", get(get_environment_handler).delete(delete_environment_handler))
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
        let state = Arc::new(MwaaState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/environments")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[tokio::test]
    async fn test_get_not_found() {
        let state = Arc::new(MwaaState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/environments/nonexistent")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_create_environments() {
        let state = Arc::new(MwaaState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/environments")
            .header("content-type", "application/json")
            .body(Body::from("{}"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert!(resp.status().is_success() || resp.status().is_client_error());
    }
}
