use std::sync::Arc;

use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use axum::http::StatusCode;

use super::error::ChimeError;
use super::state::ChimeState;
use super::types::*;

async fn create_account_handler(
    State(state): State<Arc<ChimeState>>,
    Json(req): Json<CreateAccountRequest>,
) -> Result<axum::response::Response, ChimeError> {
    let detail = state.create_account(req).await?;
    Ok((StatusCode::CREATED, Json(detail)).into_response())
}

async fn get_account_handler(
    State(state): State<Arc<ChimeState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, ChimeError> {
    let detail = state.get_account(&name).await?;
    Ok(Json(detail).into_response())
}

async fn list_accounts_handler(
    State(state): State<Arc<ChimeState>>,
) -> Result<axum::response::Response, ChimeError> {
    let resp = state.list_accounts().await?;
    Ok(Json(resp).into_response())
}

async fn delete_account_handler(
    State(state): State<Arc<ChimeState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, ChimeError> {
    state.delete_account(&name).await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}

pub fn create_router(state: Arc<ChimeState>) -> Router {
    Router::new()
        .route("/accounts", post(create_account_handler).get(list_accounts_handler))
        .route("/accounts/{name}", get(get_account_handler).delete(delete_account_handler))
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
        let state = Arc::new(ChimeState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/accounts")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[tokio::test]
    async fn test_get_not_found() {
        let state = Arc::new(ChimeState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/accounts/nonexistent")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_create_accounts() {
        let state = Arc::new(ChimeState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/accounts")
            .header("content-type", "application/json")
            .body(Body::from("{}"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert!(resp.status().is_success() || resp.status().is_client_error());
    }
}
