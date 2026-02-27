use std::collections::HashMap;
use std::sync::Arc;

use axum::extract::State;
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use axum::http::StatusCode;
use axum::Router;

use super::error::IAMError;
use super::state::IAMState;

type Params = HashMap<String, String>;

fn parse_form(body: &str) -> Params {
    form_urlencoded::parse(body.as_bytes()).into_owned().collect()
}

fn require(params: &Params, key: &str) -> Result<String, IAMError> {
    params.get(key).cloned().ok_or_else(|| IAMError::ValidationException(format!("Missing required parameter: {}", key)))
}

fn xml_ok(action: &str, body: &str) -> Response {
    let xml = format!(
        r#"<{action}Response xmlns="https://iam.amazonaws.com/doc/2012-10-01/">
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
    State(state): State<Arc<IAMState>>,
    body: String,
) -> Result<Response, IAMError> {
    let params = parse_form(&body);
    let action = require(&params, "Action")?;

    match action.as_str() {
        "CreateUser" => {
            let name = require(&params, "UserName")?;
            let info = state.create_user(name).await?;
            Ok(xml_ok("CreateUser", &format!("    <UserArn>{}</UserArn>", info.user_arn)))
        }
        "DescribeUsers" => {
            let items = state.list_users().await?;
            let mut xml = String::new();
            for item in &items {
                xml.push_str(&format!("    <member><UserName>{}</UserName><UserArn>{}</UserArn><Status>{}</Status></member>\n", xml_escape(&item.user_name), xml_escape(&item.user_arn), xml_escape(&item.status)));
            }
            Ok(xml_ok("DescribeUsers", &xml))
        }
        "DeleteUser" => {
            let name = require(&params, "UserName")?;
            state.delete_user(&name).await?;
            Ok(xml_ok("DeleteUser", ""))
        }
        "CreateRole" => {
            let name = require(&params, "RoleName")?;
            let info = state.create_role(name).await?;
            Ok(xml_ok("CreateRole", &format!("    <RoleArn>{}</RoleArn>", info.role_arn)))
        }
        "DescribeRoles" => {
            let items = state.list_roles().await?;
            let mut xml = String::new();
            for item in &items {
                xml.push_str(&format!("    <member><RoleName>{}</RoleName><RoleArn>{}</RoleArn><Status>{}</Status></member>\n", xml_escape(&item.role_name), xml_escape(&item.role_arn), xml_escape(&item.status)));
            }
            Ok(xml_ok("DescribeRoles", &xml))
        }
        "DeleteRole" => {
            let name = require(&params, "RoleName")?;
            state.delete_role(&name).await?;
            Ok(xml_ok("DeleteRole", ""))
        }
        "CreatePolicy" => {
            let name = require(&params, "PolicyName")?;
            let info = state.create_policy(name).await?;
            Ok(xml_ok("CreatePolicy", &format!("    <PolicyArn>{}</PolicyArn>", info.policy_arn)))
        }
        "DescribePolicys" => {
            let items = state.list_policys().await?;
            let mut xml = String::new();
            for item in &items {
                xml.push_str(&format!("    <member><PolicyName>{}</PolicyName><PolicyArn>{}</PolicyArn><Status>{}</Status></member>\n", xml_escape(&item.policy_name), xml_escape(&item.policy_arn), xml_escape(&item.status)));
            }
            Ok(xml_ok("DescribePolicys", &xml))
        }
        "DeletePolicy" => {
            let name = require(&params, "PolicyName")?;
            state.delete_policy(&name).await?;
            Ok(xml_ok("DeletePolicy", ""))
        }
        _ => Err(IAMError::InvalidAction(format!("Unknown action: {}", action))),
    }
}

pub fn create_router(state: Arc<IAMState>) -> Router {
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
        let state = Arc::new(IAMState::new("123456789012".to_string(), "us-east-1".to_string()));
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
        let state = Arc::new(IAMState::new("123456789012".to_string(), "us-east-1".to_string()));
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
    async fn test_createuser_action() {
        let state = Arc::new(IAMState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/")
            .header("content-type", "application/x-www-form-urlencoded")
            .body(Body::from("Action=CreateUser&UserName=test-resource"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[tokio::test]
    async fn test_createrole_action() {
        let state = Arc::new(IAMState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/")
            .header("content-type", "application/x-www-form-urlencoded")
            .body(Body::from("Action=CreateRole&RoleName=test-resource"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[tokio::test]
    async fn test_createpolicy_action() {
        let state = Arc::new(IAMState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/")
            .header("content-type", "application/x-www-form-urlencoded")
            .body(Body::from("Action=CreatePolicy&PolicyName=test-resource"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
