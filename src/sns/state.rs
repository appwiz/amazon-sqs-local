use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::sns::error::SnsError;
use crate::sns::topic::*;
use crate::sns::types::*;

struct SnsStateInner {
    topics: HashMap<String, Topic>,
    account_id: String,
    region: String,
    sequence_counter: u64,
}

pub struct SnsState {
    inner: Arc<Mutex<SnsStateInner>>,
}

impl SnsState {
    pub fn new(account_id: String, region: String) -> Self {
        SnsState {
            inner: Arc::new(Mutex::new(SnsStateInner {
                topics: HashMap::new(),
                account_id,
                region,
                sequence_counter: 0,
            })),
        }
    }

    // --- Topic operations ---

    pub async fn create_topic(
        &self,
        req: CreateTopicRequest,
    ) -> Result<CreateTopicResponse, SnsError> {
        let mut inner = self.inner.lock().await;

        let is_fifo = req
            .attributes
            .as_ref()
            .and_then(|a| a.get("FifoTopic"))
            .map(|v| v == "true")
            .unwrap_or_else(|| req.name.ends_with(".fifo"));

        if is_fifo && !req.name.ends_with(".fifo") {
            return Err(SnsError::InvalidParameter(
                "FIFO topic name must end with .fifo".into(),
            ));
        }

        if req.name.is_empty() || req.name.len() > 256 {
            return Err(SnsError::InvalidParameter(
                "Topic name must be between 1 and 256 characters".into(),
            ));
        }

        let arn = format!(
            "arn:aws:sns:{}:{}:{}",
            inner.region, inner.account_id, req.name
        );

        if let Some(existing) = inner.topics.get(&arn) {
            // Idempotent: return existing if attributes match
            return Ok(CreateTopicResponse {
                topic_arn: existing.arn.clone(),
            });
        }

        let mut topic = Topic::new(
            req.name,
            arn.clone(),
            inner.account_id.clone(),
            is_fifo,
        );

        if let Some(attrs) = req.attributes {
            for (key, value) in attrs {
                match key.as_str() {
                    "DisplayName" => topic.attributes.display_name = value,
                    "Policy" => topic.attributes.policy = value,
                    "DeliveryPolicy" => topic.attributes.delivery_policy = value,
                    "KmsMasterKeyId" => {
                        topic.attributes.kms_master_key_id = Some(value);
                    }
                    "FifoTopic" => {} // Already handled
                    "ContentBasedDeduplication" => {
                        topic.attributes.content_based_deduplication = value == "true";
                    }
                    _ => {}
                }
            }
        }

        if let Some(tags) = req.tags {
            for tag in tags {
                topic.tags.insert(tag.key, tag.value);
            }
        }

        inner.topics.insert(arn.clone(), topic);
        Ok(CreateTopicResponse { topic_arn: arn })
    }

    pub async fn delete_topic(&self, req: DeleteTopicRequest) -> Result<(), SnsError> {
        let mut inner = self.inner.lock().await;
        inner.topics.remove(&req.topic_arn);
        Ok(())
    }

    pub async fn list_topics(
        &self,
        req: ListTopicsRequest,
    ) -> Result<ListTopicsResponse, SnsError> {
        let inner = self.inner.lock().await;
        let mut arns: Vec<String> = inner.topics.keys().cloned().collect();
        arns.sort();

        let start = if let Some(ref token) = req.next_token {
            arns.iter()
                .position(|a| a.as_str() > token.as_str())
                .unwrap_or(arns.len())
        } else {
            0
        };

        let max = 100;
        let page: Vec<TopicArnEntry> = arns[start..]
            .iter()
            .take(max)
            .map(|a| TopicArnEntry {
                topic_arn: a.clone(),
            })
            .collect();

        let next_token = if start + max < arns.len() {
            arns.get(start + max).cloned()
        } else {
            None
        };

        Ok(ListTopicsResponse {
            topics: page,
            next_token,
        })
    }

    pub async fn get_topic_attributes(
        &self,
        req: GetTopicAttributesRequest,
    ) -> Result<GetTopicAttributesResponse, SnsError> {
        let inner = self.inner.lock().await;
        let topic = inner.topics.get(&req.topic_arn).ok_or_else(|| {
            SnsError::NotFound("Topic does not exist".into())
        })?;
        Ok(GetTopicAttributesResponse {
            attributes: topic.attributes.to_map(topic),
        })
    }

