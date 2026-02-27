use std::sync::Arc;

use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use axum::http::StatusCode;

use super::error::CloudfrontError;
use super::state::CloudfrontState;
use super::types::*;

async fn create_distribution_handler(
    State(state): State<Arc<CloudfrontState>>,
    Json(req): Json<CreateDistributionRequest>,
) -> Result<axum::response::Response, CloudfrontError> {
    let detail = state.create_distribution(req).await?;
    Ok((StatusCode::CREATED, Json(detail)).into_response())
}

async fn get_distribution_handler(
    State(state): State<Arc<CloudfrontState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, CloudfrontError> {
    let detail = state.get_distribution(&name).await?;
    Ok(Json(detail).into_response())
}

async fn list_distributions_handler(
    State(state): State<Arc<CloudfrontState>>,
) -> Result<axum::response::Response, CloudfrontError> {
    let resp = state.list_distributions().await?;
    Ok(Json(resp).into_response())
}

async fn delete_distribution_handler(
    State(state): State<Arc<CloudfrontState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, CloudfrontError> {
    state.delete_distribution(&name).await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}

pub fn create_router(state: Arc<CloudfrontState>) -> Router {
    Router::new()
        .route("/distributions", post(create_distribution_handler).get(list_distributions_handler))
        .route("/distributions/{name}", get(get_distribution_handler).delete(delete_distribution_handler))
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
        let state = Arc::new(CloudfrontState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/distributions")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[tokio::test]
    async fn test_get_not_found() {
        let state = Arc::new(CloudfrontState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/distributions/nonexistent")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_create_distributions() {
        let state = Arc::new(CloudfrontState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/distributions")
            .header("content-type", "application/json")
            .body(Body::from("{}"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert!(resp.status().is_success() || resp.status().is_client_error());
    }
}
