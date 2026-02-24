use std::sync::Arc;

use axum::body::Bytes;
use axum::extract::State;
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};

use super::error::ConfigError;
use super::state::ConfigState;
use super::types::*;

macro_rules! dispatch {
    ($state:expr, $body:expr, $req_type:ty, $method:ident) => {{
        let req: $req_type = serde_json::from_slice(&$body)
            .map_err(|e| ConfigError::InvalidParameterValueException(e.to_string()))?;
        let resp = $state.$method(req).await?;
        Ok(Json(serde_json::to_value(resp).unwrap()).into_response())
    }};
}

macro_rules! dispatch_empty {
    ($state:expr, $body:expr, $req_type:ty, $method:ident) => {{
        let req: $req_type = serde_json::from_slice(&$body)
            .map_err(|e| ConfigError::InvalidParameterValueException(e.to_string()))?;
        $state.$method(req).await?;
        Ok(Json(serde_json::json!({})).into_response())
    }};
}

async fn handle_request(
    State(state): State<Arc<ConfigState>>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<axum::response::Response, ConfigError> {
    let target = headers
        .get("x-amz-target")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| ConfigError::InvalidAction("Missing X-Amz-Target header".into()))?;

    let action = target
        .strip_prefix("StarlingDoveService.")
        .ok_or_else(|| ConfigError::InvalidAction(format!("Invalid target: {target}")))?;

    match action {
        "PutConfigurationRecorder" => {
            dispatch_empty!(state, body, PutConfigurationRecorderRequest, put_configuration_recorder)
        }
        "DescribeConfigurationRecorders" => {
            dispatch!(state, body, DescribeConfigurationRecordersRequest, describe_configuration_recorders)
        }
        "DeleteConfigurationRecorder" => {
            dispatch_empty!(state, body, DeleteConfigurationRecorderRequest, delete_configuration_recorder)
        }
        "DescribeConfigurationRecorderStatus" => {
            dispatch!(state, body, DescribeConfigurationRecorderStatusRequest, describe_configuration_recorder_status)
        }
        "StartConfigurationRecorder" => {
            dispatch_empty!(state, body, StartConfigurationRecorderRequest, start_configuration_recorder)
        }
        "StopConfigurationRecorder" => {
            dispatch_empty!(state, body, StopConfigurationRecorderRequest, stop_configuration_recorder)
        }
        "PutDeliveryChannel" => {
            dispatch_empty!(state, body, PutDeliveryChannelRequest, put_delivery_channel)
        }
        "DescribeDeliveryChannels" => {
            dispatch!(state, body, DescribeDeliveryChannelsRequest, describe_delivery_channels)
        }
        "DeleteDeliveryChannel" => {
            dispatch_empty!(state, body, DeleteDeliveryChannelRequest, delete_delivery_channel)
        }
        "PutConfigRule" => {
            dispatch_empty!(state, body, PutConfigRuleRequest, put_config_rule)
        }
        "DescribeConfigRules" => {
            dispatch!(state, body, DescribeConfigRulesRequest, describe_config_rules)
        }
        "DeleteConfigRule" => {
            dispatch_empty!(state, body, DeleteConfigRuleRequest, delete_config_rule)
        }
        "PutEvaluations" => {
            dispatch!(state, body, PutEvaluationsRequest, put_evaluations)
        }
        "GetComplianceDetailsByConfigRule" => {
            dispatch!(state, body, GetComplianceDetailsByConfigRuleRequest, get_compliance_details_by_config_rule)
        }
        "DescribeComplianceByConfigRule" => {
            dispatch!(state, body, DescribeComplianceByConfigRuleRequest, describe_compliance_by_config_rule)
        }
        "DescribeComplianceByResource" => {
            dispatch!(state, body, DescribeComplianceByResourceRequest, describe_compliance_by_resource)
        }
        "TagResource" => {
            dispatch_empty!(state, body, TagResourceRequest, tag_resource)
        }
        "UntagResource" => {
            dispatch_empty!(state, body, UntagResourceRequest, untag_resource)
        }
        "ListTagsForResource" => {
            dispatch!(state, body, ListTagsForResourceRequest, list_tags_for_resource)
        }
        _ => Err(ConfigError::InvalidAction(format!(
            "Unknown action: {action}"
        ))),
    }
}

pub fn create_router(state: Arc<ConfigState>) -> Router {
    Router::new()
        .route("/", post(handle_request))
        .with_state(state)
}