    pub async fn set_topic_attributes(
        &self,
        req: SetTopicAttributesRequest,
    ) -> Result<(), SnsError> {
        let mut inner = self.inner.lock().await;
        let topic = inner.topics.get_mut(&req.topic_arn).ok_or_else(|| {
            SnsError::NotFound("Topic does not exist".into())
        })?;

        let value = req.attribute_value.unwrap_or_default();
        match req.attribute_name.as_str() {
            "DisplayName" => topic.attributes.display_name = value,
            "Policy" => topic.attributes.policy = value,
            "DeliveryPolicy" => topic.attributes.delivery_policy = value,
            "KmsMasterKeyId" => {
                topic.attributes.kms_master_key_id = if value.is_empty() {
                    None
                } else {
                    Some(value)
                };
            }
            "ContentBasedDeduplication" => {
                topic.attributes.content_based_deduplication = value == "true";
            }
            _ => {
                return Err(SnsError::InvalidParameter(format!(
                    "Invalid attribute name: {}",
                    req.attribute_name
                )));
            }
        }
        Ok(())
    }

    // --- Subscription operations ---

    pub async fn subscribe(
        &self,
        req: SubscribeRequest,
    ) -> Result<SubscribeResponse, SnsError> {
        let mut inner = self.inner.lock().await;
        let region = inner.region.clone();
        let account_id = inner.account_id.clone();
        let topic = inner.topics.get_mut(&req.topic_arn).ok_or_else(|| {
            SnsError::NotFound("Topic does not exist".into())
        })?;

        let endpoint = req.endpoint.unwrap_or_default();

        let mut sub = Subscription::new(
            req.topic_arn.clone(),
            req.protocol.clone(),
            endpoint,
            account_id.clone(),
            &region,
            &account_id,
        );

        if let Some(attrs) = req.attributes {
            for (key, value) in attrs {
                match key.as_str() {
                    "RawMessageDelivery" => {
                        sub.attributes.raw_message_delivery = value == "true";
                    }
                    "FilterPolicy" => {
                        sub.attributes.filter_policy = Some(value);
                    }
                    "FilterPolicyScope" => {
                        sub.attributes.filter_policy_scope = value;
                    }
                    "RedrivePolicy" => {
                        sub.attributes.redrive_policy = Some(value);
                    }
                    _ => {}
                }
            }
        }

        let arn = sub.arn.clone();
        topic.subscriptions.insert(arn.clone(), sub);

        Ok(SubscribeResponse {
            subscription_arn: arn,
        })
    }

    pub async fn unsubscribe(&self, req: UnsubscribeRequest) -> Result<(), SnsError> {
        let mut inner = self.inner.lock().await;
        // Find the topic that contains this subscription
        for topic in inner.topics.values_mut() {
            if topic.subscriptions.remove(&req.subscription_arn).is_some() {
                return Ok(());
            }
        }
        Err(SnsError::NotFound("Subscription does not exist".into()))
    }

    pub async fn confirm_subscription(
        &self,
        req: ConfirmSubscriptionRequest,
    ) -> Result<ConfirmSubscriptionResponse, SnsError> {
        let inner = self.inner.lock().await;
        let topic = inner.topics.get(&req.topic_arn).ok_or_else(|| {
            SnsError::NotFound("Topic does not exist".into())
        })?;

        // For local service, just return the first pending subscription or generate a token
        for sub in topic.subscriptions.values() {
            if !sub.confirmed {
                return Ok(ConfirmSubscriptionResponse {
                    subscription_arn: sub.arn.clone(),
                });
            }
        }

        // If all confirmed, return first sub
        if let Some(sub) = topic.subscriptions.values().next() {
            return Ok(ConfirmSubscriptionResponse {
                subscription_arn: sub.arn.clone(),
            });
        }

        Err(SnsError::NotFound("No subscription found".into()))
    }

    pub async fn list_subscriptions(
        &self,
        _req: ListSubscriptionsRequest,
    ) -> Result<ListSubscriptionsResponse, SnsError> {
        let inner = self.inner.lock().await;
        let mut entries = Vec::new();
        for topic in inner.topics.values() {
            for sub in topic.subscriptions.values() {
                entries.push(SubscriptionEntry {
                    subscription_arn: sub.arn.clone(),
                    owner: sub.owner.clone(),
                    protocol: sub.protocol.clone(),
                    endpoint: sub.endpoint.clone(),
                    topic_arn: sub.topic_arn.clone(),
                });
            }
        }
        entries.sort_by(|a, b| a.subscription_arn.cmp(&b.subscription_arn));
        Ok(ListSubscriptionsResponse {
            subscriptions: entries,
            next_token: None,
        })
    }

