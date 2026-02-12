use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicI64, Ordering};
use std::sync::Arc;
use std::time::Duration;

use tokio::sync::{Mutex, Notify};
use uuid::Uuid;

use super::error::SqsError;
use super::queue::{DlqRedrive, Permission, Queue, QueueAttributes};
use super::types::*;

struct MessageMoveTask {
    task_handle: String,
    source_arn: String,
    destination_arn: Option<String>,
    status: String,
    messages_moved: Arc<AtomicI64>,
    messages_to_move: i64,
    max_per_second: Option<i32>,
    started_timestamp: i64,
    cancel_flag: Arc<AtomicBool>,
}

struct QueueEntry {
    queue: Queue,
    notify: Arc<Notify>,
}

struct SqsStateInner {
    queues: HashMap<String, QueueEntry>,
    move_tasks: Vec<MessageMoveTask>,
    account_id: String,
    region: String,
    base_url: String,
}

pub struct SqsState {
    inner: Arc<Mutex<SqsStateInner>>,
}

impl SqsState {
    pub fn new(account_id: String, region: String, port: u16) -> Self {
        SqsState {
            inner: Arc::new(Mutex::new(SqsStateInner {
                queues: HashMap::new(),
                move_tasks: Vec::new(),
                account_id,
                region,
                base_url: format!("http://localhost:{}", port),
            })),
        }
    }

    fn resolve_queue_name(queue_url: &str) -> Result<String, SqsError> {
        queue_url
            .split('/')
            .last()
            .filter(|s| !s.is_empty())
            .map(String::from)
            .ok_or_else(|| SqsError::QueueDoesNotExist("Invalid queue URL".into()))
    }

    fn resolve_queue_name_from_arn(arn: &str) -> Result<String, SqsError> {
        arn.split(':')
            .last()
            .filter(|s| !s.is_empty())
            .map(String::from)
            .ok_or_else(|| {
                SqsError::ResourceNotFoundException("Invalid ARN".into())
            })
    }

    fn validate_queue_name(name: &str, is_fifo: bool) -> Result<(), SqsError> {
        if name.is_empty() || name.len() > 80 {
            return Err(SqsError::InvalidParameterValue(
                "Queue name must be 1-80 characters".into(),
            ));
        }
        let valid = name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.');
        if !valid {
            return Err(SqsError::InvalidParameterValue(
                "Queue name can only contain alphanumeric characters, hyphens, and underscores"
                    .into(),
            ));
        }
        if is_fifo && !name.ends_with(".fifo") {
            return Err(SqsError::InvalidParameterValue(
                "FIFO queue name must end with .fifo".into(),
            ));
        }
        if !is_fifo && name.ends_with(".fifo") {
            return Err(SqsError::InvalidParameterValue(
                "Non-FIFO queue name must not end with .fifo".into(),
            ));
        }
        Ok(())
    }

    fn validate_batch_ids(ids: &[String]) -> Result<(), SqsError> {
        if ids.is_empty() {
            return Err(SqsError::EmptyBatchRequest(
                "Batch request must contain at least one entry".into(),
            ));
        }
        if ids.len() > 10 {
            return Err(SqsError::TooManyEntriesInBatchRequest(
                "Batch request must contain at most 10 entries".into(),
            ));
        }
        let mut seen = std::collections::HashSet::new();
        for id in ids {
            if !id
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
            {
                return Err(SqsError::InvalidBatchEntryId(format!(
                    "Invalid batch entry Id: {id}"
                )));
            }
            if !seen.insert(id) {
                return Err(SqsError::BatchEntryIdsNotDistinct(
                    "Batch entry IDs must be distinct".into(),
                ));
            }
        }
        Ok(())
    }

    fn handle_dlq_redrives(
        queues: &mut HashMap<String, QueueEntry>,
        redrives: Vec<DlqRedrive>,
    ) {
        for redrive in redrives {
            // Find queue by ARN
            let dlq_name = redrive.dlq_arn.split(':').last().unwrap_or("");
            if let Some(dlq_entry) = queues.get_mut(dlq_name) {
                dlq_entry.queue.messages.push_back(redrive.message);
                dlq_entry.notify.notify_waiters();
            }
            // If DLQ doesn't exist, message is lost (same as AWS behavior when DLQ is deleted)
        }
    }

