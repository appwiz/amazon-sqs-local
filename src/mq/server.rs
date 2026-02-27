use std::sync::Arc;

use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use axum::http::StatusCode;

use super::error::MQError;
use super::state::MQState;
use super::types::*;

async fn create_broker_handler(
    State(state): State<Arc<MQState>>,
    Json(req): Json<CreateBrokerRequest>,
) -> Result<axum::response::Response, MQError> {
    let detail = state.create_broker(req).await?;
    Ok((StatusCode::CREATED, Json(detail)).into_response())
}

async fn get_broker_handler(
    State(state): State<Arc<MQState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, MQError> {
    let detail = state.get_broker(&name).await?;
    Ok(Json(detail).into_response())
}

async fn list_brokers_handler(
    State(state): State<Arc<MQState>>,
) -> Result<axum::response::Response, MQError> {
    let resp = state.list_brokers().await?;
    Ok(Json(resp).into_response())
}

async fn delete_broker_handler(
    State(state): State<Arc<MQState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, MQError> {
    state.delete_broker(&name).await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}

pub fn create_router(state: Arc<MQState>) -> Router {
    Router::new()
        .route("/brokers", post(create_broker_handler).get(list_brokers_handler))
        .route("/brokers/{name}", get(get_broker_handler).delete(delete_broker_handler))
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
        let state = Arc::new(MQState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/brokers")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[tokio::test]
    async fn test_get_not_found() {
        let state = Arc::new(MQState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/brokers/nonexistent")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_create_brokers() {
        let state = Arc::new(MQState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/brokers")
            .header("content-type", "application/json")
            .body(Body::from("{}"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert!(resp.status().is_success() || resp.status().is_client_error());
    }
}
