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

    #[allow(dead_code)]
    fn resolve_topic_name(arn: &str) -> String {
        arn.split(':').last().unwrap_or("").to_string()
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
