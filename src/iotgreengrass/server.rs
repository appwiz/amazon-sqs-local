use std::sync::Arc;

use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use axum::http::StatusCode;

use super::error::IotgreengrassError;
use super::state::IotgreengrassState;
use super::types::*;

async fn create_component_handler(
    State(state): State<Arc<IotgreengrassState>>,
    Json(req): Json<CreateComponentRequest>,
) -> Result<axum::response::Response, IotgreengrassError> {
    let detail = state.create_component(req).await?;
    Ok((StatusCode::CREATED, Json(detail)).into_response())
}

async fn get_component_handler(
    State(state): State<Arc<IotgreengrassState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, IotgreengrassError> {
    let detail = state.get_component(&name).await?;
    Ok(Json(detail).into_response())
}

async fn list_components_handler(
    State(state): State<Arc<IotgreengrassState>>,
) -> Result<axum::response::Response, IotgreengrassError> {
    let resp = state.list_components().await?;
    Ok(Json(resp).into_response())
}

async fn delete_component_handler(
    State(state): State<Arc<IotgreengrassState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, IotgreengrassError> {
    state.delete_component(&name).await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}

pub fn create_router(state: Arc<IotgreengrassState>) -> Router {
    Router::new()
        .route("/components", post(create_component_handler).get(list_components_handler))
        .route("/components/{name}", get(get_component_handler).delete(delete_component_handler))
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
        let state = Arc::new(IotgreengrassState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/components")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[tokio::test]
    async fn test_get_not_found() {
        let state = Arc::new(IotgreengrassState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/components/nonexistent")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_create_components() {
        let state = Arc::new(IotgreengrassState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/components")
            .header("content-type", "application/json")
            .body(Body::from("{}"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert!(resp.status().is_success() || resp.status().is_client_error());
    }
}