    pub async fn list_subscriptions_by_topic(
        &self,
        req: ListSubscriptionsByTopicRequest,
    ) -> Result<ListSubscriptionsResponse, SnsError> {
        let inner = self.inner.lock().await;
        let topic = inner.topics.get(&req.topic_arn).ok_or_else(|| {
            SnsError::NotFound("Topic does not exist".into())
        })?;

        let mut entries: Vec<SubscriptionEntry> = topic
            .subscriptions
            .values()
            .map(|sub| SubscriptionEntry {
                subscription_arn: sub.arn.clone(),
                owner: sub.owner.clone(),
                protocol: sub.protocol.clone(),
                endpoint: sub.endpoint.clone(),
                topic_arn: sub.topic_arn.clone(),
            })
            .collect();
        entries.sort_by(|a, b| a.subscription_arn.cmp(&b.subscription_arn));

        Ok(ListSubscriptionsResponse {
            subscriptions: entries,
            next_token: None,
        })
    }

    pub async fn get_subscription_attributes(
        &self,
        req: GetSubscriptionAttributesRequest,
    ) -> Result<GetSubscriptionAttributesResponse, SnsError> {
        let inner = self.inner.lock().await;
        for topic in inner.topics.values() {
            if let Some(sub) = topic.subscriptions.get(&req.subscription_arn) {
                return Ok(GetSubscriptionAttributesResponse {
                    attributes: sub.attributes.to_map(sub),
                });
            }
        }
        Err(SnsError::NotFound("Subscription does not exist".into()))
    }

    pub async fn set_subscription_attributes(
        &self,
        req: SetSubscriptionAttributesRequest,
    ) -> Result<(), SnsError> {
        let mut inner = self.inner.lock().await;
        for topic in inner.topics.values_mut() {
            if let Some(sub) = topic.subscriptions.get_mut(&req.subscription_arn) {
                let value = req.attribute_value.unwrap_or_default();
                match req.attribute_name.as_str() {
                    "RawMessageDelivery" => {
                        sub.attributes.raw_message_delivery = value == "true";
                    }
                    "FilterPolicy" => {
                        sub.attributes.filter_policy = if value.is_empty() {
                            None
                        } else {
                            Some(value)
                        };
                    }
                    "FilterPolicyScope" => {
                        sub.attributes.filter_policy_scope = value;
                    }
                    "RedrivePolicy" => {
                        sub.attributes.redrive_policy = if value.is_empty() {
                            None
                        } else {
                            Some(value)
                        };
                    }
                    _ => {
                        return Err(SnsError::InvalidParameter(format!(
                            "Invalid attribute name: {}",
                            req.attribute_name
                        )));
                    }
                }
                return Ok(());
            }
        }
        Err(SnsError::NotFound("Subscription does not exist".into()))
    }

    // --- Publish ---

    pub async fn publish(
        &self,
        req: PublishRequest,
    ) -> Result<PublishResponse, SnsError> {
        let mut inner = self.inner.lock().await;
        let topic_arn = req
            .topic_arn
            .or(req.target_arn)
            .ok_or_else(|| SnsError::InvalidParameter("TopicArn is required".into()))?;

        let topic = inner.topics.get(&topic_arn).ok_or_else(|| {
            SnsError::NotFound("Topic does not exist".into())
        })?;

        if topic.attributes.fifo_topic && req.message_group_id.is_none() {
            return Err(SnsError::InvalidParameter(
                "MessageGroupId is required for FIFO topics".into(),
            ));
        }

        if req.message.is_empty() {
            return Err(SnsError::InvalidParameter(
                "Message must not be empty".into(),
            ));
        }

        if req.message.len() > 262144 {
            return Err(SnsError::InvalidParameter(
                "Message must be shorter than 262144 bytes".into(),
            ));
        }

        let message_id = Uuid::new_v4().to_string();
        let sequence_number = if topic.attributes.fifo_topic {
            inner.sequence_counter += 1;
            Some(format!("{:020}", inner.sequence_counter))
        } else {
            None
        };

        Ok(PublishResponse {
            message_id,
            sequence_number,
        })
    }

