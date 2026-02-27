use std::sync::Arc;

use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use axum::http::StatusCode;

use super::error::LexError;
use super::state::LexState;
use super::types::*;

async fn create_bot_handler(
    State(state): State<Arc<LexState>>,
    Json(req): Json<CreateBotRequest>,
) -> Result<axum::response::Response, LexError> {
    let detail = state.create_bot(req).await?;
    Ok((StatusCode::CREATED, Json(detail)).into_response())
}

async fn get_bot_handler(
    State(state): State<Arc<LexState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, LexError> {
    let detail = state.get_bot(&name).await?;
    Ok(Json(detail).into_response())
}

async fn list_bots_handler(
    State(state): State<Arc<LexState>>,
) -> Result<axum::response::Response, LexError> {
    let resp = state.list_bots().await?;
    Ok(Json(resp).into_response())
}

async fn delete_bot_handler(
    State(state): State<Arc<LexState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, LexError> {
    state.delete_bot(&name).await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}

pub fn create_router(state: Arc<LexState>) -> Router {
    Router::new()
        .route("/bots", post(create_bot_handler).get(list_bots_handler))
        .route("/bots/{name}", get(get_bot_handler).delete(delete_bot_handler))
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
        let state = Arc::new(LexState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/bots")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[tokio::test]
    async fn test_get_not_found() {
        let state = Arc::new(LexState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/bots/nonexistent")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_create_bots() {
        let state = Arc::new(LexState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/bots")
            .header("content-type", "application/json")
            .body(Body::from("{}"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert!(resp.status().is_success() || resp.status().is_client_error());
    }
}
