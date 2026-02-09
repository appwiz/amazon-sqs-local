use std::collections::HashMap;

use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct TopicAttributes {
    pub display_name: String,
    pub policy: String,
    pub delivery_policy: String,
    pub kms_master_key_id: Option<String>,
    pub fifo_topic: bool,
    pub content_based_deduplication: bool,
}

impl Default for TopicAttributes {
    fn default() -> Self {
        TopicAttributes {
            display_name: String::new(),
            policy: String::new(),
            delivery_policy: String::new(),
            kms_master_key_id: None,
            fifo_topic: false,
            content_based_deduplication: false,
        }
    }
}

impl TopicAttributes {
    pub fn to_map(&self, topic: &Topic) -> HashMap<String, String> {
        let mut m = HashMap::new();
        m.insert("TopicArn".into(), topic.arn.clone());
        m.insert("DisplayName".into(), self.display_name.clone());
        m.insert("Owner".into(), topic.owner.clone());
        m.insert(
            "SubscriptionsConfirmed".into(),
            topic
                .subscriptions
                .values()
                .filter(|s| s.confirmed)
                .count()
                .to_string(),
        );
        m.insert(
            "SubscriptionsPending".into(),
            topic
                .subscriptions
                .values()
                .filter(|s| !s.confirmed)
                .count()
                .to_string(),
        );
        m.insert("SubscriptionsDeleted".into(), "0".into());
        if !self.policy.is_empty() {
            m.insert("Policy".into(), self.policy.clone());
        }
        if !self.delivery_policy.is_empty() {
            m.insert("DeliveryPolicy".into(), self.delivery_policy.clone());
        }
        if let Some(ref key) = self.kms_master_key_id {
            m.insert("KmsMasterKeyId".into(), key.clone());
        }
        m.insert("FifoTopic".into(), self.fifo_topic.to_string());
        if self.fifo_topic {
            m.insert(
                "ContentBasedDeduplication".into(),
                self.content_based_deduplication.to_string(),
            );
        }
        m.insert(
            "EffectiveDeliveryPolicy".into(),
            self.delivery_policy.clone(),
        );
        m
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Topic {
    pub name: String,
    pub arn: String,
    pub owner: String,
    pub attributes: TopicAttributes,
    pub subscriptions: HashMap<String, Subscription>,
    pub tags: HashMap<String, String>,
}

impl Topic {
    pub fn new(name: String, arn: String, owner: String, is_fifo: bool) -> Self {
        let mut attrs = TopicAttributes::default();
        attrs.fifo_topic = is_fifo;
        Topic {
            name,
            arn,
            owner,
            attributes: attrs,
            subscriptions: HashMap::new(),
            tags: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SubscriptionAttributes {
    pub raw_message_delivery: bool,
    pub filter_policy: Option<String>,
    pub filter_policy_scope: String,
    pub redrive_policy: Option<String>,
}

impl Default for SubscriptionAttributes {
    fn default() -> Self {
        SubscriptionAttributes {
            raw_message_delivery: false,
            filter_policy: None,
            filter_policy_scope: "MessageAttributes".into(),
            redrive_policy: None,
        }
    }
}

impl SubscriptionAttributes {
    pub fn to_map(&self, sub: &Subscription) -> HashMap<String, String> {
        let mut m = HashMap::new();
        m.insert("SubscriptionArn".into(), sub.arn.clone());
        m.insert("TopicArn".into(), sub.topic_arn.clone());
        m.insert("Protocol".into(), sub.protocol.clone());
        m.insert("Endpoint".into(), sub.endpoint.clone());
        m.insert("Owner".into(), sub.owner.clone());
        m.insert(
            "ConfirmationWasAuthenticated".into(),
            sub.confirmed.to_string(),
        );
        m.insert(
            "RawMessageDelivery".into(),
            self.raw_message_delivery.to_string(),
        );
        if let Some(ref fp) = self.filter_policy {
            m.insert("FilterPolicy".into(), fp.clone());
        }
        m.insert("FilterPolicyScope".into(), self.filter_policy_scope.clone());
        if let Some(ref rp) = self.redrive_policy {
            m.insert("RedrivePolicy".into(), rp.clone());
        }
        m.insert(
            "PendingConfirmation".into(),
            (!sub.confirmed).to_string(),
        );
        m
    }
}

#[derive(Debug, Clone)]
pub struct Subscription {
    pub arn: String,
    pub topic_arn: String,
    pub protocol: String,
    pub endpoint: String,
    pub owner: String,
    pub confirmed: bool,
    pub attributes: SubscriptionAttributes,
}

impl Subscription {
    pub fn new(
        topic_arn: String,
        protocol: String,
        endpoint: String,
        owner: String,
        region: &str,
        account_id: &str,
    ) -> Self {
        let sub_id = Uuid::new_v4().to_string();
        let arn = format!(
            "arn:aws:sns:{}:{}:{}:{}",
            region,
            account_id,
            topic_arn.split(':').last().unwrap_or(""),
            sub_id
        );
        Subscription {
            arn,
            topic_arn,
            protocol,
            endpoint,
            owner,
            confirmed: true, // Auto-confirm for local
            attributes: SubscriptionAttributes::default(),
        }
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct PublishedMessage {
    pub message_id: String,
    pub topic_arn: String,
    pub message: String,
    pub subject: Option<String>,
    pub message_attributes: HashMap<String, MessageAttributeValue>,
    pub timestamp: u64,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct MessageAttributeValue {
    pub data_type: String,
    pub string_value: Option<String>,
    pub binary_value: Option<String>,
}
