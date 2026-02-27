use std::collections::HashMap;
use std::sync::Arc;

use axum::extract::State;
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use axum::http::StatusCode;
use axum::Router;

use super::error::EC2Error;
use super::state::EC2State;

type Params = HashMap<String, String>;

fn parse_form(body: &str) -> Params {
    form_urlencoded::parse(body.as_bytes()).into_owned().collect()
}

fn require(params: &Params, key: &str) -> Result<String, EC2Error> {
    params.get(key).cloned().ok_or_else(|| EC2Error::ValidationException(format!("Missing required parameter: {}", key)))
}

fn xml_ok(action: &str, body: &str) -> Response {
    let xml = format!(
        r#"<{action}Response xmlns="https://ec2.amazonaws.com/doc/2012-10-01/">
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
    State(state): State<Arc<EC2State>>,
    body: String,
) -> Result<Response, EC2Error> {
    let params = parse_form(&body);
    let action = require(&params, "Action")?;

    match action.as_str() {
        "CreateInstance" => {
            let name = require(&params, "InstanceName")?;
            let info = state.create_instance(name).await?;
            Ok(xml_ok("CreateInstance", &format!("    <InstanceArn>{}</InstanceArn>", info.instance_arn)))
        }
        "DescribeInstances" => {
            let items = state.list_instances().await?;
            let mut xml = String::new();
            for item in &items {
                xml.push_str(&format!("    <member><InstanceName>{}</InstanceName><InstanceArn>{}</InstanceArn><Status>{}</Status></member>\n", xml_escape(&item.instance_name), xml_escape(&item.instance_arn), xml_escape(&item.status)));
            }
            Ok(xml_ok("DescribeInstances", &xml))
        }
        "DeleteInstance" => {
            let name = require(&params, "InstanceName")?;
            state.delete_instance(&name).await?;
            Ok(xml_ok("DeleteInstance", ""))
        }
        "CreateVpc" => {
            let name = require(&params, "VpcName")?;
            let info = state.create_vpc(name).await?;
            Ok(xml_ok("CreateVpc", &format!("    <VpcArn>{}</VpcArn>", info.vpc_arn)))
        }
        "DescribeVpcs" => {
            let items = state.list_vpcs().await?;
            let mut xml = String::new();
            for item in &items {
                xml.push_str(&format!("    <member><VpcName>{}</VpcName><VpcArn>{}</VpcArn><Status>{}</Status></member>\n", xml_escape(&item.vpc_name), xml_escape(&item.vpc_arn), xml_escape(&item.status)));
            }
            Ok(xml_ok("DescribeVpcs", &xml))
        }
        "DeleteVpc" => {
            let name = require(&params, "VpcName")?;
            state.delete_vpc(&name).await?;
            Ok(xml_ok("DeleteVpc", ""))
        }
        "CreateSecurityGroup" => {
            let name = require(&params, "SecurityGroupName")?;
            let info = state.create_security_group(name).await?;
            Ok(xml_ok("CreateSecurityGroup", &format!("    <SecurityGroupArn>{}</SecurityGroupArn>", info.security_group_arn)))
        }
        "DescribeSecurityGroups" => {
            let items = state.list_security_groups().await?;
            let mut xml = String::new();
            for item in &items {
                xml.push_str(&format!("    <member><SecurityGroupName>{}</SecurityGroupName><SecurityGroupArn>{}</SecurityGroupArn><Status>{}</Status></member>\n", xml_escape(&item.security_group_name), xml_escape(&item.security_group_arn), xml_escape(&item.status)));
            }
            Ok(xml_ok("DescribeSecurityGroups", &xml))
        }
        "DeleteSecurityGroup" => {
            let name = require(&params, "SecurityGroupName")?;
            state.delete_security_group(&name).await?;
            Ok(xml_ok("DeleteSecurityGroup", ""))
        }
        _ => Err(EC2Error::InvalidAction(format!("Unknown action: {}", action))),
    }
}

pub fn create_router(state: Arc<EC2State>) -> Router {
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
        let state = Arc::new(EC2State::new("123456789012".to_string(), "us-east-1".to_string()));
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
        let state = Arc::new(EC2State::new("123456789012".to_string(), "us-east-1".to_string()));
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
    async fn test_createinstance_action() {
        let state = Arc::new(EC2State::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/")
            .header("content-type", "application/x-www-form-urlencoded")
            .body(Body::from("Action=CreateInstance&InstanceName=test-resource"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[tokio::test]
    async fn test_createvpc_action() {
        let state = Arc::new(EC2State::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/")
            .header("content-type", "application/x-www-form-urlencoded")
            .body(Body::from("Action=CreateVpc&VpcName=test-resource"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[tokio::test]
    async fn test_createsecuritygroup_action() {
        let state = Arc::new(EC2State::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/")
            .header("content-type", "application/x-www-form-urlencoded")
            .body(Body::from("Action=CreateSecurityGroup&SecurityGroupName=test-resource"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
