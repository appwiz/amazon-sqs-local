use std::sync::Arc;

use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use axum::http::StatusCode;

use super::error::AppfabricError;
use super::state::AppfabricState;
use super::types::*;

async fn create_app_bundle_handler(
    State(state): State<Arc<AppfabricState>>,
    Json(req): Json<CreateAppBundleRequest>,
) -> Result<axum::response::Response, AppfabricError> {
    let detail = state.create_app_bundle(req).await?;
    Ok((StatusCode::CREATED, Json(detail)).into_response())
}

async fn get_app_bundle_handler(
    State(state): State<Arc<AppfabricState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, AppfabricError> {
    let detail = state.get_app_bundle(&name).await?;
    Ok(Json(detail).into_response())
}

async fn list_app_bundles_handler(
    State(state): State<Arc<AppfabricState>>,
) -> Result<axum::response::Response, AppfabricError> {
    let resp = state.list_app_bundles().await?;
    Ok(Json(resp).into_response())
}

async fn delete_app_bundle_handler(
    State(state): State<Arc<AppfabricState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, AppfabricError> {
    state.delete_app_bundle(&name).await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}

pub fn create_router(state: Arc<AppfabricState>) -> Router {
    Router::new()
        .route("/app-bundles", post(create_app_bundle_handler).get(list_app_bundles_handler))
        .route("/app-bundles/{name}", get(get_app_bundle_handler).delete(delete_app_bundle_handler))
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
        let state = Arc::new(AppfabricState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/app-bundles")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[tokio::test]
    async fn test_get_not_found() {
        let state = Arc::new(AppfabricState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/app-bundles/nonexistent")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_create_app_bundles() {
        let state = Arc::new(AppfabricState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/app-bundles")
            .header("content-type", "application/json")
            .body(Body::from("{}"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert!(resp.status().is_success() || resp.status().is_client_error());
    }
}