    pub async fn create_queue(
        &self,
        req: CreateQueueRequest,
    ) -> Result<CreateQueueResponse, SqsError> {
        let mut inner = self.inner.lock().await;

        // Determine if FIFO from attributes or name
        let is_fifo = req
            .attributes
            .as_ref()
            .and_then(|a| a.get("FifoQueue"))
            .map(|v| v == "true")
            .unwrap_or_else(|| req.queue_name.ends_with(".fifo"));

        Self::validate_queue_name(&req.queue_name, is_fifo)?;

        // Check if queue already exists
        if let Some(entry) = inner.queues.get(&req.queue_name) {
            // Compare attributes
            if let Some(ref attrs) = req.attributes {
                let existing = entry.queue.attributes.to_map();
                for (key, value) in attrs {
                    if let Some(existing_val) = existing.get(key.as_str()) {
                        if existing_val != value {
                            return Err(SqsError::QueueAlreadyExists(format!(
                                "A queue named {} already exists with different attributes",
                                req.queue_name
                            )));
                        }
                    }
                }
            }
            return Ok(CreateQueueResponse {
                queue_url: entry.queue.url.clone(),
            });
        }

        let url = format!(
            "{}/{}/{}",
            inner.base_url, inner.account_id, req.queue_name
        );
        let arn = format!(
            "arn:aws:sqs:{}:{}:{}",
            inner.region, inner.account_id, req.queue_name
        );

        let mut attributes = QueueAttributes::default();
        if is_fifo {
            attributes.fifo_queue = true;
        }
        if let Some(ref attrs) = req.attributes {
            // Remove FifoQueue from attrs before applying since we already set it
            let mut attrs = attrs.clone();
            attrs.remove("FifoQueue");
            attributes.apply(&attrs)?;
        }

        let mut queue = Queue::new(req.queue_name.clone(), arn, url.clone(), attributes);

        if let Some(tags) = req.tags {
            queue.tags = tags;
        }

        inner.queues.insert(
            req.queue_name,
            QueueEntry {
                queue,
                notify: Arc::new(Notify::new()),
            },
        );

        Ok(CreateQueueResponse { queue_url: url })
    }

    pub async fn delete_queue(&self, req: DeleteQueueRequest) -> Result<(), SqsError> {
        let name = Self::resolve_queue_name(&req.queue_url)?;
        let mut inner = self.inner.lock().await;
        inner.queues.remove(&name);
        Ok(())
    }

    pub async fn get_queue_url(
        &self,
        req: GetQueueUrlRequest,
    ) -> Result<GetQueueUrlResponse, SqsError> {
        let inner = self.inner.lock().await;
        inner
            .queues
            .get(&req.queue_name)
            .map(|e| GetQueueUrlResponse {
                queue_url: e.queue.url.clone(),
            })
            .ok_or_else(|| {
                SqsError::QueueDoesNotExist("The specified queue does not exist.".into())
            })
    }

    pub async fn list_queues(
        &self,
        req: ListQueuesRequest,
    ) -> Result<ListQueuesResponse, SqsError> {
        let inner = self.inner.lock().await;
        let max = req.max_results.unwrap_or(1000).min(1000) as usize;

        let mut names: Vec<&String> = inner.queues.keys().collect();
        names.sort();

        // Apply prefix filter
        if let Some(ref prefix) = req.queue_name_prefix {
            names.retain(|n| n.starts_with(prefix));
        }

        // Apply pagination
        let start = if let Some(ref token) = req.next_token {
            names.iter().position(|n| n.as_str() > token.as_str()).unwrap_or(names.len())
        } else {
            0
        };

        let page: Vec<String> = names[start..]
            .iter()
            .take(max)
            .map(|n| inner.queues[*n].queue.url.clone())
            .collect();

        let next_token = if start + max < names.len() {
            names.get(start + max).map(|n| (*n).clone())
        } else {
            None
        };

        Ok(ListQueuesResponse {
            queue_urls: if page.is_empty() { None } else { Some(page) },
            next_token,
        })
    }

