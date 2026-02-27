use std::sync::Arc;

use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use axum::http::StatusCode;

use super::error::MSKError;
use super::state::MSKState;
use super::types::*;

async fn create_cluster_handler(
    State(state): State<Arc<MSKState>>,
    Json(req): Json<CreateClusterRequest>,
) -> Result<axum::response::Response, MSKError> {
    let detail = state.create_cluster(req).await?;
    Ok((StatusCode::CREATED, Json(detail)).into_response())
}

async fn get_cluster_handler(
    State(state): State<Arc<MSKState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, MSKError> {
    let detail = state.get_cluster(&name).await?;
    Ok(Json(detail).into_response())
}

async fn list_clusters_handler(
    State(state): State<Arc<MSKState>>,
) -> Result<axum::response::Response, MSKError> {
    let resp = state.list_clusters().await?;
    Ok(Json(resp).into_response())
}

async fn delete_cluster_handler(
    State(state): State<Arc<MSKState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, MSKError> {
    state.delete_cluster(&name).await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}

pub fn create_router(state: Arc<MSKState>) -> Router {
    Router::new()
        .route("/clusters", post(create_cluster_handler).get(list_clusters_handler))
        .route("/clusters/{name}", get(get_cluster_handler).delete(delete_cluster_handler))
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
        let state = Arc::new(MSKState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/clusters")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[tokio::test]
    async fn test_get_not_found() {
        let state = Arc::new(MSKState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/clusters/nonexistent")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_create_clusters() {
        let state = Arc::new(MSKState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/clusters")
            .header("content-type", "application/json")
            .body(Body::from("{}"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert!(resp.status().is_success() || resp.status().is_client_error());
    }
}
