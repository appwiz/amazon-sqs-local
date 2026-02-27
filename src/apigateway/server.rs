use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post, put};
use axum::{Json, Router};
use serde::Deserialize;

use super::error::ApiGatewayError;
use super::state::ApiGatewayState;
use super::types::*;

fn json_response<T: serde::Serialize>(status: StatusCode, value: &T) -> Response {
    (status, Json(value)).into_response()
}

// --- REST API handlers ---

async fn create_rest_api_handler(
    State(state): State<Arc<ApiGatewayState>>,
    Json(req): Json<CreateRestApiRequest>,
) -> Result<Response, ApiGatewayError> {
    let resp = state.create_rest_api(req).await?;
    Ok(json_response(StatusCode::CREATED, &resp))
}

async fn get_rest_apis_handler(
    State(state): State<Arc<ApiGatewayState>>,
) -> Result<Response, ApiGatewayError> {
    let resp = state.get_rest_apis().await?;
    Ok(json_response(StatusCode::OK, &resp))
}

async fn get_rest_api_handler(
    State(state): State<Arc<ApiGatewayState>>,
    Path(rest_api_id): Path<String>,
) -> Result<Response, ApiGatewayError> {
    let resp = state.get_rest_api(&rest_api_id).await?;
    Ok(json_response(StatusCode::OK, &resp))
}

async fn delete_rest_api_handler(
    State(state): State<Arc<ApiGatewayState>>,
    Path(rest_api_id): Path<String>,
) -> Result<Response, ApiGatewayError> {
    state.delete_rest_api(&rest_api_id).await?;
    Ok(StatusCode::ACCEPTED.into_response())
}

async fn update_rest_api_handler(
    State(state): State<Arc<ApiGatewayState>>,
    Path(rest_api_id): Path<String>,
    Json(req): Json<UpdateRestApiRequest>,
) -> Result<Response, ApiGatewayError> {
    let resp = state.update_rest_api(&rest_api_id, req).await?;
    Ok(json_response(StatusCode::OK, &resp))
}

// --- Resource handlers ---

async fn get_resources_handler(
    State(state): State<Arc<ApiGatewayState>>,
    Path(rest_api_id): Path<String>,
) -> Result<Response, ApiGatewayError> {
    let resp = state.get_resources(&rest_api_id).await?;
    Ok(json_response(StatusCode::OK, &resp))
}

async fn create_resource_handler(
    State(state): State<Arc<ApiGatewayState>>,
    Path((rest_api_id, parent_id)): Path<(String, String)>,
    Json(req): Json<CreateResourceRequest>,
) -> Result<Response, ApiGatewayError> {
    let resp = state.create_resource(&rest_api_id, &parent_id, req).await?;
    Ok(json_response(StatusCode::CREATED, &resp))
}

async fn get_resource_handler(
    State(state): State<Arc<ApiGatewayState>>,
    Path((rest_api_id, resource_id)): Path<(String, String)>,
) -> Result<Response, ApiGatewayError> {
    let resp = state.get_resource(&rest_api_id, &resource_id).await?;
    Ok(json_response(StatusCode::OK, &resp))
}

async fn delete_resource_handler(
    State(state): State<Arc<ApiGatewayState>>,
    Path((rest_api_id, resource_id)): Path<(String, String)>,
) -> Result<Response, ApiGatewayError> {
    state.delete_resource(&rest_api_id, &resource_id).await?;
    Ok(StatusCode::ACCEPTED.into_response())
}

// --- Method handlers ---

async fn put_method_handler(
    State(state): State<Arc<ApiGatewayState>>,
    Path((rest_api_id, resource_id, http_method)): Path<(String, String, String)>,
    Json(req): Json<PutMethodRequest>,
) -> Result<Response, ApiGatewayError> {
    let resp = state.put_method(&rest_api_id, &resource_id, &http_method, req).await?;
    Ok(json_response(StatusCode::CREATED, &resp))
}

async fn get_method_handler(
    State(state): State<Arc<ApiGatewayState>>,
    Path((rest_api_id, resource_id, http_method)): Path<(String, String, String)>,
) -> Result<Response, ApiGatewayError> {
    let resp = state.get_method(&rest_api_id, &resource_id, &http_method).await?;
    Ok(json_response(StatusCode::OK, &resp))
}

async fn delete_method_handler(
    State(state): State<Arc<ApiGatewayState>>,
    Path((rest_api_id, resource_id, http_method)): Path<(String, String, String)>,
) -> Result<Response, ApiGatewayError> {
    state.delete_method(&rest_api_id, &resource_id, &http_method).await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}

