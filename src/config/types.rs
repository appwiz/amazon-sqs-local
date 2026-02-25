use serde::{Deserialize, Serialize};

// --- Shared types ---

// ConfigurationRecorder uses camelCase for its members
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ConfigurationRecorder {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(rename = "roleARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "recordingGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recording_group: Option<RecordingGroup>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RecordingGroup {
    #[serde(rename = "allSupported")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_supported: Option<bool>,
    #[serde(rename = "includeGlobalResourceTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_global_resource_types: Option<bool>,
    #[serde(rename = "resourceTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_types: Option<Vec<String>>,
}

// DeliveryChannel uses camelCase for its members
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DeliveryChannel {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(rename = "s3BucketName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket_name: Option<String>,
    #[serde(rename = "s3KeyPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_key_prefix: Option<String>,
    #[serde(rename = "snsTopicARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic_arn: Option<String>,
}

// ConfigurationRecorderStatus uses camelCase for its members
#[derive(Debug, Clone, Serialize)]
pub struct ConfigurationRecorderStatus {
    pub name: String,
    pub recording: bool,
    #[serde(rename = "lastStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_start_time: Option<f64>,
    #[serde(rename = "lastStopTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_stop_time: Option<f64>,
    #[serde(rename = "lastStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_status: Option<String>,
}

// ConfigRule uses PascalCase for its members
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ConfigRule {
    #[serde(rename = "ConfigRuleName")]
    pub config_rule_name: String,
    #[serde(rename = "ConfigRuleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_rule_arn: Option<String>,
    #[serde(rename = "ConfigRuleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_rule_id: Option<String>,
    #[serde(rename = "ConfigRuleState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_rule_state: Option<String>,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "InputParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_parameters: Option<String>,
    #[serde(rename = "Scope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<Scope>,
    #[serde(rename = "Source")]
    pub source: Source,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Scope {
    #[serde(rename = "ComplianceResourceTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_resource_types: Option<Vec<String>>,
    #[serde(rename = "ComplianceResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_resource_id: Option<String>,
    #[serde(rename = "TagKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_key: Option<String>,
    #[serde(rename = "TagValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_value: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Source {
    #[serde(rename = "Owner")]
    pub owner: String,
    #[serde(rename = "SourceIdentifier")]
    pub source_identifier: String,
    #[serde(rename = "SourceDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_details: Option<Vec<SourceDetail>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SourceDetail {
    #[serde(rename = "EventSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_source: Option<String>,
    #[serde(rename = "MessageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_type: Option<String>,
    #[serde(rename = "MaximumExecutionFrequency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_execution_frequency: Option<String>,
}

// Evaluation uses PascalCase for its members
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Evaluation {
    #[serde(rename = "ComplianceResourceId")]
    pub compliance_resource_id: String,
    #[serde(rename = "ComplianceResourceType")]
    pub compliance_resource_type: String,
    #[serde(rename = "ComplianceType")]
    pub compliance_type: String,
    #[serde(rename = "OrderingTimestamp")]
    pub ordering_timestamp: f64,
    #[serde(rename = "Annotation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotation: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,
}

// --- Evaluation result types for responses ---

#[derive(Debug, Clone, Serialize)]
pub struct EvaluationResult {
    #[serde(rename = "EvaluationResultIdentifier")]
    pub evaluation_result_identifier: EvaluationResultIdentifier,
    #[serde(rename = "ComplianceType")]
    pub compliance_type: String,
    #[serde(rename = "ResultRecordedTime")]
    pub result_recorded_time: f64,
    #[serde(rename = "ConfigRuleInvokedTime")]
    pub config_rule_invoked_time: f64,
    #[serde(rename = "Annotation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotation: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct EvaluationResultIdentifier {
    #[serde(rename = "EvaluationResultQualifier")]
    pub evaluation_result_qualifier: EvaluationResultQualifier,
    #[serde(rename = "OrderingTimestamp")]
    pub ordering_timestamp: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct EvaluationResultQualifier {
    #[serde(rename = "ConfigRuleName")]
    pub config_rule_name: String,
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
}

// --- Compliance types ---

#[derive(Debug, Clone, Serialize)]
pub struct ComplianceByConfigRule {
    #[serde(rename = "ConfigRuleName")]
    pub config_rule_name: String,
    #[serde(rename = "Compliance")]
    pub compliance: Compliance,
}

#[derive(Debug, Clone, Serialize)]
pub struct ComplianceByResource {
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    #[serde(rename = "Compliance")]
    pub compliance: Compliance,
}

#[derive(Debug, Clone, Serialize)]
pub struct Compliance {
    #[serde(rename = "ComplianceType")]
    pub compliance_type: String,
    #[serde(rename = "ComplianceContributorCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_contributor_count: Option<ComplianceContributorCount>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ComplianceContributorCount {
    #[serde(rename = "CappedCount")]
    pub capped_count: i32,
    #[serde(rename = "CapExceeded")]
    pub cap_exceeded: bool,
}

// --- Request/Response types (top-level fields are always PascalCase) ---

// 1. PutConfigurationRecorder
#[derive(Debug, Deserialize)]
pub struct PutConfigurationRecorderRequest {
    #[serde(rename = "ConfigurationRecorder")]
    pub configuration_recorder: ConfigurationRecorder,
}

// 2. DescribeConfigurationRecorders
#[derive(Debug, Deserialize, Default)]
pub struct DescribeConfigurationRecordersRequest {
    #[serde(rename = "ConfigurationRecorderNames")]
    pub configuration_recorder_names: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
pub struct DescribeConfigurationRecordersResponse {
    #[serde(rename = "ConfigurationRecorders")]
    pub configuration_recorders: Vec<ConfigurationRecorder>,
}

// 3. DeleteConfigurationRecorder
#[derive(Debug, Deserialize)]
pub struct DeleteConfigurationRecorderRequest {
    #[serde(rename = "ConfigurationRecorderName")]
    pub configuration_recorder_name: String,
}

// 4. DescribeConfigurationRecorderStatus
#[derive(Debug, Deserialize, Default)]
pub struct DescribeConfigurationRecorderStatusRequest {
    #[serde(rename = "ConfigurationRecorderNames")]
    pub configuration_recorder_names: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
pub struct DescribeConfigurationRecorderStatusResponse {
    #[serde(rename = "ConfigurationRecordersStatus")]
    pub configuration_recorders_status: Vec<ConfigurationRecorderStatus>,
}

// 5. StartConfigurationRecorder
#[derive(Debug, Deserialize)]
pub struct StartConfigurationRecorderRequest {
    #[serde(rename = "ConfigurationRecorderName")]
    pub configuration_recorder_name: String,
}

// 6. StopConfigurationRecorder
#[derive(Debug, Deserialize)]
pub struct StopConfigurationRecorderRequest {
    #[serde(rename = "ConfigurationRecorderName")]
    pub configuration_recorder_name: String,
}

// 7. PutDeliveryChannel
#[derive(Debug, Deserialize)]
pub struct PutDeliveryChannelRequest {
    #[serde(rename = "DeliveryChannel")]
    pub delivery_channel: DeliveryChannel,
}

// 8. DescribeDeliveryChannels
#[derive(Debug, Deserialize, Default)]
pub struct DescribeDeliveryChannelsRequest {
    #[serde(rename = "DeliveryChannelNames")]
    pub delivery_channel_names: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
pub struct DescribeDeliveryChannelsResponse {
    #[serde(rename = "DeliveryChannels")]
    pub delivery_channels: Vec<DeliveryChannel>,
}

// 9. DeleteDeliveryChannel
#[derive(Debug, Deserialize)]
pub struct DeleteDeliveryChannelRequest {
    #[serde(rename = "DeliveryChannelName")]
    pub delivery_channel_name: String,
}

// 10. PutConfigRule
#[derive(Debug, Deserialize)]
pub struct PutConfigRuleRequest {
    #[serde(rename = "ConfigRule")]
    pub config_rule: ConfigRule,
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
}

// 11. DescribeConfigRules
#[derive(Debug, Deserialize, Default)]
pub struct DescribeConfigRulesRequest {
    #[serde(rename = "ConfigRuleNames")]
    pub config_rule_names: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
pub struct DescribeConfigRulesResponse {
    #[serde(rename = "ConfigRules")]
    pub config_rules: Vec<ConfigRule>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

// 12. DeleteConfigRule
#[derive(Debug, Deserialize)]
pub struct DeleteConfigRuleRequest {
    #[serde(rename = "ConfigRuleName")]
    pub config_rule_name: String,
}

// 13. PutEvaluations
#[derive(Debug, Deserialize)]
pub struct PutEvaluationsRequest {
    #[serde(rename = "Evaluations")]
    pub evaluations: Vec<Evaluation>,
    #[serde(rename = "ResultToken")]
    pub result_token: String,
}

#[derive(Debug, Serialize)]
pub struct PutEvaluationsResponse {
    #[serde(rename = "FailedEvaluations")]
    pub failed_evaluations: Vec<Evaluation>,
}

// 14. GetComplianceDetailsByConfigRule
#[derive(Debug, Deserialize)]
pub struct GetComplianceDetailsByConfigRuleRequest {
    #[serde(rename = "ConfigRuleName")]
    pub config_rule_name: String,
    #[serde(rename = "ComplianceTypes")]
    pub compliance_types: Option<Vec<String>>,
    #[serde(rename = "Limit")]
    pub limit: Option<usize>,
}

#[derive(Debug, Serialize)]
pub struct GetComplianceDetailsByConfigRuleResponse {
    #[serde(rename = "EvaluationResults")]
    pub evaluation_results: Vec<EvaluationResult>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

// 15. DescribeComplianceByConfigRule
#[derive(Debug, Deserialize, Default)]
pub struct DescribeComplianceByConfigRuleRequest {
    #[serde(rename = "ConfigRuleNames")]
    pub config_rule_names: Option<Vec<String>>,
    #[serde(rename = "ComplianceTypes")]
    pub compliance_types: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
pub struct DescribeComplianceByConfigRuleResponse {
    #[serde(rename = "ComplianceByConfigRules")]
    pub compliance_by_config_rules: Vec<ComplianceByConfigRule>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

// 16. DescribeComplianceByResource
#[derive(Debug, Deserialize, Default)]
pub struct DescribeComplianceByResourceRequest {
    #[serde(rename = "ResourceType")]
    pub resource_type: Option<String>,
    #[serde(rename = "ResourceId")]
    pub resource_id: Option<String>,
    #[serde(rename = "ComplianceTypes")]
    pub compliance_types: Option<Vec<String>>,
    #[serde(rename = "Limit")]
    pub limit: Option<usize>,
}

#[derive(Debug, Serialize)]
pub struct DescribeComplianceByResourceResponse {
    #[serde(rename = "ComplianceByResources")]
    pub compliance_by_resources: Vec<ComplianceByResource>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

// 17. TagResource
#[derive(Debug, Deserialize)]
pub struct TagResourceRequest {
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

// 18. UntagResource
#[derive(Debug, Deserialize)]
pub struct UntagResourceRequest {
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

// 19. ListTagsForResource
#[derive(Debug, Deserialize)]
pub struct ListTagsForResourceRequest {
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    #[serde(rename = "Limit")]
    pub limit: Option<usize>,
}

#[derive(Debug, Serialize)]
pub struct ListTagsForResourceResponse {
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}
