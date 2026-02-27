use std::sync::Arc;

use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use axum::http::StatusCode;

use super::error::ElastictranscoderError;
use super::state::ElastictranscoderState;
use super::types::*;

async fn create_pipeline_handler(
    State(state): State<Arc<ElastictranscoderState>>,
    Json(req): Json<CreatePipelineRequest>,
) -> Result<axum::response::Response, ElastictranscoderError> {
    let detail = state.create_pipeline(req).await?;
    Ok((StatusCode::CREATED, Json(detail)).into_response())
}

async fn get_pipeline_handler(
    State(state): State<Arc<ElastictranscoderState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, ElastictranscoderError> {
    let detail = state.get_pipeline(&name).await?;
    Ok(Json(detail).into_response())
}

async fn list_pipelines_handler(
    State(state): State<Arc<ElastictranscoderState>>,
) -> Result<axum::response::Response, ElastictranscoderError> {
    let resp = state.list_pipelines().await?;
    Ok(Json(resp).into_response())
}

async fn delete_pipeline_handler(
    State(state): State<Arc<ElastictranscoderState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, ElastictranscoderError> {
    state.delete_pipeline(&name).await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}

pub fn create_router(state: Arc<ElastictranscoderState>) -> Router {
    Router::new()
        .route("/pipelines", post(create_pipeline_handler).get(list_pipelines_handler))
        .route("/pipelines/{name}", get(get_pipeline_handler).delete(delete_pipeline_handler))
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
        let state = Arc::new(ElastictranscoderState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/pipelines")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[tokio::test]
    async fn test_get_not_found() {
        let state = Arc::new(ElastictranscoderState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/pipelines/nonexistent")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_create_pipelines() {
        let state = Arc::new(ElastictranscoderState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/pipelines")
            .header("content-type", "application/json")
            .body(Body::from("{}"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert!(resp.status().is_success() || resp.status().is_client_error());
    }
}