    pub async fn publish_batch(
        &self,
        req: PublishBatchRequest,
    ) -> Result<PublishBatchResponse, SnsError> {
        let mut inner = self.inner.lock().await;
        let topic = inner.topics.get(&req.topic_arn).ok_or_else(|| {
            SnsError::NotFound("Topic does not exist".into())
        })?;

        if req.publish_batch_request_entries.is_empty() {
            return Err(SnsError::InvalidParameter(
                "Batch must contain at least one entry".into(),
            ));
        }
        if req.publish_batch_request_entries.len() > 10 {
            return Err(SnsError::InvalidParameter(
                "Batch must contain at most 10 entries".into(),
            ));
        }

        let is_fifo = topic.attributes.fifo_topic;
        let mut successful = Vec::new();
        let mut failed = Vec::new();

        for entry in req.publish_batch_request_entries {
            if entry.message.is_empty() {
                failed.push(BatchResultErrorEntry {
                    id: entry.id,
                    code: "InvalidParameter".into(),
                    message: "Message must not be empty".into(),
                    sender_fault: true,
                });
                continue;
            }

            if is_fifo && entry.message_group_id.is_none() {
                failed.push(BatchResultErrorEntry {
                    id: entry.id,
                    code: "InvalidParameter".into(),
                    message: "MessageGroupId is required for FIFO topics".into(),
                    sender_fault: true,
                });
                continue;
            }

            let message_id = Uuid::new_v4().to_string();
            let sequence_number = if is_fifo {
                inner.sequence_counter += 1;
                Some(format!("{:020}", inner.sequence_counter))
            } else {
                None
            };

            successful.push(PublishBatchResultEntry {
                id: entry.id,
                message_id,
                sequence_number,
            });
        }

        Ok(PublishBatchResponse {
            successful,
            failed,
        })
    }

    // --- Tagging ---

    pub async fn tag_resource(&self, req: TagResourceRequest) -> Result<(), SnsError> {
        let mut inner = self.inner.lock().await;
        let topic = inner.topics.get_mut(&req.resource_arn).ok_or_else(|| {
            SnsError::NotFound("Resource does not exist".into())
        })?;

        for tag in req.tags {
            topic.tags.insert(tag.key, tag.value);
        }

        if topic.tags.len() > 50 {
            return Err(SnsError::TagLimitExceeded(
                "Maximum 50 tags per resource".into(),
            ));
        }
        Ok(())
    }

    pub async fn untag_resource(&self, req: UntagResourceRequest) -> Result<(), SnsError> {
        let mut inner = self.inner.lock().await;
        let topic = inner.topics.get_mut(&req.resource_arn).ok_or_else(|| {
            SnsError::NotFound("Resource does not exist".into())
        })?;

        for key in &req.tag_keys {
            topic.tags.remove(key);
        }
        Ok(())
    }

