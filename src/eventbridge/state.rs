use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;
use uuid::Uuid;

use super::error::EventBridgeError;
use super::types::*;

struct EventBusData {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
    rules: HashMap<String, RuleData>,
    events: Vec<StoredEvent>,
}

struct RuleData {
    name: String,
    arn: String,
    event_bus_name: String,
    event_pattern: Option<String>,
    schedule_expression: Option<String>,
    state: String,
    description: Option<String>,
    targets: HashMap<String, Target>,
    tags: HashMap<String, String>,
}

struct EventBridgeStateInner {
    buses: HashMap<String, EventBusData>,
    account_id: String,
    region: String,
}

pub struct EventBridgeState {
    inner: Arc<Mutex<EventBridgeStateInner>>,
}

impl EventBridgeState {
    pub fn new(account_id: String, region: String) -> Self {
        let default_bus_arn = format!("arn:aws:events:{}:{}:event-bus/default", region, account_id);
        let mut buses = HashMap::new();
        buses.insert("default".to_string(), EventBusData {
            name: "default".to_string(),
            arn: default_bus_arn,
            tags: HashMap::new(),
            rules: HashMap::new(),
            events: Vec::new(),
        });
        EventBridgeState {
            inner: Arc::new(Mutex::new(EventBridgeStateInner {
                buses,
                account_id,
                region,
            })),
        }
    }

