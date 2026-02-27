use std::collections::HashMap;
use std::sync::Arc;

use axum::extract::State;
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use axum::Router;
use uuid::Uuid;

use crate::sns::error::SnsError;
use crate::sns::state::SnsState;
use crate::sns::types::*;

const NS: &str = "http://sns.amazonaws.com/doc/2010-03-31/";

// ── form parsing helpers ───────────────────────────────────────────────

type Params = HashMap<String, String>;

fn parse_form(body: &str) -> Params {
    form_urlencoded::parse(body.as_bytes())
        .into_owned()
        .collect()
}

fn param(params: &Params, key: &str) -> Option<String> {
    params.get(key).cloned()
}

fn require(params: &Params, key: &str) -> Result<String, SnsError> {
    param(params, key)
        .ok_or_else(|| SnsError::InvalidParameter(format!("Missing required parameter: {key}")))
}

/// Parse AWS query nested map: `Prefix.entry.N.key` / `Prefix.entry.N.value`
fn parse_attributes(params: &Params, prefix: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for i in 1..=100 {
        let k = format!("{prefix}.entry.{i}.key");
        let v = format!("{prefix}.entry.{i}.value");
        match (params.get(&k), params.get(&v)) {
            (Some(key), Some(val)) => {
                map.insert(key.clone(), val.clone());
            }
            _ => break,
        }
    }
    map
}

/// Parse AWS query nested list: `Prefix.member.N.Key` / `Prefix.member.N.Value`
fn parse_tags(params: &Params, prefix: &str) -> Vec<TagJson> {
    let mut tags = Vec::new();
    for i in 1..=100 {
        let k = format!("{prefix}.member.{i}.Key");
        let v = format!("{prefix}.member.{i}.Value");
        match (params.get(&k), params.get(&v)) {
            (Some(key), Some(val)) => {
                tags.push(TagJson {
                    key: key.clone(),
                    value: val.clone(),
                });
            }
            _ => break,
        }
    }
    tags
}

/// Parse AWS query nested string list: `Prefix.member.N`
fn parse_string_list(params: &Params, prefix: &str) -> Vec<String> {
    let mut items = Vec::new();
    for i in 1..=100 {
        let k = format!("{prefix}.member.{i}");
        match params.get(&k) {
            Some(val) => items.push(val.clone()),
            None => break,
        }
    }
    items
}

/// Parse AWS query nested batch entries: `Prefix.member.N.Id`, `.Message`, etc.
fn parse_batch_entries(params: &Params, prefix: &str) -> Vec<PublishBatchEntry> {
    let mut entries = Vec::new();
    for i in 1..=10 {
        let id_key = format!("{prefix}.member.{i}.Id");
        let msg_key = format!("{prefix}.member.{i}.Message");
        match (params.get(&id_key), params.get(&msg_key)) {
            (Some(id), Some(msg)) => {
                let subject_key = format!("{prefix}.member.{i}.Subject");
                let group_key = format!("{prefix}.member.{i}.MessageGroupId");
                let dedup_key = format!("{prefix}.member.{i}.MessageDeduplicationId");
                entries.push(PublishBatchEntry {
                    id: id.clone(),
                    message: msg.clone(),
                    _subject: params.get(&subject_key).cloned(),
                    _message_attributes: None,
                    _message_deduplication_id: params.get(&dedup_key).cloned(),
                    message_group_id: params.get(&group_key).cloned(),
                });
            }
            _ => break,
        }
    }
    entries
}

// ── XML response helpers ───────────────────────────────────────────────

fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

fn request_id() -> String {
    Uuid::new_v4().to_string()
}

fn xml_ok(action: &str, result_body: &str) -> Response {
    let xml = format!(
        r#"<{action}Response xmlns="{NS}">
  <{action}Result>
{result_body}
  </{action}Result>
  <ResponseMetadata>
    <RequestId>{rid}</RequestId>
  </ResponseMetadata>
</{action}Response>"#,
        action = action,
        NS = NS,
        result_body = result_body,
        rid = request_id(),
    );
    (
        axum::http::StatusCode::OK,
        [("content-type", "text/xml")],
        xml,
    )
        .into_response()
}

