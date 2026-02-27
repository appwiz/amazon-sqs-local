use std::sync::Arc;

use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use axum::http::StatusCode;

use super::error::DevopsguruError;
use super::state::DevopsguruState;
use super::types::*;

async fn create_insight_handler(
    State(state): State<Arc<DevopsguruState>>,
    Json(req): Json<CreateInsightRequest>,
) -> Result<axum::response::Response, DevopsguruError> {
    let detail = state.create_insight(req).await?;
    Ok((StatusCode::CREATED, Json(detail)).into_response())
}

async fn get_insight_handler(
    State(state): State<Arc<DevopsguruState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, DevopsguruError> {
    let detail = state.get_insight(&name).await?;
    Ok(Json(detail).into_response())
}

async fn list_insights_handler(
    State(state): State<Arc<DevopsguruState>>,
) -> Result<axum::response::Response, DevopsguruError> {
    let resp = state.list_insights().await?;
    Ok(Json(resp).into_response())
}

async fn delete_insight_handler(
    State(state): State<Arc<DevopsguruState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, DevopsguruError> {
    state.delete_insight(&name).await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}

pub fn create_router(state: Arc<DevopsguruState>) -> Router {
    Router::new()
        .route("/insights", post(create_insight_handler).get(list_insights_handler))
        .route("/insights/{name}", get(get_insight_handler).delete(delete_insight_handler))
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
        let state = Arc::new(DevopsguruState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/insights")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[tokio::test]
    async fn test_get_not_found() {
        let state = Arc::new(DevopsguruState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/insights/nonexistent")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_create_insights() {
        let state = Arc::new(DevopsguruState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/insights")
            .header("content-type", "application/json")
            .body(Body::from("{}"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert!(resp.status().is_success() || resp.status().is_client_error());
    }
}
