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
    Ok((StatusCode::OK, Json(serde_json::to_value(resp).unwrap())).into_response())
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
    Ok((StatusCode::OK, Json(serde_json::to_value(resp).unwrap())).into_response())
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
    Ok((StatusCode::OK, Json(serde_json::to_value(resp).unwrap())).into_response())
}

#[derive(Deserialize)]
struct ListIdentitiesQuery {
    #[serde(rename = "PageSize")]
    page_size: Option<usize>,
    #[serde(rename = "NextToken")]
    next_token: Option<String>,
}

// GET /v2/email/identities
async fn list_email_identities(
    State(state): State<Arc<SesState>>,
    Query(query): Query<ListIdentitiesQuery>,
) -> Result<axum::response::Response, SesError> {
    let resp = state.list_email_identities(query.page_size).await?;
    Ok((StatusCode::OK, Json(serde_json::to_value(resp).unwrap())).into_response())
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