// --- Integration handlers ---

async fn put_integration_handler(
    State(state): State<Arc<ApiGatewayState>>,
    Path((rest_api_id, resource_id, http_method)): Path<(String, String, String)>,
    Json(req): Json<PutIntegrationRequest>,
) -> Result<Response, ApiGatewayError> {
    let resp = state.put_integration(&rest_api_id, &resource_id, &http_method, req).await?;
    Ok(json_response(StatusCode::CREATED, &resp))
}

async fn get_integration_handler(
    State(state): State<Arc<ApiGatewayState>>,
    Path((rest_api_id, resource_id, http_method)): Path<(String, String, String)>,
) -> Result<Response, ApiGatewayError> {
    let resp = state.get_integration(&rest_api_id, &resource_id, &http_method).await?;
    Ok(json_response(StatusCode::OK, &resp))
}

async fn delete_integration_handler(
    State(state): State<Arc<ApiGatewayState>>,
    Path((rest_api_id, resource_id, http_method)): Path<(String, String, String)>,
) -> Result<Response, ApiGatewayError> {
    state.delete_integration(&rest_api_id, &resource_id, &http_method).await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}

// --- MethodResponse handlers ---

async fn put_method_response_handler(
    State(state): State<Arc<ApiGatewayState>>,
    Path((rest_api_id, resource_id, http_method, status_code)): Path<(String, String, String, String)>,
    Json(req): Json<PutMethodResponseRequest>,
) -> Result<Response, ApiGatewayError> {
    let resp = state.put_method_response(&rest_api_id, &resource_id, &http_method, &status_code, req).await?;
    Ok(json_response(StatusCode::CREATED, &resp))
}

async fn get_method_response_handler(
    State(state): State<Arc<ApiGatewayState>>,
    Path((rest_api_id, resource_id, http_method, status_code)): Path<(String, String, String, String)>,
) -> Result<Response, ApiGatewayError> {
    let resp = state.get_method_response(&rest_api_id, &resource_id, &http_method, &status_code).await?;
    Ok(json_response(StatusCode::OK, &resp))
}

async fn delete_method_response_handler(
    State(state): State<Arc<ApiGatewayState>>,
    Path((rest_api_id, resource_id, http_method, status_code)): Path<(String, String, String, String)>,
) -> Result<Response, ApiGatewayError> {
    state.delete_method_response(&rest_api_id, &resource_id, &http_method, &status_code).await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}

// --- IntegrationResponse handlers ---

async fn put_integration_response_handler(
    State(state): State<Arc<ApiGatewayState>>,
    Path((rest_api_id, resource_id, http_method, status_code)): Path<(String, String, String, String)>,
    Json(req): Json<PutIntegrationResponseRequest>,
) -> Result<Response, ApiGatewayError> {
    let resp = state.put_integration_response(&rest_api_id, &resource_id, &http_method, &status_code, req).await?;
    Ok(json_response(StatusCode::CREATED, &resp))
}

// --- Deployment handlers ---

async fn create_deployment_handler(
    State(state): State<Arc<ApiGatewayState>>,
    Path(rest_api_id): Path<String>,
    body: axum::body::Bytes,
) -> Result<Response, ApiGatewayError> {
    let req: CreateDeploymentRequest = if body.is_empty() {
        CreateDeploymentRequest::default()
    } else {
        serde_json::from_slice(&body)
            .map_err(|e| ApiGatewayError::BadRequestException(e.to_string()))?
    };
    let resp = state.create_deployment(&rest_api_id, req).await?;
    Ok(json_response(StatusCode::CREATED, &resp))
}

async fn get_deployments_handler(
    State(state): State<Arc<ApiGatewayState>>,
    Path(rest_api_id): Path<String>,
) -> Result<Response, ApiGatewayError> {
    let resp = state.get_deployments(&rest_api_id).await?;
    Ok(json_response(StatusCode::OK, &resp))
}

async fn get_deployment_handler(
    State(state): State<Arc<ApiGatewayState>>,
    Path((rest_api_id, deployment_id)): Path<(String, String)>,
) -> Result<Response, ApiGatewayError> {
    let resp = state.get_deployment(&rest_api_id, &deployment_id).await?;
    Ok(json_response(StatusCode::OK, &resp))
}

// --- Stage handlers ---

async fn create_stage_handler(
    State(state): State<Arc<ApiGatewayState>>,
    Path(rest_api_id): Path<String>,
    Json(req): Json<CreateStageRequest>,
) -> Result<Response, ApiGatewayError> {
    let resp = state.create_stage(&rest_api_id, req).await?;
    Ok(json_response(StatusCode::CREATED, &resp))
}

