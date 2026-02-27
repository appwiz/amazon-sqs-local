use std::sync::Arc;

use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use axum::http::StatusCode;

use super::error::ManagedprometheusError;
use super::state::ManagedprometheusState;
use super::types::*;

async fn create_workspace_handler(
    State(state): State<Arc<ManagedprometheusState>>,
    Json(req): Json<CreateWorkspaceRequest>,
) -> Result<axum::response::Response, ManagedprometheusError> {
    let detail = state.create_workspace(req).await?;
    Ok((StatusCode::CREATED, Json(detail)).into_response())
}

async fn get_workspace_handler(
    State(state): State<Arc<ManagedprometheusState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, ManagedprometheusError> {
    let detail = state.get_workspace(&name).await?;
    Ok(Json(detail).into_response())
}

async fn list_workspaces_handler(
    State(state): State<Arc<ManagedprometheusState>>,
) -> Result<axum::response::Response, ManagedprometheusError> {
    let resp = state.list_workspaces().await?;
    Ok(Json(resp).into_response())
}

async fn delete_workspace_handler(
    State(state): State<Arc<ManagedprometheusState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, ManagedprometheusError> {
    state.delete_workspace(&name).await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}

pub fn create_router(state: Arc<ManagedprometheusState>) -> Router {
    Router::new()
        .route("/workspaces", post(create_workspace_handler).get(list_workspaces_handler))
        .route("/workspaces/{name}", get(get_workspace_handler).delete(delete_workspace_handler))
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
        let state = Arc::new(ManagedprometheusState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/workspaces")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[tokio::test]
    async fn test_get_not_found() {
        let state = Arc::new(ManagedprometheusState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/workspaces/nonexistent")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_create_workspaces() {
        let state = Arc::new(ManagedprometheusState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/workspaces")
            .header("content-type", "application/json")
            .body(Body::from("{}"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert!(resp.status().is_success() || resp.status().is_client_error());
    }
}