fn xml_empty(action: &str) -> Response {
    let xml = format!(
        r#"<{action}Response xmlns="{NS}">
  <{action}Result/>
  <ResponseMetadata>
    <RequestId>{rid}</RequestId>
  </ResponseMetadata>
</{action}Response>"#,
        action = action,
        NS = NS,
        rid = request_id(),
    );
    (
        axum::http::StatusCode::OK,
        [("content-type", "text/xml")],
        xml,
    )
        .into_response()
}

fn attributes_xml(attrs: &HashMap<String, String>) -> String {
    let mut xml = String::from("    <Attributes>\n");
    let mut keys: Vec<&String> = attrs.keys().collect();
    keys.sort();
    for key in keys {
        let val = &attrs[key];
        xml.push_str(&format!(
            "      <entry><key>{}</key><value>{}</value></entry>\n",
            xml_escape(key),
            xml_escape(val),
        ));
    }
    xml.push_str("    </Attributes>");
    xml
}

// ── dispatch ───────────────────────────────────────────────────────────

async fn handle_request(
    State(state): State<Arc<SnsState>>,
    body: String,
) -> Result<Response, SnsError> {
    let params = parse_form(&body);
    let action = param(&params, "Action")
        .ok_or_else(|| SnsError::InvalidAction("Missing Action parameter".into()))?;

    match action.as_str() {
        "CreateTopic" => handle_create_topic(state, params).await,
        "DeleteTopic" => handle_delete_topic(state, params).await,
        "ListTopics" => handle_list_topics(state, params).await,
        "GetTopicAttributes" => handle_get_topic_attributes(state, params).await,
        "SetTopicAttributes" => handle_set_topic_attributes(state, params).await,
        "Subscribe" => handle_subscribe(state, params).await,
        "Unsubscribe" => handle_unsubscribe(state, params).await,
        "ConfirmSubscription" => handle_confirm_subscription(state, params).await,
        "ListSubscriptions" => handle_list_subscriptions(state, params).await,
        "ListSubscriptionsByTopic" => handle_list_subscriptions_by_topic(state, params).await,
        "GetSubscriptionAttributes" => {
            handle_get_subscription_attributes(state, params).await
        }
        "SetSubscriptionAttributes" => {
            handle_set_subscription_attributes(state, params).await
        }
        "Publish" => handle_publish(state, params).await,
        "PublishBatch" => handle_publish_batch(state, params).await,
        "TagResource" => handle_tag_resource(state, params).await,
        "UntagResource" => handle_untag_resource(state, params).await,
        "ListTagsForResource" => handle_list_tags_for_resource(state, params).await,
        _ => Err(SnsError::InvalidAction(format!(
            "Unknown action: {action}"
        ))),
    }
}

// ── action handlers ────────────────────────────────────────────────────

async fn handle_create_topic(
    state: Arc<SnsState>,
    params: Params,
) -> Result<Response, SnsError> {
    let name = require(&params, "Name")?;
    let attributes = parse_attributes(&params, "Attributes");
    let tags = parse_tags(&params, "Tags");

    let req = CreateTopicRequest {
        name,
        attributes: if attributes.is_empty() {
            None
        } else {
            Some(attributes)
        },
        tags: if tags.is_empty() { None } else { Some(tags) },
    };
    let resp = state.create_topic(req).await?;
    Ok(xml_ok(
        "CreateTopic",
        &format!("    <TopicArn>{}</TopicArn>", xml_escape(&resp.topic_arn)),
    ))
}

async fn handle_delete_topic(
    state: Arc<SnsState>,
    params: Params,
) -> Result<Response, SnsError> {
    let topic_arn = require(&params, "TopicArn")?;
    state
        .delete_topic(DeleteTopicRequest { topic_arn })
        .await?;
    Ok(xml_empty("DeleteTopic"))
}

