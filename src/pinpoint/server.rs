use std::sync::Arc;

use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use axum::http::StatusCode;

use super::error::PinpointError;
use super::state::PinpointState;
use super::types::*;

async fn create_app_handler(
    State(state): State<Arc<PinpointState>>,
    Json(req): Json<CreateAppRequest>,
) -> Result<axum::response::Response, PinpointError> {
    let detail = state.create_app(req).await?;
    Ok((StatusCode::CREATED, Json(detail)).into_response())
}

async fn get_app_handler(
    State(state): State<Arc<PinpointState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, PinpointError> {
    let detail = state.get_app(&name).await?;
    Ok(Json(detail).into_response())
}

async fn list_apps_handler(
    State(state): State<Arc<PinpointState>>,
) -> Result<axum::response::Response, PinpointError> {
    let resp = state.list_apps().await?;
    Ok(Json(resp).into_response())
}

async fn delete_app_handler(
    State(state): State<Arc<PinpointState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, PinpointError> {
    state.delete_app(&name).await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}

pub fn create_router(state: Arc<PinpointState>) -> Router {
    Router::new()
        .route("/apps", post(create_app_handler).get(list_apps_handler))
        .route("/apps/{name}", get(get_app_handler).delete(delete_app_handler))
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
        let state = Arc::new(PinpointState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/apps")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[tokio::test]
    async fn test_get_not_found() {
        let state = Arc::new(PinpointState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/apps/nonexistent")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_create_apps() {
        let state = Arc::new(PinpointState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/apps")
            .header("content-type", "application/json")
            .body(Body::from("{}"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert!(resp.status().is_success() || resp.status().is_client_error());
    }
}
