use std::sync::Arc;

use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use axum::http::StatusCode;

use super::error::XrayError;
use super::state::XrayState;
use super::types::*;

async fn create_group_handler(
    State(state): State<Arc<XrayState>>,
    Json(req): Json<CreateGroupRequest>,
) -> Result<axum::response::Response, XrayError> {
    let detail = state.create_group(req).await?;
    Ok((StatusCode::CREATED, Json(detail)).into_response())
}

async fn get_group_handler(
    State(state): State<Arc<XrayState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, XrayError> {
    let detail = state.get_group(&name).await?;
    Ok(Json(detail).into_response())
}

async fn list_groups_handler(
    State(state): State<Arc<XrayState>>,
) -> Result<axum::response::Response, XrayError> {
    let resp = state.list_groups().await?;
    Ok(Json(resp).into_response())
}

async fn delete_group_handler(
    State(state): State<Arc<XrayState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, XrayError> {
    state.delete_group(&name).await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}

pub fn create_router(state: Arc<XrayState>) -> Router {
    Router::new()
        .route("/groups", post(create_group_handler).get(list_groups_handler))
        .route("/groups/{name}", get(get_group_handler).delete(delete_group_handler))
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
        let state = Arc::new(XrayState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/groups")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[tokio::test]
    async fn test_get_not_found() {
        let state = Arc::new(XrayState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/groups/nonexistent")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_create_groups() {
        let state = Arc::new(XrayState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/groups")
            .header("content-type", "application/json")
            .body(Body::from("{}"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert!(resp.status().is_success() || resp.status().is_client_error());
    }
}
