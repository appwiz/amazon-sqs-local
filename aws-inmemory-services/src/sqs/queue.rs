use std::collections::{HashMap, HashSet, VecDeque};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use md5::{Digest as Md5Digest, Md5};
use sha2::Sha256;
use uuid::Uuid;

use super::error::SqsError;
use super::types::{MessageAttributeValue, ReceiveMessageResult, SendMessageResponse};

fn now_millis() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}

fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn md5_hex(data: &[u8]) -> String {
    let mut hasher = Md5::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

fn sha256_hex(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

pub fn compute_md5_of_attributes(attrs: &HashMap<String, MessageAttributeValue>) -> Option<String> {
    if attrs.is_empty() {
        return None;
    }
    let mut keys: Vec<&String> = attrs.keys().collect();
    keys.sort();

    let mut buf: Vec<u8> = Vec::new();
    for key in keys {
        let attr = &attrs[key];
        // Encode name
        let name_bytes = key.as_bytes();
        buf.extend_from_slice(&(name_bytes.len() as u32).to_be_bytes());
        buf.extend_from_slice(name_bytes);

        // Encode data type
        let dt_bytes = attr.data_type.as_bytes();
        buf.extend_from_slice(&(dt_bytes.len() as u32).to_be_bytes());
        buf.extend_from_slice(dt_bytes);

        // Transport type: 1 for String/Number, 2 for Binary
        if attr.data_type.starts_with("Binary") {
            buf.push(2);
            if let Some(ref bv) = attr.binary_value {
                let decoded = base64::Engine::decode(
                    &base64::engine::general_purpose::STANDARD,
                    bv,
                )
                .unwrap_or_default();
                buf.extend_from_slice(&(decoded.len() as u32).to_be_bytes());
                buf.extend_from_slice(&decoded);
            }
        } else {
            buf.push(1);
            if let Some(ref sv) = attr.string_value {
                let sv_bytes = sv.as_bytes();
                buf.extend_from_slice(&(sv_bytes.len() as u32).to_be_bytes());
                buf.extend_from_slice(sv_bytes);
            }
        }
    }

    Some(md5_hex(&buf))
}

#[derive(Debug, Clone)]
pub struct RedrivePolicy {
    pub dead_letter_target_arn: String,
    pub max_receive_count: u32,
}

impl RedrivePolicy {
    pub fn from_json(s: &str) -> Result<Self, SqsError> {
        let v: serde_json::Value = serde_json::from_str(s)
            .map_err(|e| SqsError::InvalidAttributeValue(format!("Invalid RedrivePolicy JSON: {e}")))?;
        let arn = v
            .get("deadLetterTargetArn")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                SqsError::InvalidAttributeValue(
                    "RedrivePolicy must contain deadLetterTargetArn".into(),
                )
            })?
            .to_string();
        let max_count = v
            .get("maxReceiveCount")
            .and_then(|v| v.as_u64().or_else(|| v.as_str().and_then(|s| s.parse().ok())))
            .ok_or_else(|| {
                SqsError::InvalidAttributeValue(
                    "RedrivePolicy must contain maxReceiveCount".into(),
                )
            })? as u32;
        if max_count < 1 {
            return Err(SqsError::InvalidAttributeValue(
                "maxReceiveCount must be at least 1".into(),
            ));
        }
        Ok(RedrivePolicy {
            dead_letter_target_arn: arn,
            max_receive_count: max_count,
        })
    }

    pub fn to_json(&self) -> String {
        serde_json::json!({
            "deadLetterTargetArn": self.dead_letter_target_arn,
            "maxReceiveCount": self.max_receive_count,
        })
        .to_string()
    }
}

#[derive(Debug, Clone)]
pub struct RedriveAllowPolicy {
    pub redrive_permission: String,
    pub source_queue_arns: Option<Vec<String>>,
}

impl RedriveAllowPolicy {
    pub fn from_json(s: &str) -> Result<Self, SqsError> {
        let v: serde_json::Value = serde_json::from_str(s)
            .map_err(|e| SqsError::InvalidAttributeValue(format!("Invalid RedriveAllowPolicy JSON: {e}")))?;
        let perm = v
            .get("redrivePermission")
            .and_then(|v| v.as_str())
            .unwrap_or("allowAll")
            .to_string();
        let arns = v.get("sourceQueueArns").and_then(|v| {
            v.as_array().map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
        });
        Ok(RedriveAllowPolicy {
            redrive_permission: perm,
            source_queue_arns: arns,
        })
    }

