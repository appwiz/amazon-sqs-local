use std::sync::Arc;

use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use axum::http::StatusCode;

use super::error::DetectiveError;
use super::state::DetectiveState;
use super::types::*;

async fn create_graph_handler(
    State(state): State<Arc<DetectiveState>>,
    Json(req): Json<CreateGraphRequest>,
) -> Result<axum::response::Response, DetectiveError> {
    let detail = state.create_graph(req).await?;
    Ok((StatusCode::CREATED, Json(detail)).into_response())
}

async fn get_graph_handler(
    State(state): State<Arc<DetectiveState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, DetectiveError> {
    let detail = state.get_graph(&name).await?;
    Ok(Json(detail).into_response())
}

async fn list_graphs_handler(
    State(state): State<Arc<DetectiveState>>,
) -> Result<axum::response::Response, DetectiveError> {
    let resp = state.list_graphs().await?;
    Ok(Json(resp).into_response())
}

async fn delete_graph_handler(
    State(state): State<Arc<DetectiveState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, DetectiveError> {
    state.delete_graph(&name).await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}

pub fn create_router(state: Arc<DetectiveState>) -> Router {
    Router::new()
        .route("/graphs", post(create_graph_handler).get(list_graphs_handler))
        .route("/graphs/{name}", get(get_graph_handler).delete(delete_graph_handler))
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
        let state = Arc::new(DetectiveState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/graphs")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[tokio::test]
    async fn test_get_not_found() {
        let state = Arc::new(DetectiveState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/graphs/nonexistent")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_create_graphs() {
        let state = Arc::new(DetectiveState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/graphs")
            .header("content-type", "application/json")
            .body(Body::from("{}"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert!(resp.status().is_success() || resp.status().is_client_error());
    }
}