    pub async fn list_tags_for_resource(
        &self,
        req: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, SnsError> {
        let inner = self.inner.lock().await;
        let topic = inner.topics.get(&req.resource_arn).ok_or_else(|| {
            SnsError::NotFound("Resource does not exist".into())
        })?;

        let tags: Vec<TagJson> = topic
            .tags
            .iter()
            .map(|(k, v)| TagJson {
                key: k.clone(),
                value: v.clone(),
            })
            .collect();

        Ok(ListTagsForResourceResponse { tags })
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = SnsState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_delete_topic_not_found() {
        let state = SnsState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DeleteTopicRequest::default();
        let result = state.delete_topic(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_unsubscribe() {
        let state = SnsState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = UnsubscribeRequest::default();
        let _ = state.unsubscribe(req).await;
    }
    #[tokio::test]
    async fn test_tag_resource() {
        let state = SnsState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = TagResourceRequest::default();
        let _ = state.tag_resource(req).await;
    }
    #[tokio::test]
    async fn test_untag_resource() {
        let state = SnsState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = UntagResourceRequest::default();
        let _ = state.untag_resource(req).await;
    }

    fn make_state() -> SnsState {
        SnsState::new("123456789012".to_string(), "us-east-1".to_string())
    }

    async fn create_topic(state: &SnsState, name: &str) -> String {
        let req = CreateTopicRequest { name: name.to_string(), ..Default::default() };
        state.create_topic(req).await.unwrap().topic_arn
    }

    #[tokio::test]
    async fn test_create_topic() {
        let state = make_state();
        let arn = create_topic(&state, "my-topic").await;
        assert!(arn.contains("my-topic"));
    }

    #[tokio::test]
    async fn test_create_topic_idempotent() {
        let state = make_state();
        let arn1 = create_topic(&state, "t1").await;
        let arn2 = create_topic(&state, "t1").await;
        assert_eq!(arn1, arn2);
    }

    #[tokio::test]
    async fn test_delete_topic() {
        let state = make_state();
        let arn = create_topic(&state, "del").await;
        assert!(state.delete_topic(DeleteTopicRequest { topic_arn: arn }).await.is_ok());
    }

    #[tokio::test]
    async fn test_list_topics() {
        let state = make_state();
        create_topic(&state, "t1").await;
        create_topic(&state, "t2").await;
        let result = state.list_topics(ListTopicsRequest::default()).await.unwrap();
        assert_eq!(result.topics.len(), 2);
    }

    #[tokio::test]
    async fn test_get_topic_attributes() {
        let state = make_state();
        let arn = create_topic(&state, "t1").await;
        let result = state.get_topic_attributes(GetTopicAttributesRequest { topic_arn: arn }).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_topic_attributes_not_found() {
        let state = make_state();
        let req = GetTopicAttributesRequest { topic_arn: "arn:fake".to_string() };
        assert!(state.get_topic_attributes(req).await.is_err());
    }

    #[tokio::test]
    async fn test_subscribe_and_list() {
        let state = make_state();
        let arn = create_topic(&state, "t1").await;
        let req = SubscribeRequest {
            topic_arn: arn.clone(),
            protocol: "email".to_string(),
            endpoint: Some("test@example.com".to_string()),
            ..Default::default()
        };
        let result = state.subscribe(req).await;
        assert!(result.is_ok());

        let subs = state.list_subscriptions(ListSubscriptionsRequest::default()).await.unwrap();
        assert_eq!(subs.subscriptions.len(), 1);

        let by_topic = state.list_subscriptions_by_topic(ListSubscriptionsByTopicRequest { topic_arn: arn, ..Default::default() }).await.unwrap();
        assert_eq!(by_topic.subscriptions.len(), 1);
    }

    #[tokio::test]
    async fn test_publish() {
        let state = make_state();
        let arn = create_topic(&state, "t1").await;
        let req = PublishRequest {
            topic_arn: Some(arn),
            message: "hello".to_string(),
            ..Default::default()
        };
        let result = state.publish(req).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_publish_no_target() {
        let state = make_state();
        let req = PublishRequest { message: "hello".to_string(), ..Default::default() };
        assert!(state.publish(req).await.is_err());
    }

    #[tokio::test]
    async fn test_tag_and_list_tags() {
        let state = make_state();
        let arn = create_topic(&state, "tagged").await;
        state.tag_resource(TagResourceRequest {
            resource_arn: arn.clone(),
            tags: vec![TagJson { key: "env".to_string(), value: "test".to_string() }],
        }).await.unwrap();
        let result = state.list_tags_for_resource(ListTagsForResourceRequest { resource_arn: arn }).await.unwrap();
        assert_eq!(result.tags.len(), 1);
    }

    #[tokio::test]
    async fn test_set_topic_attributes() {
        let state = make_state();
        let arn = create_topic(&state, "t1").await;
        let req = SetTopicAttributesRequest {
            topic_arn: arn,
            attribute_name: "DisplayName".to_string(),
            attribute_value: Some("My Topic".to_string()),
        };
        assert!(state.set_topic_attributes(req).await.is_ok());
    }

    // --- Comprehensive additional tests ---

    #[tokio::test]
    async fn test_create_topic_empty_name() {
        let state = make_state();
        let result = state.create_topic(CreateTopicRequest {
            name: "".to_string(),
            ..Default::default()
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_fifo_topic() {
        let state = make_state();
        let mut attrs = HashMap::new();
        attrs.insert("FifoTopic".to_string(), "true".to_string());
        let result = state.create_topic(CreateTopicRequest {
            name: "my-topic.fifo".to_string(),
            attributes: Some(attrs),
            ..Default::default()
        }).await.unwrap();
        assert!(result.topic_arn.contains("my-topic.fifo"));
    }

    #[tokio::test]
    async fn test_create_fifo_topic_without_suffix() {
        let state = make_state();
        let mut attrs = HashMap::new();
        attrs.insert("FifoTopic".to_string(), "true".to_string());
        let result = state.create_topic(CreateTopicRequest {
            name: "no-suffix".to_string(),
            attributes: Some(attrs),
            ..Default::default()
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_topic_with_tags() {
        let state = make_state();
        let result = state.create_topic(CreateTopicRequest {
            name: "tagged-topic".to_string(),
            tags: Some(vec![TagJson { key: "env".to_string(), value: "test".to_string() }]),
            ..Default::default()
        }).await.unwrap();
        let tags = state.list_tags_for_resource(ListTagsForResourceRequest {
            resource_arn: result.topic_arn,
        }).await.unwrap();
        assert_eq!(tags.tags.len(), 1);
    }

    #[tokio::test]
    async fn test_set_topic_attributes_invalid() {
        let state = make_state();
        let arn = create_topic(&state, "t1").await;
        let result = state.set_topic_attributes(SetTopicAttributesRequest {
            topic_arn: arn,
            attribute_name: "InvalidAttributeName".to_string(),
            attribute_value: Some("val".to_string()),
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_topic_attributes_not_found() {
        let state = make_state();
        let result = state.set_topic_attributes(SetTopicAttributesRequest {
            topic_arn: "arn:fake".to_string(),
            attribute_name: "DisplayName".to_string(),
            attribute_value: Some("val".to_string()),
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_subscribe_not_found_topic() {
        let state = make_state();
        let result = state.subscribe(SubscribeRequest {
            topic_arn: "arn:fake".to_string(),
            protocol: "email".to_string(),
            endpoint: Some("test@example.com".to_string()),
            ..Default::default()
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_unsubscribe_success() {
        let state = make_state();
        let arn = create_topic(&state, "unsub-topic").await;
        let sub = state.subscribe(SubscribeRequest {
            topic_arn: arn,
            protocol: "email".to_string(),
            endpoint: Some("test@example.com".to_string()),
            ..Default::default()
        }).await.unwrap();

        let result = state.unsubscribe(UnsubscribeRequest {
            subscription_arn: sub.subscription_arn,
        }).await;
        assert!(result.is_ok());

        let subs = state.list_subscriptions(ListSubscriptionsRequest::default()).await.unwrap();
        assert!(subs.subscriptions.is_empty());
    }

    #[tokio::test]
    async fn test_unsubscribe_not_found() {
        let state = make_state();
        let result = state.unsubscribe(UnsubscribeRequest {
            subscription_arn: "arn:fake:sub".to_string(),
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_confirm_subscription() {
        let state = make_state();
        let arn = create_topic(&state, "confirm-topic").await;
        state.subscribe(SubscribeRequest {
            topic_arn: arn.clone(),
            protocol: "email".to_string(),
            endpoint: Some("test@example.com".to_string()),
            ..Default::default()
        }).await.unwrap();

        let result = state.confirm_subscription(ConfirmSubscriptionRequest {
            topic_arn: arn,
            _token: "token123".to_string(),
            ..Default::default()
        }).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_confirm_subscription_no_subs() {
        let state = make_state();
        let arn = create_topic(&state, "empty-confirm").await;
        let result = state.confirm_subscription(ConfirmSubscriptionRequest {
            topic_arn: arn,
            _token: "token".to_string(),
            ..Default::default()
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_confirm_subscription_topic_not_found() {
        let state = make_state();
        let result = state.confirm_subscription(ConfirmSubscriptionRequest {
            topic_arn: "arn:fake".to_string(),
            _token: "token".to_string(),
            ..Default::default()
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_subscription_attributes() {
        let state = make_state();
        let arn = create_topic(&state, "sub-attrs").await;
        let sub = state.subscribe(SubscribeRequest {
            topic_arn: arn,
            protocol: "sqs".to_string(),
            endpoint: Some("arn:aws:sqs:us-east-1:123456789012:my-queue".to_string()),
            ..Default::default()
        }).await.unwrap();

        let result = state.get_subscription_attributes(GetSubscriptionAttributesRequest {
            subscription_arn: sub.subscription_arn,
        }).await;
        assert!(result.is_ok());
        let attrs = result.unwrap().attributes;
        assert!(attrs.contains_key("Protocol"));
    }

    #[tokio::test]
    async fn test_get_subscription_attributes_not_found() {
        let state = make_state();
        let result = state.get_subscription_attributes(GetSubscriptionAttributesRequest {
            subscription_arn: "arn:fake".to_string(),
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_subscription_attributes() {
        let state = make_state();
        let arn = create_topic(&state, "set-sub").await;
        let sub = state.subscribe(SubscribeRequest {
            topic_arn: arn,
            protocol: "sqs".to_string(),
            endpoint: Some("arn:aws:sqs:us-east-1:123456789012:my-queue".to_string()),
            ..Default::default()
        }).await.unwrap();

        let result = state.set_subscription_attributes(SetSubscriptionAttributesRequest {
            subscription_arn: sub.subscription_arn,
            attribute_name: "RawMessageDelivery".to_string(),
            attribute_value: Some("true".to_string()),
        }).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_set_subscription_attributes_invalid() {
        let state = make_state();
        let arn = create_topic(&state, "inv-sub").await;
        let sub = state.subscribe(SubscribeRequest {
            topic_arn: arn,
            protocol: "sqs".to_string(),
            endpoint: Some("arn:aws:sqs:us-east-1:123456789012:q".to_string()),
            ..Default::default()
        }).await.unwrap();

        let result = state.set_subscription_attributes(SetSubscriptionAttributesRequest {
            subscription_arn: sub.subscription_arn,
            attribute_name: "InvalidAttr".to_string(),
            attribute_value: Some("val".to_string()),
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_subscription_attributes_not_found() {
        let state = make_state();
        let result = state.set_subscription_attributes(SetSubscriptionAttributesRequest {
            subscription_arn: "arn:fake".to_string(),
            attribute_name: "RawMessageDelivery".to_string(),
            attribute_value: Some("true".to_string()),
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_publish_empty_message() {
        let state = make_state();
        let arn = create_topic(&state, "empty-msg").await;
        let result = state.publish(PublishRequest {
            topic_arn: Some(arn),
            message: "".to_string(),
            ..Default::default()
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_publish_topic_not_found() {
        let state = make_state();
        let result = state.publish(PublishRequest {
            topic_arn: Some("arn:fake".to_string()),
            message: "hello".to_string(),
            ..Default::default()
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_publish_fifo_requires_group_id() {
        let state = make_state();
        let mut attrs = HashMap::new();
        attrs.insert("FifoTopic".to_string(), "true".to_string());
        let resp = state.create_topic(CreateTopicRequest {
            name: "fifo-pub.fifo".to_string(),
            attributes: Some(attrs),
            ..Default::default()
        }).await.unwrap();

        let result = state.publish(PublishRequest {
            topic_arn: Some(resp.topic_arn.clone()),
            message: "hello".to_string(),
            message_group_id: None,
            ..Default::default()
        }).await;
        assert!(result.is_err());

        // With group_id should succeed
        let result = state.publish(PublishRequest {
            topic_arn: Some(resp.topic_arn),
            message: "hello".to_string(),
            message_group_id: Some("group1".to_string()),
            ..Default::default()
        }).await;
        assert!(result.is_ok());
        assert!(result.unwrap().sequence_number.is_some());
    }

    #[tokio::test]
    async fn test_publish_batch_success() {
        let state = make_state();
        let arn = create_topic(&state, "batch-pub").await;
        let result = state.publish_batch(PublishBatchRequest {
            topic_arn: arn,
            publish_batch_request_entries: vec![
                PublishBatchEntry {
                    id: "1".to_string(),
                    message: "msg1".to_string(),
                    _subject: None,
                    _message_attributes: None,
                    _message_deduplication_id: None,
                    message_group_id: None,
                },
                PublishBatchEntry {
                    id: "2".to_string(),
                    message: "msg2".to_string(),
                    _subject: None,
                    _message_attributes: None,
                    _message_deduplication_id: None,
                    message_group_id: None,
                },
            ],
        }).await.unwrap();
        assert_eq!(result.successful.len(), 2);
        assert!(result.failed.is_empty());
    }

    #[tokio::test]
    async fn test_publish_batch_empty() {
        let state = make_state();
        let arn = create_topic(&state, "batch-empty").await;
        let result = state.publish_batch(PublishBatchRequest {
            topic_arn: arn,
            publish_batch_request_entries: vec![],
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_publish_batch_too_many() {
        let state = make_state();
        let arn = create_topic(&state, "batch-many").await;
        let entries: Vec<PublishBatchEntry> = (0..11).map(|i| PublishBatchEntry {
            id: format!("{}", i),
            message: format!("msg{}", i),
            _subject: None,
            _message_attributes: None,
            _message_deduplication_id: None,
            message_group_id: None,
        }).collect();
        let result = state.publish_batch(PublishBatchRequest {
            topic_arn: arn,
            publish_batch_request_entries: entries,
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_publish_batch_with_empty_message() {
        let state = make_state();
        let arn = create_topic(&state, "batch-fail").await;
        let result = state.publish_batch(PublishBatchRequest {
            topic_arn: arn,
            publish_batch_request_entries: vec![
                PublishBatchEntry {
                    id: "ok".to_string(),
                    message: "valid".to_string(),
                    _subject: None,
                    _message_attributes: None,
                    _message_deduplication_id: None,
                    message_group_id: None,
                },
                PublishBatchEntry {
                    id: "bad".to_string(),
                    message: "".to_string(),
                    _subject: None,
                    _message_attributes: None,
                    _message_deduplication_id: None,
                    message_group_id: None,
                },
            ],
        }).await.unwrap();
        assert_eq!(result.successful.len(), 1);
        assert_eq!(result.failed.len(), 1);
        assert_eq!(result.failed[0].id, "bad");
    }

    #[tokio::test]
    async fn test_publish_batch_topic_not_found() {
        let state = make_state();
        let result = state.publish_batch(PublishBatchRequest {
            topic_arn: "arn:fake".to_string(),
            publish_batch_request_entries: vec![PublishBatchEntry {
                id: "1".to_string(),
                message: "msg".to_string(),
                _subject: None,
                _message_attributes: None,
                _message_deduplication_id: None,
                message_group_id: None,
            }],
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_untag_resource_success() {
        let state = make_state();
        let arn = create_topic(&state, "untag-topic").await;
        state.tag_resource(TagResourceRequest {
            resource_arn: arn.clone(),
            tags: vec![
                TagJson { key: "a".to_string(), value: "1".to_string() },
                TagJson { key: "b".to_string(), value: "2".to_string() },
            ],
        }).await.unwrap();

        state.untag_resource(UntagResourceRequest {
            resource_arn: arn.clone(),
            tag_keys: vec!["a".to_string()],
        }).await.unwrap();

        let tags = state.list_tags_for_resource(ListTagsForResourceRequest {
            resource_arn: arn,
        }).await.unwrap();
        assert_eq!(tags.tags.len(), 1);
        assert_eq!(tags.tags[0].key, "b");
    }

    #[tokio::test]
    async fn test_list_tags_for_resource_not_found() {
        let state = make_state();
        let result = state.list_tags_for_resource(ListTagsForResourceRequest {
            resource_arn: "arn:fake".to_string(),
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_subscriptions_by_topic_not_found() {
        let state = make_state();
        let result = state.list_subscriptions_by_topic(ListSubscriptionsByTopicRequest {
            topic_arn: "arn:fake".to_string(),
            ..Default::default()
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_subscribe_with_attributes() {
        let state = make_state();
        let arn = create_topic(&state, "sub-attrs-topic").await;
        let mut attrs = HashMap::new();
        attrs.insert("RawMessageDelivery".to_string(), "true".to_string());
        attrs.insert("FilterPolicy".to_string(), r#"{"color":["red"]}"#.to_string());
        let sub = state.subscribe(SubscribeRequest {
            topic_arn: arn,
            protocol: "sqs".to_string(),
            endpoint: Some("arn:aws:sqs:us-east-1:123456789012:q".to_string()),
            attributes: Some(attrs),
            ..Default::default()
        }).await.unwrap();

        let sub_attrs = state.get_subscription_attributes(GetSubscriptionAttributesRequest {
            subscription_arn: sub.subscription_arn,
        }).await.unwrap();
        assert_eq!(sub_attrs.attributes.get("RawMessageDelivery").unwrap(), "true");
    }

    #[tokio::test]
    async fn test_publish_with_target_arn() {
        let state = make_state();
        let arn = create_topic(&state, "target-topic").await;
        let result = state.publish(PublishRequest {
            target_arn: Some(arn),
            message: "hello".to_string(),
            ..Default::default()
        }).await;
        assert!(result.is_ok());
    }
}