async fn handle_list_topics(
    state: Arc<SnsState>,
    params: Params,
) -> Result<Response, SnsError> {
    let req = ListTopicsRequest {
        next_token: param(&params, "NextToken"),
    };
    let resp = state.list_topics(req).await?;
    let mut body = String::from("    <Topics>\n");
    for t in &resp.topics {
        body.push_str(&format!(
            "      <member><TopicArn>{}</TopicArn></member>\n",
            xml_escape(&t.topic_arn)
        ));
    }
    body.push_str("    </Topics>");
    if let Some(ref token) = resp.next_token {
        body.push_str(&format!(
            "\n    <NextToken>{}</NextToken>",
            xml_escape(token)
        ));
    }
    Ok(xml_ok("ListTopics", &body))
}

async fn handle_get_topic_attributes(
    state: Arc<SnsState>,
    params: Params,
) -> Result<Response, SnsError> {
    let topic_arn = require(&params, "TopicArn")?;
    let resp = state
        .get_topic_attributes(GetTopicAttributesRequest { topic_arn })
        .await?;
    Ok(xml_ok("GetTopicAttributes", &attributes_xml(&resp.attributes)))
}

async fn handle_set_topic_attributes(
    state: Arc<SnsState>,
    params: Params,
) -> Result<Response, SnsError> {
    let req = SetTopicAttributesRequest {
        topic_arn: require(&params, "TopicArn")?,
        attribute_name: require(&params, "AttributeName")?,
        attribute_value: param(&params, "AttributeValue"),
    };
    state.set_topic_attributes(req).await?;
    Ok(xml_empty("SetTopicAttributes"))
}

async fn handle_subscribe(
    state: Arc<SnsState>,
    params: Params,
) -> Result<Response, SnsError> {
    let attributes = parse_attributes(&params, "Attributes");
    let req = SubscribeRequest {
        topic_arn: require(&params, "TopicArn")?,
        protocol: require(&params, "Protocol")?,
        endpoint: param(&params, "Endpoint"),
        attributes: if attributes.is_empty() {
            None
        } else {
            Some(attributes)
        },
        _return_subscription_arn: param(&params, "ReturnSubscriptionArn")
            .map(|v| v == "true"),
    };
    let resp = state.subscribe(req).await?;
    Ok(xml_ok(
        "Subscribe",
        &format!(
            "    <SubscriptionArn>{}</SubscriptionArn>",
            xml_escape(&resp.subscription_arn)
        ),
    ))
}

async fn handle_unsubscribe(
    state: Arc<SnsState>,
    params: Params,
) -> Result<Response, SnsError> {
    let req = UnsubscribeRequest {
        subscription_arn: require(&params, "SubscriptionArn")?,
    };
    state.unsubscribe(req).await?;
    Ok(xml_empty("Unsubscribe"))
}

async fn handle_confirm_subscription(
    state: Arc<SnsState>,
    params: Params,
) -> Result<Response, SnsError> {
    let req = ConfirmSubscriptionRequest {
        topic_arn: require(&params, "TopicArn")?,
        _token: require(&params, "Token")?,
        _authenticate_on_unsubscribe: param(&params, "AuthenticateOnUnsubscribe"),
    };
    let resp = state.confirm_subscription(req).await?;
    Ok(xml_ok(
        "ConfirmSubscription",
        &format!(
            "    <SubscriptionArn>{}</SubscriptionArn>",
            xml_escape(&resp.subscription_arn)
        ),
    ))
}

async fn handle_list_subscriptions(
    state: Arc<SnsState>,
    params: Params,
) -> Result<Response, SnsError> {
    let req = ListSubscriptionsRequest {
        _next_token: param(&params, "NextToken"),
    };
    let resp = state.list_subscriptions(req).await?;
    Ok(xml_ok(
        "ListSubscriptions",
        &subscriptions_xml(&resp.subscriptions, resp.next_token.as_deref()),
    ))
}