async fn get_stages_handler(
    State(state): State<Arc<ApiGatewayState>>,
    Path(rest_api_id): Path<String>,
) -> Result<Response, ApiGatewayError> {
    let resp = state.get_stages(&rest_api_id).await?;
    Ok(json_response(StatusCode::OK, &resp))
}

async fn get_stage_handler(
    State(state): State<Arc<ApiGatewayState>>,
    Path((rest_api_id, stage_name)): Path<(String, String)>,
) -> Result<Response, ApiGatewayError> {
    let resp = state.get_stage(&rest_api_id, &stage_name).await?;
    Ok(json_response(StatusCode::OK, &resp))
}

async fn update_stage_handler(
    State(state): State<Arc<ApiGatewayState>>,
    Path((rest_api_id, stage_name)): Path<(String, String)>,
    Json(req): Json<UpdateStageRequest>,
) -> Result<Response, ApiGatewayError> {
    let resp = state.update_stage(&rest_api_id, &stage_name, req).await?;
    Ok(json_response(StatusCode::OK, &resp))
}

async fn delete_stage_handler(
    State(state): State<Arc<ApiGatewayState>>,
    Path((rest_api_id, stage_name)): Path<(String, String)>,
) -> Result<Response, ApiGatewayError> {
    state.delete_stage(&rest_api_id, &stage_name).await?;
    Ok(StatusCode::ACCEPTED.into_response())
}

// --- Tag handlers ---

async fn tag_resource_handler(
    State(state): State<Arc<ApiGatewayState>>,
    Path(rest_api_id): Path<String>,
    Json(req): Json<TagResourceRequest>,
) -> Result<Response, ApiGatewayError> {
    state.tag_resource(&rest_api_id, req).await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}

#[derive(Deserialize)]
struct UntagQuery {
    #[serde(rename = "tagKeys")]
    tag_keys: Option<String>,
}

async fn untag_resource_handler(
    State(state): State<Arc<ApiGatewayState>>,
    Path(rest_api_id): Path<String>,
    Query(q): Query<UntagQuery>,
) -> Result<Response, ApiGatewayError> {
    let keys: Vec<String> = q
        .tag_keys
        .map(|s| s.split(',').map(|k| k.to_string()).collect())
        .unwrap_or_default();
    state.untag_resource(&rest_api_id, keys).await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}

async fn get_tags_handler(
    State(state): State<Arc<ApiGatewayState>>,
    Path(rest_api_id): Path<String>,
) -> Result<Response, ApiGatewayError> {
    let tags = state.get_tags(&rest_api_id).await?;
    Ok(json_response(StatusCode::OK, &serde_json::json!({ "tags": tags })))
}

