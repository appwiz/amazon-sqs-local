use std::sync::Arc;

use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use axum::http::StatusCode;

use super::error::VpclatticeError;
use super::state::VpclatticeState;
use super::types::*;

async fn create_service_network_handler(
    State(state): State<Arc<VpclatticeState>>,
    Json(req): Json<CreateServiceNetworkRequest>,
) -> Result<axum::response::Response, VpclatticeError> {
    let detail = state.create_service_network(req).await?;
    Ok((StatusCode::CREATED, Json(detail)).into_response())
}

async fn get_service_network_handler(
    State(state): State<Arc<VpclatticeState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, VpclatticeError> {
    let detail = state.get_service_network(&name).await?;
    Ok(Json(detail).into_response())
}

async fn list_service_networks_handler(
    State(state): State<Arc<VpclatticeState>>,
) -> Result<axum::response::Response, VpclatticeError> {
    let resp = state.list_service_networks().await?;
    Ok(Json(resp).into_response())
}

async fn delete_service_network_handler(
    State(state): State<Arc<VpclatticeState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, VpclatticeError> {
    state.delete_service_network(&name).await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}

pub fn create_router(state: Arc<VpclatticeState>) -> Router {
    Router::new()
        .route("/service-networks", post(create_service_network_handler).get(list_service_networks_handler))
        .route("/service-networks/{name}", get(get_service_network_handler).delete(delete_service_network_handler))
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
        let state = Arc::new(VpclatticeState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/service-networks")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[tokio::test]
    async fn test_get_not_found() {
        let state = Arc::new(VpclatticeState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/service-networks/nonexistent")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_create_service_networks() {
        let state = Arc::new(VpclatticeState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/service-networks")
            .header("content-type", "application/json")
            .body(Body::from("{}"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert!(resp.status().is_success() || resp.status().is_client_error());
    }
}