    pub async fn get_queue_attributes(
        &self,
        req: GetQueueAttributesRequest,
    ) -> Result<GetQueueAttributesResponse, SqsError> {
        let name = Self::resolve_queue_name(&req.queue_url)?;
        let inner = self.inner.lock().await;
        let entry = inner.queues.get(&name).ok_or_else(|| {
            SqsError::QueueDoesNotExist("The specified queue does not exist.".into())
        })?;
        Ok(GetQueueAttributesResponse {
            attributes: entry.queue.get_attributes(&req.attribute_names),
        })
    }

    pub async fn set_queue_attributes(
        &self,
        req: SetQueueAttributesRequest,
    ) -> Result<(), SqsError> {
        let name = Self::resolve_queue_name(&req.queue_url)?;
        let mut inner = self.inner.lock().await;
        let entry = inner.queues.get_mut(&name).ok_or_else(|| {
            SqsError::QueueDoesNotExist("The specified queue does not exist.".into())
        })?;
        entry.queue.set_attributes(&req.attributes)
    }

    pub async fn purge_queue(&self, req: PurgeQueueRequest) -> Result<(), SqsError> {
        let name = Self::resolve_queue_name(&req.queue_url)?;
        let mut inner = self.inner.lock().await;
        let entry = inner.queues.get_mut(&name).ok_or_else(|| {
            SqsError::QueueDoesNotExist("The specified queue does not exist.".into())
        })?;
        entry.queue.purge()
    }

    pub async fn send_message(
        &self,
        req: SendMessageRequest,
    ) -> Result<SendMessageResponse, SqsError> {
        let name = Self::resolve_queue_name(&req.queue_url)?;
        let mut inner = self.inner.lock().await;
        let account_id = inner.account_id.clone();
        let entry = inner.queues.get_mut(&name).ok_or_else(|| {
            SqsError::QueueDoesNotExist("The specified queue does not exist.".into())
        })?;

        let result = entry.queue.send_message(
            req.message_body,
            req.delay_seconds,
            req.message_attributes,
            req.message_system_attributes,
            req.message_deduplication_id,
            req.message_group_id,
            &account_id,
        )?;

        entry.notify.notify_waiters();
        Ok(result)
    }

    pub async fn send_message_batch(
        &self,
        req: SendMessageBatchRequest,
    ) -> Result<SendMessageBatchResponse, SqsError> {
        let ids: Vec<String> = req.entries.iter().map(|e| e.id.clone()).collect();
        Self::validate_batch_ids(&ids)?;

        let name = Self::resolve_queue_name(&req.queue_url)?;
        let mut inner = self.inner.lock().await;
        let account_id = inner.account_id.clone();
        let entry = inner.queues.get_mut(&name).ok_or_else(|| {
            SqsError::QueueDoesNotExist("The specified queue does not exist.".into())
        })?;

        let mut successful = Vec::new();
        let mut failed = Vec::new();
        let mut any_success = false;

        for e in req.entries {
            match entry.queue.send_message(
                e.message_body,
                e.delay_seconds,
                e.message_attributes,
                e.message_system_attributes,
                e.message_deduplication_id,
                e.message_group_id,
                &account_id,
            ) {
                Ok(resp) => {
                    any_success = true;
                    successful.push(SendMessageBatchResultEntry {
                        id: e.id,
                        message_id: resp.message_id,
                        md5_of_message_body: resp.md5_of_message_body,
                        md5_of_message_attributes: resp.md5_of_message_attributes,
                        md5_of_message_system_attributes: resp.md5_of_message_system_attributes,
                        sequence_number: resp.sequence_number,
                    });
                }
                Err(err) => {
                    let code = match &err {
                        SqsError::InvalidParameterValue(_) => "InvalidParameterValue",
                        SqsError::InvalidMessageContents(_) => "InvalidMessageContents",
                        SqsError::MissingParameter(_) => "MissingParameter",
                        _ => "InternalError",
                    };
                    failed.push(BatchResultErrorEntry {
                        id: e.id,
                        code: code.to_string(),
                        message: format!("{}", err.message_str()),
                        sender_fault: true,
                    });
                }
            }
        }

        if any_success {
            entry.notify.notify_waiters();
        }

        Ok(SendMessageBatchResponse {
            successful,
            failed,
        })
    }