async fn handle_list_subscriptions_by_topic(
    state: Arc<SnsState>,
    params: Params,
) -> Result<Response, SnsError> {
    let req = ListSubscriptionsByTopicRequest {
        topic_arn: require(&params, "TopicArn")?,
        _next_token: param(&params, "NextToken"),
    };
    let resp = state.list_subscriptions_by_topic(req).await?;
    Ok(xml_ok(
        "ListSubscriptionsByTopic",
        &subscriptions_xml(&resp.subscriptions, resp.next_token.as_deref()),
    ))
}

fn subscriptions_xml(subs: &[SubscriptionEntry], next_token: Option<&str>) -> String {
    let mut body = String::from("    <Subscriptions>\n");
    for s in subs {
        body.push_str(&format!(
            "      <member>\n        <TopicArn>{}</TopicArn>\n        <Protocol>{}</Protocol>\n        <SubscriptionArn>{}</SubscriptionArn>\n        <Owner>{}</Owner>\n        <Endpoint>{}</Endpoint>\n      </member>\n",
            xml_escape(&s.topic_arn),
            xml_escape(&s.protocol),
            xml_escape(&s.subscription_arn),
            xml_escape(&s.owner),
            xml_escape(&s.endpoint),
        ));
    }
    body.push_str("    </Subscriptions>");
    if let Some(token) = next_token {
        body.push_str(&format!(
            "\n    <NextToken>{}</NextToken>",
            xml_escape(token)
        ));
    }
    body
}

async fn handle_get_subscription_attributes(
    state: Arc<SnsState>,
    params: Params,
) -> Result<Response, SnsError> {
    let req = GetSubscriptionAttributesRequest {
        subscription_arn: require(&params, "SubscriptionArn")?,
    };
    let resp = state.get_subscription_attributes(req).await?;
    Ok(xml_ok(
        "GetSubscriptionAttributes",
        &attributes_xml(&resp.attributes),
    ))
}

async fn handle_set_subscription_attributes(
    state: Arc<SnsState>,
    params: Params,
) -> Result<Response, SnsError> {
    let req = SetSubscriptionAttributesRequest {
        subscription_arn: require(&params, "SubscriptionArn")?,
        attribute_name: require(&params, "AttributeName")?,
        attribute_value: param(&params, "AttributeValue"),
    };
    state.set_subscription_attributes(req).await?;
    Ok(xml_empty("SetSubscriptionAttributes"))
}

async fn handle_publish(
    state: Arc<SnsState>,
    params: Params,
) -> Result<Response, SnsError> {
    let req = PublishRequest {
        topic_arn: param(&params, "TopicArn"),
        target_arn: param(&params, "TargetArn"),
        message: require(&params, "Message")?,
        _subject: param(&params, "Subject"),
        _message_structure: param(&params, "MessageStructure"),
        _message_attributes: None,
        _message_deduplication_id: param(&params, "MessageDeduplicationId"),
        message_group_id: param(&params, "MessageGroupId"),
    };
    let resp = state.publish(req).await?;
    let mut body = format!(
        "    <MessageId>{}</MessageId>",
        xml_escape(&resp.message_id)
    );
    if let Some(ref seq) = resp.sequence_number {
        body.push_str(&format!(
            "\n    <SequenceNumber>{}</SequenceNumber>",
            xml_escape(seq)
        ));
    }
    Ok(xml_ok("Publish", &body))
}

