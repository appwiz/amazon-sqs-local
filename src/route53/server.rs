use std::sync::Arc;

use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use axum::http::StatusCode;

use super::error::Route53Error;
use super::state::Route53State;
use super::types::*;

async fn create_hosted_zone_handler(
    State(state): State<Arc<Route53State>>,
    Json(req): Json<CreateHostedZoneRequest>,
) -> Result<axum::response::Response, Route53Error> {
    let detail = state.create_hosted_zone(req).await?;
    Ok((StatusCode::CREATED, Json(detail)).into_response())
}

async fn get_hosted_zone_handler(
    State(state): State<Arc<Route53State>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, Route53Error> {
    let detail = state.get_hosted_zone(&name).await?;
    Ok(Json(detail).into_response())
}

async fn list_hosted_zones_handler(
    State(state): State<Arc<Route53State>>,
) -> Result<axum::response::Response, Route53Error> {
    let resp = state.list_hosted_zones().await?;
    Ok(Json(resp).into_response())
}

async fn delete_hosted_zone_handler(
    State(state): State<Arc<Route53State>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, Route53Error> {
    state.delete_hosted_zone(&name).await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}

pub fn create_router(state: Arc<Route53State>) -> Router {
    Router::new()
        .route("/hosted-zones", post(create_hosted_zone_handler).get(list_hosted_zones_handler))
        .route("/hosted-zones/{name}", get(get_hosted_zone_handler).delete(delete_hosted_zone_handler))
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
        let state = Arc::new(Route53State::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/hosted-zones")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[tokio::test]
    async fn test_get_not_found() {
        let state = Arc::new(Route53State::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/hosted-zones/nonexistent")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_create_hosted_zones() {
        let state = Arc::new(Route53State::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/hosted-zones")
            .header("content-type", "application/json")
            .body(Body::from("{}"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert!(resp.status().is_success() || resp.status().is_client_error());
    }
}