    pub async fn receive_message(
        &self,
        req: ReceiveMessageRequest,
    ) -> Result<ReceiveMessageResponse, SqsError> {
        let name = Self::resolve_queue_name(&req.queue_url)?;
        let max_count = req.max_number_of_messages.unwrap_or(1).min(10).max(1);

        // First attempt
        let (wait_time, notify) = {
            let mut inner = self.inner.lock().await;
            let account_id = inner.account_id.clone();

            if !inner.queues.contains_key(&name) {
                return Err(SqsError::QueueDoesNotExist(
                    "The specified queue does not exist.".into(),
                ));
            }

            // Handle DLQ redrives
            let redrives = inner.queues.get_mut(&name).unwrap().queue.return_expired_inflight();
            if !redrives.is_empty() {
                Self::handle_dlq_redrives(&mut inner.queues, redrives);
            }

            if let Some(entry) = inner.queues.get_mut(&name) {
                let results =
                    entry
                        .queue
                        .receive_messages(max_count, req.visibility_timeout, &account_id)?;
                if !results.is_empty() {
                    let results = filter_receive_results(
                        results,
                        &req.attribute_names,
                        &req.message_attribute_names,
                    );
                    return Ok(ReceiveMessageResponse {
                        messages: Some(results),
                    });
                }

                let wait_time = req
                    .wait_time_seconds
                    .unwrap_or(entry.queue.attributes.receive_message_wait_time_seconds as i32);
                let notify = entry.notify.clone();
                (wait_time, notify)
            } else {
                return Err(SqsError::QueueDoesNotExist(
                    "The specified queue does not exist.".into(),
                ));
            }
        };

        if wait_time <= 0 {
            return Ok(ReceiveMessageResponse { messages: None });
        }

        // Long polling: wait for notification or timeout
        let _ = tokio::time::timeout(
            Duration::from_secs(wait_time as u64),
            notify.notified(),
        )
        .await;

        // Second attempt after waiting
        let mut inner = self.inner.lock().await;
        let account_id = inner.account_id.clone();

        // Handle DLQ redrives again
        if let Some(entry) = inner.queues.get_mut(&name) {
            let redrives = entry.queue.return_expired_inflight();
            if !redrives.is_empty() {
                Self::handle_dlq_redrives(&mut inner.queues, redrives);
            }
        }
        if let Some(entry) = inner.queues.get_mut(&name) {
            let results =
                entry
                    .queue
                    .receive_messages(max_count, req.visibility_timeout, &account_id)?;
            if !results.is_empty() {
                let results = filter_receive_results(
                    results,
                    &req.attribute_names,
                    &req.message_attribute_names,
                );
                return Ok(ReceiveMessageResponse {
                    messages: Some(results),
                });
            }
        }

        Ok(ReceiveMessageResponse { messages: None })
    }

    pub async fn delete_message(&self, req: DeleteMessageRequest) -> Result<(), SqsError> {
        let name = Self::resolve_queue_name(&req.queue_url)?;
        let mut inner = self.inner.lock().await;
        let entry = inner.queues.get_mut(&name).ok_or_else(|| {
            SqsError::QueueDoesNotExist("The specified queue does not exist.".into())
        })?;
        entry.queue.delete_message(&req.receipt_handle)
    }

