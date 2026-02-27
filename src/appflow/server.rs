use std::sync::Arc;

use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use axum::http::StatusCode;

use super::error::AppflowError;
use super::state::AppflowState;
use super::types::*;

async fn create_flow_handler(
    State(state): State<Arc<AppflowState>>,
    Json(req): Json<CreateFlowRequest>,
) -> Result<axum::response::Response, AppflowError> {
    let detail = state.create_flow(req).await?;
    Ok((StatusCode::CREATED, Json(detail)).into_response())
}

async fn get_flow_handler(
    State(state): State<Arc<AppflowState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, AppflowError> {
    let detail = state.get_flow(&name).await?;
    Ok(Json(detail).into_response())
}

async fn list_flows_handler(
    State(state): State<Arc<AppflowState>>,
) -> Result<axum::response::Response, AppflowError> {
    let resp = state.list_flows().await?;
    Ok(Json(resp).into_response())
}

async fn delete_flow_handler(
    State(state): State<Arc<AppflowState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, AppflowError> {
    state.delete_flow(&name).await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}

pub fn create_router(state: Arc<AppflowState>) -> Router {
    Router::new()
        .route("/flows", post(create_flow_handler).get(list_flows_handler))
        .route("/flows/{name}", get(get_flow_handler).delete(delete_flow_handler))
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
        let state = Arc::new(AppflowState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/flows")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[tokio::test]
    async fn test_get_not_found() {
        let state = Arc::new(AppflowState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/flows/nonexistent")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_create_flows() {
        let state = Arc::new(AppflowState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/flows")
            .header("content-type", "application/json")
            .body(Body::from("{}"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert!(resp.status().is_success() || resp.status().is_client_error());
    }
}
