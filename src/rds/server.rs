use std::collections::HashMap;
use std::sync::Arc;

use axum::extract::State;
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use axum::http::StatusCode;
use axum::Router;

use super::error::RDSError;
use super::state::RDSState;

type Params = HashMap<String, String>;

fn parse_form(body: &str) -> Params {
    form_urlencoded::parse(body.as_bytes()).into_owned().collect()
}

fn require(params: &Params, key: &str) -> Result<String, RDSError> {
    params.get(key).cloned().ok_or_else(|| RDSError::ValidationException(format!("Missing required parameter: {}", key)))
}

fn xml_ok(action: &str, body: &str) -> Response {
    let xml = format!(
        r#"<{action}Response xmlns="https://rds.amazonaws.com/doc/2012-10-01/">
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
    State(state): State<Arc<RDSState>>,
    body: String,
) -> Result<Response, RDSError> {
    let params = parse_form(&body);
    let action = require(&params, "Action")?;

    match action.as_str() {
        "CreateDBInstance" => {
            let name = require(&params, "DBInstanceName")?;
            let info = state.create_d_b_instance(name).await?;
            Ok(xml_ok("CreateDBInstance", &format!("    <DBInstanceArn>{}</DBInstanceArn>", info.d_b_instance_arn)))
        }
        "DescribeDBInstances" => {
            let items = state.list_d_b_instances().await?;
            let mut xml = String::new();
            for item in &items {
                xml.push_str(&format!("    <member><DBInstanceName>{}</DBInstanceName><DBInstanceArn>{}</DBInstanceArn><Status>{}</Status></member>\n", xml_escape(&item.d_b_instance_name), xml_escape(&item.d_b_instance_arn), xml_escape(&item.status)));
            }
            Ok(xml_ok("DescribeDBInstances", &xml))
        }
        "DeleteDBInstance" => {
            let name = require(&params, "DBInstanceName")?;
            state.delete_d_b_instance(&name).await?;
            Ok(xml_ok("DeleteDBInstance", ""))
        }
        "CreateDBCluster" => {
            let name = require(&params, "DBClusterName")?;
            let info = state.create_d_b_cluster(name).await?;
            Ok(xml_ok("CreateDBCluster", &format!("    <DBClusterArn>{}</DBClusterArn>", info.d_b_cluster_arn)))
        }
        "DescribeDBClusters" => {
            let items = state.list_d_b_clusters().await?;
            let mut xml = String::new();
            for item in &items {
                xml.push_str(&format!("    <member><DBClusterName>{}</DBClusterName><DBClusterArn>{}</DBClusterArn><Status>{}</Status></member>\n", xml_escape(&item.d_b_cluster_name), xml_escape(&item.d_b_cluster_arn), xml_escape(&item.status)));
            }
            Ok(xml_ok("DescribeDBClusters", &xml))
        }
        "DeleteDBCluster" => {
            let name = require(&params, "DBClusterName")?;
            state.delete_d_b_cluster(&name).await?;
            Ok(xml_ok("DeleteDBCluster", ""))
        }
        _ => Err(RDSError::InvalidAction(format!("Unknown action: {}", action))),
    }
}

pub fn create_router(state: Arc<RDSState>) -> Router {
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
        let state = Arc::new(RDSState::new("123456789012".to_string(), "us-east-1".to_string()));
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
        let state = Arc::new(RDSState::new("123456789012".to_string(), "us-east-1".to_string()));
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
    async fn test_createdbinstance_action() {
        let state = Arc::new(RDSState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/")
            .header("content-type", "application/x-www-form-urlencoded")
            .body(Body::from("Action=CreateDBInstance&DBInstanceName=test-resource"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[tokio::test]
    async fn test_createdbcluster_action() {
        let state = Arc::new(RDSState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/")
            .header("content-type", "application/x-www-form-urlencoded")
            .body(Body::from("Action=CreateDBCluster&DBClusterName=test-resource"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
