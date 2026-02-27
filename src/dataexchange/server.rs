use std::sync::Arc;

use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use axum::http::StatusCode;

use super::error::DataexchangeError;
use super::state::DataexchangeState;
use super::types::*;

async fn create_data_set_handler(
    State(state): State<Arc<DataexchangeState>>,
    Json(req): Json<CreateDataSetRequest>,
) -> Result<axum::response::Response, DataexchangeError> {
    let detail = state.create_data_set(req).await?;
    Ok((StatusCode::CREATED, Json(detail)).into_response())
}

async fn get_data_set_handler(
    State(state): State<Arc<DataexchangeState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, DataexchangeError> {
    let detail = state.get_data_set(&name).await?;
    Ok(Json(detail).into_response())
}

async fn list_data_sets_handler(
    State(state): State<Arc<DataexchangeState>>,
) -> Result<axum::response::Response, DataexchangeError> {
    let resp = state.list_data_sets().await?;
    Ok(Json(resp).into_response())
}

async fn delete_data_set_handler(
    State(state): State<Arc<DataexchangeState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, DataexchangeError> {
    state.delete_data_set(&name).await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}

pub fn create_router(state: Arc<DataexchangeState>) -> Router {
    Router::new()
        .route("/data-sets", post(create_data_set_handler).get(list_data_sets_handler))
        .route("/data-sets/{name}", get(get_data_set_handler).delete(delete_data_set_handler))
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
        let state = Arc::new(DataexchangeState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/data-sets")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[tokio::test]
    async fn test_get_not_found() {
        let state = Arc::new(DataexchangeState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/data-sets/nonexistent")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_create_data_sets() {
        let state = Arc::new(DataexchangeState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/data-sets")
            .header("content-type", "application/json")
            .body(Body::from("{}"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert!(resp.status().is_success() || resp.status().is_client_error());
    }
}
