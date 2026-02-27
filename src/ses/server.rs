use std::sync::Arc;

use axum::body::Bytes;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{delete, get, post};
use axum::{Json, Router};
use serde::Deserialize;

use super::error::SesError;
use super::state::SesState;
use super::types::*;

// POST /v2/email/outbound-emails
async fn send_email(
    State(state): State<Arc<SesState>>,
    body: Bytes,
) -> Result<axum::response::Response, SesError> {
    let req: SendEmailRequest = serde_json::from_slice(&body)
        .map_err(|e| SesError::BadRequestException(e.to_string()))?;
    let resp = state.send_email(req).await?;
    Ok((StatusCode::OK, Json(resp)).into_response())
}

// POST /v2/email/identities
async fn create_email_identity(
    State(state): State<Arc<SesState>>,
    body: Bytes,
) -> Result<axum::response::Response, SesError> {
    let req: CreateEmailIdentityRequest = serde_json::from_slice(&body)
        .map_err(|e| SesError::BadRequestException(e.to_string()))?;
    let name = req.email_identity.clone();
    let resp = state.create_email_identity(name, req).await?;
    Ok((StatusCode::OK, Json(resp)).into_response())
}

// DELETE /v2/email/identities/{EmailIdentity}
async fn delete_email_identity(
    State(state): State<Arc<SesState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, SesError> {
    state.delete_email_identity(name).await?;
    Ok((StatusCode::OK, Json(serde_json::json!({}))).into_response())
}

// GET /v2/email/identities/{EmailIdentity}
async fn get_email_identity(
    State(state): State<Arc<SesState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, SesError> {
    let resp = state.get_email_identity(name).await?;
    Ok((StatusCode::OK, Json(resp)).into_response())
}

#[derive(Deserialize)]
struct ListIdentitiesQuery {
    #[serde(rename = "PageSize")]
    page_size: Option<usize>,
}

// GET /v2/email/identities
async fn list_email_identities(
    State(state): State<Arc<SesState>>,
    Query(query): Query<ListIdentitiesQuery>,
) -> Result<axum::response::Response, SesError> {
    let resp = state.list_email_identities(query.page_size).await?;
    Ok((StatusCode::OK, Json(resp)).into_response())
}

pub fn create_router(state: Arc<SesState>) -> Router {
    Router::new()
        .route("/v2/email/outbound-emails", post(send_email))
        .route("/v2/email/identities", post(create_email_identity))
        .route("/v2/email/identities", get(list_email_identities))
        .route("/v2/email/identities/{email_identity}", get(get_email_identity))
        .route("/v2/email/identities/{email_identity}", delete(delete_email_identity))
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
        let state = Arc::new(SesState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/v2/email/identities")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[tokio::test]
    async fn test_get_not_found() {
        let state = Arc::new(SesState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/v2/email/identities/nonexistent")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_create_outbound_emails() {
        let state = Arc::new(SesState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/v2/email/outbound-emails")
            .header("content-type", "application/json")
            .body(Body::from("{}"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert!(resp.status().is_success() || resp.status().is_client_error());
    }
    #[tokio::test]
    async fn test_create_identities() {
        let state = Arc::new(SesState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/v2/email/identities")
            .header("content-type", "application/json")
            .body(Body::from("{}"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert!(resp.status().is_success() || resp.status().is_client_error());
    }
    #[tokio::test]
    async fn test_create_identities_2() {
        let state = Arc::new(SesState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/v2/email/identities")
            .header("content-type", "application/json")
            .body(Body::from("{}"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert!(resp.status().is_success() || resp.status().is_client_error());
    }
}
