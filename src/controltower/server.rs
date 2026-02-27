use std::sync::Arc;

use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use axum::http::StatusCode;

use super::error::ControltowerError;
use super::state::ControltowerState;
use super::types::*;

async fn create_landing_zone_handler(
    State(state): State<Arc<ControltowerState>>,
    Json(req): Json<CreateLandingZoneRequest>,
) -> Result<axum::response::Response, ControltowerError> {
    let detail = state.create_landing_zone(req).await?;
    Ok((StatusCode::CREATED, Json(detail)).into_response())
}

async fn get_landing_zone_handler(
    State(state): State<Arc<ControltowerState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, ControltowerError> {
    let detail = state.get_landing_zone(&name).await?;
    Ok(Json(detail).into_response())
}

async fn list_landing_zones_handler(
    State(state): State<Arc<ControltowerState>>,
) -> Result<axum::response::Response, ControltowerError> {
    let resp = state.list_landing_zones().await?;
    Ok(Json(resp).into_response())
}

async fn delete_landing_zone_handler(
    State(state): State<Arc<ControltowerState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, ControltowerError> {
    state.delete_landing_zone(&name).await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}

pub fn create_router(state: Arc<ControltowerState>) -> Router {
    Router::new()
        .route("/landing-zones", post(create_landing_zone_handler).get(list_landing_zones_handler))
        .route("/landing-zones/{name}", get(get_landing_zone_handler).delete(delete_landing_zone_handler))
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
        let state = Arc::new(ControltowerState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/landing-zones")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[tokio::test]
    async fn test_get_not_found() {
        let state = Arc::new(ControltowerState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/landing-zones/nonexistent")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_create_landing_zones() {
        let state = Arc::new(ControltowerState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/landing-zones")
            .header("content-type", "application/json")
            .body(Body::from("{}"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert!(resp.status().is_success() || resp.status().is_client_error());
    }
}