    pub async fn delete_message_batch(
        &self,
        req: DeleteMessageBatchRequest,
    ) -> Result<DeleteMessageBatchResponse, SqsError> {
        let ids: Vec<String> = req.entries.iter().map(|e| e.id.clone()).collect();
        Self::validate_batch_ids(&ids)?;

        let name = Self::resolve_queue_name(&req.queue_url)?;
        let mut inner = self.inner.lock().await;
        let entry = inner.queues.get_mut(&name).ok_or_else(|| {
            SqsError::QueueDoesNotExist("The specified queue does not exist.".into())
        })?;

        let mut successful = Vec::new();
        let mut failed = Vec::new();

        for e in req.entries {
            match entry.queue.delete_message(&e.receipt_handle) {
                Ok(()) => {
                    successful.push(DeleteMessageBatchResultEntry { id: e.id });
                }
                Err(err) => {
                    failed.push(BatchResultErrorEntry {
                        id: e.id,
                        code: "ReceiptHandleIsInvalid".to_string(),
                        message: err.message_str().to_string(),
                        sender_fault: true,
                    });
                }
            }
        }

        Ok(DeleteMessageBatchResponse {
            successful,
            failed,
        })
    }

    pub async fn change_message_visibility(
        &self,
        req: ChangeMessageVisibilityRequest,
    ) -> Result<(), SqsError> {
        let name = Self::resolve_queue_name(&req.queue_url)?;
        let mut inner = self.inner.lock().await;
        let entry = inner.queues.get_mut(&name).ok_or_else(|| {
            SqsError::QueueDoesNotExist("The specified queue does not exist.".into())
        })?;
        entry
            .queue
            .change_message_visibility(&req.receipt_handle, req.visibility_timeout)
    }

    pub async fn change_message_visibility_batch(
        &self,
        req: ChangeMessageVisibilityBatchRequest,
    ) -> Result<ChangeMessageVisibilityBatchResponse, SqsError> {
        let ids: Vec<String> = req.entries.iter().map(|e| e.id.clone()).collect();
        Self::validate_batch_ids(&ids)?;

        let name = Self::resolve_queue_name(&req.queue_url)?;
        let mut inner = self.inner.lock().await;
        let entry = inner.queues.get_mut(&name).ok_or_else(|| {
            SqsError::QueueDoesNotExist("The specified queue does not exist.".into())
        })?;

        let mut successful = Vec::new();
        let mut failed = Vec::new();

        for e in req.entries {
            match entry
                .queue
                .change_message_visibility(&e.receipt_handle, e.visibility_timeout)
            {
                Ok(()) => {
                    successful.push(ChangeMessageVisibilityBatchResultEntry { id: e.id });
                }
                Err(err) => {
                    let code = match &err {
                        SqsError::MessageNotInflight(_) => "MessageNotInflight",
                        _ => "InvalidParameterValue",
                    };
                    failed.push(BatchResultErrorEntry {
                        id: e.id,
                        code: code.to_string(),
                        message: err.message_str().to_string(),
                        sender_fault: true,
                    });
                }
            }
        }

        Ok(ChangeMessageVisibilityBatchResponse {
            successful,
            failed,
        })
    }

    pub async fn tag_queue(&self, req: TagQueueRequest) -> Result<(), SqsError> {
        let name = Self::resolve_queue_name(&req.queue_url)?;
        let mut inner = self.inner.lock().await;
        let entry = inner.queues.get_mut(&name).ok_or_else(|| {
            SqsError::QueueDoesNotExist("The specified queue does not exist.".into())
        })?;
        for (k, v) in req.tags {
            entry.queue.tags.insert(k, v);
        }
        if entry.queue.tags.len() > 50 {
            return Err(SqsError::InvalidParameterValue(
                "Maximum 50 tags per queue".into(),
            ));
        }
        Ok(())
    }

    pub async fn untag_queue(&self, req: UntagQueueRequest) -> Result<(), SqsError> {
        let name = Self::resolve_queue_name(&req.queue_url)?;
        let mut inner = self.inner.lock().await;
        let entry = inner.queues.get_mut(&name).ok_or_else(|| {
            SqsError::QueueDoesNotExist("The specified queue does not exist.".into())
        })?;
        for key in &req.tag_keys {
            entry.queue.tags.remove(key);
        }
        Ok(())
    }

