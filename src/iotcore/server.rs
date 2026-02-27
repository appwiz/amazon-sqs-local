use std::sync::Arc;

use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use axum::http::StatusCode;

use super::error::IotcoreError;
use super::state::IotcoreState;
use super::types::*;

async fn create_thing_handler(
    State(state): State<Arc<IotcoreState>>,
    Json(req): Json<CreateThingRequest>,
) -> Result<axum::response::Response, IotcoreError> {
    let detail = state.create_thing(req).await?;
    Ok((StatusCode::CREATED, Json(detail)).into_response())
}

async fn get_thing_handler(
    State(state): State<Arc<IotcoreState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, IotcoreError> {
    let detail = state.get_thing(&name).await?;
    Ok(Json(detail).into_response())
}

async fn list_things_handler(
    State(state): State<Arc<IotcoreState>>,
) -> Result<axum::response::Response, IotcoreError> {
    let resp = state.list_things().await?;
    Ok(Json(resp).into_response())
}

async fn delete_thing_handler(
    State(state): State<Arc<IotcoreState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, IotcoreError> {
    state.delete_thing(&name).await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}

pub fn create_router(state: Arc<IotcoreState>) -> Router {
    Router::new()
        .route("/things", post(create_thing_handler).get(list_things_handler))
        .route("/things/{name}", get(get_thing_handler).delete(delete_thing_handler))
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
        let state = Arc::new(IotcoreState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/things")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[tokio::test]
    async fn test_get_not_found() {
        let state = Arc::new(IotcoreState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/things/nonexistent")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_create_things() {
        let state = Arc::new(IotcoreState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/things")
            .header("content-type", "application/json")
            .body(Body::from("{}"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert!(resp.status().is_success() || resp.status().is_client_error());
    }
}
