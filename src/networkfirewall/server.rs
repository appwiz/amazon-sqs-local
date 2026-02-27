use std::sync::Arc;

use axum::body::Bytes;
use axum::extract::State;
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};

use super::error::NetworkfirewallError;
use super::state::NetworkfirewallState;
use super::types::*;

macro_rules! dispatch {
    ($state:expr, $body:expr, $req_type:ty, $method:ident) => {{
        let req: $req_type = serde_json::from_slice(&$body)
            .map_err(|e| NetworkfirewallError::ValidationException(e.to_string()))?;
        let resp = $state.$method(req).await?;
        Ok(Json(resp).into_response())
    }};
}

macro_rules! dispatch_empty {
    ($state:expr, $body:expr, $req_type:ty, $method:ident) => {{
        let req: $req_type = serde_json::from_slice(&$body)
            .map_err(|e| NetworkfirewallError::ValidationException(e.to_string()))?;
        $state.$method(req).await?;
        Ok(Json(serde_json::json!({})).into_response())
    }};
}

async fn handle_request(
    State(state): State<Arc<NetworkfirewallState>>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<axum::response::Response, NetworkfirewallError> {
    let target = headers
        .get("x-amz-target")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| NetworkfirewallError::InvalidAction("Missing X-Amz-Target header".into()))?;

    let action = target
        .strip_prefix("NetworkFirewall_20201112.")
        .ok_or_else(|| NetworkfirewallError::InvalidAction(format!("Invalid target: {target}")))?;

    match action {
        "CreateFirewall" => dispatch!(state, body, CreateFirewallRequest, create_firewall),
        "DescribeFirewall" => dispatch!(state, body, DescribeFirewallRequest, describe_firewall),
        "ListFirewalls" => dispatch!(state, body, ListFirewallsRequest, list_firewalls),
        "DeleteFirewall" => dispatch_empty!(state, body, DeleteFirewallRequest, delete_firewall),
        "CreateFirewallPolicy" => dispatch!(state, body, CreateFirewallPolicyRequest, create_firewall_policy),
        "DescribeFirewallPolicy" => dispatch!(state, body, DescribeFirewallPolicyRequest, describe_firewall_policy),
        "ListFirewallPolicys" => dispatch!(state, body, ListFirewallPolicysRequest, list_firewall_policys),
        "DeleteFirewallPolicy" => dispatch_empty!(state, body, DeleteFirewallPolicyRequest, delete_firewall_policy),
        _ => Err(NetworkfirewallError::InvalidAction(format!("Unknown action: {action}"))),
    }
}

pub fn create_router(state: Arc<NetworkfirewallState>) -> Router {
    Router::new()
        .route("/", post(handle_request))
        .with_state(state)
}


#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_missing_target_header() {
        let state = Arc::new(NetworkfirewallState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/")
            .header("content-type", "application/x-amz-json-1.1")
            .body(Body::from("{}"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_unknown_action() {
        let state = Arc::new(NetworkfirewallState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/")
            .header("content-type", "application/x-amz-json-1.1")
            .header("x-amz-target", "NetworkFirewall_20201112.FakeAction")
            .body(Body::from("{}"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }
    #[tokio::test]
    async fn test_describefirewall_requires_params() {
        let state = Arc::new(NetworkfirewallState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/")
            .header("content-type", "application/x-amz-json-1.1")
            .header("x-amz-target", "NetworkFirewall_20201112.DescribeFirewall")
            .body(Body::from("{}"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_ne!(resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_createfirewall_action() {
        let state = Arc::new(NetworkfirewallState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/")
            .header("content-type", "application/x-amz-json-1.1")
            .header("x-amz-target", "NetworkFirewall_20201112.CreateFirewall")
            .body(Body::from("{}"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert!(resp.status().is_success() || resp.status().is_client_error());
    }
    #[tokio::test]
    async fn test_createfirewallpolicy_action() {
        let state = Arc::new(NetworkfirewallState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/")
            .header("content-type", "application/x-amz-json-1.1")
            .header("x-amz-target", "NetworkFirewall_20201112.CreateFirewallPolicy")
            .body(Body::from("{}"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert!(resp.status().is_success() || resp.status().is_client_error());
    }
    #[tokio::test]
    async fn test_listfirewalls_action() {
        let state = Arc::new(NetworkfirewallState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/")
            .header("content-type", "application/x-amz-json-1.1")
            .header("x-amz-target", "NetworkFirewall_20201112.ListFirewalls")
            .body(Body::from("{}"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[tokio::test]
    async fn test_listfirewallpolicys_action() {
        let state = Arc::new(NetworkfirewallState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/")
            .header("content-type", "application/x-amz-json-1.1")
            .header("x-amz-target", "NetworkFirewall_20201112.ListFirewallPolicys")
            .body(Body::from("{}"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
