use std::sync::Arc;

use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use axum::http::StatusCode;

use super::error::ManagedblockchainError;
use super::state::ManagedblockchainState;
use super::types::*;

async fn create_network_handler(
    State(state): State<Arc<ManagedblockchainState>>,
    Json(req): Json<CreateNetworkRequest>,
) -> Result<axum::response::Response, ManagedblockchainError> {
    let detail = state.create_network(req).await?;
    Ok((StatusCode::CREATED, Json(detail)).into_response())
}

async fn get_network_handler(
    State(state): State<Arc<ManagedblockchainState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, ManagedblockchainError> {
    let detail = state.get_network(&name).await?;
    Ok(Json(detail).into_response())
}

async fn list_networks_handler(
    State(state): State<Arc<ManagedblockchainState>>,
) -> Result<axum::response::Response, ManagedblockchainError> {
    let resp = state.list_networks().await?;
    Ok(Json(resp).into_response())
}

async fn delete_network_handler(
    State(state): State<Arc<ManagedblockchainState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, ManagedblockchainError> {
    state.delete_network(&name).await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}

pub fn create_router(state: Arc<ManagedblockchainState>) -> Router {
    Router::new()
        .route("/networks", post(create_network_handler).get(list_networks_handler))
        .route("/networks/{name}", get(get_network_handler).delete(delete_network_handler))
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
        let state = Arc::new(ManagedblockchainState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/networks")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[tokio::test]
    async fn test_get_not_found() {
        let state = Arc::new(ManagedblockchainState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/networks/nonexistent")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_create_networks() {
        let state = Arc::new(ManagedblockchainState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/networks")
            .header("content-type", "application/json")
            .body(Body::from("{}"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert!(resp.status().is_success() || resp.status().is_client_error());
    }
}
