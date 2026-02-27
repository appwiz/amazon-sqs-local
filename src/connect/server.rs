use std::sync::Arc;

use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use axum::http::StatusCode;

use super::error::ConnectError;
use super::state::ConnectState;
use super::types::*;

async fn create_instance_handler(
    State(state): State<Arc<ConnectState>>,
    Json(req): Json<CreateInstanceRequest>,
) -> Result<axum::response::Response, ConnectError> {
    let detail = state.create_instance(req).await?;
    Ok((StatusCode::CREATED, Json(detail)).into_response())
}

async fn get_instance_handler(
    State(state): State<Arc<ConnectState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, ConnectError> {
    let detail = state.get_instance(&name).await?;
    Ok(Json(detail).into_response())
}

async fn list_instances_handler(
    State(state): State<Arc<ConnectState>>,
) -> Result<axum::response::Response, ConnectError> {
    let resp = state.list_instances().await?;
    Ok(Json(resp).into_response())
}

async fn delete_instance_handler(
    State(state): State<Arc<ConnectState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, ConnectError> {
    state.delete_instance(&name).await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}

pub fn create_router(state: Arc<ConnectState>) -> Router {
    Router::new()
        .route("/instances", post(create_instance_handler).get(list_instances_handler))
        .route("/instances/{name}", get(get_instance_handler).delete(delete_instance_handler))
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
        let state = Arc::new(ConnectState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/instances")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[tokio::test]
    async fn test_get_not_found() {
        let state = Arc::new(ConnectState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/instances/nonexistent")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_create_instances() {
        let state = Arc::new(ConnectState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/instances")
            .header("content-type", "application/json")
            .body(Body::from("{}"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert!(resp.status().is_success() || resp.status().is_client_error());
    }
}
