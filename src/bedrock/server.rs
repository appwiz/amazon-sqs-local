use std::sync::Arc;

use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use axum::http::StatusCode;

use super::error::BedrockError;
use super::state::BedrockState;
use super::types::*;

async fn create_model_customization_job_handler(
    State(state): State<Arc<BedrockState>>,
    Json(req): Json<CreateModelCustomizationJobRequest>,
) -> Result<axum::response::Response, BedrockError> {
    let detail = state.create_model_customization_job(req).await?;
    Ok((StatusCode::CREATED, Json(detail)).into_response())
}

async fn get_model_customization_job_handler(
    State(state): State<Arc<BedrockState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, BedrockError> {
    let detail = state.get_model_customization_job(&name).await?;
    Ok(Json(detail).into_response())
}

async fn list_model_customization_jobs_handler(
    State(state): State<Arc<BedrockState>>,
) -> Result<axum::response::Response, BedrockError> {
    let resp = state.list_model_customization_jobs().await?;
    Ok(Json(resp).into_response())
}

async fn delete_model_customization_job_handler(
    State(state): State<Arc<BedrockState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, BedrockError> {
    state.delete_model_customization_job(&name).await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}

pub fn create_router(state: Arc<BedrockState>) -> Router {
    Router::new()
        .route("/model-customization-jobs", post(create_model_customization_job_handler).get(list_model_customization_jobs_handler))
        .route("/model-customization-jobs/{name}", get(get_model_customization_job_handler).delete(delete_model_customization_job_handler))
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
        let state = Arc::new(BedrockState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/model-customization-jobs")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[tokio::test]
    async fn test_get_not_found() {
        let state = Arc::new(BedrockState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/model-customization-jobs/nonexistent")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_create_model_customization_jobs() {
        let state = Arc::new(BedrockState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/model-customization-jobs")
            .header("content-type", "application/json")
            .body(Body::from("{}"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert!(resp.status().is_success() || resp.status().is_client_error());
    }
}
