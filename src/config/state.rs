use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;
use uuid::Uuid;

use super::error::ConfigError;
use super::types::*;

struct StoredEvaluation {
    compliance_resource_id: String,
    compliance_resource_type: String,
    compliance_type: String,
    ordering_timestamp: f64,
    annotation: Option<String>,
    recorded_time: f64,
}

struct RecorderEntry {
    recorder: ConfigurationRecorder,
    recording: bool,
    start_time: Option<f64>,
    stop_time: Option<f64>,
}

struct ConfigStateInner {
    recorder: Option<RecorderEntry>,
    delivery_channel: Option<DeliveryChannel>,
    config_rules: HashMap<String, ConfigRule>,
    evaluations: HashMap<String, Vec<StoredEvaluation>>,
    tags: HashMap<String, HashMap<String, String>>,
    account_id: String,
    region: String,
}

pub struct ConfigState {
    inner: Arc<Mutex<ConfigStateInner>>,
}

impl ConfigState {
    pub fn new(account_id: String, region: String) -> Self {
        ConfigState {
            inner: Arc::new(Mutex::new(ConfigStateInner {
                recorder: None,
                delivery_channel: None,
                config_rules: HashMap::new(),
                evaluations: HashMap::new(),
                tags: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    fn now() -> f64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or(std::time::Duration::from_secs(0))
            .as_secs_f64()
    }

    // --- Configuration Recorder operations ---

    pub async fn put_configuration_recorder(
        &self,
        req: PutConfigurationRecorderRequest,
    ) -> Result<(), ConfigError> {
        let mut state = self.inner.lock().await;
        let mut recorder = req.configuration_recorder;
        let name = recorder.name.clone().unwrap_or_else(|| "default".to_string());
        recorder.name = Some(name.clone());

        if let Some(ref existing) = state.recorder {
            let existing_name = existing.recorder.name.as_deref().unwrap_or("default");
            if existing_name != name {
                return Err(ConfigError::MaxNumberOfConfigurationRecordersExceededException(
                    "You have reached the limit of the number of configuration recorders you can create.".to_string(),
                ));
            }
            // Update existing recorder, preserve recording state
            let entry = state.recorder.as_mut().unwrap();
            entry.recorder = recorder;
        } else {
            state.recorder = Some(RecorderEntry {
                recorder,
                recording: false,
                start_time: None,
                stop_time: None,
            });
        }
        Ok(())
    }

    pub async fn describe_configuration_recorders(
        &self,
        req: DescribeConfigurationRecordersRequest,
    ) -> Result<DescribeConfigurationRecordersResponse, ConfigError> {
        let state = self.inner.lock().await;
        let recorders = if let Some(ref entry) = state.recorder {
            let recorder_name = entry.recorder.name.as_deref().unwrap_or("default");
            if let Some(ref names) = req.configuration_recorder_names {
                if names.iter().any(|n| n == recorder_name) {
                    vec![entry.recorder.clone()]
                } else {
                    let missing = &names[0];
                    return Err(ConfigError::NoSuchConfigurationRecorderException(
                        format!("Cannot find configuration recorder with the specified name '{missing}'."),
                    ));
                }
            } else {
                vec![entry.recorder.clone()]
            }
        } else if let Some(ref names) = req.configuration_recorder_names {
            if !names.is_empty() {
                let missing = &names[0];
                return Err(ConfigError::NoSuchConfigurationRecorderException(
                    format!("Cannot find configuration recorder with the specified name '{missing}'."),
                ));
            }
            vec![]
        } else {
            vec![]
        };
        Ok(DescribeConfigurationRecordersResponse {
            configuration_recorders: recorders,
        })
    }

    pub async fn delete_configuration_recorder(
        &self,
        req: DeleteConfigurationRecorderRequest,
    ) -> Result<(), ConfigError> {
        let mut state = self.inner.lock().await;
        if let Some(ref entry) = state.recorder {
            let recorder_name = entry.recorder.name.as_deref().unwrap_or("default");
            if recorder_name == req.configuration_recorder_name {
                state.recorder = None;
                return Ok(());
            }
        }
        Err(ConfigError::NoSuchConfigurationRecorderException(
            format!(
                "Cannot find configuration recorder with the specified name '{}'.",
                req.configuration_recorder_name
            ),
        ))
    }

    pub async fn describe_configuration_recorder_status(
        &self,
        req: DescribeConfigurationRecorderStatusRequest,
    ) -> Result<DescribeConfigurationRecorderStatusResponse, ConfigError> {
        let state = self.inner.lock().await;
        let statuses = if let Some(ref entry) = state.recorder {
            let recorder_name = entry.recorder.name.as_deref().unwrap_or("default").to_string();
            if let Some(ref names) = req.configuration_recorder_names {
                if names.iter().any(|n| n == &recorder_name) {
                    vec![ConfigurationRecorderStatus {
                        name: recorder_name,
                        recording: entry.recording,
                        last_start_time: entry.start_time,
                        last_stop_time: entry.stop_time,
                        last_status: if entry.recording {
                            Some("SUCCESS".to_string())
                        } else {
                            None
                        },
                    }]
                } else {
                    let missing = &names[0];
                    return Err(ConfigError::NoSuchConfigurationRecorderException(
                        format!("Cannot find configuration recorder with the specified name '{missing}'."),
                    ));
                }
            } else {
                vec![ConfigurationRecorderStatus {
                    name: recorder_name,
                    recording: entry.recording,
                    last_start_time: entry.start_time,
                    last_stop_time: entry.stop_time,
                    last_status: if entry.recording {
                        Some("SUCCESS".to_string())
                    } else {
                        None
                    },
                }]
            }
        } else if let Some(ref names) = req.configuration_recorder_names {
            if !names.is_empty() {
                let missing = &names[0];
                return Err(ConfigError::NoSuchConfigurationRecorderException(
                    format!("Cannot find configuration recorder with the specified name '{missing}'."),
                ));
            }
            vec![]
        } else {
            vec![]
        };
        Ok(DescribeConfigurationRecorderStatusResponse {
            configuration_recorders_status: statuses,
        })
    }

    pub async fn start_configuration_recorder(
        &self,
        req: StartConfigurationRecorderRequest,
    ) -> Result<(), ConfigError> {
        let mut state = self.inner.lock().await;
        if let Some(ref mut entry) = state.recorder {
            let recorder_name = entry.recorder.name.as_deref().unwrap_or("default");
            if recorder_name == req.configuration_recorder_name {
                entry.recording = true;
                entry.start_time = Some(Self::now());
                return Ok(());
            }
        }
        Err(ConfigError::NoSuchConfigurationRecorderException(
            format!(
                "Cannot find configuration recorder with the specified name '{}'.",
                req.configuration_recorder_name
            ),
        ))
    }

    pub async fn stop_configuration_recorder(
        &self,
        req: StopConfigurationRecorderRequest,
    ) -> Result<(), ConfigError> {
        let mut state = self.inner.lock().await;
        if let Some(ref mut entry) = state.recorder {
            let recorder_name = entry.recorder.name.as_deref().unwrap_or("default");
            if recorder_name == req.configuration_recorder_name {
                entry.recording = false;
                entry.stop_time = Some(Self::now());
                return Ok(());
            }
        }
        Err(ConfigError::NoSuchConfigurationRecorderException(
            format!(
                "Cannot find configuration recorder with the specified name '{}'.",
                req.configuration_recorder_name
            ),
        ))
    }

    // --- Delivery Channel operations ---

    pub async fn put_delivery_channel(
        &self,
        req: PutDeliveryChannelRequest,
    ) -> Result<(), ConfigError> {
        let mut state = self.inner.lock().await;
        let mut channel = req.delivery_channel;
        let name = channel.name.clone().unwrap_or_else(|| "default".to_string());
        channel.name = Some(name.clone());

        if let Some(ref existing) = state.delivery_channel {
            let existing_name = existing.name.as_deref().unwrap_or("default");
            if existing_name != name {
                return Err(ConfigError::MaxNumberOfDeliveryChannelsExceededException(
                    "You have reached the limit of the number of delivery channels you can create.".to_string(),
                ));
            }
        }
        state.delivery_channel = Some(channel);
        Ok(())
    }

    pub async fn describe_delivery_channels(
        &self,
        req: DescribeDeliveryChannelsRequest,
    ) -> Result<DescribeDeliveryChannelsResponse, ConfigError> {
        let state = self.inner.lock().await;
        let channels = if let Some(ref channel) = state.delivery_channel {
            let channel_name = channel.name.as_deref().unwrap_or("default");
            if let Some(ref names) = req.delivery_channel_names {
                if names.iter().any(|n| n == channel_name) {
                    vec![channel.clone()]
                } else {
                    let missing = &names[0];
                    return Err(ConfigError::NoSuchDeliveryChannelException(
                        format!("Cannot find delivery channel with the specified name '{missing}'."),
                    ));
                }
            } else {
                vec![channel.clone()]
            }
        } else if let Some(ref names) = req.delivery_channel_names {
            if !names.is_empty() {
                let missing = &names[0];
                return Err(ConfigError::NoSuchDeliveryChannelException(
                    format!("Cannot find delivery channel with the specified name '{missing}'."),
                ));
            }
            vec![]
        } else {
            vec![]
        };
        Ok(DescribeDeliveryChannelsResponse {
            delivery_channels: channels,
        })
    }

    pub async fn delete_delivery_channel(
        &self,
        req: DeleteDeliveryChannelRequest,
    ) -> Result<(), ConfigError> {
        let mut state = self.inner.lock().await;
        if let Some(ref channel) = state.delivery_channel {
            let channel_name = channel.name.as_deref().unwrap_or("default");
            if channel_name == req.delivery_channel_name {
                state.delivery_channel = None;
                return Ok(());
            }
        }
        Err(ConfigError::NoSuchDeliveryChannelException(
            format!(
                "Cannot find delivery channel with the specified name '{}'.",
                req.delivery_channel_name
            ),
        ))
    }

    // --- Config Rule operations ---

    pub async fn put_config_rule(
        &self,
        req: PutConfigRuleRequest,
    ) -> Result<(), ConfigError> {
        let mut state = self.inner.lock().await;
        let mut rule = req.config_rule;
        let name = rule.config_rule_name.clone();

        if state.config_rules.contains_key(&name) {
            // Update existing rule, preserve ARN and ID
            let existing = state.config_rules.get(&name).unwrap();
            rule.config_rule_arn = existing.config_rule_arn.clone();
            rule.config_rule_id = existing.config_rule_id.clone();
        } else {
            let rule_id = format!("config-rule-{}", Uuid::new_v4());
            rule.config_rule_arn = Some(format!(
                "arn:aws:config:{}:{}:config-rule/{}",
                state.region, state.account_id, rule_id
            ));
            rule.config_rule_id = Some(rule_id);
        }
        rule.config_rule_state = Some("ACTIVE".to_string());

        // Handle tags
        if let Some(tags) = req.tags {
            let arn = rule.config_rule_arn.clone().unwrap();
            let tag_map = state.tags.entry(arn).or_default();
            for tag in tags {
                tag_map.insert(tag.key, tag.value);
            }
        }

        state.config_rules.insert(name, rule);
        Ok(())
    }

    pub async fn describe_config_rules(
        &self,
        req: DescribeConfigRulesRequest,
    ) -> Result<DescribeConfigRulesResponse, ConfigError> {
        let state = self.inner.lock().await;
        let mut rules: Vec<ConfigRule> = if let Some(ref names) = req.config_rule_names {
            let mut result = Vec::new();
            for name in names {
                match state.config_rules.get(name) {
                    Some(rule) => result.push(rule.clone()),
                    None => {
                        return Err(ConfigError::NoSuchConfigRuleException(
                            format!("The ConfigRule '{name}' provided in the request is invalid."),
                        ));
                    }
                }
            }
            result
        } else {
            state.config_rules.values().cloned().collect()
        };
        rules.sort_by(|a, b| a.config_rule_name.cmp(&b.config_rule_name));
        Ok(DescribeConfigRulesResponse {
            config_rules: rules,
            next_token: None,
        })
    }

    pub async fn delete_config_rule(
        &self,
        req: DeleteConfigRuleRequest,
    ) -> Result<(), ConfigError> {
        let mut state = self.inner.lock().await;
        if state.config_rules.remove(&req.config_rule_name).is_none() {
            return Err(ConfigError::NoSuchConfigRuleException(
                format!(
                    "The ConfigRule '{}' provided in the request is invalid.",
                    req.config_rule_name
                ),
            ));
        }
        // Clean up evaluations for this rule
        state.evaluations.remove(&req.config_rule_name);
        Ok(())
    }

    // --- Evaluation operations ---

    pub async fn put_evaluations(
        &self,
        req: PutEvaluationsRequest,
    ) -> Result<PutEvaluationsResponse, ConfigError> {
        let mut state = self.inner.lock().await;
        let now = Self::now();
        // The result_token typically maps to a rule name in real AWS,
        // but for simplicity we use it as the key
        let rule_name = req.result_token.clone();

        let stored: Vec<StoredEvaluation> = req
            .evaluations
            .into_iter()
            .map(|e| StoredEvaluation {
                compliance_resource_id: e.compliance_resource_id,
                compliance_resource_type: e.compliance_resource_type,
                compliance_type: e.compliance_type,
                ordering_timestamp: e.ordering_timestamp,
                annotation: e.annotation,
                recorded_time: now,
            })
            .collect();

        state
            .evaluations
            .entry(rule_name)
            .or_default()
            .extend(stored);

        Ok(PutEvaluationsResponse {
            failed_evaluations: vec![],
        })
    }

    pub async fn get_compliance_details_by_config_rule(
        &self,
        req: GetComplianceDetailsByConfigRuleRequest,
    ) -> Result<GetComplianceDetailsByConfigRuleResponse, ConfigError> {
        let state = self.inner.lock().await;
        let rule_name = &req.config_rule_name;

        // Verify rule exists
        if !state.config_rules.contains_key(rule_name) {
            return Err(ConfigError::NoSuchConfigRuleException(
                format!("The ConfigRule '{rule_name}' provided in the request is invalid."),
            ));
        }

        let evals = state.evaluations.get(rule_name);
        let mut results: Vec<EvaluationResult> = match evals {
            Some(stored) => stored
                .iter()
                .filter(|e| {
                    if let Some(ref types) = req.compliance_types {
                        types.contains(&e.compliance_type)
                    } else {
                        true
                    }
                })
                .map(|e| EvaluationResult {
                    evaluation_result_identifier: EvaluationResultIdentifier {
                        evaluation_result_qualifier: EvaluationResultQualifier {
                            config_rule_name: rule_name.clone(),
                            resource_type: e.compliance_resource_type.clone(),
                            resource_id: e.compliance_resource_id.clone(),
                        },
                        ordering_timestamp: e.ordering_timestamp,
                    },
                    compliance_type: e.compliance_type.clone(),
                    result_recorded_time: e.recorded_time,
                    config_rule_invoked_time: e.recorded_time,
                    annotation: e.annotation.clone(),
                })
                .collect(),
            None => vec![],
        };

        let limit = req.limit.unwrap_or(100);
        results.truncate(limit);

        Ok(GetComplianceDetailsByConfigRuleResponse {
            evaluation_results: results,
            next_token: None,
        })
    }

    pub async fn describe_compliance_by_config_rule(
        &self,
        req: DescribeComplianceByConfigRuleRequest,
    ) -> Result<DescribeComplianceByConfigRuleResponse, ConfigError> {
        let state = self.inner.lock().await;

        let rule_names: Vec<String> = if let Some(ref names) = req.config_rule_names {
            names.clone()
        } else {
            let mut names: Vec<String> = state.config_rules.keys().cloned().collect();
            names.sort();
            names
        };

        let mut compliance_list = Vec::new();
        for rule_name in &rule_names {
            if !state.config_rules.contains_key(rule_name) {
                return Err(ConfigError::NoSuchConfigRuleException(
                    format!("The ConfigRule '{rule_name}' provided in the request is invalid."),
                ));
            }

            let evals = state.evaluations.get(rule_name);
            let (compliance_type, non_compliant_count) = match evals {
                Some(stored) if !stored.is_empty() => {
                    let non_compliant = stored
                        .iter()
                        .filter(|e| e.compliance_type == "NON_COMPLIANT")
                        .count();
                    if non_compliant > 0 {
                        ("NON_COMPLIANT".to_string(), non_compliant as i32)
                    } else {
                        ("COMPLIANT".to_string(), 0)
                    }
                }
                _ => ("INSUFFICIENT_DATA".to_string(), 0),
            };

            if let Some(ref filter_types) = req.compliance_types {
                if !filter_types.contains(&compliance_type) {
                    continue;
                }
            }

            compliance_list.push(ComplianceByConfigRule {
                config_rule_name: rule_name.clone(),
                compliance: Compliance {
                    compliance_type: compliance_type.clone(),
                    compliance_contributor_count: if compliance_type == "NON_COMPLIANT" {
                        Some(ComplianceContributorCount {
                            capped_count: non_compliant_count,
                            cap_exceeded: false,
                        })
                    } else {
                        Some(ComplianceContributorCount {
                            capped_count: 0,
                            cap_exceeded: false,
                        })
                    },
                },
            });
        }

        Ok(DescribeComplianceByConfigRuleResponse {
            compliance_by_config_rules: compliance_list,
            next_token: None,
        })
    }

    pub async fn describe_compliance_by_resource(
        &self,
        req: DescribeComplianceByResourceRequest,
    ) -> Result<DescribeComplianceByResourceResponse, ConfigError> {
        let state = self.inner.lock().await;

        // Collect all unique (resource_type, resource_id) pairs from evaluations
        let mut resource_compliance: HashMap<(String, String), Vec<String>> = HashMap::new();

        for evals in state.evaluations.values() {
            for eval in evals {
                let key = (
                    eval.compliance_resource_type.clone(),
                    eval.compliance_resource_id.clone(),
                );
                resource_compliance
                    .entry(key)
                    .or_default()
                    .push(eval.compliance_type.clone());
            }
        }

        let mut results = Vec::new();
        for ((res_type, res_id), types) in &resource_compliance {
            // Apply filters
            if let Some(ref filter_type) = req.resource_type {
                if res_type != filter_type {
                    continue;
                }
            }
            if let Some(ref filter_id) = req.resource_id {
                if res_id != filter_id {
                    continue;
                }
            }

            let non_compliant = types.iter().filter(|t| *t == "NON_COMPLIANT").count();
            let compliance_type = if non_compliant > 0 {
                "NON_COMPLIANT".to_string()
            } else {
                "COMPLIANT".to_string()
            };

            if let Some(ref filter_compliance) = req.compliance_types {
                if !filter_compliance.contains(&compliance_type) {
                    continue;
                }
            }

            results.push(ComplianceByResource {
                resource_type: res_type.clone(),
                resource_id: res_id.clone(),
                compliance: Compliance {
                    compliance_type: compliance_type.clone(),
                    compliance_contributor_count: if compliance_type == "NON_COMPLIANT" {
                        Some(ComplianceContributorCount {
                            capped_count: non_compliant as i32,
                            cap_exceeded: false,
                        })
                    } else {
                        Some(ComplianceContributorCount {
                            capped_count: 0,
                            cap_exceeded: false,
                        })
                    },
                },
            });
        }

        results.sort_by(|a, b| {
            a.resource_type
                .cmp(&b.resource_type)
                .then(a.resource_id.cmp(&b.resource_id))
        });

        let limit = req.limit.unwrap_or(100);
        results.truncate(limit);

        Ok(DescribeComplianceByResourceResponse {
            compliance_by_resources: results,
            next_token: None,
        })
    }

    // --- Tag operations ---

    pub async fn tag_resource(
        &self,
        req: TagResourceRequest,
    ) -> Result<(), ConfigError> {
        let mut state = self.inner.lock().await;

        // Validate the ARN refers to a known resource
        let arn = &req.resource_arn;
        let found = self.arn_exists_in_state(&state, arn);
        if !found {
            return Err(ConfigError::ResourceNotFoundException(
                format!("ResourceArn '{arn}' does not exist."),
            ));
        }

        let tag_map = state.tags.entry(arn.clone()).or_default();
        for tag in req.tags {
            tag_map.insert(tag.key, tag.value);
        }
        Ok(())
    }

    pub async fn untag_resource(
        &self,
        req: UntagResourceRequest,
    ) -> Result<(), ConfigError> {
        let mut state = self.inner.lock().await;

        let arn = &req.resource_arn;
        let found = self.arn_exists_in_state(&state, arn);
        if !found {
            return Err(ConfigError::ResourceNotFoundException(
                format!("ResourceArn '{arn}' does not exist."),
            ));
        }

        if let Some(tag_map) = state.tags.get_mut(arn) {
            for key in &req.tag_keys {
                tag_map.remove(key);
            }
        }
        Ok(())
    }

    pub async fn list_tags_for_resource(
        &self,
        req: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, ConfigError> {
        let state = self.inner.lock().await;

        let arn = &req.resource_arn;
        let found = self.arn_exists_in_state(&state, arn);
        if !found {
            return Err(ConfigError::ResourceNotFoundException(
                format!("ResourceArn '{arn}' does not exist."),
            ));
        }

        let mut tags: Vec<Tag> = state
            .tags
            .get(arn)
            .map(|m| {
                m.iter()
                    .map(|(k, v)| Tag {
                        key: k.clone(),
                        value: v.clone(),
                    })
                    .collect()
            })
            .unwrap_or_default();

        tags.sort_by(|a, b| a.key.cmp(&b.key));

        let limit = req.limit.unwrap_or(100);
        tags.truncate(limit);

        Ok(ListTagsForResourceResponse {
            tags,
            next_token: None,
        })
    }

    fn arn_exists_in_state(&self, state: &ConfigStateInner, arn: &str) -> bool {
        // Check recorder ARN
        if let Some(ref entry) = state.recorder {
            let recorder_name = entry.recorder.name.as_deref().unwrap_or("default");
            let recorder_arn = format!(
                "arn:aws:config:{}:{}:configuration-recorder/{}",
                state.region, state.account_id, recorder_name
            );
            if recorder_arn == arn {
                return true;
            }
        }

        // Check config rule ARNs
        for rule in state.config_rules.values() {
            if let Some(ref rule_arn) = rule.config_rule_arn {
                if rule_arn == arn {
                    return true;
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_state() -> ConfigState {
        ConfigState::new("123456789012".to_string(), "us-east-1".to_string())
    }

    #[tokio::test]
    async fn test_new_state() {
        let _state = make_state();
    }

    #[tokio::test]
    async fn test_put_configuration_recorder() {
        let state = make_state();
        let req = PutConfigurationRecorderRequest {
            configuration_recorder: ConfigurationRecorder {
                name: Some("default".to_string()),
                role_arn: Some("arn:aws:iam::123456789012:role/config".to_string()),
                ..Default::default()
            },
        };
        assert!(state.put_configuration_recorder(req).await.is_ok());
    }

    #[tokio::test]
    async fn test_describe_configuration_recorders() {
        let state = make_state();
        state.put_configuration_recorder(PutConfigurationRecorderRequest {
            configuration_recorder: ConfigurationRecorder {
                name: Some("default".to_string()),
                ..Default::default()
            },
        }).await.unwrap();
        let result = state.describe_configuration_recorders(DescribeConfigurationRecordersRequest::default()).await.unwrap();
        assert_eq!(result.configuration_recorders.len(), 1);
    }

    #[tokio::test]
    async fn test_delete_configuration_recorder() {
        let state = make_state();
        state.put_configuration_recorder(PutConfigurationRecorderRequest {
            configuration_recorder: ConfigurationRecorder {
                name: Some("default".to_string()),
                ..Default::default()
            },
        }).await.unwrap();
        let req = DeleteConfigurationRecorderRequest { configuration_recorder_name: "default".to_string() };
        assert!(state.delete_configuration_recorder(req).await.is_ok());
    }

    #[tokio::test]
    async fn test_delete_configuration_recorder_not_found() {
        let state = make_state();
        let req = DeleteConfigurationRecorderRequest { configuration_recorder_name: "nope".to_string() };
        assert!(state.delete_configuration_recorder(req).await.is_err());
    }

    #[tokio::test]
    async fn test_start_stop_recorder() {
        let state = make_state();
        state.put_configuration_recorder(PutConfigurationRecorderRequest {
            configuration_recorder: ConfigurationRecorder {
                name: Some("default".to_string()),
                ..Default::default()
            },
        }).await.unwrap();
        assert!(state.start_configuration_recorder(StartConfigurationRecorderRequest {
            configuration_recorder_name: "default".to_string(),
        }).await.is_ok());
        assert!(state.stop_configuration_recorder(StopConfigurationRecorderRequest {
            configuration_recorder_name: "default".to_string(),
        }).await.is_ok());
    }

    #[tokio::test]
    async fn test_describe_configuration_recorder_status() {
        let state = make_state();
        state.put_configuration_recorder(PutConfigurationRecorderRequest {
            configuration_recorder: ConfigurationRecorder {
                name: Some("default".to_string()),
                ..Default::default()
            },
        }).await.unwrap();
        let result = state.describe_configuration_recorder_status(DescribeConfigurationRecorderStatusRequest::default()).await.unwrap();
        assert_eq!(result.configuration_recorders_status.len(), 1);
    }

    #[tokio::test]
    async fn test_put_delivery_channel() {
        let state = make_state();
        let req = PutDeliveryChannelRequest {
            delivery_channel: DeliveryChannel {
                name: Some("default".to_string()),
                s3_bucket_name: Some("my-bucket".to_string()),
                ..Default::default()
            },
        };
        assert!(state.put_delivery_channel(req).await.is_ok());
    }

    #[tokio::test]
    async fn test_describe_delivery_channels() {
        let state = make_state();
        state.put_delivery_channel(PutDeliveryChannelRequest {
            delivery_channel: DeliveryChannel {
                name: Some("default".to_string()),
                ..Default::default()
            },
        }).await.unwrap();
        let result = state.describe_delivery_channels(DescribeDeliveryChannelsRequest::default()).await.unwrap();
        assert_eq!(result.delivery_channels.len(), 1);
    }

    #[tokio::test]
    async fn test_delete_delivery_channel() {
        let state = make_state();
        state.put_delivery_channel(PutDeliveryChannelRequest {
            delivery_channel: DeliveryChannel {
                name: Some("default".to_string()),
                ..Default::default()
            },
        }).await.unwrap();
        assert!(state.delete_delivery_channel(DeleteDeliveryChannelRequest { delivery_channel_name: "default".to_string() }).await.is_ok());
    }

    #[tokio::test]
    async fn test_put_config_rule() {
        let state = make_state();
        let req = PutConfigRuleRequest {
            config_rule: ConfigRule {
                config_rule_name: "my-rule".to_string(),
                source: Source {
                    owner: "AWS".to_string(),
                    source_identifier: "S3_BUCKET_VERSIONING_ENABLED".to_string(),
                    ..Default::default()
                },
                ..Default::default()
            },
            tags: None,
        };
        assert!(state.put_config_rule(req).await.is_ok());
    }

    #[tokio::test]
    async fn test_describe_config_rules() {
        let state = make_state();
        state.put_config_rule(PutConfigRuleRequest {
            config_rule: ConfigRule {
                config_rule_name: "r1".to_string(),
                source: Source { owner: "AWS".to_string(), source_identifier: "test".to_string(), ..Default::default() },
                ..Default::default()
            },
            tags: None,
        }).await.unwrap();
        let result = state.describe_config_rules(DescribeConfigRulesRequest::default()).await.unwrap();
        assert_eq!(result.config_rules.len(), 1);
    }

    #[tokio::test]
    async fn test_delete_config_rule() {
        let state = make_state();
        state.put_config_rule(PutConfigRuleRequest {
            config_rule: ConfigRule {
                config_rule_name: "r1".to_string(),
                source: Source { owner: "AWS".to_string(), source_identifier: "test".to_string(), ..Default::default() },
                ..Default::default()
            },
            tags: None,
        }).await.unwrap();
        assert!(state.delete_config_rule(DeleteConfigRuleRequest { config_rule_name: "r1".to_string() }).await.is_ok());
    }

    #[tokio::test]
    async fn test_delete_config_rule_not_found() {
        let state = make_state();
        assert!(state.delete_config_rule(DeleteConfigRuleRequest { config_rule_name: "nope".to_string() }).await.is_err());
    }

    #[tokio::test]
    async fn test_put_evaluations() {
        let state = make_state();
        state.put_config_rule(PutConfigRuleRequest {
            config_rule: ConfigRule {
                config_rule_name: "r1".to_string(),
                source: Source { owner: "AWS".to_string(), source_identifier: "test".to_string(), ..Default::default() },
                ..Default::default()
            },
            tags: None,
        }).await.unwrap();
        let req = PutEvaluationsRequest {
            evaluations: vec![Evaluation {
                compliance_resource_id: "bucket-1".to_string(),
                compliance_resource_type: "AWS::S3::Bucket".to_string(),
                compliance_type: "COMPLIANT".to_string(),
                ordering_timestamp: 1234567890.0,
                annotation: None,
            }],
            result_token: "r1".to_string(),
        };
        assert!(state.put_evaluations(req).await.is_ok());
    }

    #[tokio::test]
    async fn test_describe_compliance_by_config_rule() {
        let state = make_state();
        state.put_config_rule(PutConfigRuleRequest {
            config_rule: ConfigRule {
                config_rule_name: "r1".to_string(),
                source: Source { owner: "AWS".to_string(), source_identifier: "test".to_string(), ..Default::default() },
                ..Default::default()
            },
            tags: None,
        }).await.unwrap();
        let result = state.describe_compliance_by_config_rule(DescribeComplianceByConfigRuleRequest::default()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_describe_compliance_by_resource() {
        let state = make_state();
        let result = state.describe_compliance_by_resource(DescribeComplianceByResourceRequest::default()).await;
        assert!(result.is_ok());
    }

    // --- Extended coverage: configuration recorder edge cases ---

    #[tokio::test]
    async fn test_put_configuration_recorder_no_name_defaults() {
        let state = make_state();
        let req = PutConfigurationRecorderRequest {
            configuration_recorder: ConfigurationRecorder {
                name: None,
                role_arn: Some("arn:aws:iam::123456789012:role/config".to_string()),
                ..Default::default()
            },
        };
        state.put_configuration_recorder(req).await.unwrap();
        let result = state.describe_configuration_recorders(DescribeConfigurationRecordersRequest::default()).await.unwrap();
        assert_eq!(result.configuration_recorders.len(), 1);
        assert_eq!(result.configuration_recorders[0].name.as_deref(), Some("default"));
    }

    #[tokio::test]
    async fn test_put_configuration_recorder_update_existing() {
        let state = make_state();
        state.put_configuration_recorder(PutConfigurationRecorderRequest {
            configuration_recorder: ConfigurationRecorder {
                name: Some("default".to_string()),
                role_arn: Some("arn:old".to_string()),
                ..Default::default()
            },
        }).await.unwrap();
        // Update the same recorder
        state.put_configuration_recorder(PutConfigurationRecorderRequest {
            configuration_recorder: ConfigurationRecorder {
                name: Some("default".to_string()),
                role_arn: Some("arn:new".to_string()),
                ..Default::default()
            },
        }).await.unwrap();
        let result = state.describe_configuration_recorders(DescribeConfigurationRecordersRequest::default()).await.unwrap();
        assert_eq!(result.configuration_recorders.len(), 1);
        assert_eq!(result.configuration_recorders[0].role_arn.as_deref(), Some("arn:new"));
    }

    #[tokio::test]
    async fn test_put_configuration_recorder_different_name_exceeds_limit() {
        let state = make_state();
        state.put_configuration_recorder(PutConfigurationRecorderRequest {
            configuration_recorder: ConfigurationRecorder {
                name: Some("first".to_string()),
                ..Default::default()
            },
        }).await.unwrap();
        let result = state.put_configuration_recorder(PutConfigurationRecorderRequest {
            configuration_recorder: ConfigurationRecorder {
                name: Some("second".to_string()),
                ..Default::default()
            },
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_describe_configuration_recorders_by_name() {
        let state = make_state();
        state.put_configuration_recorder(PutConfigurationRecorderRequest {
            configuration_recorder: ConfigurationRecorder {
                name: Some("default".to_string()),
                ..Default::default()
            },
        }).await.unwrap();
        let result = state.describe_configuration_recorders(DescribeConfigurationRecordersRequest {
            configuration_recorder_names: Some(vec!["default".to_string()]),
        }).await.unwrap();
        assert_eq!(result.configuration_recorders.len(), 1);
    }

    #[tokio::test]
    async fn test_describe_configuration_recorders_by_wrong_name() {
        let state = make_state();
        state.put_configuration_recorder(PutConfigurationRecorderRequest {
            configuration_recorder: ConfigurationRecorder {
                name: Some("default".to_string()),
                ..Default::default()
            },
        }).await.unwrap();
        let result = state.describe_configuration_recorders(DescribeConfigurationRecordersRequest {
            configuration_recorder_names: Some(vec!["wrong".to_string()]),
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_describe_configuration_recorders_empty_no_recorder() {
        let state = make_state();
        let result = state.describe_configuration_recorders(DescribeConfigurationRecordersRequest {
            configuration_recorder_names: Some(vec!["missing".to_string()]),
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_describe_configuration_recorders_empty_names_no_recorder() {
        let state = make_state();
        let result = state.describe_configuration_recorders(DescribeConfigurationRecordersRequest {
            configuration_recorder_names: Some(vec![]),
        }).await.unwrap();
        assert!(result.configuration_recorders.is_empty());
    }

    #[tokio::test]
    async fn test_start_recorder_not_found() {
        let state = make_state();
        let result = state.start_configuration_recorder(StartConfigurationRecorderRequest {
            configuration_recorder_name: "nonexistent".to_string(),
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_stop_recorder_not_found() {
        let state = make_state();
        let result = state.stop_configuration_recorder(StopConfigurationRecorderRequest {
            configuration_recorder_name: "nonexistent".to_string(),
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_describe_recorder_status_recording_shows_success() {
        let state = make_state();
        state.put_configuration_recorder(PutConfigurationRecorderRequest {
            configuration_recorder: ConfigurationRecorder {
                name: Some("default".to_string()),
                ..Default::default()
            },
        }).await.unwrap();
        state.start_configuration_recorder(StartConfigurationRecorderRequest {
            configuration_recorder_name: "default".to_string(),
        }).await.unwrap();
        let result = state.describe_configuration_recorder_status(DescribeConfigurationRecorderStatusRequest::default()).await.unwrap();
        assert_eq!(result.configuration_recorders_status[0].recording, true);
        assert_eq!(result.configuration_recorders_status[0].last_status.as_deref(), Some("SUCCESS"));
        assert!(result.configuration_recorders_status[0].last_start_time.is_some());
    }

    #[tokio::test]
    async fn test_describe_recorder_status_by_name() {
        let state = make_state();
        state.put_configuration_recorder(PutConfigurationRecorderRequest {
            configuration_recorder: ConfigurationRecorder {
                name: Some("default".to_string()),
                ..Default::default()
            },
        }).await.unwrap();
        let result = state.describe_configuration_recorder_status(DescribeConfigurationRecorderStatusRequest {
            configuration_recorder_names: Some(vec!["default".to_string()]),
        }).await.unwrap();
        assert_eq!(result.configuration_recorders_status.len(), 1);
    }

    #[tokio::test]
    async fn test_describe_recorder_status_wrong_name() {
        let state = make_state();
        state.put_configuration_recorder(PutConfigurationRecorderRequest {
            configuration_recorder: ConfigurationRecorder {
                name: Some("default".to_string()),
                ..Default::default()
            },
        }).await.unwrap();
        let result = state.describe_configuration_recorder_status(DescribeConfigurationRecorderStatusRequest {
            configuration_recorder_names: Some(vec!["wrong".to_string()]),
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_describe_recorder_status_no_recorder_with_names() {
        let state = make_state();
        let result = state.describe_configuration_recorder_status(DescribeConfigurationRecorderStatusRequest {
            configuration_recorder_names: Some(vec!["missing".to_string()]),
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_describe_recorder_status_no_recorder_empty_names() {
        let state = make_state();
        let result = state.describe_configuration_recorder_status(DescribeConfigurationRecorderStatusRequest {
            configuration_recorder_names: Some(vec![]),
        }).await.unwrap();
        assert!(result.configuration_recorders_status.is_empty());
    }

    // --- Extended coverage: delivery channel edge cases ---

    #[tokio::test]
    async fn test_put_delivery_channel_no_name_defaults() {
        let state = make_state();
        state.put_delivery_channel(PutDeliveryChannelRequest {
            delivery_channel: DeliveryChannel {
                name: None,
                s3_bucket_name: Some("bucket".to_string()),
                ..Default::default()
            },
        }).await.unwrap();
        let result = state.describe_delivery_channels(DescribeDeliveryChannelsRequest::default()).await.unwrap();
        assert_eq!(result.delivery_channels[0].name.as_deref(), Some("default"));
    }

    #[tokio::test]
    async fn test_put_delivery_channel_different_name_exceeds_limit() {
        let state = make_state();
        state.put_delivery_channel(PutDeliveryChannelRequest {
            delivery_channel: DeliveryChannel {
                name: Some("first".to_string()),
                ..Default::default()
            },
        }).await.unwrap();
        let result = state.put_delivery_channel(PutDeliveryChannelRequest {
            delivery_channel: DeliveryChannel {
                name: Some("second".to_string()),
                ..Default::default()
            },
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_describe_delivery_channels_by_name() {
        let state = make_state();
        state.put_delivery_channel(PutDeliveryChannelRequest {
            delivery_channel: DeliveryChannel {
                name: Some("default".to_string()),
                ..Default::default()
            },
        }).await.unwrap();
        let result = state.describe_delivery_channels(DescribeDeliveryChannelsRequest {
            delivery_channel_names: Some(vec!["default".to_string()]),
        }).await.unwrap();
        assert_eq!(result.delivery_channels.len(), 1);
    }

    #[tokio::test]
    async fn test_describe_delivery_channels_wrong_name() {
        let state = make_state();
        state.put_delivery_channel(PutDeliveryChannelRequest {
            delivery_channel: DeliveryChannel {
                name: Some("default".to_string()),
                ..Default::default()
            },
        }).await.unwrap();
        let result = state.describe_delivery_channels(DescribeDeliveryChannelsRequest {
            delivery_channel_names: Some(vec!["wrong".to_string()]),
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_describe_delivery_channels_no_channel_with_names() {
        let state = make_state();
        let result = state.describe_delivery_channels(DescribeDeliveryChannelsRequest {
            delivery_channel_names: Some(vec!["missing".to_string()]),
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_describe_delivery_channels_no_channel_empty_names() {
        let state = make_state();
        let result = state.describe_delivery_channels(DescribeDeliveryChannelsRequest {
            delivery_channel_names: Some(vec![]),
        }).await.unwrap();
        assert!(result.delivery_channels.is_empty());
    }

    #[tokio::test]
    async fn test_delete_delivery_channel_not_found() {
        let state = make_state();
        let result = state.delete_delivery_channel(DeleteDeliveryChannelRequest {
            delivery_channel_name: "nonexistent".to_string(),
        }).await;
        assert!(result.is_err());
    }

    // --- Extended coverage: config rule edge cases ---

    #[tokio::test]
    async fn test_put_config_rule_update_preserves_arn_and_id() {
        let state = make_state();
        state.put_config_rule(PutConfigRuleRequest {
            config_rule: ConfigRule {
                config_rule_name: "my-rule".to_string(),
                source: Source { owner: "AWS".to_string(), source_identifier: "S3".to_string(), ..Default::default() },
                ..Default::default()
            },
            tags: None,
        }).await.unwrap();
        let rules = state.describe_config_rules(DescribeConfigRulesRequest::default()).await.unwrap();
        let original_arn = rules.config_rules[0].config_rule_arn.clone();
        let original_id = rules.config_rules[0].config_rule_id.clone();
        // Update same rule
        state.put_config_rule(PutConfigRuleRequest {
            config_rule: ConfigRule {
                config_rule_name: "my-rule".to_string(),
                description: Some("updated".to_string()),
                source: Source { owner: "AWS".to_string(), source_identifier: "S3".to_string(), ..Default::default() },
                ..Default::default()
            },
            tags: None,
        }).await.unwrap();
        let rules = state.describe_config_rules(DescribeConfigRulesRequest::default()).await.unwrap();
        assert_eq!(rules.config_rules[0].config_rule_arn, original_arn);
        assert_eq!(rules.config_rules[0].config_rule_id, original_id);
        assert_eq!(rules.config_rules[0].description.as_deref(), Some("updated"));
    }

    #[tokio::test]
    async fn test_put_config_rule_with_tags() {
        let state = make_state();
        state.put_config_rule(PutConfigRuleRequest {
            config_rule: ConfigRule {
                config_rule_name: "tagged-rule".to_string(),
                source: Source { owner: "AWS".to_string(), source_identifier: "S3".to_string(), ..Default::default() },
                ..Default::default()
            },
            tags: Some(vec![Tag { key: "env".to_string(), value: "prod".to_string() }]),
        }).await.unwrap();
        let rules = state.describe_config_rules(DescribeConfigRulesRequest::default()).await.unwrap();
        let arn = rules.config_rules[0].config_rule_arn.clone().unwrap();
        let tags = state.list_tags_for_resource(ListTagsForResourceRequest { resource_arn: arn, limit: None }).await.unwrap();
        assert_eq!(tags.tags.len(), 1);
        assert_eq!(tags.tags[0].key, "env");
    }

    #[tokio::test]
    async fn test_describe_config_rules_by_name() {
        let state = make_state();
        state.put_config_rule(PutConfigRuleRequest {
            config_rule: ConfigRule {
                config_rule_name: "r1".to_string(),
                source: Source { owner: "AWS".to_string(), source_identifier: "test".to_string(), ..Default::default() },
                ..Default::default()
            },
            tags: None,
        }).await.unwrap();
        state.put_config_rule(PutConfigRuleRequest {
            config_rule: ConfigRule {
                config_rule_name: "r2".to_string(),
                source: Source { owner: "AWS".to_string(), source_identifier: "test".to_string(), ..Default::default() },
                ..Default::default()
            },
            tags: None,
        }).await.unwrap();
        let result = state.describe_config_rules(DescribeConfigRulesRequest {
            config_rule_names: Some(vec!["r1".to_string()]),
        }).await.unwrap();
        assert_eq!(result.config_rules.len(), 1);
        assert_eq!(result.config_rules[0].config_rule_name, "r1");
    }

    #[tokio::test]
    async fn test_describe_config_rules_not_found() {
        let state = make_state();
        let result = state.describe_config_rules(DescribeConfigRulesRequest {
            config_rule_names: Some(vec!["nonexistent".to_string()]),
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_config_rule_cleans_up_evaluations() {
        let state = make_state();
        state.put_config_rule(PutConfigRuleRequest {
            config_rule: ConfigRule {
                config_rule_name: "r1".to_string(),
                source: Source { owner: "AWS".to_string(), source_identifier: "test".to_string(), ..Default::default() },
                ..Default::default()
            },
            tags: None,
        }).await.unwrap();
        state.put_evaluations(PutEvaluationsRequest {
            evaluations: vec![Evaluation {
                compliance_resource_id: "bucket".to_string(),
                compliance_resource_type: "AWS::S3::Bucket".to_string(),
                compliance_type: "COMPLIANT".to_string(),
                ordering_timestamp: 1234567890.0,
                annotation: None,
            }],
            result_token: "r1".to_string(),
        }).await.unwrap();
        state.delete_config_rule(DeleteConfigRuleRequest { config_rule_name: "r1".to_string() }).await.unwrap();
        // Rule is gone
        let result = state.describe_config_rules(DescribeConfigRulesRequest::default()).await.unwrap();
        assert!(result.config_rules.is_empty());
    }

    // --- Extended coverage: evaluations and compliance ---

    #[tokio::test]
    async fn test_get_compliance_details_by_config_rule() {
        let state = make_state();
        state.put_config_rule(PutConfigRuleRequest {
            config_rule: ConfigRule {
                config_rule_name: "r1".to_string(),
                source: Source { owner: "AWS".to_string(), source_identifier: "test".to_string(), ..Default::default() },
                ..Default::default()
            },
            tags: None,
        }).await.unwrap();
        state.put_evaluations(PutEvaluationsRequest {
            evaluations: vec![
                Evaluation {
                    compliance_resource_id: "b1".to_string(),
                    compliance_resource_type: "AWS::S3::Bucket".to_string(),
                    compliance_type: "COMPLIANT".to_string(),
                    ordering_timestamp: 100.0,
                    annotation: Some("ok".to_string()),
                },
                Evaluation {
                    compliance_resource_id: "b2".to_string(),
                    compliance_resource_type: "AWS::S3::Bucket".to_string(),
                    compliance_type: "NON_COMPLIANT".to_string(),
                    ordering_timestamp: 200.0,
                    annotation: None,
                },
            ],
            result_token: "r1".to_string(),
        }).await.unwrap();
        let result = state.get_compliance_details_by_config_rule(GetComplianceDetailsByConfigRuleRequest {
            config_rule_name: "r1".to_string(),
            compliance_types: None,
            limit: None,
        }).await.unwrap();
        assert_eq!(result.evaluation_results.len(), 2);
    }

    #[tokio::test]
    async fn test_get_compliance_details_filter_by_type() {
        let state = make_state();
        state.put_config_rule(PutConfigRuleRequest {
            config_rule: ConfigRule {
                config_rule_name: "r1".to_string(),
                source: Source { owner: "AWS".to_string(), source_identifier: "test".to_string(), ..Default::default() },
                ..Default::default()
            },
            tags: None,
        }).await.unwrap();
        state.put_evaluations(PutEvaluationsRequest {
            evaluations: vec![
                Evaluation {
                    compliance_resource_id: "b1".to_string(),
                    compliance_resource_type: "AWS::S3::Bucket".to_string(),
                    compliance_type: "COMPLIANT".to_string(),
                    ordering_timestamp: 100.0,
                    annotation: None,
                },
                Evaluation {
                    compliance_resource_id: "b2".to_string(),
                    compliance_resource_type: "AWS::S3::Bucket".to_string(),
                    compliance_type: "NON_COMPLIANT".to_string(),
                    ordering_timestamp: 200.0,
                    annotation: None,
                },
            ],
            result_token: "r1".to_string(),
        }).await.unwrap();
        let result = state.get_compliance_details_by_config_rule(GetComplianceDetailsByConfigRuleRequest {
            config_rule_name: "r1".to_string(),
            compliance_types: Some(vec!["NON_COMPLIANT".to_string()]),
            limit: None,
        }).await.unwrap();
        assert_eq!(result.evaluation_results.len(), 1);
        assert_eq!(result.evaluation_results[0].compliance_type, "NON_COMPLIANT");
    }

    #[tokio::test]
    async fn test_get_compliance_details_with_limit() {
        let state = make_state();
        state.put_config_rule(PutConfigRuleRequest {
            config_rule: ConfigRule {
                config_rule_name: "r1".to_string(),
                source: Source { owner: "AWS".to_string(), source_identifier: "test".to_string(), ..Default::default() },
                ..Default::default()
            },
            tags: None,
        }).await.unwrap();
        state.put_evaluations(PutEvaluationsRequest {
            evaluations: vec![
                Evaluation { compliance_resource_id: "b1".to_string(), compliance_resource_type: "AWS::S3::Bucket".to_string(), compliance_type: "COMPLIANT".to_string(), ordering_timestamp: 100.0, annotation: None },
                Evaluation { compliance_resource_id: "b2".to_string(), compliance_resource_type: "AWS::S3::Bucket".to_string(), compliance_type: "COMPLIANT".to_string(), ordering_timestamp: 200.0, annotation: None },
            ],
            result_token: "r1".to_string(),
        }).await.unwrap();
        let result = state.get_compliance_details_by_config_rule(GetComplianceDetailsByConfigRuleRequest {
            config_rule_name: "r1".to_string(),
            compliance_types: None,
            limit: Some(1),
        }).await.unwrap();
        assert_eq!(result.evaluation_results.len(), 1);
    }

    #[tokio::test]
    async fn test_get_compliance_details_rule_not_found() {
        let state = make_state();
        let result = state.get_compliance_details_by_config_rule(GetComplianceDetailsByConfigRuleRequest {
            config_rule_name: "nope".to_string(),
            compliance_types: None,
            limit: None,
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_compliance_details_no_evaluations() {
        let state = make_state();
        state.put_config_rule(PutConfigRuleRequest {
            config_rule: ConfigRule {
                config_rule_name: "r1".to_string(),
                source: Source { owner: "AWS".to_string(), source_identifier: "test".to_string(), ..Default::default() },
                ..Default::default()
            },
            tags: None,
        }).await.unwrap();
        let result = state.get_compliance_details_by_config_rule(GetComplianceDetailsByConfigRuleRequest {
            config_rule_name: "r1".to_string(),
            compliance_types: None,
            limit: None,
        }).await.unwrap();
        assert!(result.evaluation_results.is_empty());
    }

    #[tokio::test]
    async fn test_describe_compliance_by_config_rule_non_compliant() {
        let state = make_state();
        state.put_config_rule(PutConfigRuleRequest {
            config_rule: ConfigRule {
                config_rule_name: "r1".to_string(),
                source: Source { owner: "AWS".to_string(), source_identifier: "test".to_string(), ..Default::default() },
                ..Default::default()
            },
            tags: None,
        }).await.unwrap();
        state.put_evaluations(PutEvaluationsRequest {
            evaluations: vec![Evaluation {
                compliance_resource_id: "b1".to_string(),
                compliance_resource_type: "AWS::S3::Bucket".to_string(),
                compliance_type: "NON_COMPLIANT".to_string(),
                ordering_timestamp: 100.0,
                annotation: None,
            }],
            result_token: "r1".to_string(),
        }).await.unwrap();
        let result = state.describe_compliance_by_config_rule(DescribeComplianceByConfigRuleRequest {
            config_rule_names: Some(vec!["r1".to_string()]),
            compliance_types: None,
        }).await.unwrap();
        assert_eq!(result.compliance_by_config_rules.len(), 1);
        assert_eq!(result.compliance_by_config_rules[0].compliance.compliance_type, "NON_COMPLIANT");
    }

    #[tokio::test]
    async fn test_describe_compliance_by_config_rule_filter() {
        let state = make_state();
        state.put_config_rule(PutConfigRuleRequest {
            config_rule: ConfigRule {
                config_rule_name: "r1".to_string(),
                source: Source { owner: "AWS".to_string(), source_identifier: "test".to_string(), ..Default::default() },
                ..Default::default()
            },
            tags: None,
        }).await.unwrap();
        // No evals = INSUFFICIENT_DATA
        let result = state.describe_compliance_by_config_rule(DescribeComplianceByConfigRuleRequest {
            config_rule_names: None,
            compliance_types: Some(vec!["COMPLIANT".to_string()]),
        }).await.unwrap();
        assert!(result.compliance_by_config_rules.is_empty());
    }

    #[tokio::test]
    async fn test_describe_compliance_by_config_rule_not_found() {
        let state = make_state();
        let result = state.describe_compliance_by_config_rule(DescribeComplianceByConfigRuleRequest {
            config_rule_names: Some(vec!["nonexistent".to_string()]),
            compliance_types: None,
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_describe_compliance_by_resource_with_filters() {
        let state = make_state();
        state.put_config_rule(PutConfigRuleRequest {
            config_rule: ConfigRule {
                config_rule_name: "r1".to_string(),
                source: Source { owner: "AWS".to_string(), source_identifier: "test".to_string(), ..Default::default() },
                ..Default::default()
            },
            tags: None,
        }).await.unwrap();
        state.put_evaluations(PutEvaluationsRequest {
            evaluations: vec![
                Evaluation { compliance_resource_id: "b1".to_string(), compliance_resource_type: "AWS::S3::Bucket".to_string(), compliance_type: "COMPLIANT".to_string(), ordering_timestamp: 100.0, annotation: None },
                Evaluation { compliance_resource_id: "i1".to_string(), compliance_resource_type: "AWS::EC2::Instance".to_string(), compliance_type: "NON_COMPLIANT".to_string(), ordering_timestamp: 200.0, annotation: None },
            ],
            result_token: "r1".to_string(),
        }).await.unwrap();
        let result = state.describe_compliance_by_resource(DescribeComplianceByResourceRequest {
            resource_type: Some("AWS::S3::Bucket".to_string()),
            resource_id: None,
            compliance_types: None,
            limit: None,
        }).await.unwrap();
        assert_eq!(result.compliance_by_resources.len(), 1);
        assert_eq!(result.compliance_by_resources[0].resource_type, "AWS::S3::Bucket");
    }

    #[tokio::test]
    async fn test_describe_compliance_by_resource_with_limit() {
        let state = make_state();
        state.put_config_rule(PutConfigRuleRequest {
            config_rule: ConfigRule {
                config_rule_name: "r1".to_string(),
                source: Source { owner: "AWS".to_string(), source_identifier: "test".to_string(), ..Default::default() },
                ..Default::default()
            },
            tags: None,
        }).await.unwrap();
        state.put_evaluations(PutEvaluationsRequest {
            evaluations: vec![
                Evaluation { compliance_resource_id: "b1".to_string(), compliance_resource_type: "AWS::S3::Bucket".to_string(), compliance_type: "COMPLIANT".to_string(), ordering_timestamp: 100.0, annotation: None },
                Evaluation { compliance_resource_id: "b2".to_string(), compliance_resource_type: "AWS::S3::Bucket".to_string(), compliance_type: "COMPLIANT".to_string(), ordering_timestamp: 200.0, annotation: None },
            ],
            result_token: "r1".to_string(),
        }).await.unwrap();
        let result = state.describe_compliance_by_resource(DescribeComplianceByResourceRequest {
            resource_type: None,
            resource_id: None,
            compliance_types: None,
            limit: Some(1),
        }).await.unwrap();
        assert_eq!(result.compliance_by_resources.len(), 1);
    }

    // --- Extended coverage: tag operations ---

    #[tokio::test]
    async fn test_tag_untag_list_tags_for_config_rule() {
        let state = make_state();
        state.put_config_rule(PutConfigRuleRequest {
            config_rule: ConfigRule {
                config_rule_name: "r1".to_string(),
                source: Source { owner: "AWS".to_string(), source_identifier: "test".to_string(), ..Default::default() },
                ..Default::default()
            },
            tags: None,
        }).await.unwrap();
        let rules = state.describe_config_rules(DescribeConfigRulesRequest::default()).await.unwrap();
        let arn = rules.config_rules[0].config_rule_arn.clone().unwrap();
        // Tag
        state.tag_resource(TagResourceRequest {
            resource_arn: arn.clone(),
            tags: vec![
                Tag { key: "env".to_string(), value: "prod".to_string() },
                Tag { key: "team".to_string(), value: "infra".to_string() },
            ],
        }).await.unwrap();
        let tags = state.list_tags_for_resource(ListTagsForResourceRequest { resource_arn: arn.clone(), limit: None }).await.unwrap();
        assert_eq!(tags.tags.len(), 2);
        // Untag
        state.untag_resource(UntagResourceRequest {
            resource_arn: arn.clone(),
            tag_keys: vec!["team".to_string()],
        }).await.unwrap();
        let tags = state.list_tags_for_resource(ListTagsForResourceRequest { resource_arn: arn, limit: None }).await.unwrap();
        assert_eq!(tags.tags.len(), 1);
        assert_eq!(tags.tags[0].key, "env");
    }

    #[tokio::test]
    async fn test_tag_resource_not_found() {
        let state = make_state();
        let result = state.tag_resource(TagResourceRequest {
            resource_arn: "arn:aws:config:us-east-1:123456789012:config-rule/fake".to_string(),
            tags: vec![Tag { key: "k".to_string(), value: "v".to_string() }],
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_untag_resource_not_found() {
        let state = make_state();
        let result = state.untag_resource(UntagResourceRequest {
            resource_arn: "arn:aws:config:us-east-1:123456789012:config-rule/fake".to_string(),
            tag_keys: vec!["k".to_string()],
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_tags_for_resource_not_found() {
        let state = make_state();
        let result = state.list_tags_for_resource(ListTagsForResourceRequest {
            resource_arn: "arn:aws:config:us-east-1:123456789012:config-rule/fake".to_string(),
            limit: None,
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_tags_with_limit() {
        let state = make_state();
        state.put_config_rule(PutConfigRuleRequest {
            config_rule: ConfigRule {
                config_rule_name: "r1".to_string(),
                source: Source { owner: "AWS".to_string(), source_identifier: "test".to_string(), ..Default::default() },
                ..Default::default()
            },
            tags: Some(vec![
                Tag { key: "a".to_string(), value: "1".to_string() },
                Tag { key: "b".to_string(), value: "2".to_string() },
                Tag { key: "c".to_string(), value: "3".to_string() },
            ]),
        }).await.unwrap();
        let rules = state.describe_config_rules(DescribeConfigRulesRequest::default()).await.unwrap();
        let arn = rules.config_rules[0].config_rule_arn.clone().unwrap();
        let tags = state.list_tags_for_resource(ListTagsForResourceRequest { resource_arn: arn, limit: Some(2) }).await.unwrap();
        assert_eq!(tags.tags.len(), 2);
    }
}
