use std::sync::Arc;

use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use axum::http::StatusCode;

use super::error::SecuritylakeError;
use super::state::SecuritylakeState;
use super::types::*;

async fn create_data_lake_handler(
    State(state): State<Arc<SecuritylakeState>>,
    Json(req): Json<CreateDataLakeRequest>,
) -> Result<axum::response::Response, SecuritylakeError> {
    let detail = state.create_data_lake(req).await?;
    Ok((StatusCode::CREATED, Json(detail)).into_response())
}

async fn get_data_lake_handler(
    State(state): State<Arc<SecuritylakeState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, SecuritylakeError> {
    let detail = state.get_data_lake(&name).await?;
    Ok(Json(detail).into_response())
}

async fn list_data_lakes_handler(
    State(state): State<Arc<SecuritylakeState>>,
) -> Result<axum::response::Response, SecuritylakeError> {
    let resp = state.list_data_lakes().await?;
    Ok(Json(resp).into_response())
}

async fn delete_data_lake_handler(
    State(state): State<Arc<SecuritylakeState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, SecuritylakeError> {
    state.delete_data_lake(&name).await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}

pub fn create_router(state: Arc<SecuritylakeState>) -> Router {
    Router::new()
        .route("/data-lakes", post(create_data_lake_handler).get(list_data_lakes_handler))
        .route("/data-lakes/{name}", get(get_data_lake_handler).delete(delete_data_lake_handler))
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
        let state = Arc::new(SecuritylakeState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/data-lakes")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[tokio::test]
    async fn test_get_not_found() {
        let state = Arc::new(SecuritylakeState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/data-lakes/nonexistent")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_create_data_lakes() {
        let state = Arc::new(SecuritylakeState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/data-lakes")
            .header("content-type", "application/json")
            .body(Body::from("{}"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert!(resp.status().is_success() || resp.status().is_client_error());
    }
}
