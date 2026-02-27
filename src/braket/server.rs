use std::sync::Arc;

use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use axum::http::StatusCode;

use super::error::BraketError;
use super::state::BraketState;
use super::types::*;

async fn create_quantum_task_handler(
    State(state): State<Arc<BraketState>>,
    Json(req): Json<CreateQuantumTaskRequest>,
) -> Result<axum::response::Response, BraketError> {
    let detail = state.create_quantum_task(req).await?;
    Ok((StatusCode::CREATED, Json(detail)).into_response())
}

async fn get_quantum_task_handler(
    State(state): State<Arc<BraketState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, BraketError> {
    let detail = state.get_quantum_task(&name).await?;
    Ok(Json(detail).into_response())
}

async fn list_quantum_tasks_handler(
    State(state): State<Arc<BraketState>>,
) -> Result<axum::response::Response, BraketError> {
    let resp = state.list_quantum_tasks().await?;
    Ok(Json(resp).into_response())
}

async fn delete_quantum_task_handler(
    State(state): State<Arc<BraketState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, BraketError> {
    state.delete_quantum_task(&name).await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}

pub fn create_router(state: Arc<BraketState>) -> Router {
    Router::new()
        .route("/quantum-tasks", post(create_quantum_task_handler).get(list_quantum_tasks_handler))
        .route("/quantum-tasks/{name}", get(get_quantum_task_handler).delete(delete_quantum_task_handler))
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
        let state = Arc::new(BraketState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/quantum-tasks")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[tokio::test]
    async fn test_get_not_found() {
        let state = Arc::new(BraketState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/quantum-tasks/nonexistent")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_create_quantum_tasks() {
        let state = Arc::new(BraketState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/quantum-tasks")
            .header("content-type", "application/json")
            .body(Body::from("{}"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert!(resp.status().is_success() || resp.status().is_client_error());
    }
}
