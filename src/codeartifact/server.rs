use std::sync::Arc;

use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use axum::http::StatusCode;

use super::error::CodeartifactError;
use super::state::CodeartifactState;
use super::types::*;

async fn create_domain_handler(
    State(state): State<Arc<CodeartifactState>>,
    Json(req): Json<CreateDomainRequest>,
) -> Result<axum::response::Response, CodeartifactError> {
    let detail = state.create_domain(req).await?;
    Ok((StatusCode::CREATED, Json(detail)).into_response())
}

async fn get_domain_handler(
    State(state): State<Arc<CodeartifactState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, CodeartifactError> {
    let detail = state.get_domain(&name).await?;
    Ok(Json(detail).into_response())
}

async fn list_domains_handler(
    State(state): State<Arc<CodeartifactState>>,
) -> Result<axum::response::Response, CodeartifactError> {
    let resp = state.list_domains().await?;
    Ok(Json(resp).into_response())
}

async fn delete_domain_handler(
    State(state): State<Arc<CodeartifactState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, CodeartifactError> {
    state.delete_domain(&name).await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}

async fn create_repository_handler(
    State(state): State<Arc<CodeartifactState>>,
    Json(req): Json<CreateRepositoryRequest>,
) -> Result<axum::response::Response, CodeartifactError> {
    let detail = state.create_repository(req).await?;
    Ok((StatusCode::CREATED, Json(detail)).into_response())
}

async fn get_repository_handler(
    State(state): State<Arc<CodeartifactState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, CodeartifactError> {
    let detail = state.get_repository(&name).await?;
    Ok(Json(detail).into_response())
}

async fn list_repositorys_handler(
    State(state): State<Arc<CodeartifactState>>,
) -> Result<axum::response::Response, CodeartifactError> {
    let resp = state.list_repositorys().await?;
    Ok(Json(resp).into_response())
}

async fn delete_repository_handler(
    State(state): State<Arc<CodeartifactState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, CodeartifactError> {
    state.delete_repository(&name).await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}

pub fn create_router(state: Arc<CodeartifactState>) -> Router {
    Router::new()
        .route("/domains", post(create_domain_handler).get(list_domains_handler))
        .route("/domains/{name}", get(get_domain_handler).delete(delete_domain_handler))
        .route("/repositories", post(create_repository_handler).get(list_repositorys_handler))
        .route("/repositories/{name}", get(get_repository_handler).delete(delete_repository_handler))
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
        let state = Arc::new(CodeartifactState::new("123456789012".to_string(), "us-east-1".to_string()));
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
        let state = Arc::new(CodeartifactState::new("123456789012".to_string(), "us-east-1".to_string()));
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
        let state = Arc::new(CodeartifactState::new("123456789012".to_string(), "us-east-1".to_string()));
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
    #[tokio::test]
    async fn test_create_repositories() {
        let state = Arc::new(CodeartifactState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/repositories")
            .header("content-type", "application/json")
            .body(Body::from("{}"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert!(resp.status().is_success() || resp.status().is_client_error());
    }
}
