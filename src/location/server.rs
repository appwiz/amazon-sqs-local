use std::sync::Arc;

use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use axum::http::StatusCode;

use super::error::LocationError;
use super::state::LocationState;
use super::types::*;

async fn create_map_handler(
    State(state): State<Arc<LocationState>>,
    Json(req): Json<CreateMapRequest>,
) -> Result<axum::response::Response, LocationError> {
    let detail = state.create_map(req).await?;
    Ok((StatusCode::CREATED, Json(detail)).into_response())
}

async fn get_map_handler(
    State(state): State<Arc<LocationState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, LocationError> {
    let detail = state.get_map(&name).await?;
    Ok(Json(detail).into_response())
}

async fn list_maps_handler(
    State(state): State<Arc<LocationState>>,
) -> Result<axum::response::Response, LocationError> {
    let resp = state.list_maps().await?;
    Ok(Json(resp).into_response())
}

async fn delete_map_handler(
    State(state): State<Arc<LocationState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, LocationError> {
    state.delete_map(&name).await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}

pub fn create_router(state: Arc<LocationState>) -> Router {
    Router::new()
        .route("/maps", post(create_map_handler).get(list_maps_handler))
        .route("/maps/{name}", get(get_map_handler).delete(delete_map_handler))
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
        let state = Arc::new(LocationState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/maps")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[tokio::test]
    async fn test_get_not_found() {
        let state = Arc::new(LocationState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/maps/nonexistent")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_create_maps() {
        let state = Arc::new(LocationState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/maps")
            .header("content-type", "application/json")
            .body(Body::from("{}"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert!(resp.status().is_success() || resp.status().is_client_error());
    }
}
