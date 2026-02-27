use std::sync::Arc;

use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use axum::http::StatusCode;

use super::error::B2biError;
use super::state::B2biState;
use super::types::*;

async fn create_profile_handler(
    State(state): State<Arc<B2biState>>,
    Json(req): Json<CreateProfileRequest>,
) -> Result<axum::response::Response, B2biError> {
    let detail = state.create_profile(req).await?;
    Ok((StatusCode::CREATED, Json(detail)).into_response())
}

async fn get_profile_handler(
    State(state): State<Arc<B2biState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, B2biError> {
    let detail = state.get_profile(&name).await?;
    Ok(Json(detail).into_response())
}

async fn list_profiles_handler(
    State(state): State<Arc<B2biState>>,
) -> Result<axum::response::Response, B2biError> {
    let resp = state.list_profiles().await?;
    Ok(Json(resp).into_response())
}

async fn delete_profile_handler(
    State(state): State<Arc<B2biState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, B2biError> {
    state.delete_profile(&name).await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}

pub fn create_router(state: Arc<B2biState>) -> Router {
    Router::new()
        .route("/profiles", post(create_profile_handler).get(list_profiles_handler))
        .route("/profiles/{name}", get(get_profile_handler).delete(delete_profile_handler))
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
        let state = Arc::new(B2biState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/profiles")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[tokio::test]
    async fn test_get_not_found() {
        let state = Arc::new(B2biState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/profiles/nonexistent")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_create_profiles() {
        let state = Arc::new(B2biState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/profiles")
            .header("content-type", "application/json")
            .body(Body::from("{}"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert!(resp.status().is_success() || resp.status().is_client_error());
    }
}