    fn now() -> f64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64()
    }

    fn resolve_bus<'a>(state: &'a EventBridgeStateInner, name: Option<&'a str>) -> &'a str {
        match name {
            Some(n) if !n.is_empty() => {
                // Could be ARN or name
                if n.starts_with("arn:") {
                    // Find by ARN
                    for (k, b) in &state.buses {
                        if b.arn == n {
                            return k.as_str();
                        }
                    }
                    "default"
                } else {
                    n
                }
            }
            _ => "default",
        }
    }

    pub async fn create_event_bus(
        &self,
        req: CreateEventBusRequest,
    ) -> Result<CreateEventBusResponse, EventBridgeError> {
        let mut state = self.inner.lock().await;
        if state.buses.contains_key(&req.name) {
            return Err(EventBridgeError::ResourceAlreadyExistsException(format!(
                "Event bus {} already exists", req.name
            )));
        }
        let arn = format!(
            "arn:aws:events:{}:{}:event-bus/{}",
            state.region, state.account_id, req.name
        );
        let mut tags = HashMap::new();
        if let Some(t) = req.tags {
            for tag in t {
                tags.insert(tag.key, tag.value);
            }
        }
        state.buses.insert(req.name.clone(), EventBusData {
            name: req.name,
            arn: arn.clone(),
            tags,
            rules: HashMap::new(),
            events: Vec::new(),
        });
        Ok(CreateEventBusResponse { event_bus_arn: arn })
    }

    pub async fn delete_event_bus(&self, req: DeleteEventBusRequest) -> Result<(), EventBridgeError> {
        let mut state = self.inner.lock().await;
        if req.name == "default" {
            return Err(EventBridgeError::InvalidAction(
                "Cannot delete the default event bus".to_string(),
            ));
        }
        if state.buses.remove(&req.name).is_none() {
            return Err(EventBridgeError::ResourceNotFoundException(format!(
                "Event bus {} not found", req.name
            )));
        }
        Ok(())
    }

    pub async fn describe_event_bus(
        &self,
        req: DescribeEventBusRequest,
    ) -> Result<DescribeEventBusResponse, EventBridgeError> {
        let state = self.inner.lock().await;
        let bus_name = req.name.as_deref().unwrap_or("default");
        let bus = state.buses.get(bus_name)
            .ok_or_else(|| EventBridgeError::ResourceNotFoundException(format!(
                "Event bus {} not found", bus_name
            )))?;
        Ok(DescribeEventBusResponse {
            name: bus.name.clone(),
            arn: bus.arn.clone(),
            policy: None,
        })
    }

    pub async fn list_event_buses(
        &self,
        req: ListEventBusesRequest,
    ) -> Result<ListEventBusesResponse, EventBridgeError> {
        let state = self.inner.lock().await;
        let mut buses: Vec<EventBus> = state.buses.values().map(|b| EventBus {
            name: b.name.clone(),
            arn: b.arn.clone(),
        }).collect();
        if let Some(ref prefix) = req.name_prefix {
            buses.retain(|b| b.name.starts_with(prefix.as_str()));
        }
        buses.sort_by(|a, b| a.name.cmp(&b.name));
        let limit = req.limit.unwrap_or(100);
        let has_more = buses.len() > limit;
        buses.truncate(limit);
        Ok(ListEventBusesResponse {
            event_buses: buses,
            next_token: if has_more { Some("next".to_string()) } else { None },
        })
    }

    pub async fn put_events(
        &self,
        req: PutEventsRequest,
    ) -> Result<PutEventsResponse, EventBridgeError> {
        let mut state = self.inner.lock().await;
        let mut results = Vec::with_capacity(req.entries.len());
        for entry in req.entries {
            let event_id = Uuid::new_v4().to_string();
            let bus_name = entry.event_bus_name.as_deref().unwrap_or("default");
            if let Some(bus) = state.buses.get_mut(bus_name) {
                bus.events.push(StoredEvent {
                    event_id: event_id.clone(),
                    event_bus_name: bus_name.to_string(),
                    source: entry.source.unwrap_or_default(),
                    detail_type: entry.detail_type.unwrap_or_default(),
                    detail: entry.detail.unwrap_or_default(),
                    timestamp: Self::now(),
                });
                results.push(PutEventsResultEntry {
                    event_id: Some(event_id),
                    error_code: None,
                    error_message: None,
                });
            } else {
                results.push(PutEventsResultEntry {
                    event_id: None,
                    error_code: Some("ResourceNotFoundException".to_string()),
                    error_message: Some(format!("Event bus {} not found", bus_name)),
                });
            }
        }
        let failed = results.iter().filter(|r| r.error_code.is_some()).count() as u32;
        Ok(PutEventsResponse {
            failed_entry_count: failed,
            entries: results,
        })
    }

    pub async fn put_rule(&self, req: PutRuleRequest) -> Result<PutRuleResponse, EventBridgeError> {
        let mut state = self.inner.lock().await;
        let bus_name = req.event_bus_name.as_deref().unwrap_or("default").to_string();
        let bus = state.buses.get_mut(&bus_name)
            .ok_or_else(|| EventBridgeError::ResourceNotFoundException(format!(
                "Event bus {} not found", bus_name
            )))?;
        let arn = format!(
            "arn:aws:events:{}:{}:rule/{}/{}",
            "us-east-1", "000000000000", bus_name, req.name
        );
        let mut tags = HashMap::new();
        if let Some(t) = req.tags {
            for tag in t { tags.insert(tag.key, tag.value); }
        }
        bus.rules.insert(req.name.clone(), RuleData {
            name: req.name,
            arn: arn.clone(),
            event_bus_name: bus_name,
            event_pattern: req.event_pattern,
            schedule_expression: req.schedule_expression,
            state: req.state.unwrap_or_else(|| "ENABLED".to_string()),
            description: req.description,
            targets: HashMap::new(),
            tags,
        });
        Ok(PutRuleResponse { rule_arn: arn })
    }

    pub async fn delete_rule(&self, req: DeleteRuleRequest) -> Result<(), EventBridgeError> {
        let mut state = self.inner.lock().await;
        let bus_name = req.event_bus_name.as_deref().unwrap_or("default");
        let bus = state.buses.get_mut(bus_name)
            .ok_or_else(|| EventBridgeError::ResourceNotFoundException(format!(
                "Event bus {} not found", bus_name
            )))?;
        if bus.rules.remove(&req.name).is_none() {
            return Err(EventBridgeError::ResourceNotFoundException(format!(
                "Rule {} not found", req.name
            )));
        }
        Ok(())
    }

    pub async fn describe_rule(
        &self,
        req: DescribeRuleRequest,
    ) -> Result<DescribeRuleResponse, EventBridgeError> {
        let state = self.inner.lock().await;
        let bus_name = req.event_bus_name.as_deref().unwrap_or("default");
        let bus = state.buses.get(bus_name)
            .ok_or_else(|| EventBridgeError::ResourceNotFoundException(format!(
                "Event bus {} not found", bus_name
            )))?;
        let rule = bus.rules.get(&req.name)
            .ok_or_else(|| EventBridgeError::ResourceNotFoundException(format!(
                "Rule {} not found", req.name
            )))?;
        Ok(DescribeRuleResponse {
            name: rule.name.clone(),
            arn: rule.arn.clone(),
            event_pattern: rule.event_pattern.clone(),
            schedule_expression: rule.schedule_expression.clone(),
            state: rule.state.clone(),
            description: rule.description.clone(),
            event_bus_name: rule.event_bus_name.clone(),
        })
    }

    pub async fn list_rules(
        &self,
        req: ListRulesRequest,
    ) -> Result<ListRulesResponse, EventBridgeError> {
        let state = self.inner.lock().await;
        let bus_name = req.event_bus_name.as_deref().unwrap_or("default");
        let bus = state.buses.get(bus_name)
            .ok_or_else(|| EventBridgeError::ResourceNotFoundException(format!(
                "Event bus {} not found", bus_name
            )))?;
        let mut rules: Vec<Rule> = bus.rules.values().map(|r| Rule {
            name: r.name.clone(),
            arn: r.arn.clone(),
            state: r.state.clone(),
            event_bus_name: r.event_bus_name.clone(),
        }).collect();
        if let Some(ref prefix) = req.name_prefix {
            rules.retain(|r| r.name.starts_with(prefix.as_str()));
        }
        rules.sort_by(|a, b| a.name.cmp(&b.name));
        let limit = req.limit.unwrap_or(100);
        let has_more = rules.len() > limit;
        rules.truncate(limit);
        Ok(ListRulesResponse {
            rules,
            next_token: if has_more { Some("next".to_string()) } else { None },
        })
    }

    pub async fn put_targets(&self, req: PutTargetsRequest) -> Result<PutTargetsResponse, EventBridgeError> {
        let mut state = self.inner.lock().await;
        let bus_name = req.event_bus_name.as_deref().unwrap_or("default");
        let bus = state.buses.get_mut(bus_name)
            .ok_or_else(|| EventBridgeError::ResourceNotFoundException(format!(
                "Event bus {} not found", bus_name
            )))?;
        let rule = bus.rules.get_mut(&req.rule)
            .ok_or_else(|| EventBridgeError::ResourceNotFoundException(format!(
                "Rule {} not found", req.rule
            )))?;
        for target in req.targets {
            rule.targets.insert(target.id.clone(), target);
        }
        Ok(PutTargetsResponse {
            failed_entry_count: 0,
            failed_entries: vec![],
        })
    }

    pub async fn remove_targets(&self, req: RemoveTargetsRequest) -> Result<RemoveTargetsResponse, EventBridgeError> {
        let mut state = self.inner.lock().await;
        let bus_name = req.event_bus_name.as_deref().unwrap_or("default");
        let bus = state.buses.get_mut(bus_name)
            .ok_or_else(|| EventBridgeError::ResourceNotFoundException(format!(
                "Event bus {} not found", bus_name
            )))?;
        let rule = bus.rules.get_mut(&req.rule)
            .ok_or_else(|| EventBridgeError::ResourceNotFoundException(format!(
                "Rule {} not found", req.rule
            )))?;
        for id in &req.ids {
            rule.targets.remove(id);
        }
        Ok(RemoveTargetsResponse {
            failed_entry_count: 0,
            failed_entries: vec![],
        })
    }

    pub async fn list_targets_by_rule(
        &self,
        req: ListTargetsByRuleRequest,
    ) -> Result<ListTargetsByRuleResponse, EventBridgeError> {
        let state = self.inner.lock().await;
        let bus_name = req.event_bus_name.as_deref().unwrap_or("default");
        let bus = state.buses.get(bus_name)
            .ok_or_else(|| EventBridgeError::ResourceNotFoundException(format!(
                "Event bus {} not found", bus_name
            )))?;
        let rule = bus.rules.get(&req.rule)
            .ok_or_else(|| EventBridgeError::ResourceNotFoundException(format!(
                "Rule {} not found", req.rule
            )))?;
        let mut targets: Vec<Target> = rule.targets.values().cloned().collect();
        targets.sort_by(|a, b| a.id.cmp(&b.id));
        Ok(ListTargetsByRuleResponse { targets, next_token: None })
    }

    pub async fn tag_resource(&self, req: TagResourceRequest) -> Result<(), EventBridgeError> {
        let mut state = self.inner.lock().await;
        // Find the bus or rule by ARN and tag it
        for bus in state.buses.values_mut() {
            if bus.arn == req.resource_arn {
                for tag in req.tags { bus.tags.insert(tag.key, tag.value); }
                return Ok(());
            }
            for rule in bus.rules.values_mut() {
                if rule.arn == req.resource_arn {
                    for tag in req.tags { rule.tags.insert(tag.key, tag.value); }
                    return Ok(());
                }
            }
        }
        Err(EventBridgeError::ResourceNotFoundException(format!(
            "Resource {} not found", req.resource_arn
        )))
    }

    pub async fn untag_resource(&self, req: UntagResourceRequest) -> Result<(), EventBridgeError> {
        let mut state = self.inner.lock().await;
        for bus in state.buses.values_mut() {
            if bus.arn == req.resource_arn {
                for key in &req.tag_keys { bus.tags.remove(key); }
                return Ok(());
            }
            for rule in bus.rules.values_mut() {
                if rule.arn == req.resource_arn {
                    for key in &req.tag_keys { rule.tags.remove(key); }
                    return Ok(());
                }
            }
        }
        Err(EventBridgeError::ResourceNotFoundException(format!(
            "Resource {} not found", req.resource_arn
        )))
    }

    pub async fn list_tags_for_resource(
        &self,
        req: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, EventBridgeError> {
        let state = self.inner.lock().await;
        for bus in state.buses.values() {
            if bus.arn == req.resource_arn {
                let tags = bus.tags.iter().map(|(k, v)| Tag { key: k.clone(), value: v.clone() }).collect();
                return Ok(ListTagsForResourceResponse { tags });
            }
            for rule in bus.rules.values() {
                if rule.arn == req.resource_arn {
                    let tags = rule.tags.iter().map(|(k, v)| Tag { key: k.clone(), value: v.clone() }).collect();
                    return Ok(ListTagsForResourceResponse { tags });
                }
            }
        }
        Err(EventBridgeError::ResourceNotFoundException(format!(
            "Resource {} not found", req.resource_arn
        )))
    }
}