pub fn create_router(state: Arc<ApiGatewayState>) -> Router {
    Router::new()
        // REST APIs
        .route(
            "/restapis",
            post(create_rest_api_handler).get(get_rest_apis_handler),
        )
        .route(
            "/restapis/{rest_api_id}",
            get(get_rest_api_handler)
                .delete(delete_rest_api_handler)
                .patch(update_rest_api_handler),
        )
        // Resources
        .route(
            "/restapis/{rest_api_id}/resources",
            get(get_resources_handler),
        )
        .route(
            "/restapis/{rest_api_id}/resources/{parent_id}",
            get(get_resource_handler)
                .post(create_resource_handler)
                .delete(delete_resource_handler),
        )
        // Methods
        .route(
            "/restapis/{rest_api_id}/resources/{resource_id}/methods/{http_method}",
            put(put_method_handler)
                .get(get_method_handler)
                .delete(delete_method_handler),
        )
        // Integrations
        .route(
            "/restapis/{rest_api_id}/resources/{resource_id}/methods/{http_method}/integration",
            put(put_integration_handler)
                .get(get_integration_handler)
                .delete(delete_integration_handler),
        )
        // MethodResponse
        .route(
            "/restapis/{rest_api_id}/resources/{resource_id}/methods/{http_method}/responses/{status_code}",
            put(put_method_response_handler)
                .get(get_method_response_handler)
                .delete(delete_method_response_handler),
        )
        // IntegrationResponse
        .route(
            "/restapis/{rest_api_id}/resources/{resource_id}/methods/{http_method}/integration/responses/{status_code}",
            put(put_integration_response_handler),
        )
        // Deployments
        .route(
            "/restapis/{rest_api_id}/deployments",
            post(create_deployment_handler).get(get_deployments_handler),
        )
        .route(
            "/restapis/{rest_api_id}/deployments/{deployment_id}",
            get(get_deployment_handler),
        )
        // Stages
        .route(
            "/restapis/{rest_api_id}/stages",
            post(create_stage_handler).get(get_stages_handler),
        )
        .route(
            "/restapis/{rest_api_id}/stages/{stage_name}",
            get(get_stage_handler)
                .patch(update_stage_handler)
                .delete(delete_stage_handler),
        )
        // Tags
        .route(
            "/tags/{rest_api_id}",
            post(tag_resource_handler)
                .delete(untag_resource_handler)
                .get(get_tags_handler),
        )
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;

    fn new_state() -> Arc<ApiGatewayState> {
        Arc::new(ApiGatewayState::new("123456789012".to_string(), "us-east-1".to_string()))
    }

    async fn extract_body(resp: axum::response::Response) -> serde_json::Value {
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX).await.unwrap();
        serde_json::from_slice(&body).unwrap()
    }

    #[tokio::test]
    async fn test_list_rest_apis_empty() {
        let app = create_router(new_state());
        let req = Request::builder()
            .method("GET")
            .uri("/restapis")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_create_rest_api() {
        let app = create_router(new_state());
        let req = Request::builder()
            .method("POST")
            .uri("/restapis")
            .header("content-type", "application/json")
            .body(Body::from(r#"{"name": "test-api"}"#))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::CREATED);
        let json = extract_body(resp).await;
        assert_eq!(json["name"], "test-api");
        assert!(json["id"].as_str().is_some());
    }

    #[tokio::test]
    async fn test_get_rest_api() {
        let state = new_state();
        let app = create_router(state.clone());
        let req = Request::builder()
            .method("POST")
            .uri("/restapis")
            .header("content-type", "application/json")
            .body(Body::from(r#"{"name": "my-api"}"#))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        let json = extract_body(resp).await;
        let api_id = json["id"].as_str().unwrap().to_string();

        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri(format!("/restapis/{}", api_id))
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let json = extract_body(resp).await;
        assert_eq!(json["name"], "my-api");
    }

    #[tokio::test]
    async fn test_get_rest_api_not_found() {
        let app = create_router(new_state());
        let req = Request::builder()
            .method("GET")
            .uri("/restapis/nonexistent")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_delete_rest_api() {
        let state = new_state();
        let app = create_router(state.clone());
        let req = Request::builder()
            .method("POST")
            .uri("/restapis")
            .header("content-type", "application/json")
            .body(Body::from(r#"{"name": "del-api"}"#))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        let json = extract_body(resp).await;
        let api_id = json["id"].as_str().unwrap().to_string();

        let app = create_router(state.clone());
        let req = Request::builder()
            .method("DELETE")
            .uri(format!("/restapis/{}", api_id))
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::ACCEPTED);

        // Verify deleted
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri(format!("/restapis/{}", api_id))
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_update_rest_api() {
        let state = new_state();
        let app = create_router(state.clone());
        let req = Request::builder()
            .method("POST")
            .uri("/restapis")
            .header("content-type", "application/json")
            .body(Body::from(r#"{"name": "orig"}"#))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        let json = extract_body(resp).await;
        let api_id = json["id"].as_str().unwrap().to_string();

        let app = create_router(state);
        let req = Request::builder()
            .method("PATCH")
            .uri(format!("/restapis/{}", api_id))
            .header("content-type", "application/json")
            .body(Body::from(r#"{"patchOperations": [{"op": "replace", "path": "/description", "value": "updated"}]}"#))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_create_and_get_resource() {
        let state = new_state();
        let app = create_router(state.clone());
        let req = Request::builder()
            .method("POST")
            .uri("/restapis")
            .header("content-type", "application/json")
            .body(Body::from(r#"{"name": "res-api"}"#))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        let json = extract_body(resp).await;
        let api_id = json["id"].as_str().unwrap().to_string();

        // Get resources to find root resource
        let app = create_router(state.clone());
        let req = Request::builder()
            .method("GET")
            .uri(format!("/restapis/{}/resources", api_id))
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let json = extract_body(resp).await;
        let root_id = json["item"][0]["id"].as_str().unwrap().to_string();

        // Create resource
        let app = create_router(state.clone());
        let req = Request::builder()
            .method("POST")
            .uri(format!("/restapis/{}/resources/{}", api_id, root_id))
            .header("content-type", "application/json")
            .body(Body::from(r#"{"pathPart": "items"}"#))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::CREATED);
        let json = extract_body(resp).await;
        let resource_id = json["id"].as_str().unwrap().to_string();

        // Get resource
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri(format!("/restapis/{}/resources/{}", api_id, resource_id))
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_create_deployment_and_stage() {
        let state = new_state();
        let app = create_router(state.clone());
        let req = Request::builder()
            .method("POST")
            .uri("/restapis")
            .header("content-type", "application/json")
            .body(Body::from(r#"{"name": "deploy-api"}"#))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        let json = extract_body(resp).await;
        let api_id = json["id"].as_str().unwrap().to_string();

        // Create deployment
        let app = create_router(state.clone());
        let req = Request::builder()
            .method("POST")
            .uri(format!("/restapis/{}/deployments", api_id))
            .header("content-type", "application/json")
            .body(Body::from(r#"{"description": "initial"}"#))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::CREATED);
        let json = extract_body(resp).await;
        let deploy_id = json["id"].as_str().unwrap().to_string();

        // List deployments
        let app = create_router(state.clone());
        let req = Request::builder()
            .method("GET")
            .uri(format!("/restapis/{}/deployments", api_id))
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        // Create stage
        let app = create_router(state.clone());
        let req = Request::builder()
            .method("POST")
            .uri(format!("/restapis/{}/stages", api_id))
            .header("content-type", "application/json")
            .body(Body::from(format!(r#"{{"stageName": "prod", "deploymentId": "{}"}}"#, deploy_id)))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::CREATED);

        // Get stage
        let app = create_router(state.clone());
        let req = Request::builder()
            .method("GET")
            .uri(format!("/restapis/{}/stages/prod", api_id))
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        // List stages
        let app = create_router(state.clone());
        let req = Request::builder()
            .method("GET")
            .uri(format!("/restapis/{}/stages", api_id))
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        // Delete stage
        let app = create_router(state);
        let req = Request::builder()
            .method("DELETE")
            .uri(format!("/restapis/{}/stages/prod", api_id))
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::ACCEPTED);
    }

    #[tokio::test]
    async fn test_tag_resource() {
        let state = new_state();
        let app = create_router(state.clone());
        let req = Request::builder()
            .method("POST")
            .uri("/restapis")
            .header("content-type", "application/json")
            .body(Body::from(r#"{"name": "tag-api"}"#))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        let json = extract_body(resp).await;
        let api_id = json["id"].as_str().unwrap().to_string();

        // Tag
        let app = create_router(state.clone());
        let req = Request::builder()
            .method("POST")
            .uri(format!("/tags/{}", api_id))
            .header("content-type", "application/json")
            .body(Body::from(r#"{"tags": {"env": "test"}}"#))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::NO_CONTENT);

        // Get tags
        let app = create_router(state.clone());
        let req = Request::builder()
            .method("GET")
            .uri(format!("/tags/{}", api_id))
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        // Untag
        let app = create_router(state);
        let req = Request::builder()
            .method("DELETE")
            .uri(format!("/tags/{}?tagKeys=env", api_id))
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::NO_CONTENT);
    }

    #[tokio::test]
    async fn test_put_method_and_integration() {
        let state = new_state();
        // Create API
        let app = create_router(state.clone());
        let req = Request::builder()
            .method("POST")
            .uri("/restapis")
            .header("content-type", "application/json")
            .body(Body::from(r#"{"name": "method-api"}"#))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        let json = extract_body(resp).await;
        let api_id = json["id"].as_str().unwrap().to_string();

        // Get root resource
        let app = create_router(state.clone());
        let req = Request::builder()
            .method("GET")
            .uri(format!("/restapis/{}/resources", api_id))
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        let json = extract_body(resp).await;
        let root_id = json["item"][0]["id"].as_str().unwrap().to_string();

        // Put method
        let app = create_router(state.clone());
        let req = Request::builder()
            .method("PUT")
            .uri(format!("/restapis/{}/resources/{}/methods/GET", api_id, root_id))
            .header("content-type", "application/json")
            .body(Body::from(r#"{"authorizationType": "NONE"}"#))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::CREATED);

        // Get method
        let app = create_router(state.clone());
        let req = Request::builder()
            .method("GET")
            .uri(format!("/restapis/{}/resources/{}/methods/GET", api_id, root_id))
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        // Put integration
        let app = create_router(state.clone());
        let req = Request::builder()
            .method("PUT")
            .uri(format!("/restapis/{}/resources/{}/methods/GET/integration", api_id, root_id))
            .header("content-type", "application/json")
            .body(Body::from(r#"{"type": "MOCK"}"#))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::CREATED);

        // Get integration
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri(format!("/restapis/{}/resources/{}/methods/GET/integration", api_id, root_id))
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