async fn handle_publish_batch(
    state: Arc<SnsState>,
    params: Params,
) -> Result<Response, SnsError> {
    let entries = parse_batch_entries(&params, "PublishBatchRequestEntries");
    let req = PublishBatchRequest {
        topic_arn: require(&params, "TopicArn")?,
        publish_batch_request_entries: entries,
    };
    let resp = state.publish_batch(req).await?;

    let mut body = String::from("    <Successful>\n");
    for s in &resp.successful {
        body.push_str("      <member>\n");
        body.push_str(&format!(
            "        <Id>{}</Id>\n",
            xml_escape(&s.id)
        ));
        body.push_str(&format!(
            "        <MessageId>{}</MessageId>\n",
            xml_escape(&s.message_id)
        ));
        if let Some(ref seq) = s.sequence_number {
            body.push_str(&format!(
                "        <SequenceNumber>{}</SequenceNumber>\n",
                xml_escape(seq)
            ));
        }
        body.push_str("      </member>\n");
    }
    body.push_str("    </Successful>\n");

    body.push_str("    <Failed>\n");
    for f in &resp.failed {
        body.push_str("      <member>\n");
        body.push_str(&format!(
            "        <Id>{}</Id>\n",
            xml_escape(&f.id)
        ));
        body.push_str(&format!(
            "        <Code>{}</Code>\n",
            xml_escape(&f.code)
        ));
        body.push_str(&format!(
            "        <Message>{}</Message>\n",
            xml_escape(&f.message)
        ));
        body.push_str(&format!(
            "        <SenderFault>{}</SenderFault>\n",
            f.sender_fault
        ));
        body.push_str("      </member>\n");
    }
    body.push_str("    </Failed>");

    Ok(xml_ok("PublishBatch", &body))
}

async fn handle_tag_resource(
    state: Arc<SnsState>,
    params: Params,
) -> Result<Response, SnsError> {
    let tags = parse_tags(&params, "Tags");
    let req = TagResourceRequest {
        resource_arn: require(&params, "ResourceArn")?,
        tags,
    };
    state.tag_resource(req).await?;
    Ok(xml_empty("TagResource"))
}

async fn handle_untag_resource(
    state: Arc<SnsState>,
    params: Params,
) -> Result<Response, SnsError> {
    let tag_keys = parse_string_list(&params, "TagKeys");
    let req = UntagResourceRequest {
        resource_arn: require(&params, "ResourceArn")?,
        tag_keys,
    };
    state.untag_resource(req).await?;
    Ok(xml_empty("UntagResource"))
}

async fn handle_list_tags_for_resource(
    state: Arc<SnsState>,
    params: Params,
) -> Result<Response, SnsError> {
    let req = ListTagsForResourceRequest {
        resource_arn: require(&params, "ResourceArn")?,
    };
    let resp = state.list_tags_for_resource(req).await?;
    let mut body = String::from("    <Tags>\n");
    for tag in &resp.tags {
        body.push_str(&format!(
            "      <member><Key>{}</Key><Value>{}</Value></member>\n",
            xml_escape(&tag.key),
            xml_escape(&tag.value),
        ));
    }
    body.push_str("    </Tags>");
    Ok(xml_ok("ListTagsForResource", &body))
}

// ── router ─────────────────────────────────────────────────────────────

