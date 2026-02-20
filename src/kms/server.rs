use std::sync::Arc;

use axum::body::Bytes;
use axum::extract::State;
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};

use super::error::KmsError;
use super::state::KmsState;
use super::types::*;

macro_rules! dispatch {
    ($state:expr, $body:expr, $req_type:ty, $method:ident) => {{
        let req: $req_type = serde_json::from_slice(&$body)
            .map_err(|e| KmsError::InvalidParameterException(e.to_string()))?;
        let resp = $state.$method(req).await?;
        Ok(Json(serde_json::to_value(resp).unwrap()).into_response())
    }};
}

macro_rules! dispatch_empty {
    ($state:expr, $body:expr, $req_type:ty, $method:ident) => {{
        let req: $req_type = serde_json::from_slice(&$body)
            .map_err(|e| KmsError::InvalidParameterException(e.to_string()))?;
        $state.$method(req).await?;
        Ok(Json(serde_json::json!({})).into_response())
    }};
}

async fn handle_request(
    State(state): State<Arc<KmsState>>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<axum::response::Response, KmsError> {
    let target = headers
        .get("x-amz-target")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| KmsError::InvalidAction("Missing X-Amz-Target header".into()))?;

    let action = target
        .strip_prefix("TrentService.")
        .ok_or_else(|| KmsError::InvalidAction(format!("Invalid target: {target}")))?;

    match action {
        "CreateKey" => dispatch!(state, body, CreateKeyRequest, create_key),
        "DescribeKey" => dispatch!(state, body, DescribeKeyRequest, describe_key),
        "ListKeys" => dispatch!(state, body, ListKeysRequest, list_keys),
        "ScheduleKeyDeletion" => {
            dispatch!(state, body, ScheduleKeyDeletionRequest, schedule_key_deletion)
        }
        "CancelKeyDeletion" => {
            dispatch!(state, body, CancelKeyDeletionRequest, cancel_key_deletion)
        }
        "EnableKey" => dispatch_empty!(state, body, EnableKeyRequest, enable_key),
        "DisableKey" => dispatch_empty!(state, body, DisableKeyRequest, disable_key),
        "Encrypt" => dispatch!(state, body, EncryptRequest, encrypt),
        "Decrypt" => dispatch!(state, body, DecryptRequest, decrypt),
        "GenerateDataKey" => dispatch!(state, body, GenerateDataKeyRequest, generate_data_key),
        "GenerateDataKeyWithoutPlaintext" => {
            dispatch!(
                state,
                body,
                GenerateDataKeyWithoutPlaintextRequest,
                generate_data_key_without_plaintext
            )
        }
        "GenerateRandom" => dispatch!(state, body, GenerateRandomRequest, generate_random),
        "Sign" => dispatch!(state, body, SignRequest, sign),
        "Verify" => dispatch!(state, body, VerifyRequest, verify),
        "TagResource" => dispatch_empty!(state, body, TagResourceRequest, tag_resource),
        "UntagResource" => dispatch_empty!(state, body, UntagResourceRequest, untag_resource),
        "ListResourceTags" => {
            dispatch!(state, body, ListResourceTagsRequest, list_resource_tags)
        }
        "CreateAlias" => dispatch_empty!(state, body, CreateAliasRequest, create_alias),
        "DeleteAlias" => dispatch_empty!(state, body, DeleteAliasRequest, delete_alias),
        "ListAliases" => dispatch!(state, body, ListAliasesRequest, list_aliases),
        "GetKeyPolicy" => dispatch!(state, body, GetKeyPolicyRequest, get_key_policy),
        "PutKeyPolicy" => dispatch_empty!(state, body, PutKeyPolicyRequest, put_key_policy),
        _ => Err(KmsError::InvalidAction(format!("Unknown action: {action}"))),
    }
}

pub fn create_router(state: Arc<KmsState>) -> Router {
    Router::new()
        .route("/", post(handle_request))
        .with_state(state)
}
