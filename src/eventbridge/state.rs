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
        });
        EventBridgeState {
            inner: Arc::new(Mutex::new(EventBridgeStateInner {
                buses,
                account_id,
                region,
            })),
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
        let state = self.inner.lock().await;
        let mut results = Vec::with_capacity(req.entries.len());
        for entry in req.entries {
            let event_id = Uuid::new_v4().to_string();
            let bus_name = entry.event_bus_name.as_deref().unwrap_or("default");
            if state.buses.contains_key(bus_name) {
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


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = EventBridgeState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_delete_event_bus_not_found() {
        let state = EventBridgeState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DeleteEventBusRequest::default();
        let result = state.delete_event_bus(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_put_rule() {
        let state = EventBridgeState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = PutRuleRequest::default();
        let _ = state.put_rule(req).await;
    }
    #[tokio::test]
    async fn test_delete_rule_not_found() {
        let state = EventBridgeState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DeleteRuleRequest::default();
        let result = state.delete_rule(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_put_targets() {
        let state = EventBridgeState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = PutTargetsRequest::default();
        let _ = state.put_targets(req).await;
    }
    #[tokio::test]
    async fn test_remove_targets() {
        let state = EventBridgeState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = RemoveTargetsRequest::default();
        let _ = state.remove_targets(req).await;
    }
    #[tokio::test]
    async fn test_tag_resource() {
        let state = EventBridgeState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = TagResourceRequest::default();
        let _ = state.tag_resource(req).await;
    }
    #[tokio::test]
    async fn test_untag_resource() {
        let state = EventBridgeState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = UntagResourceRequest::default();
        let _ = state.untag_resource(req).await;
    }

    fn make_state() -> EventBridgeState {
        EventBridgeState::new("123456789012".to_string(), "us-east-1".to_string())
    }

    // --- Extended coverage: event bus operations ---

    #[tokio::test]
    async fn test_create_event_bus() {
        let state = make_state();
        let result = state.create_event_bus(CreateEventBusRequest {
            name: "my-bus".to_string(),
            tags: None,
        }).await.unwrap();
        assert!(result.event_bus_arn.contains("my-bus"));
    }

    #[tokio::test]
    async fn test_create_event_bus_duplicate() {
        let state = make_state();
        state.create_event_bus(CreateEventBusRequest {
            name: "my-bus".to_string(),
            tags: None,
        }).await.unwrap();
        let result = state.create_event_bus(CreateEventBusRequest {
            name: "my-bus".to_string(),
            tags: None,
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_event_bus_with_tags() {
        let state = make_state();
        let resp = state.create_event_bus(CreateEventBusRequest {
            name: "tagged-bus".to_string(),
            tags: Some(vec![Tag { key: "env".to_string(), value: "test".to_string() }]),
        }).await.unwrap();
        let tags = state.list_tags_for_resource(ListTagsForResourceRequest {
            resource_arn: resp.event_bus_arn,
        }).await.unwrap();
        assert_eq!(tags.tags.len(), 1);
    }

    #[tokio::test]
    async fn test_delete_event_bus() {
        let state = make_state();
        state.create_event_bus(CreateEventBusRequest {
            name: "my-bus".to_string(),
            tags: None,
        }).await.unwrap();
        assert!(state.delete_event_bus(DeleteEventBusRequest { name: "my-bus".to_string() }).await.is_ok());
    }

    #[tokio::test]
    async fn test_delete_default_event_bus_fails() {
        let state = make_state();
        let result = state.delete_event_bus(DeleteEventBusRequest { name: "default".to_string() }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_describe_event_bus_default() {
        let state = make_state();
        let result = state.describe_event_bus(DescribeEventBusRequest { name: None }).await.unwrap();
        assert_eq!(result.name, "default");
    }

    #[tokio::test]
    async fn test_describe_event_bus_custom() {
        let state = make_state();
        state.create_event_bus(CreateEventBusRequest { name: "custom".to_string(), tags: None }).await.unwrap();
        let result = state.describe_event_bus(DescribeEventBusRequest { name: Some("custom".to_string()) }).await.unwrap();
        assert_eq!(result.name, "custom");
    }

    #[tokio::test]
    async fn test_describe_event_bus_not_found() {
        let state = make_state();
        let result = state.describe_event_bus(DescribeEventBusRequest { name: Some("nope".to_string()) }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_event_buses() {
        let state = make_state();
        state.create_event_bus(CreateEventBusRequest { name: "bus-a".to_string(), tags: None }).await.unwrap();
        state.create_event_bus(CreateEventBusRequest { name: "bus-b".to_string(), tags: None }).await.unwrap();
        let result = state.list_event_buses(ListEventBusesRequest::default()).await.unwrap();
        assert_eq!(result.event_buses.len(), 3); // default + 2
    }

    #[tokio::test]
    async fn test_list_event_buses_with_prefix() {
        let state = make_state();
        state.create_event_bus(CreateEventBusRequest { name: "app-bus".to_string(), tags: None }).await.unwrap();
        state.create_event_bus(CreateEventBusRequest { name: "test-bus".to_string(), tags: None }).await.unwrap();
        let result = state.list_event_buses(ListEventBusesRequest {
            name_prefix: Some("app".to_string()),
            limit: None,
        }).await.unwrap();
        assert_eq!(result.event_buses.len(), 1);
        assert_eq!(result.event_buses[0].name, "app-bus");
    }

    // --- Extended coverage: rule operations ---

    #[tokio::test]
    async fn test_put_rule_with_details() {
        let state = make_state();
        let result = state.put_rule(PutRuleRequest {
            name: "my-rule".to_string(),
            event_bus_name: None,
            event_pattern: Some("{\"source\":[\"aws.ec2\"]}".to_string()),
            schedule_expression: None,
            state: Some("ENABLED".to_string()),
            description: Some("test rule".to_string()),
            tags: Some(vec![Tag { key: "env".to_string(), value: "test".to_string() }]),
        }).await.unwrap();
        assert!(result.rule_arn.contains("my-rule"));
    }

    #[tokio::test]
    async fn test_put_rule_bus_not_found() {
        let state = make_state();
        let result = state.put_rule(PutRuleRequest {
            name: "rule".to_string(),
            event_bus_name: Some("nonexistent".to_string()),
            ..Default::default()
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_describe_rule() {
        let state = make_state();
        state.put_rule(PutRuleRequest {
            name: "my-rule".to_string(),
            description: Some("desc".to_string()),
            ..Default::default()
        }).await.unwrap();
        let result = state.describe_rule(DescribeRuleRequest {
            name: "my-rule".to_string(),
            event_bus_name: None,
        }).await.unwrap();
        assert_eq!(result.name, "my-rule");
        assert_eq!(result.description.as_deref(), Some("desc"));
    }

    #[tokio::test]
    async fn test_describe_rule_not_found() {
        let state = make_state();
        let result = state.describe_rule(DescribeRuleRequest {
            name: "nope".to_string(),
            event_bus_name: None,
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_describe_rule_bus_not_found() {
        let state = make_state();
        let result = state.describe_rule(DescribeRuleRequest {
            name: "rule".to_string(),
            event_bus_name: Some("nope".to_string()),
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_rules() {
        let state = make_state();
        state.put_rule(PutRuleRequest { name: "rule-a".to_string(), ..Default::default() }).await.unwrap();
        state.put_rule(PutRuleRequest { name: "rule-b".to_string(), ..Default::default() }).await.unwrap();
        let result = state.list_rules(ListRulesRequest::default()).await.unwrap();
        assert_eq!(result.rules.len(), 2);
        assert_eq!(result.rules[0].name, "rule-a");
    }

    #[tokio::test]
    async fn test_list_rules_with_prefix() {
        let state = make_state();
        state.put_rule(PutRuleRequest { name: "app-rule".to_string(), ..Default::default() }).await.unwrap();
        state.put_rule(PutRuleRequest { name: "test-rule".to_string(), ..Default::default() }).await.unwrap();
        let result = state.list_rules(ListRulesRequest {
            name_prefix: Some("app".to_string()),
            ..Default::default()
        }).await.unwrap();
        assert_eq!(result.rules.len(), 1);
    }

    #[tokio::test]
    async fn test_list_rules_bus_not_found() {
        let state = make_state();
        let result = state.list_rules(ListRulesRequest {
            event_bus_name: Some("nope".to_string()),
            ..Default::default()
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_rule() {
        let state = make_state();
        state.put_rule(PutRuleRequest { name: "my-rule".to_string(), ..Default::default() }).await.unwrap();
        assert!(state.delete_rule(DeleteRuleRequest { name: "my-rule".to_string(), event_bus_name: None }).await.is_ok());
    }

    #[tokio::test]
    async fn test_delete_rule_bus_not_found() {
        let state = make_state();
        let result = state.delete_rule(DeleteRuleRequest {
            name: "rule".to_string(),
            event_bus_name: Some("nope".to_string()),
        }).await;
        assert!(result.is_err());
    }

    // --- Extended coverage: targets ---

    #[tokio::test]
    async fn test_put_and_list_targets() {
        let state = make_state();
        state.put_rule(PutRuleRequest { name: "my-rule".to_string(), ..Default::default() }).await.unwrap();
        state.put_targets(PutTargetsRequest {
            rule: "my-rule".to_string(),
            event_bus_name: None,
            targets: vec![
                Target { id: "t1".to_string(), arn: "arn:aws:lambda:us-east-1:123:function:f1".to_string(), role_arn: None, input: None, input_path: None },
                Target { id: "t2".to_string(), arn: "arn:aws:sqs:us-east-1:123:queue1".to_string(), role_arn: None, input: None, input_path: None },
            ],
        }).await.unwrap();
        let result = state.list_targets_by_rule(ListTargetsByRuleRequest {
            rule: "my-rule".to_string(),
            event_bus_name: None,
        }).await.unwrap();
        assert_eq!(result.targets.len(), 2);
    }

    #[tokio::test]
    async fn test_put_targets_bus_not_found() {
        let state = make_state();
        let result = state.put_targets(PutTargetsRequest {
            rule: "rule".to_string(),
            event_bus_name: Some("nope".to_string()),
            targets: vec![],
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_put_targets_rule_not_found() {
        let state = make_state();
        let result = state.put_targets(PutTargetsRequest {
            rule: "nope".to_string(),
            event_bus_name: None,
            targets: vec![],
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_remove_targets_success() {
        let state = make_state();
        state.put_rule(PutRuleRequest { name: "my-rule".to_string(), ..Default::default() }).await.unwrap();
        state.put_targets(PutTargetsRequest {
            rule: "my-rule".to_string(),
            event_bus_name: None,
            targets: vec![Target { id: "t1".to_string(), arn: "arn".to_string(), role_arn: None, input: None, input_path: None }],
        }).await.unwrap();
        state.remove_targets(RemoveTargetsRequest {
            rule: "my-rule".to_string(),
            event_bus_name: None,
            ids: vec!["t1".to_string()],
        }).await.unwrap();
        let result = state.list_targets_by_rule(ListTargetsByRuleRequest {
            rule: "my-rule".to_string(),
            event_bus_name: None,
        }).await.unwrap();
        assert!(result.targets.is_empty());
    }

    #[tokio::test]
    async fn test_remove_targets_bus_not_found() {
        let state = make_state();
        let result = state.remove_targets(RemoveTargetsRequest {
            rule: "rule".to_string(),
            event_bus_name: Some("nope".to_string()),
            ids: vec![],
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_remove_targets_rule_not_found() {
        let state = make_state();
        let result = state.remove_targets(RemoveTargetsRequest {
            rule: "nope".to_string(),
            event_bus_name: None,
            ids: vec![],
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_targets_by_rule_bus_not_found() {
        let state = make_state();
        let result = state.list_targets_by_rule(ListTargetsByRuleRequest {
            rule: "rule".to_string(),
            event_bus_name: Some("nope".to_string()),
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_targets_by_rule_rule_not_found() {
        let state = make_state();
        let result = state.list_targets_by_rule(ListTargetsByRuleRequest {
            rule: "nope".to_string(),
            event_bus_name: None,
        }).await;
        assert!(result.is_err());
    }

    // --- Extended coverage: put_events ---

    #[tokio::test]
    async fn test_put_events_success() {
        let state = make_state();
        let result = state.put_events(PutEventsRequest {
            entries: vec![PutEventsRequestEntry { event_bus_name: None }],
        }).await.unwrap();
        assert_eq!(result.failed_entry_count, 0);
        assert_eq!(result.entries.len(), 1);
        assert!(result.entries[0].event_id.is_some());
    }

    #[tokio::test]
    async fn test_put_events_nonexistent_bus() {
        let state = make_state();
        let result = state.put_events(PutEventsRequest {
            entries: vec![PutEventsRequestEntry { event_bus_name: Some("nope".to_string()) }],
        }).await.unwrap();
        assert_eq!(result.failed_entry_count, 1);
        assert!(result.entries[0].error_code.is_some());
    }

    #[tokio::test]
    async fn test_put_events_mixed() {
        let state = make_state();
        let result = state.put_events(PutEventsRequest {
            entries: vec![
                PutEventsRequestEntry { event_bus_name: None },
                PutEventsRequestEntry { event_bus_name: Some("nope".to_string()) },
            ],
        }).await.unwrap();
        assert_eq!(result.failed_entry_count, 1);
        assert_eq!(result.entries.len(), 2);
    }

    // --- Extended coverage: tag operations ---

    #[tokio::test]
    async fn test_tag_and_list_tags_for_bus() {
        let state = make_state();
        let bus_arn = format!("arn:aws:events:us-east-1:123456789012:event-bus/default");
        state.tag_resource(TagResourceRequest {
            resource_arn: bus_arn.clone(),
            tags: vec![Tag { key: "env".to_string(), value: "prod".to_string() }],
        }).await.unwrap();
        let result = state.list_tags_for_resource(ListTagsForResourceRequest {
            resource_arn: bus_arn,
        }).await.unwrap();
        assert_eq!(result.tags.len(), 1);
    }

    #[tokio::test]
    async fn test_tag_rule() {
        let state = make_state();
        let rule_resp = state.put_rule(PutRuleRequest { name: "my-rule".to_string(), ..Default::default() }).await.unwrap();
        state.tag_resource(TagResourceRequest {
            resource_arn: rule_resp.rule_arn.clone(),
            tags: vec![Tag { key: "team".to_string(), value: "infra".to_string() }],
        }).await.unwrap();
        let result = state.list_tags_for_resource(ListTagsForResourceRequest {
            resource_arn: rule_resp.rule_arn,
        }).await.unwrap();
        assert_eq!(result.tags.len(), 1);
    }

    #[tokio::test]
    async fn test_untag_bus() {
        let state = make_state();
        let bus_arn = format!("arn:aws:events:us-east-1:123456789012:event-bus/default");
        state.tag_resource(TagResourceRequest {
            resource_arn: bus_arn.clone(),
            tags: vec![
                Tag { key: "a".to_string(), value: "1".to_string() },
                Tag { key: "b".to_string(), value: "2".to_string() },
            ],
        }).await.unwrap();
        state.untag_resource(UntagResourceRequest {
            resource_arn: bus_arn.clone(),
            tag_keys: vec!["a".to_string()],
        }).await.unwrap();
        let result = state.list_tags_for_resource(ListTagsForResourceRequest {
            resource_arn: bus_arn,
        }).await.unwrap();
        assert_eq!(result.tags.len(), 1);
        assert_eq!(result.tags[0].key, "b");
    }

    #[tokio::test]
    async fn test_tag_resource_not_found() {
        let state = make_state();
        let result = state.tag_resource(TagResourceRequest {
            resource_arn: "arn:fake".to_string(),
            tags: vec![Tag { key: "k".to_string(), value: "v".to_string() }],
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_untag_resource_not_found() {
        let state = make_state();
        let result = state.untag_resource(UntagResourceRequest {
            resource_arn: "arn:fake".to_string(),
            tag_keys: vec!["k".to_string()],
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_tags_for_resource_not_found() {
        let state = make_state();
        let result = state.list_tags_for_resource(ListTagsForResourceRequest {
            resource_arn: "arn:fake".to_string(),
        }).await;
        assert!(result.is_err());
    }
}