    pub async fn list_queue_tags(
        &self,
        req: ListQueueTagsRequest,
    ) -> Result<ListQueueTagsResponse, SqsError> {
        let name = Self::resolve_queue_name(&req.queue_url)?;
        let inner = self.inner.lock().await;
        let entry = inner.queues.get(&name).ok_or_else(|| {
            SqsError::QueueDoesNotExist("The specified queue does not exist.".into())
        })?;
        Ok(ListQueueTagsResponse {
            tags: if entry.queue.tags.is_empty() {
                None
            } else {
                Some(entry.queue.tags.clone())
            },
        })
    }

    pub async fn add_permission(&self, req: AddPermissionRequest) -> Result<(), SqsError> {
        let name = Self::resolve_queue_name(&req.queue_url)?;
        let mut inner = self.inner.lock().await;
        let entry = inner.queues.get_mut(&name).ok_or_else(|| {
            SqsError::QueueDoesNotExist("The specified queue does not exist.".into())
        })?;

        if entry.queue.permissions.contains_key(&req.label) {
            return Err(SqsError::InvalidParameterValue(format!(
                "Permission label {} already exists",
                req.label
            )));
        }
        if entry.queue.permissions.len() >= 7 {
            return Err(SqsError::OverLimit(
                "Maximum 7 permission statements per queue".into(),
            ));
        }

        entry.queue.permissions.insert(
            req.label.clone(),
            Permission {
                label: req.label,
                aws_account_ids: req.aws_account_ids,
                actions: req.actions,
            },
        );
        Ok(())
    }

    pub async fn remove_permission(&self, req: RemovePermissionRequest) -> Result<(), SqsError> {
        let name = Self::resolve_queue_name(&req.queue_url)?;
        let mut inner = self.inner.lock().await;
        let entry = inner.queues.get_mut(&name).ok_or_else(|| {
            SqsError::QueueDoesNotExist("The specified queue does not exist.".into())
        })?;
        if entry.queue.permissions.remove(&req.label).is_none() {
            return Err(SqsError::InvalidParameterValue(format!(
                "Permission label {} not found",
                req.label
            )));
        }
        Ok(())
    }

    pub async fn list_dead_letter_source_queues(
        &self,
        req: ListDeadLetterSourceQueuesRequest,
    ) -> Result<ListDeadLetterSourceQueuesResponse, SqsError> {
        let name = Self::resolve_queue_name(&req.queue_url)?;
        let inner = self.inner.lock().await;
        let entry = inner.queues.get(&name).ok_or_else(|| {
            SqsError::QueueDoesNotExist("The specified queue does not exist.".into())
        })?;
        let target_arn = entry.queue.arn.clone();

        let mut source_urls: Vec<String> = inner
            .queues
            .values()
            .filter(|e| {
                e.queue
                    .attributes
                    .redrive_policy
                    .as_ref()
                    .map(|rp| rp.dead_letter_target_arn == target_arn)
                    .unwrap_or(false)
            })
            .map(|e| e.queue.url.clone())
            .collect();
        source_urls.sort();

        let max = req.max_results.unwrap_or(1000).min(1000) as usize;
        let start = if let Some(ref token) = req.next_token {
            source_urls
                .iter()
                .position(|u| u.as_str() > token.as_str())
                .unwrap_or(source_urls.len())
        } else {
            0
        };

        let page: Vec<String> = source_urls[start..].iter().take(max).cloned().collect();
        let next_token = if start + max < source_urls.len() {
            source_urls.get(start + max).cloned()
        } else {
            None
        };

        Ok(ListDeadLetterSourceQueuesResponse {
            queue_urls: page,
            next_token,
        })
    }

