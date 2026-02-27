use std::sync::Arc;

use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use axum::http::StatusCode;

use super::error::OutpostsError;
use super::state::OutpostsState;
use super::types::*;

async fn create_outpost_handler(
    State(state): State<Arc<OutpostsState>>,
    Json(req): Json<CreateOutpostRequest>,
) -> Result<axum::response::Response, OutpostsError> {
    let detail = state.create_outpost(req).await?;
    Ok((StatusCode::CREATED, Json(detail)).into_response())
}

async fn get_outpost_handler(
    State(state): State<Arc<OutpostsState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, OutpostsError> {
    let detail = state.get_outpost(&name).await?;
    Ok(Json(detail).into_response())
}

async fn list_outposts_handler(
    State(state): State<Arc<OutpostsState>>,
) -> Result<axum::response::Response, OutpostsError> {
    let resp = state.list_outposts().await?;
    Ok(Json(resp).into_response())
}

async fn delete_outpost_handler(
    State(state): State<Arc<OutpostsState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, OutpostsError> {
    state.delete_outpost(&name).await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}

pub fn create_router(state: Arc<OutpostsState>) -> Router {
    Router::new()
        .route("/outposts", post(create_outpost_handler).get(list_outposts_handler))
        .route("/outposts/{name}", get(get_outpost_handler).delete(delete_outpost_handler))
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
        let state = Arc::new(OutpostsState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/outposts")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[tokio::test]
    async fn test_get_not_found() {
        let state = Arc::new(OutpostsState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/outposts/nonexistent")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_create_outposts() {
        let state = Arc::new(OutpostsState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/outposts")
            .header("content-type", "application/json")
            .body(Body::from("{}"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert!(resp.status().is_success() || resp.status().is_client_error());
    }
}
