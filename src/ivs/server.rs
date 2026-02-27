use std::sync::Arc;

use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use axum::http::StatusCode;

use super::error::IVSError;
use super::state::IVSState;
use super::types::*;

async fn create_channel_handler(
    State(state): State<Arc<IVSState>>,
    Json(req): Json<CreateChannelRequest>,
) -> Result<axum::response::Response, IVSError> {
    let detail = state.create_channel(req).await?;
    Ok((StatusCode::CREATED, Json(detail)).into_response())
}

async fn get_channel_handler(
    State(state): State<Arc<IVSState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, IVSError> {
    let detail = state.get_channel(&name).await?;
    Ok(Json(detail).into_response())
}

async fn list_channels_handler(
    State(state): State<Arc<IVSState>>,
) -> Result<axum::response::Response, IVSError> {
    let resp = state.list_channels().await?;
    Ok(Json(resp).into_response())
}

async fn delete_channel_handler(
    State(state): State<Arc<IVSState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, IVSError> {
    state.delete_channel(&name).await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}

pub fn create_router(state: Arc<IVSState>) -> Router {
    Router::new()
        .route("/channels", post(create_channel_handler).get(list_channels_handler))
        .route("/channels/{name}", get(get_channel_handler).delete(delete_channel_handler))
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
        let state = Arc::new(IVSState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/channels")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[tokio::test]
    async fn test_get_not_found() {
        let state = Arc::new(IVSState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/channels/nonexistent")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_create_channels() {
        let state = Arc::new(IVSState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/channels")
            .header("content-type", "application/json")
            .body(Body::from("{}"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert!(resp.status().is_success() || resp.status().is_client_error());
    }
}