pub fn create_router(state: Arc<SnsState>) -> Router {
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
        let state = Arc::new(SnsState::new("123456789012".to_string(), "us-east-1".to_string()));
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
        let state = Arc::new(SnsState::new("123456789012".to_string(), "us-east-1".to_string()));
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

    fn new_state() -> Arc<SnsState> {
        Arc::new(SnsState::new("123456789012".to_string(), "us-east-1".to_string()))
    }

    fn sns_req(body: &str) -> Request<Body> {
        Request::builder()
            .method("POST")
            .uri("/")
            .header("content-type", "application/x-www-form-urlencoded")
            .body(Body::from(body.to_string()))
            .unwrap()
    }

    #[tokio::test]
    async fn test_create_topic() {
        let app = create_router(new_state());
        let resp = app.oneshot(sns_req("Action=CreateTopic&Name=test-topic")).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX).await.unwrap();
        let body_str = String::from_utf8(body.to_vec()).unwrap();
        assert!(body_str.contains("TopicArn"));
        assert!(body_str.contains("test-topic"));
    }

    #[tokio::test]
    async fn test_create_topic_missing_name() {
        let app = create_router(new_state());
        let resp = app.oneshot(sns_req("Action=CreateTopic")).await.unwrap();
        assert_ne!(resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_list_topics() {
        let state = new_state();
        let app = create_router(state.clone());
        app.oneshot(sns_req("Action=CreateTopic&Name=topic1")).await.unwrap();

        let app = create_router(state);
        let resp = app.oneshot(sns_req("Action=ListTopics")).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX).await.unwrap();
        let body_str = String::from_utf8(body.to_vec()).unwrap();
        assert!(body_str.contains("topic1"));
    }

    #[tokio::test]
    async fn test_delete_topic() {
        let state = new_state();
        let app = create_router(state.clone());
        let resp = app.oneshot(sns_req("Action=CreateTopic&Name=del-me")).await.unwrap();
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX).await.unwrap();
        let body_str = String::from_utf8(body.to_vec()).unwrap();
        // Extract ARN from response
        let arn_start = body_str.find("<TopicArn>").unwrap() + 10;
        let arn_end = body_str.find("</TopicArn>").unwrap();
        let arn = &body_str[arn_start..arn_end];

        let app = create_router(state);
        let resp = app
            .oneshot(sns_req(&format!("Action=DeleteTopic&TopicArn={}", arn)))
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_get_topic_attributes() {
        let state = new_state();
        let app = create_router(state.clone());
        let resp = app.oneshot(sns_req("Action=CreateTopic&Name=attr-topic")).await.unwrap();
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX).await.unwrap();
        let body_str = String::from_utf8(body.to_vec()).unwrap();
        let arn_start = body_str.find("<TopicArn>").unwrap() + 10;
        let arn_end = body_str.find("</TopicArn>").unwrap();
        let arn = &body_str[arn_start..arn_end];

        let app = create_router(state);
        let resp = app
            .oneshot(sns_req(&format!("Action=GetTopicAttributes&TopicArn={}", arn)))
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX).await.unwrap();
        let body_str = String::from_utf8(body.to_vec()).unwrap();
        assert!(body_str.contains("Attributes"));
    }

    #[tokio::test]
    async fn test_set_topic_attributes() {
        let state = new_state();
        let app = create_router(state.clone());
        let resp = app.oneshot(sns_req("Action=CreateTopic&Name=set-attr")).await.unwrap();
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX).await.unwrap();
        let body_str = String::from_utf8(body.to_vec()).unwrap();
        let arn_start = body_str.find("<TopicArn>").unwrap() + 10;
        let arn_end = body_str.find("</TopicArn>").unwrap();
        let arn = &body_str[arn_start..arn_end];

        let app = create_router(state);
        let resp = app
            .oneshot(sns_req(&format!(
                "Action=SetTopicAttributes&TopicArn={}&AttributeName=DisplayName&AttributeValue=MyTopic",
                arn
            )))
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_subscribe_and_list_subscriptions() {
        let state = new_state();
        let app = create_router(state.clone());
        let resp = app.oneshot(sns_req("Action=CreateTopic&Name=sub-topic")).await.unwrap();
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX).await.unwrap();
        let body_str = String::from_utf8(body.to_vec()).unwrap();
        let arn_start = body_str.find("<TopicArn>").unwrap() + 10;
        let arn_end = body_str.find("</TopicArn>").unwrap();
        let arn = body_str[arn_start..arn_end].to_string();

        let app = create_router(state.clone());
        let resp = app
            .oneshot(sns_req(&format!(
                "Action=Subscribe&TopicArn={}&Protocol=email&Endpoint=test@example.com",
                arn
            )))
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX).await.unwrap();
        let body_str = String::from_utf8(body.to_vec()).unwrap();
        assert!(body_str.contains("SubscriptionArn"));

        // List subscriptions
        let app = create_router(state.clone());
        let resp = app.oneshot(sns_req("Action=ListSubscriptions")).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        // List subscriptions by topic
        let app = create_router(state);
        let resp = app
            .oneshot(sns_req(&format!(
                "Action=ListSubscriptionsByTopic&TopicArn={}",
                arn
            )))
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_unsubscribe() {
        let state = new_state();
        let app = create_router(state.clone());
        let resp = app.oneshot(sns_req("Action=CreateTopic&Name=unsub-topic")).await.unwrap();
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX).await.unwrap();
        let body_str = String::from_utf8(body.to_vec()).unwrap();
        let arn_start = body_str.find("<TopicArn>").unwrap() + 10;
        let arn_end = body_str.find("</TopicArn>").unwrap();
        let topic_arn = body_str[arn_start..arn_end].to_string();

        let app = create_router(state.clone());
        let resp = app
            .oneshot(sns_req(&format!(
                "Action=Subscribe&TopicArn={}&Protocol=sqs&Endpoint=arn:aws:sqs:us-east-1:123456789012:myqueue",
                topic_arn
            )))
            .await
            .unwrap();
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX).await.unwrap();
        let body_str = String::from_utf8(body.to_vec()).unwrap();
        let sub_start = body_str.find("<SubscriptionArn>").unwrap() + 17;
        let sub_end = body_str.find("</SubscriptionArn>").unwrap();
        let sub_arn = &body_str[sub_start..sub_end];

        let app = create_router(state);
        let resp = app
            .oneshot(sns_req(&format!("Action=Unsubscribe&SubscriptionArn={}", sub_arn)))
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_publish() {
        let state = new_state();
        let app = create_router(state.clone());
        let resp = app.oneshot(sns_req("Action=CreateTopic&Name=pub-topic")).await.unwrap();
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX).await.unwrap();
        let body_str = String::from_utf8(body.to_vec()).unwrap();
        let arn_start = body_str.find("<TopicArn>").unwrap() + 10;
        let arn_end = body_str.find("</TopicArn>").unwrap();
        let arn = &body_str[arn_start..arn_end];

        let app = create_router(state);
        let resp = app
            .oneshot(sns_req(&format!(
                "Action=Publish&TopicArn={}&Message=hello",
                arn
            )))
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX).await.unwrap();
        let body_str = String::from_utf8(body.to_vec()).unwrap();
        assert!(body_str.contains("MessageId"));
    }

    #[tokio::test]
    async fn test_publish_batch() {
        let state = new_state();
        let app = create_router(state.clone());
        let resp = app.oneshot(sns_req("Action=CreateTopic&Name=batch-topic")).await.unwrap();
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX).await.unwrap();
        let body_str = String::from_utf8(body.to_vec()).unwrap();
        let arn_start = body_str.find("<TopicArn>").unwrap() + 10;
        let arn_end = body_str.find("</TopicArn>").unwrap();
        let arn = &body_str[arn_start..arn_end];

        let app = create_router(state);
        let resp = app
            .oneshot(sns_req(&format!(
                "Action=PublishBatch&TopicArn={}&PublishBatchRequestEntries.member.1.Id=msg1&PublishBatchRequestEntries.member.1.Message=hello",
                arn
            )))
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX).await.unwrap();
        let body_str = String::from_utf8(body.to_vec()).unwrap();
        assert!(body_str.contains("Successful"));
    }

    #[tokio::test]
    async fn test_tag_and_list_tags() {
        let state = new_state();
        let app = create_router(state.clone());
        let resp = app.oneshot(sns_req("Action=CreateTopic&Name=tag-topic")).await.unwrap();
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX).await.unwrap();
        let body_str = String::from_utf8(body.to_vec()).unwrap();
        let arn_start = body_str.find("<TopicArn>").unwrap() + 10;
        let arn_end = body_str.find("</TopicArn>").unwrap();
        let arn = body_str[arn_start..arn_end].to_string();

        let app = create_router(state.clone());
        let resp = app
            .oneshot(sns_req(&format!(
                "Action=TagResource&ResourceArn={}&Tags.member.1.Key=env&Tags.member.1.Value=prod",
                arn
            )))
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        let app = create_router(state.clone());
        let resp = app
            .oneshot(sns_req(&format!("Action=ListTagsForResource&ResourceArn={}", arn)))
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX).await.unwrap();
        let body_str = String::from_utf8(body.to_vec()).unwrap();
        assert!(body_str.contains("env"));

        // Untag
        let app = create_router(state);
        let resp = app
            .oneshot(sns_req(&format!(
                "Action=UntagResource&ResourceArn={}&TagKeys.member.1=env",
                arn
            )))
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_confirm_subscription() {
        let state = new_state();
        let app = create_router(state.clone());
        let resp = app.oneshot(sns_req("Action=CreateTopic&Name=confirm-topic")).await.unwrap();
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX).await.unwrap();
        let body_str = String::from_utf8(body.to_vec()).unwrap();
        let arn_start = body_str.find("<TopicArn>").unwrap() + 10;
        let arn_end = body_str.find("</TopicArn>").unwrap();
        let arn = body_str[arn_start..arn_end].to_string();

        // Subscribe first so there is a subscription to confirm
        let app = create_router(state.clone());
        app.oneshot(sns_req(&format!(
            "Action=Subscribe&TopicArn={}&Protocol=sqs&Endpoint=arn:aws:sqs:us-east-1:123456789012:q",
            arn
        )))
        .await
        .unwrap();

        let app = create_router(state);
        let resp = app
            .oneshot(sns_req(&format!(
                "Action=ConfirmSubscription&TopicArn={}&Token=fake-token",
                arn
            )))
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_get_subscription_attributes() {
        let state = new_state();
        let app = create_router(state.clone());
        let resp = app.oneshot(sns_req("Action=CreateTopic&Name=gsa-topic")).await.unwrap();
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX).await.unwrap();
        let body_str = String::from_utf8(body.to_vec()).unwrap();
        let arn_start = body_str.find("<TopicArn>").unwrap() + 10;
        let arn_end = body_str.find("</TopicArn>").unwrap();
        let topic_arn = body_str[arn_start..arn_end].to_string();

        let app = create_router(state.clone());
        let resp = app
            .oneshot(sns_req(&format!(
                "Action=Subscribe&TopicArn={}&Protocol=sqs&Endpoint=arn:aws:sqs:us-east-1:123456789012:q",
                topic_arn
            )))
            .await
            .unwrap();
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX).await.unwrap();
        let body_str = String::from_utf8(body.to_vec()).unwrap();
        let sub_start = body_str.find("<SubscriptionArn>").unwrap() + 17;
        let sub_end = body_str.find("</SubscriptionArn>").unwrap();
        let sub_arn = &body_str[sub_start..sub_end];

        let app = create_router(state);
        let resp = app
            .oneshot(sns_req(&format!(
                "Action=GetSubscriptionAttributes&SubscriptionArn={}",
                sub_arn
            )))
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_set_subscription_attributes() {
        let state = new_state();
        let app = create_router(state.clone());
        let resp = app.oneshot(sns_req("Action=CreateTopic&Name=ssa-topic")).await.unwrap();
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX).await.unwrap();
        let body_str = String::from_utf8(body.to_vec()).unwrap();
        let arn_start = body_str.find("<TopicArn>").unwrap() + 10;
        let arn_end = body_str.find("</TopicArn>").unwrap();
        let topic_arn = body_str[arn_start..arn_end].to_string();

        let app = create_router(state.clone());
        let resp = app
            .oneshot(sns_req(&format!(
                "Action=Subscribe&TopicArn={}&Protocol=sqs&Endpoint=arn:aws:sqs:us-east-1:123456789012:q2",
                topic_arn
            )))
            .await
            .unwrap();
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX).await.unwrap();
        let body_str = String::from_utf8(body.to_vec()).unwrap();
        let sub_start = body_str.find("<SubscriptionArn>").unwrap() + 17;
        let sub_end = body_str.find("</SubscriptionArn>").unwrap();
        let sub_arn = body_str[sub_start..sub_end].to_string();

        let app = create_router(state);
        let resp = app
            .oneshot(sns_req(&format!(
                "Action=SetSubscriptionAttributes&SubscriptionArn={}&AttributeName=RawMessageDelivery&AttributeValue=true",
                sub_arn
            )))
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
