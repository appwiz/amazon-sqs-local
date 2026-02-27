use std::sync::Arc;

use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use axum::http::StatusCode;

use super::error::CleanroomsError;
use super::state::CleanroomsState;
use super::types::*;

async fn create_collaboration_handler(
    State(state): State<Arc<CleanroomsState>>,
    Json(req): Json<CreateCollaborationRequest>,
) -> Result<axum::response::Response, CleanroomsError> {
    let detail = state.create_collaboration(req).await?;
    Ok((StatusCode::CREATED, Json(detail)).into_response())
}

async fn get_collaboration_handler(
    State(state): State<Arc<CleanroomsState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, CleanroomsError> {
    let detail = state.get_collaboration(&name).await?;
    Ok(Json(detail).into_response())
}

async fn list_collaborations_handler(
    State(state): State<Arc<CleanroomsState>>,
) -> Result<axum::response::Response, CleanroomsError> {
    let resp = state.list_collaborations().await?;
    Ok(Json(resp).into_response())
}

async fn delete_collaboration_handler(
    State(state): State<Arc<CleanroomsState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, CleanroomsError> {
    state.delete_collaboration(&name).await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}

pub fn create_router(state: Arc<CleanroomsState>) -> Router {
    Router::new()
        .route("/collaborations", post(create_collaboration_handler).get(list_collaborations_handler))
        .route("/collaborations/{name}", get(get_collaboration_handler).delete(delete_collaboration_handler))
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
        let state = Arc::new(CleanroomsState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/collaborations")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[tokio::test]
    async fn test_get_not_found() {
        let state = Arc::new(CleanroomsState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/collaborations/nonexistent")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_create_collaborations() {
        let state = Arc::new(CleanroomsState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/collaborations")
            .header("content-type", "application/json")
            .body(Body::from("{}"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert!(resp.status().is_success() || resp.status().is_client_error());
    }
}