    pub fn to_json(&self) -> String {
        let mut m = serde_json::Map::new();
        m.insert(
            "redrivePermission".into(),
            serde_json::Value::String(self.redrive_permission.clone()),
        );
        if let Some(ref arns) = self.source_queue_arns {
            m.insert(
                "sourceQueueArns".into(),
                serde_json::Value::Array(arns.iter().map(|a| serde_json::Value::String(a.clone())).collect()),
            );
        }
        serde_json::Value::Object(m).to_string()
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Permission {
    pub label: String,
    pub aws_account_ids: Vec<String>,
    pub actions: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct QueueAttributes {
    pub visibility_timeout: u32,
    pub message_retention_period: u32,
    pub delay_seconds: u32,
    pub maximum_message_size: u32,
    pub receive_message_wait_time_seconds: u32,
    pub redrive_policy: Option<RedrivePolicy>,
    pub redrive_allow_policy: Option<RedriveAllowPolicy>,
    pub fifo_queue: bool,
    pub content_based_deduplication: bool,
    pub deduplication_scope: String,
    pub fifo_throughput_limit: String,
    pub sqs_managed_sse_enabled: bool,
    pub kms_master_key_id: Option<String>,
    pub kms_data_key_reuse_period_seconds: u32,
}

impl Default for QueueAttributes {
    fn default() -> Self {
        QueueAttributes {
            visibility_timeout: 30,
            message_retention_period: 345600,
            delay_seconds: 0,
            maximum_message_size: 262144,
            receive_message_wait_time_seconds: 0,
            redrive_policy: None,
            redrive_allow_policy: None,
            fifo_queue: false,
            content_based_deduplication: false,
            deduplication_scope: "Queue".into(),
            fifo_throughput_limit: "PerQueue".into(),
            sqs_managed_sse_enabled: true,
            kms_master_key_id: None,
            kms_data_key_reuse_period_seconds: 300,
        }
    }
}

impl QueueAttributes {
    pub fn to_map(&self) -> HashMap<String, String> {
        let mut m = HashMap::new();
        m.insert("VisibilityTimeout".into(), self.visibility_timeout.to_string());
        m.insert("MessageRetentionPeriod".into(), self.message_retention_period.to_string());
        m.insert("DelaySeconds".into(), self.delay_seconds.to_string());
        m.insert("MaximumMessageSize".into(), self.maximum_message_size.to_string());
        m.insert("ReceiveMessageWaitTimeSeconds".into(), self.receive_message_wait_time_seconds.to_string());
        if let Some(ref rp) = self.redrive_policy {
            m.insert("RedrivePolicy".into(), rp.to_json());
        }
        if let Some(ref rap) = self.redrive_allow_policy {
            m.insert("RedriveAllowPolicy".into(), rap.to_json());
        }
        m.insert("FifoQueue".into(), self.fifo_queue.to_string());
        if self.fifo_queue {
            m.insert("ContentBasedDeduplication".into(), self.content_based_deduplication.to_string());
            m.insert("DeduplicationScope".into(), self.deduplication_scope.clone());
            m.insert("FifoThroughputLimit".into(), self.fifo_throughput_limit.clone());
        }
        m.insert("SqsManagedSseEnabled".into(), self.sqs_managed_sse_enabled.to_string());
        if let Some(ref key) = self.kms_master_key_id {
            m.insert("KmsMasterKeyId".into(), key.clone());
        }
        m.insert("KmsDataKeyReusePeriodSeconds".into(), self.kms_data_key_reuse_period_seconds.to_string());
        m
    }

    pub fn apply(&mut self, attrs: &HashMap<String, String>) -> Result<(), SqsError> {
        for (key, value) in attrs {
            match key.as_str() {
                "VisibilityTimeout" => {
                    let v: u32 = value.parse().map_err(|_| {
                        SqsError::InvalidAttributeValue(format!("Invalid VisibilityTimeout: {value}"))
                    })?;
                    if v > 43200 {
                        return Err(SqsError::InvalidAttributeValue(
                            "VisibilityTimeout must be between 0 and 43200".into(),
                        ));
                    }
                    self.visibility_timeout = v;
                }
                "MessageRetentionPeriod" => {
                    let v: u32 = value.parse().map_err(|_| {
                        SqsError::InvalidAttributeValue(format!("Invalid MessageRetentionPeriod: {value}"))
                    })?;
                    if !(60..=1209600).contains(&v) {
                        return Err(SqsError::InvalidAttributeValue(
                            "MessageRetentionPeriod must be between 60 and 1209600".into(),
                        ));
                    }
                    self.message_retention_period = v;
                }
                "DelaySeconds" => {
                    let v: u32 = value.parse().map_err(|_| {
                        SqsError::InvalidAttributeValue(format!("Invalid DelaySeconds: {value}"))
                    })?;
                    if v > 900 {
                        return Err(SqsError::InvalidAttributeValue(
                            "DelaySeconds must be between 0 and 900".into(),
                        ));
                    }
                    self.delay_seconds = v;
                }
                "MaximumMessageSize" => {
                    let v: u32 = value.parse().map_err(|_| {
                        SqsError::InvalidAttributeValue(format!("Invalid MaximumMessageSize: {value}"))
                    })?;
                    if !(1024..=262144).contains(&v) {
                        return Err(SqsError::InvalidAttributeValue(
                            "MaximumMessageSize must be between 1024 and 262144".into(),
                        ));
                    }
                    self.maximum_message_size = v;
                }
                "ReceiveMessageWaitTimeSeconds" => {
                    let v: u32 = value.parse().map_err(|_| {
                        SqsError::InvalidAttributeValue(format!(
                            "Invalid ReceiveMessageWaitTimeSeconds: {value}"
                        ))
                    })?;
                    if v > 20 {
                        return Err(SqsError::InvalidAttributeValue(
                            "ReceiveMessageWaitTimeSeconds must be between 0 and 20".into(),
                        ));
                    }
                    self.receive_message_wait_time_seconds = v;
                }
                "RedrivePolicy" => {
                    if value.is_empty() {
                        self.redrive_policy = None;
                    } else {
                        self.redrive_policy = Some(RedrivePolicy::from_json(value)?);
                    }
                }
                "RedriveAllowPolicy" => {
                    if value.is_empty() {
                        self.redrive_allow_policy = None;
                    } else {
                        self.redrive_allow_policy = Some(RedriveAllowPolicy::from_json(value)?);
                    }
                }
                "FifoQueue" => {
                    // Immutable after creation — only allowed at create time
                    self.fifo_queue = value == "true";
                }
                "ContentBasedDeduplication" => {
                    self.content_based_deduplication = value == "true";
                }
                "DeduplicationScope" => {
                    if value != "Queue" && value != "MessageGroup" {
                        return Err(SqsError::InvalidAttributeValue(
                            "DeduplicationScope must be Queue or MessageGroup".into(),
                        ));
                    }
                    self.deduplication_scope = value.clone();
                }
                "FifoThroughputLimit" => {
                    if value != "PerQueue" && value != "PerMessageGroupId" {
                        return Err(SqsError::InvalidAttributeValue(
                            "FifoThroughputLimit must be PerQueue or PerMessageGroupId".into(),
                        ));
                    }
                    self.fifo_throughput_limit = value.clone();
                }
                "SqsManagedSseEnabled" => {
                    self.sqs_managed_sse_enabled = value == "true";
                }
                "KmsMasterKeyId" => {
                    self.kms_master_key_id = if value.is_empty() {
                        None
                    } else {
                        Some(value.clone())
                    };
                }
                "KmsDataKeyReusePeriodSeconds" => {
                    let v: u32 = value.parse().map_err(|_| {
                        SqsError::InvalidAttributeValue(format!(
                            "Invalid KmsDataKeyReusePeriodSeconds: {value}"
                        ))
                    })?;
                    if !(60..=86400).contains(&v) {
                        return Err(SqsError::InvalidAttributeValue(
                            "KmsDataKeyReusePeriodSeconds must be between 60 and 86400".into(),
                        ));
                    }
                    self.kms_data_key_reuse_period_seconds = v;
                }
                _ => {
                    return Err(SqsError::InvalidAttributeName(format!(
                        "Unknown attribute: {key}"
                    )));
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Message {
    pub message_id: String,
    pub body: String,
    pub md5_of_body: String,
    pub message_attributes: HashMap<String, MessageAttributeValue>,
    pub md5_of_message_attributes: Option<String>,
    pub system_attributes: HashMap<String, MessageAttributeValue>,
    pub md5_of_system_attributes: Option<String>,
    pub sent_timestamp: u64,
    pub visible_at: Instant,
    pub receive_count: u32,
    pub first_receive_timestamp: Option<u64>,
    pub receipt_handle: Option<String>,
    pub visibility_deadline: Option<Instant>,
    pub message_group_id: Option<String>,
    pub dedup_id: Option<String>,
    pub sequence_number: Option<String>,
    pub sender_id: String,
}

/// Information about a message that needs to be moved to a DLQ.
pub struct DlqRedrive {
    pub message: Message,
    pub dlq_arn: String,
}

#[allow(dead_code)]
pub struct Queue {
    pub name: String,
    pub arn: String,
    pub url: String,
    pub attributes: QueueAttributes,
    pub messages: VecDeque<Message>,
    pub inflight: HashMap<String, Message>,
    pub tags: HashMap<String, String>,
    pub permissions: HashMap<String, Permission>,
    pub created_at: u64,
    pub last_modified: u64,
    last_purge: Option<Instant>,
    // FIFO state
    dedup_cache: HashMap<String, (SendMessageResponse, Instant)>,
    sequence_counter: u64,
    locked_groups: HashSet<String>,
}

impl Queue {
    pub fn new(name: String, arn: String, url: String, attributes: QueueAttributes) -> Self {
        let now = now_secs();
        Queue {
            name,
            arn,
            url,
            attributes,
            messages: VecDeque::new(),
            inflight: HashMap::new(),
            tags: HashMap::new(),
            permissions: HashMap::new(),
            created_at: now,
            last_modified: now,
            last_purge: None,
            dedup_cache: HashMap::new(),
            sequence_counter: 0,
            locked_groups: HashSet::new(),
        }
    }

    pub fn send_message(
        &mut self,
        body: String,
        delay_seconds: Option<i32>,
        msg_attrs: Option<HashMap<String, MessageAttributeValue>>,
        sys_attrs: Option<HashMap<String, MessageAttributeValue>>,
        dedup_id: Option<String>,
        group_id: Option<String>,
        account_id: &str,
    ) -> Result<SendMessageResponse, SqsError> {
        // Validate body size
        if body.len() > self.attributes.maximum_message_size as usize {
            return Err(SqsError::InvalidParameterValue(format!(
                "Message body must be shorter than {} bytes",
                self.attributes.maximum_message_size
            )));
        }
        if body.is_empty() {
            return Err(SqsError::InvalidParameterValue(
                "Message body must not be empty".into(),
            ));
        }

        // FIFO checks
        if self.attributes.fifo_queue {
            if group_id.is_none() {
                return Err(SqsError::MissingParameter(
                    "MessageGroupId is required for FIFO queues".into(),
                ));
            }
        }

        // Compute dedup ID for FIFO
        let effective_dedup_id = if self.attributes.fifo_queue {
            if let Some(id) = dedup_id.clone() {
                Some(id)
            } else if self.attributes.content_based_deduplication {
                Some(sha256_hex(body.as_bytes()))
            } else {
                return Err(SqsError::InvalidParameterValue(
                    "MessageDeduplicationId is required for FIFO queues without ContentBasedDeduplication".into(),
                ));
            }
        } else {
            dedup_id.clone()
        };

        // Check dedup cache (lazy clean)
        self.clean_dedup_cache();
        if let Some(ref did) = effective_dedup_id {
            if let Some((prev_response, ts)) = self.dedup_cache.get(did) {
                if ts.elapsed() < Duration::from_secs(300) {
                    return Ok(prev_response.clone());
                }
            }
        }

        let delay = delay_seconds.unwrap_or(self.attributes.delay_seconds as i32) as u64;
        let message_id = Uuid::new_v4().to_string();
        let md5_of_body = md5_hex(body.as_bytes());
        let msg_attrs = msg_attrs.unwrap_or_default();
        let sys_attrs = sys_attrs.unwrap_or_default();
        let md5_of_msg_attrs = compute_md5_of_attributes(&msg_attrs);
        let md5_of_sys_attrs = compute_md5_of_attributes(&sys_attrs);

        let sequence_number = if self.attributes.fifo_queue {
            self.sequence_counter += 1;
            Some(format!("{:020}", self.sequence_counter))
        } else {
            None
        };

        let now = Instant::now();
        let msg = Message {
            message_id: message_id.clone(),
            body,
            md5_of_body: md5_of_body.clone(),
            message_attributes: msg_attrs,
            md5_of_message_attributes: md5_of_msg_attrs.clone(),
            system_attributes: sys_attrs,
            md5_of_system_attributes: md5_of_sys_attrs.clone(),
            sent_timestamp: now_millis(),
            visible_at: now + Duration::from_secs(delay),
            receive_count: 0,
            first_receive_timestamp: None,
            receipt_handle: None,
            visibility_deadline: None,
            message_group_id: group_id,
            dedup_id: effective_dedup_id.clone(),
            sequence_number: sequence_number.clone(),
            sender_id: account_id.to_string(),
        };

        self.messages.push_back(msg);

        let response = SendMessageResponse {
            message_id,
            md5_of_message_body: md5_of_body,
            md5_of_message_attributes: md5_of_msg_attrs,
            md5_of_message_system_attributes: md5_of_sys_attrs,
            sequence_number,
        };

        if let Some(ref did) = effective_dedup_id {
            self.dedup_cache
                .insert(did.clone(), (response.clone(), Instant::now()));
        }

        Ok(response)
    }

    /// Returns messages that expired from inflight along with DLQ info if applicable.
    pub fn return_expired_inflight(&mut self) -> Vec<DlqRedrive> {
        let now = Instant::now();
        let mut expired_handles: Vec<String> = Vec::new();

        for (handle, msg) in &self.inflight {
            if let Some(deadline) = msg.visibility_deadline {
                if now >= deadline {
                    expired_handles.push(handle.clone());
                }
            }
        }

        let mut dlq_redrives = Vec::new();

        for handle in expired_handles {
            if let Some(mut msg) = self.inflight.remove(&handle) {
                // Unlock FIFO group
                if let Some(ref gid) = msg.message_group_id {
                    self.locked_groups.remove(gid);
                }

                msg.receipt_handle = None;
                msg.visibility_deadline = None;

                // Check redrive policy
                if let Some(ref rp) = self.attributes.redrive_policy {
                    if msg.receive_count >= rp.max_receive_count {
                        dlq_redrives.push(DlqRedrive {
                            dlq_arn: rp.dead_letter_target_arn.clone(),
                            message: msg,
                        });
                        continue;
                    }
                }

                self.messages.push_back(msg);
            }
        }

        dlq_redrives
    }

    pub fn receive_messages(
        &mut self,
        max_count: i32,
        visibility_timeout: Option<i32>,
        account_id: &str,
    ) -> Result<Vec<ReceiveMessageResult>, SqsError> {
        let vis_timeout = visibility_timeout.unwrap_or(self.attributes.visibility_timeout as i32);
        let now = Instant::now();
        let now_ms = now_millis();
        let retention_deadline = now_ms
            .saturating_sub(self.attributes.message_retention_period as u64 * 1000);

        // Check inflight limits
        let inflight_limit: usize = if self.attributes.fifo_queue {
            20_000
        } else {
            120_000
        };
        if self.inflight.len() >= inflight_limit {
            return Err(SqsError::OverLimit(
                "Too many inflight messages".into(),
            ));
        }

        let mut results = Vec::new();
        let mut indices_to_remove: Vec<usize> = Vec::new();

        if self.attributes.fifo_queue {
            // FIFO: deliver in order, one per group, skip locked groups
            let mut seen_groups: HashSet<String> = HashSet::new();
            for (i, msg) in self.messages.iter().enumerate() {
                if results.len() >= max_count as usize {
                    break;
                }
                if msg.visible_at > now {
                    continue;
                }
                // Skip expired messages
                if msg.sent_timestamp < retention_deadline {
                    indices_to_remove.push(i);
                    continue;
                }
                if let Some(ref gid) = msg.message_group_id {
                    if self.locked_groups.contains(gid) || seen_groups.contains(gid) {
                        continue;
                    }
                    seen_groups.insert(gid.clone());
                }
                indices_to_remove.push(i);
                results.push(i);
            }
        } else {
            // Standard queue: take visible messages
            for (i, msg) in self.messages.iter().enumerate() {
                if results.len() >= max_count as usize {
                    break;
                }
                if msg.visible_at > now {
                    continue;
                }
                if msg.sent_timestamp < retention_deadline {
                    indices_to_remove.push(i);
                    continue;
                }
                indices_to_remove.push(i);
                results.push(i);
            }
        }

        // Collect messages to move to inflight (the ones in results)
        let result_set: HashSet<usize> = results.iter().copied().collect();

        // Remove from deque in reverse order
        let all_remove: Vec<usize> = indices_to_remove;
        let mut removed_indices: Vec<usize> = all_remove.clone();
        removed_indices.sort_unstable();
        removed_indices.dedup();

        // Extract messages by index
        let mut extracted: HashMap<usize, Message> = HashMap::new();
        for &idx in removed_indices.iter().rev() {
            if let Some(msg) = self.messages.remove(idx) {
                extracted.insert(idx, msg);
            }
        }

        let mut receive_results = Vec::new();
        for &idx in &all_remove {
            if let Some(mut msg) = extracted.remove(&idx) {
                if result_set.contains(&idx) {
                    // This is a received message
                    msg.receive_count += 1;
                    if msg.first_receive_timestamp.is_none() {
                        msg.first_receive_timestamp = Some(now_ms);
                    }
                    let receipt_handle = Uuid::new_v4().to_string();
                    msg.receipt_handle = Some(receipt_handle.clone());
                    msg.visibility_deadline =
                        Some(now + Duration::from_secs(vis_timeout as u64));

                    // Lock FIFO group
                    if let Some(ref gid) = msg.message_group_id {
                        self.locked_groups.insert(gid.clone());
                    }

                    let mut sys_attrs = HashMap::new();
                    sys_attrs.insert("SenderId".into(), account_id.to_string());
                    sys_attrs.insert("SentTimestamp".into(), msg.sent_timestamp.to_string());
                    sys_attrs.insert(
                        "ApproximateReceiveCount".into(),
                        msg.receive_count.to_string(),
                    );
                    sys_attrs.insert(
                        "ApproximateFirstReceiveTimestamp".into(),
                        msg.first_receive_timestamp.unwrap().to_string(),
                    );
                    if let Some(ref did) = msg.dedup_id {
                        sys_attrs.insert("MessageDeduplicationId".into(), did.clone());
                    }
                    if let Some(ref gid) = msg.message_group_id {
                        sys_attrs.insert("MessageGroupId".into(), gid.clone());
                    }
                    if let Some(ref seq) = msg.sequence_number {
                        sys_attrs.insert("SequenceNumber".into(), seq.clone());
                    }
                    // AWSTraceHeader from system attributes
                    if let Some(trace) = msg.system_attributes.get("AWSTraceHeader") {
                        if let Some(ref sv) = trace.string_value {
                            sys_attrs.insert("AWSTraceHeader".into(), sv.clone());
                        }
                    }

                    let result = ReceiveMessageResult {
                        message_id: msg.message_id.clone(),
                        receipt_handle: receipt_handle.clone(),
                        body: msg.body.clone(),
                        md5_of_body: msg.md5_of_body.clone(),
                        md5_of_message_attributes: msg.md5_of_message_attributes.clone(),
                        attributes: Some(sys_attrs),
                        message_attributes: if msg.message_attributes.is_empty() {
                            None
                        } else {
                            Some(msg.message_attributes.clone())
                        },
                    };

                    self.inflight.insert(receipt_handle, msg);
                    receive_results.push(result);
                }
                // else: expired message, just dropped
            }
        }

        Ok(receive_results)
    }

    pub fn delete_message(&mut self, receipt_handle: &str) -> Result<(), SqsError> {
        if let Some(msg) = self.inflight.remove(receipt_handle) {
            // Unlock FIFO group
            if let Some(ref gid) = msg.message_group_id {
                self.locked_groups.remove(gid);
            }
        }
        // Idempotent: succeed even if not found
        Ok(())
    }

    pub fn change_message_visibility(
        &mut self,
        receipt_handle: &str,
        timeout: i32,
    ) -> Result<(), SqsError> {
        if timeout < 0 || timeout > 43200 {
            return Err(SqsError::InvalidParameterValue(
                "VisibilityTimeout must be between 0 and 43200".into(),
            ));
        }

        if let Some(msg) = self.inflight.get_mut(receipt_handle) {
            if timeout == 0 {
                // Make immediately visible
                let mut msg = self.inflight.remove(receipt_handle).unwrap();
                if let Some(ref gid) = msg.message_group_id {
                    self.locked_groups.remove(gid);
                }
                msg.receipt_handle = None;
                msg.visibility_deadline = None;
                self.messages.push_back(msg);
            } else {
                msg.visibility_deadline =
                    Some(Instant::now() + Duration::from_secs(timeout as u64));
            }
            Ok(())
        } else {
            Err(SqsError::MessageNotInflight(
                "The message is not in flight.".into(),
            ))
        }
    }

    pub fn purge(&mut self) -> Result<(), SqsError> {
        if let Some(last) = self.last_purge {
            if last.elapsed() < Duration::from_secs(60) {
                return Err(SqsError::PurgeQueueInProgress(
                    "A purge was already initiated within the last 60 seconds.".into(),
                ));
            }
        }
        self.messages.clear();
        self.inflight.clear();
        self.locked_groups.clear();
        self.last_purge = Some(Instant::now());
        Ok(())
    }

    pub fn get_attributes(&self, names: &Option<Vec<String>>) -> HashMap<String, String> {
        let all = match names {
            None => true,
            Some(names) => names.is_empty() || names.contains(&"All".to_string()),
        };

        let mut result = HashMap::new();
        let attr_map = self.attributes.to_map();

        if all {
            result = attr_map;
        } else if let Some(names) = names {
            for name in names {
                if let Some(v) = attr_map.get(name) {
                    result.insert(name.clone(), v.clone());
                }
            }
        }

        // Always include computed attributes if requesting All, or if specifically requested
        let include = |attr_name: &str| -> bool {
            all || names
                .as_ref()
                .map(|n| n.contains(&attr_name.to_string()))
                .unwrap_or(false)
        };

        if include("QueueArn") {
            result.insert("QueueArn".into(), self.arn.clone());
        }
        if include("CreatedTimestamp") {
            result.insert("CreatedTimestamp".into(), self.created_at.to_string());
        }
        if include("LastModifiedTimestamp") {
            result.insert("LastModifiedTimestamp".into(), self.last_modified.to_string());
        }
        if include("ApproximateNumberOfMessages") {
            let now = Instant::now();
            let visible = self.messages.iter().filter(|m| m.visible_at <= now).count();
            result.insert(
                "ApproximateNumberOfMessages".into(),
                visible.to_string(),
            );
        }
        if include("ApproximateNumberOfMessagesNotVisible") {
            result.insert(
                "ApproximateNumberOfMessagesNotVisible".into(),
                self.inflight.len().to_string(),
            );
        }
        if include("ApproximateNumberOfMessagesDelayed") {
            let now = Instant::now();
            let delayed = self.messages.iter().filter(|m| m.visible_at > now).count();
            result.insert(
                "ApproximateNumberOfMessagesDelayed".into(),
                delayed.to_string(),
            );
        }

        result
    }

    pub fn set_attributes(&mut self, attrs: &HashMap<String, String>) -> Result<(), SqsError> {
        // Check for immutable FifoQueue
        if attrs.contains_key("FifoQueue") {
            return Err(SqsError::InvalidAttributeName(
                "FifoQueue cannot be changed after creation".into(),
            ));
        }
        self.attributes.apply(attrs)?;
        self.last_modified = now_secs();
        Ok(())
    }

    fn clean_dedup_cache(&mut self) {
        self.dedup_cache
            .retain(|_, (_, ts)| ts.elapsed() < Duration::from_secs(300));
    }

    pub fn approximate_messages(&self) -> usize {
        self.messages.len()
    }
}
