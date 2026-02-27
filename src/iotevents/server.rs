use std::sync::Arc;

use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use axum::http::StatusCode;

use super::error::IoteventsError;
use super::state::IoteventsState;
use super::types::*;

async fn create_detector_model_handler(
    State(state): State<Arc<IoteventsState>>,
    Json(req): Json<CreateDetectorModelRequest>,
) -> Result<axum::response::Response, IoteventsError> {
    let detail = state.create_detector_model(req).await?;
    Ok((StatusCode::CREATED, Json(detail)).into_response())
}

async fn get_detector_model_handler(
    State(state): State<Arc<IoteventsState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, IoteventsError> {
    let detail = state.get_detector_model(&name).await?;
    Ok(Json(detail).into_response())
}

async fn list_detector_models_handler(
    State(state): State<Arc<IoteventsState>>,
) -> Result<axum::response::Response, IoteventsError> {
    let resp = state.list_detector_models().await?;
    Ok(Json(resp).into_response())
}

async fn delete_detector_model_handler(
    State(state): State<Arc<IoteventsState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, IoteventsError> {
    state.delete_detector_model(&name).await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}

pub fn create_router(state: Arc<IoteventsState>) -> Router {
    Router::new()
        .route("/detector-models", post(create_detector_model_handler).get(list_detector_models_handler))
        .route("/detector-models/{name}", get(get_detector_model_handler).delete(delete_detector_model_handler))
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
        let state = Arc::new(IoteventsState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/detector-models")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[tokio::test]
    async fn test_get_not_found() {
        let state = Arc::new(IoteventsState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/detector-models/nonexistent")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_create_detector_models() {
        let state = Arc::new(IoteventsState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/detector-models")
            .header("content-type", "application/json")
            .body(Body::from("{}"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert!(resp.status().is_success() || resp.status().is_client_error());
    }
}
