use std::sync::Arc;

use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use axum::http::StatusCode;

use super::error::MainframemodError;
use super::state::MainframemodState;
use super::types::*;

async fn create_application_handler(
    State(state): State<Arc<MainframemodState>>,
    Json(req): Json<CreateApplicationRequest>,
) -> Result<axum::response::Response, MainframemodError> {
    let detail = state.create_application(req).await?;
    Ok((StatusCode::CREATED, Json(detail)).into_response())
}

async fn get_application_handler(
    State(state): State<Arc<MainframemodState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, MainframemodError> {
    let detail = state.get_application(&name).await?;
    Ok(Json(detail).into_response())
}

async fn list_applications_handler(
    State(state): State<Arc<MainframemodState>>,
) -> Result<axum::response::Response, MainframemodError> {
    let resp = state.list_applications().await?;
    Ok(Json(resp).into_response())
}

async fn delete_application_handler(
    State(state): State<Arc<MainframemodState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, MainframemodError> {
    state.delete_application(&name).await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}

pub fn create_router(state: Arc<MainframemodState>) -> Router {
    Router::new()
        .route("/applications", post(create_application_handler).get(list_applications_handler))
        .route("/applications/{name}", get(get_application_handler).delete(delete_application_handler))
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
        let state = Arc::new(MainframemodState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/applications")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[tokio::test]
    async fn test_get_not_found() {
        let state = Arc::new(MainframemodState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/applications/nonexistent")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_create_applications() {
        let state = Arc::new(MainframemodState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/applications")
            .header("content-type", "application/json")
            .body(Body::from("{}"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert!(resp.status().is_success() || resp.status().is_client_error());
    }
}
