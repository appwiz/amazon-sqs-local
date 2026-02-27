use std::sync::Arc;

use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use axum::http::StatusCode;

use super::error::GroundstationError;
use super::state::GroundstationState;
use super::types::*;

async fn create_config_handler(
    State(state): State<Arc<GroundstationState>>,
    Json(req): Json<CreateConfigRequest>,
) -> Result<axum::response::Response, GroundstationError> {
    let detail = state.create_config(req).await?;
    Ok((StatusCode::CREATED, Json(detail)).into_response())
}

async fn get_config_handler(
    State(state): State<Arc<GroundstationState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, GroundstationError> {
    let detail = state.get_config(&name).await?;
    Ok(Json(detail).into_response())
}

async fn list_configs_handler(
    State(state): State<Arc<GroundstationState>>,
) -> Result<axum::response::Response, GroundstationError> {
    let resp = state.list_configs().await?;
    Ok(Json(resp).into_response())
}

async fn delete_config_handler(
    State(state): State<Arc<GroundstationState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, GroundstationError> {
    state.delete_config(&name).await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}

pub fn create_router(state: Arc<GroundstationState>) -> Router {
    Router::new()
        .route("/configs", post(create_config_handler).get(list_configs_handler))
        .route("/configs/{name}", get(get_config_handler).delete(delete_config_handler))
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
        let state = Arc::new(GroundstationState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/configs")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[tokio::test]
    async fn test_get_not_found() {
        let state = Arc::new(GroundstationState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/configs/nonexistent")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_create_configs() {
        let state = Arc::new(GroundstationState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/configs")
            .header("content-type", "application/json")
            .body(Body::from("{}"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert!(resp.status().is_success() || resp.status().is_client_error());
    }
}
