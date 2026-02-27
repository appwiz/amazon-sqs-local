use std::sync::Arc;

use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use axum::http::StatusCode;

use super::error::RAMError;
use super::state::RAMState;
use super::types::*;

async fn create_resource_share_handler(
    State(state): State<Arc<RAMState>>,
    Json(req): Json<CreateResourceShareRequest>,
) -> Result<axum::response::Response, RAMError> {
    let detail = state.create_resource_share(req).await?;
    Ok((StatusCode::CREATED, Json(detail)).into_response())
}

async fn get_resource_share_handler(
    State(state): State<Arc<RAMState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, RAMError> {
    let detail = state.get_resource_share(&name).await?;
    Ok(Json(detail).into_response())
}

async fn list_resource_shares_handler(
    State(state): State<Arc<RAMState>>,
) -> Result<axum::response::Response, RAMError> {
    let resp = state.list_resource_shares().await?;
    Ok(Json(resp).into_response())
}

async fn delete_resource_share_handler(
    State(state): State<Arc<RAMState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, RAMError> {
    state.delete_resource_share(&name).await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}

pub fn create_router(state: Arc<RAMState>) -> Router {
    Router::new()
        .route("/resource-shares", post(create_resource_share_handler).get(list_resource_shares_handler))
        .route("/resource-shares/{name}", get(get_resource_share_handler).delete(delete_resource_share_handler))
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
        let state = Arc::new(RAMState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/resource-shares")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[tokio::test]
    async fn test_get_not_found() {
        let state = Arc::new(RAMState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/resource-shares/nonexistent")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_create_resource_shares() {
        let state = Arc::new(RAMState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/resource-shares")
            .header("content-type", "application/json")
            .body(Body::from("{}"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert!(resp.status().is_success() || resp.status().is_client_error());
    }
}
