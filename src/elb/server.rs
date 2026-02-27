use std::collections::HashMap;
use std::sync::Arc;

use axum::extract::State;
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use axum::http::StatusCode;
use axum::Router;

use super::error::ELBError;
use super::state::ELBState;

type Params = HashMap<String, String>;

fn parse_form(body: &str) -> Params {
    form_urlencoded::parse(body.as_bytes()).into_owned().collect()
}

fn require(params: &Params, key: &str) -> Result<String, ELBError> {
    params.get(key).cloned().ok_or_else(|| ELBError::ValidationException(format!("Missing required parameter: {}", key)))
}

fn xml_ok(action: &str, body: &str) -> Response {
    let xml = format!(
        r#"<{action}Response xmlns="https://elb.amazonaws.com/doc/2012-10-01/">
  <{action}Result>
{body}
  </{action}Result>
  <ResponseMetadata>
    <RequestId>{rid}</RequestId>
  </ResponseMetadata>
</{action}Response>"#,
        action = action,
        body = body, rid = uuid::Uuid::new_v4(),
    );
    (StatusCode::OK, [("content-type", "text/xml")], xml).into_response()
}


fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

async fn handle_request(
    State(state): State<Arc<ELBState>>,
    body: String,
) -> Result<Response, ELBError> {
    let params = parse_form(&body);
    let action = require(&params, "Action")?;

    match action.as_str() {
        "CreateLoadBalancer" => {
            let name = require(&params, "LoadBalancerName")?;
            let info = state.create_load_balancer(name).await?;
            Ok(xml_ok("CreateLoadBalancer", &format!("    <LoadBalancerArn>{}</LoadBalancerArn>", info.load_balancer_arn)))
        }
        "DescribeLoadBalancers" => {
            let items = state.list_load_balancers().await?;
            let mut xml = String::new();
            for item in &items {
                xml.push_str(&format!("    <member><LoadBalancerName>{}</LoadBalancerName><LoadBalancerArn>{}</LoadBalancerArn><Status>{}</Status></member>\n", xml_escape(&item.load_balancer_name), xml_escape(&item.load_balancer_arn), xml_escape(&item.status)));
            }
            Ok(xml_ok("DescribeLoadBalancers", &xml))
        }
        "DeleteLoadBalancer" => {
            let name = require(&params, "LoadBalancerName")?;
            state.delete_load_balancer(&name).await?;
            Ok(xml_ok("DeleteLoadBalancer", ""))
        }
        "CreateTargetGroup" => {
            let name = require(&params, "TargetGroupName")?;
            let info = state.create_target_group(name).await?;
            Ok(xml_ok("CreateTargetGroup", &format!("    <TargetGroupArn>{}</TargetGroupArn>", info.target_group_arn)))
        }
        "DescribeTargetGroups" => {
            let items = state.list_target_groups().await?;
            let mut xml = String::new();
            for item in &items {
                xml.push_str(&format!("    <member><TargetGroupName>{}</TargetGroupName><TargetGroupArn>{}</TargetGroupArn><Status>{}</Status></member>\n", xml_escape(&item.target_group_name), xml_escape(&item.target_group_arn), xml_escape(&item.status)));
            }
            Ok(xml_ok("DescribeTargetGroups", &xml))
        }
        "DeleteTargetGroup" => {
            let name = require(&params, "TargetGroupName")?;
            state.delete_target_group(&name).await?;
            Ok(xml_ok("DeleteTargetGroup", ""))
        }
        _ => Err(ELBError::InvalidAction(format!("Unknown action: {}", action))),
    }
}

pub fn create_router(state: Arc<ELBState>) -> Router {
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
    async fn test_missing_action() {
        let state = Arc::new(ELBState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/")
            .header("content-type", "application/x-www-form-urlencoded")
            .body(Body::from(""))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_ne!(resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_unknown_action() {
        let state = Arc::new(ELBState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/")
            .header("content-type", "application/x-www-form-urlencoded")
            .body(Body::from("Action=FakeAction"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_ne!(resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_createloadbalancer_action() {
        let state = Arc::new(ELBState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/")
            .header("content-type", "application/x-www-form-urlencoded")
            .body(Body::from("Action=CreateLoadBalancer&LoadBalancerName=test-resource"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[tokio::test]
    async fn test_createtargetgroup_action() {
        let state = Arc::new(ELBState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/")
            .header("content-type", "application/x-www-form-urlencoded")
            .body(Body::from("Action=CreateTargetGroup&TargetGroupName=test-resource"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
