use std::sync::Arc;

use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use axum::http::StatusCode;

use super::error::IotsitewiseError;
use super::state::IotsitewiseState;
use super::types::*;

async fn create_asset_handler(
    State(state): State<Arc<IotsitewiseState>>,
    Json(req): Json<CreateAssetRequest>,
) -> Result<axum::response::Response, IotsitewiseError> {
    let detail = state.create_asset(req).await?;
    Ok((StatusCode::CREATED, Json(detail)).into_response())
}

async fn get_asset_handler(
    State(state): State<Arc<IotsitewiseState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, IotsitewiseError> {
    let detail = state.get_asset(&name).await?;
    Ok(Json(detail).into_response())
}

async fn list_assets_handler(
    State(state): State<Arc<IotsitewiseState>>,
) -> Result<axum::response::Response, IotsitewiseError> {
    let resp = state.list_assets().await?;
    Ok(Json(resp).into_response())
}

async fn delete_asset_handler(
    State(state): State<Arc<IotsitewiseState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, IotsitewiseError> {
    state.delete_asset(&name).await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}

pub fn create_router(state: Arc<IotsitewiseState>) -> Router {
    Router::new()
        .route("/assets", post(create_asset_handler).get(list_assets_handler))
        .route("/assets/{name}", get(get_asset_handler).delete(delete_asset_handler))
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
        let state = Arc::new(IotsitewiseState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/assets")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[tokio::test]
    async fn test_get_not_found() {
        let state = Arc::new(IotsitewiseState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/assets/nonexistent")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_create_assets() {
        let state = Arc::new(IotsitewiseState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/assets")
            .header("content-type", "application/json")
            .body(Body::from("{}"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert!(resp.status().is_success() || resp.status().is_client_error());
    }
}
