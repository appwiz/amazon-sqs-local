use std::collections::HashMap;
use std::sync::Arc;

use axum::extract::State;
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use axum::http::StatusCode;
use axum::Router;

use super::error::ElasticbeanstalkError;
use super::state::ElasticbeanstalkState;

type Params = HashMap<String, String>;

fn parse_form(body: &str) -> Params {
    form_urlencoded::parse(body.as_bytes()).into_owned().collect()
}

fn require(params: &Params, key: &str) -> Result<String, ElasticbeanstalkError> {
    params.get(key).cloned().ok_or_else(|| ElasticbeanstalkError::ValidationException(format!("Missing required parameter: {}", key)))
}

fn xml_ok(action: &str, body: &str) -> Response {
    let xml = format!(
        r#"<{action}Response xmlns="https://elasticbeanstalk.amazonaws.com/doc/2012-10-01/">
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
    State(state): State<Arc<ElasticbeanstalkState>>,
    body: String,
) -> Result<Response, ElasticbeanstalkError> {
    let params = parse_form(&body);
    let action = require(&params, "Action")?;

    match action.as_str() {
        "CreateApplication" => {
            let name = require(&params, "ApplicationName")?;
            let info = state.create_application(name).await?;
            Ok(xml_ok("CreateApplication", &format!("    <ApplicationArn>{}</ApplicationArn>", info.application_arn)))
        }
        "DescribeApplications" => {
            let items = state.list_applications().await?;
            let mut xml = String::new();
            for item in &items {
                xml.push_str(&format!("    <member><ApplicationName>{}</ApplicationName><ApplicationArn>{}</ApplicationArn><Status>{}</Status></member>\n", xml_escape(&item.application_name), xml_escape(&item.application_arn), xml_escape(&item.status)));
            }
            Ok(xml_ok("DescribeApplications", &xml))
        }
        "DeleteApplication" => {
            let name = require(&params, "ApplicationName")?;
            state.delete_application(&name).await?;
            Ok(xml_ok("DeleteApplication", ""))
        }
        _ => Err(ElasticbeanstalkError::InvalidAction(format!("Unknown action: {}", action))),
    }
}

pub fn create_router(state: Arc<ElasticbeanstalkState>) -> Router {
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
        let state = Arc::new(ElasticbeanstalkState::new("123456789012".to_string(), "us-east-1".to_string()));
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
        let state = Arc::new(ElasticbeanstalkState::new("123456789012".to_string(), "us-east-1".to_string()));
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
    async fn test_createapplication_action() {
        let state = Arc::new(ElasticbeanstalkState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/")
            .header("content-type", "application/x-www-form-urlencoded")
            .body(Body::from("Action=CreateApplication&ApplicationName=test-resource"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
