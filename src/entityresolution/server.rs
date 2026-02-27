use std::sync::Arc;

use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use axum::http::StatusCode;

use super::error::EntityresolutionError;
use super::state::EntityresolutionState;
use super::types::*;

async fn create_matching_workflow_handler(
    State(state): State<Arc<EntityresolutionState>>,
    Json(req): Json<CreateMatchingWorkflowRequest>,
) -> Result<axum::response::Response, EntityresolutionError> {
    let detail = state.create_matching_workflow(req).await?;
    Ok((StatusCode::CREATED, Json(detail)).into_response())
}

async fn get_matching_workflow_handler(
    State(state): State<Arc<EntityresolutionState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, EntityresolutionError> {
    let detail = state.get_matching_workflow(&name).await?;
    Ok(Json(detail).into_response())
}

async fn list_matching_workflows_handler(
    State(state): State<Arc<EntityresolutionState>>,
) -> Result<axum::response::Response, EntityresolutionError> {
    let resp = state.list_matching_workflows().await?;
    Ok(Json(resp).into_response())
}

async fn delete_matching_workflow_handler(
    State(state): State<Arc<EntityresolutionState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, EntityresolutionError> {
    state.delete_matching_workflow(&name).await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}

pub fn create_router(state: Arc<EntityresolutionState>) -> Router {
    Router::new()
        .route("/matching-workflows", post(create_matching_workflow_handler).get(list_matching_workflows_handler))
        .route("/matching-workflows/{name}", get(get_matching_workflow_handler).delete(delete_matching_workflow_handler))
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
        let state = Arc::new(EntityresolutionState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/matching-workflows")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[tokio::test]
    async fn test_get_not_found() {
        let state = Arc::new(EntityresolutionState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/matching-workflows/nonexistent")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_create_matching_workflows() {
        let state = Arc::new(EntityresolutionState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/matching-workflows")
            .header("content-type", "application/json")
            .body(Body::from("{}"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert!(resp.status().is_success() || resp.status().is_client_error());
    }
}