    pub async fn start_message_move_task(
        &self,
        req: StartMessageMoveTaskRequest,
    ) -> Result<StartMessageMoveTaskResponse, SqsError> {
        let source_name = Self::resolve_queue_name_from_arn(&req.source_arn)?;

        let mut inner = self.inner.lock().await;

        // Validate source exists
        if !inner.queues.contains_key(&source_name) {
            return Err(SqsError::ResourceNotFoundException(
                "Source queue does not exist".into(),
            ));
        }

        // Validate destination if provided
        if let Some(ref dest_arn) = req.destination_arn {
            let dest_name = Self::resolve_queue_name_from_arn(dest_arn)?;
            if !inner.queues.contains_key(&dest_name) {
                return Err(SqsError::ResourceNotFoundException(
                    "Destination queue does not exist".into(),
                ));
            }
        }

        // Check no active task for this source
        for task in &inner.move_tasks {
            if task.source_arn == req.source_arn
                && (task.status == "RUNNING" || task.status == "CANCELLING")
            {
                return Err(SqsError::InvalidParameterValue(
                    "An active move task already exists for this source queue".into(),
                ));
            }
        }

        let task_handle = Uuid::new_v4().to_string();
        let messages_to_move = inner.queues[&source_name].queue.approximate_messages() as i64;
        let cancel_flag = Arc::new(AtomicBool::new(false));
        let messages_moved = Arc::new(AtomicI64::new(0));

        let task = MessageMoveTask {
            task_handle: task_handle.clone(),
            source_arn: req.source_arn.clone(),
            destination_arn: req.destination_arn.clone(),
            status: "RUNNING".into(),
            messages_moved: messages_moved.clone(),
            messages_to_move,
            max_per_second: req.max_number_of_messages_per_second,
            started_timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as i64,
            cancel_flag: cancel_flag.clone(),
        };
        inner.move_tasks.push(task);

        // Spawn the move task
        let state = self.inner.clone();
        let source_arn = req.source_arn.clone();
        let dest_arn = req.destination_arn.clone();
        let max_per_sec = req.max_number_of_messages_per_second;
        let task_handle_clone = task_handle.clone();

        tokio::spawn(async move {
            let delay = if let Some(rate) = max_per_sec {
                if rate > 0 {
                    Duration::from_millis(1000 / rate as u64)
                } else {
                    Duration::from_millis(10)
                }
            } else {
                Duration::from_millis(10)
            };

            loop {
                if cancel_flag.load(Ordering::Relaxed) {
                    let mut inner = state.lock().await;
                    if let Some(task) = inner
                        .move_tasks
                        .iter_mut()
                        .find(|t| t.task_handle == task_handle_clone)
                    {
                        task.status = "CANCELLED".into();
                    }
                    return;
                }

                let mut inner = state.lock().await;
                let source_name = source_arn.split(':').last().unwrap_or("");
                let source_msg = inner
                    .queues
                    .get_mut(source_name)
                    .and_then(|e| e.queue.messages.pop_front());

                if let Some(msg) = source_msg {
                    // Determine destination
                    let dest_name = if let Some(ref da) = dest_arn {
                        da.split(':').last().unwrap_or("").to_string()
                    } else {
                        // Move back to original source (from DLQ redrive)
                        // This is a simplification; in practice we'd track original source
                        source_name.to_string()
                    };

                    if let Some(dest_entry) = inner.queues.get_mut(&dest_name) {
                        dest_entry.queue.messages.push_back(msg);
                        dest_entry.notify.notify_waiters();
                    }
                    messages_moved.fetch_add(1, Ordering::Relaxed);
                    drop(inner);
                    tokio::time::sleep(delay).await;
                } else {
                    // No more messages, task complete
                    if let Some(task) = inner
                        .move_tasks
                        .iter_mut()
                        .find(|t| t.task_handle == task_handle_clone)
                    {
                        task.status = "COMPLETED".into();
                    }
                    return;
                }
            }
        });

        Ok(StartMessageMoveTaskResponse { task_handle })
    }

    pub async fn cancel_message_move_task(
        &self,
        req: CancelMessageMoveTaskRequest,
    ) -> Result<CancelMessageMoveTaskResponse, SqsError> {
        let inner = self.inner.lock().await;
        let task = inner
            .move_tasks
            .iter()
            .find(|t| t.task_handle == req.task_handle)
            .ok_or_else(|| {
                SqsError::ResourceNotFoundException("Task not found".into())
            })?;

        if task.status != "RUNNING" {
            return Err(SqsError::ResourceNotFoundException(
                "Task is not running".into(),
            ));
        }

        task.cancel_flag.store(true, Ordering::Relaxed);
        let moved = task.messages_moved.load(Ordering::Relaxed);

        Ok(CancelMessageMoveTaskResponse {
            approximate_number_of_messages_moved: moved,
        })
    }

