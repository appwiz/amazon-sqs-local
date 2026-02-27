use std::sync::Arc;

use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use axum::http::StatusCode;

use super::error::AppmeshError;
use super::state::AppmeshState;
use super::types::*;

async fn create_mesh_handler(
    State(state): State<Arc<AppmeshState>>,
    Json(req): Json<CreateMeshRequest>,
) -> Result<axum::response::Response, AppmeshError> {
    let detail = state.create_mesh(req).await?;
    Ok((StatusCode::CREATED, Json(detail)).into_response())
}

async fn get_mesh_handler(
    State(state): State<Arc<AppmeshState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, AppmeshError> {
    let detail = state.get_mesh(&name).await?;
    Ok(Json(detail).into_response())
}

async fn list_meshs_handler(
    State(state): State<Arc<AppmeshState>>,
) -> Result<axum::response::Response, AppmeshError> {
    let resp = state.list_meshs().await?;
    Ok(Json(resp).into_response())
}

async fn delete_mesh_handler(
    State(state): State<Arc<AppmeshState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, AppmeshError> {
    state.delete_mesh(&name).await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}

pub fn create_router(state: Arc<AppmeshState>) -> Router {
    Router::new()
        .route("/meshes", post(create_mesh_handler).get(list_meshs_handler))
        .route("/meshes/{name}", get(get_mesh_handler).delete(delete_mesh_handler))
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
        let state = Arc::new(AppmeshState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/meshes")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[tokio::test]
    async fn test_get_not_found() {
        let state = Arc::new(AppmeshState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/meshes/nonexistent")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_create_meshes() {
        let state = Arc::new(AppmeshState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/meshes")
            .header("content-type", "application/json")
            .body(Body::from("{}"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert!(resp.status().is_success() || resp.status().is_client_error());
    }
}
