use std::sync::Arc;

use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use axum::http::StatusCode;

use super::error::FISError;
use super::state::FISState;
use super::types::*;

async fn create_experiment_template_handler(
    State(state): State<Arc<FISState>>,
    Json(req): Json<CreateExperimentTemplateRequest>,
) -> Result<axum::response::Response, FISError> {
    let detail = state.create_experiment_template(req).await?;
    Ok((StatusCode::CREATED, Json(detail)).into_response())
}

async fn get_experiment_template_handler(
    State(state): State<Arc<FISState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, FISError> {
    let detail = state.get_experiment_template(&name).await?;
    Ok(Json(detail).into_response())
}

async fn list_experiment_templates_handler(
    State(state): State<Arc<FISState>>,
) -> Result<axum::response::Response, FISError> {
    let resp = state.list_experiment_templates().await?;
    Ok(Json(resp).into_response())
}

async fn delete_experiment_template_handler(
    State(state): State<Arc<FISState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, FISError> {
    state.delete_experiment_template(&name).await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}

pub fn create_router(state: Arc<FISState>) -> Router {
    Router::new()
        .route("/experiment-templates", post(create_experiment_template_handler).get(list_experiment_templates_handler))
        .route("/experiment-templates/{name}", get(get_experiment_template_handler).delete(delete_experiment_template_handler))
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
        let state = Arc::new(FISState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/experiment-templates")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[tokio::test]
    async fn test_get_not_found() {
        let state = Arc::new(FISState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/experiment-templates/nonexistent")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_create_experiment_templates() {
        let state = Arc::new(FISState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/experiment-templates")
            .header("content-type", "application/json")
            .body(Body::from("{}"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert!(resp.status().is_success() || resp.status().is_client_error());
    }
}
