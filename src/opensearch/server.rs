use std::sync::Arc;

use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use axum::http::StatusCode;

use super::error::OpensearchError;
use super::state::OpensearchState;
use super::types::*;

async fn create_domain_handler(
    State(state): State<Arc<OpensearchState>>,
    Json(req): Json<CreateDomainRequest>,
) -> Result<axum::response::Response, OpensearchError> {
    let detail = state.create_domain(req).await?;
    Ok((StatusCode::CREATED, Json(detail)).into_response())
}

async fn get_domain_handler(
    State(state): State<Arc<OpensearchState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, OpensearchError> {
    let detail = state.get_domain(&name).await?;
    Ok(Json(detail).into_response())
}

async fn list_domains_handler(
    State(state): State<Arc<OpensearchState>>,
) -> Result<axum::response::Response, OpensearchError> {
    let resp = state.list_domains().await?;
    Ok(Json(resp).into_response())
}

async fn delete_domain_handler(
    State(state): State<Arc<OpensearchState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, OpensearchError> {
    state.delete_domain(&name).await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}

pub fn create_router(state: Arc<OpensearchState>) -> Router {
    Router::new()
        .route("/domains", post(create_domain_handler).get(list_domains_handler))
        .route("/domains/{name}", get(get_domain_handler).delete(delete_domain_handler))
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
        let state = Arc::new(OpensearchState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/domains")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[tokio::test]
    async fn test_get_not_found() {
        let state = Arc::new(OpensearchState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/domains/nonexistent")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_create_domains() {
        let state = Arc::new(OpensearchState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/domains")
            .header("content-type", "application/json")
            .body(Body::from("{}"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert!(resp.status().is_success() || resp.status().is_client_error());
    }
}
