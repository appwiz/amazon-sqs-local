use std::sync::Arc;

use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use axum::http::StatusCode;

use super::error::ImagebuilderError;
use super::state::ImagebuilderState;
use super::types::*;

async fn create_image_pipeline_handler(
    State(state): State<Arc<ImagebuilderState>>,
    Json(req): Json<CreateImagePipelineRequest>,
) -> Result<axum::response::Response, ImagebuilderError> {
    let detail = state.create_image_pipeline(req).await?;
    Ok((StatusCode::CREATED, Json(detail)).into_response())
}

async fn get_image_pipeline_handler(
    State(state): State<Arc<ImagebuilderState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, ImagebuilderError> {
    let detail = state.get_image_pipeline(&name).await?;
    Ok(Json(detail).into_response())
}

async fn list_image_pipelines_handler(
    State(state): State<Arc<ImagebuilderState>>,
) -> Result<axum::response::Response, ImagebuilderError> {
    let resp = state.list_image_pipelines().await?;
    Ok(Json(resp).into_response())
}

async fn delete_image_pipeline_handler(
    State(state): State<Arc<ImagebuilderState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, ImagebuilderError> {
    state.delete_image_pipeline(&name).await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}

pub fn create_router(state: Arc<ImagebuilderState>) -> Router {
    Router::new()
        .route("/image-pipelines", post(create_image_pipeline_handler).get(list_image_pipelines_handler))
        .route("/image-pipelines/{name}", get(get_image_pipeline_handler).delete(delete_image_pipeline_handler))
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
        let state = Arc::new(ImagebuilderState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/image-pipelines")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[tokio::test]
    async fn test_get_not_found() {
        let state = Arc::new(ImagebuilderState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/image-pipelines/nonexistent")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_create_image_pipelines() {
        let state = Arc::new(ImagebuilderState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/image-pipelines")
            .header("content-type", "application/json")
            .body(Body::from("{}"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert!(resp.status().is_success() || resp.status().is_client_error());
    }
}