    pub async fn list_message_move_tasks(
        &self,
        req: ListMessageMoveTasksRequest,
    ) -> Result<ListMessageMoveTasksResponse, SqsError> {
        let inner = self.inner.lock().await;
        let max = req.max_results.unwrap_or(10).min(10) as usize;

        let results: Vec<MessageMoveTaskResult> = inner
            .move_tasks
            .iter()
            .filter(|t| t.source_arn == req.source_arn)
            .take(max)
            .map(|t| MessageMoveTaskResult {
                task_handle: t.task_handle.clone(),
                status: t.status.clone(),
                source_arn: t.source_arn.clone(),
                destination_arn: t.destination_arn.clone(),
                approximate_number_of_messages_moved: t.messages_moved.load(Ordering::Relaxed),
                approximate_number_of_messages_to_move: Some(t.messages_to_move),
                max_number_of_messages_per_second: t.max_per_second,
                started_timestamp: Some(t.started_timestamp),
            })
            .collect();

        Ok(ListMessageMoveTasksResponse {
            results,
            next_token: None,
        })
    }
}

fn filter_receive_results(
    results: Vec<ReceiveMessageResult>,
    attr_names: &Option<Vec<String>>,
    msg_attr_names: &Option<Vec<String>>,
) -> Vec<ReceiveMessageResult> {
    results
        .into_iter()
        .map(|mut r| {
            // Filter system attributes
            if let Some(ref names) = attr_names {
                if !names.contains(&"All".to_string()) {
                    if let Some(ref attrs) = r.attributes {
                        let filtered: HashMap<String, String> = attrs
                            .iter()
                            .filter(|(k, _)| names.contains(k))
                            .map(|(k, v)| (k.clone(), v.clone()))
                            .collect();
                        r.attributes = if filtered.is_empty() {
                            None
                        } else {
                            Some(filtered)
                        };
                    }
                }
            } else {
                r.attributes = None;
            }

            // Filter message attributes
            if let Some(ref names) = msg_attr_names {
                if !names.contains(&"All".to_string()) {
                    if let Some(ref attrs) = r.message_attributes {
                        let filtered: HashMap<String, MessageAttributeValue> = attrs
                            .iter()
                            .filter(|(k, _)| {
                                names.contains(k)
                                    || names.iter().any(|n| {
                                        n.ends_with(".*")
                                            && k.starts_with(&n[..n.len() - 2])
                                    })
                            })
                            .map(|(k, v)| (k.clone(), v.clone()))
                            .collect();
                        r.message_attributes = if filtered.is_empty() {
                            None
                        } else {
                            Some(filtered)
                        };
                    }
                }
            } else {
                r.message_attributes = None;
            }

            r
        })
        .collect()
}

// Helper trait to access error messages
trait ErrorMessage {
    fn message_str(&self) -> &str;
}

impl ErrorMessage for SqsError {
    fn message_str(&self) -> &str {
        match self {
            SqsError::QueueAlreadyExists(m)
            | SqsError::QueueDoesNotExist(m)
            | SqsError::InvalidAttributeName(m)
            | SqsError::InvalidAttributeValue(m)
            | SqsError::InvalidParameterValue(m)
            | SqsError::InvalidMessageContents(m)
            | SqsError::UnsupportedOperation(m)
            | SqsError::PurgeQueueInProgress(m)
            | SqsError::ReceiptHandleIsInvalid(m)
            | SqsError::MessageNotInflight(m)
            | SqsError::OverLimit(m)
            | SqsError::EmptyBatchRequest(m)
            | SqsError::TooManyEntriesInBatchRequest(m)
            | SqsError::BatchEntryIdsNotDistinct(m)
            | SqsError::BatchRequestTooLong(m)
            | SqsError::InvalidBatchEntryId(m)
            | SqsError::ResourceNotFoundException(m)
            | SqsError::InvalidIdFormat(m)
            | SqsError::MissingParameter(m)
            | SqsError::InvalidAction(m) => m,
        }
    }
}
