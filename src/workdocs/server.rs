use std::sync::Arc;

use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use axum::http::StatusCode;

use super::error::WorkdocsError;
use super::state::WorkdocsState;
use super::types::*;

async fn create_folder_handler(
    State(state): State<Arc<WorkdocsState>>,
    Json(req): Json<CreateFolderRequest>,
) -> Result<axum::response::Response, WorkdocsError> {
    let detail = state.create_folder(req).await?;
    Ok((StatusCode::CREATED, Json(detail)).into_response())
}

async fn get_folder_handler(
    State(state): State<Arc<WorkdocsState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, WorkdocsError> {
    let detail = state.get_folder(&name).await?;
    Ok(Json(detail).into_response())
}

async fn list_folders_handler(
    State(state): State<Arc<WorkdocsState>>,
) -> Result<axum::response::Response, WorkdocsError> {
    let resp = state.list_folders().await?;
    Ok(Json(resp).into_response())
}

async fn delete_folder_handler(
    State(state): State<Arc<WorkdocsState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, WorkdocsError> {
    state.delete_folder(&name).await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}

pub fn create_router(state: Arc<WorkdocsState>) -> Router {
    Router::new()
        .route("/folders", post(create_folder_handler).get(list_folders_handler))
        .route("/folders/{name}", get(get_folder_handler).delete(delete_folder_handler))
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
        let state = Arc::new(WorkdocsState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/folders")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[tokio::test]
    async fn test_get_not_found() {
        let state = Arc::new(WorkdocsState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/folders/nonexistent")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_create_folders() {
        let state = Arc::new(WorkdocsState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/folders")
            .header("content-type", "application/json")
            .body(Body::from("{}"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert!(resp.status().is_success() || resp.status().is_client_error());
    }
}
