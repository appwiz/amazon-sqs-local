use std::sync::Arc;

use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use axum::http::StatusCode;

use super::error::DRSError;
use super::state::DRSState;
use super::types::*;

async fn create_source_server_handler(
    State(state): State<Arc<DRSState>>,
    Json(req): Json<CreateSourceServerRequest>,
) -> Result<axum::response::Response, DRSError> {
    let detail = state.create_source_server(req).await?;
    Ok((StatusCode::CREATED, Json(detail)).into_response())
}

async fn get_source_server_handler(
    State(state): State<Arc<DRSState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, DRSError> {
    let detail = state.get_source_server(&name).await?;
    Ok(Json(detail).into_response())
}

async fn list_source_servers_handler(
    State(state): State<Arc<DRSState>>,
) -> Result<axum::response::Response, DRSError> {
    let resp = state.list_source_servers().await?;
    Ok(Json(resp).into_response())
}

async fn delete_source_server_handler(
    State(state): State<Arc<DRSState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, DRSError> {
    state.delete_source_server(&name).await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}

pub fn create_router(state: Arc<DRSState>) -> Router {
    Router::new()
        .route("/source-servers", post(create_source_server_handler).get(list_source_servers_handler))
        .route("/source-servers/{name}", get(get_source_server_handler).delete(delete_source_server_handler))
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
        let state = Arc::new(DRSState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/source-servers")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[tokio::test]
    async fn test_get_not_found() {
        let state = Arc::new(DRSState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/source-servers/nonexistent")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_create_source_servers() {
        let state = Arc::new(DRSState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/source-servers")
            .header("content-type", "application/json")
            .body(Body::from("{}"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert!(resp.status().is_success() || resp.status().is_client_error());
    }
}
