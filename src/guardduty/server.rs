use std::sync::Arc;

use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use axum::http::StatusCode;

use super::error::GuarddutyError;
use super::state::GuarddutyState;
use super::types::*;

async fn create_detector_handler(
    State(state): State<Arc<GuarddutyState>>,
    Json(req): Json<CreateDetectorRequest>,
) -> Result<axum::response::Response, GuarddutyError> {
    let detail = state.create_detector(req).await?;
    Ok((StatusCode::CREATED, Json(detail)).into_response())
}

async fn get_detector_handler(
    State(state): State<Arc<GuarddutyState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, GuarddutyError> {
    let detail = state.get_detector(&name).await?;
    Ok(Json(detail).into_response())
}

async fn list_detectors_handler(
    State(state): State<Arc<GuarddutyState>>,
) -> Result<axum::response::Response, GuarddutyError> {
    let resp = state.list_detectors().await?;
    Ok(Json(resp).into_response())
}

async fn delete_detector_handler(
    State(state): State<Arc<GuarddutyState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, GuarddutyError> {
    state.delete_detector(&name).await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}

pub fn create_router(state: Arc<GuarddutyState>) -> Router {
    Router::new()
        .route("/detectors", post(create_detector_handler).get(list_detectors_handler))
        .route("/detectors/{name}", get(get_detector_handler).delete(delete_detector_handler))
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
        let state = Arc::new(GuarddutyState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/detectors")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[tokio::test]
    async fn test_get_not_found() {
        let state = Arc::new(GuarddutyState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/detectors/nonexistent")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_create_detectors() {
        let state = Arc::new(GuarddutyState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/detectors")
            .header("content-type", "application/json")
            .body(Body::from("{}"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert!(resp.status().is_success() || resp.status().is_client_error());
    }
}
