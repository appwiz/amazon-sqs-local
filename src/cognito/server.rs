use std::sync::Arc;

use axum::body::Bytes;
use axum::extract::State;
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};

use super::error::CognitoError;
use super::state::CognitoState;
use super::types::*;

macro_rules! dispatch {
    ($state:expr, $body:expr, $req_type:ty, $method:ident) => {{
        let req: $req_type = serde_json::from_slice(&$body)
            .map_err(|e| CognitoError::InvalidParameterException(e.to_string()))?;
        let resp = $state.$method(req).await?;
        Ok(Json(serde_json::to_value(resp).unwrap()).into_response())
    }};
}

macro_rules! dispatch_empty {
    ($state:expr, $body:expr, $req_type:ty, $method:ident) => {{
        let req: $req_type = serde_json::from_slice(&$body)
            .map_err(|e| CognitoError::InvalidParameterException(e.to_string()))?;
        $state.$method(req).await?;
        Ok(Json(serde_json::json!({})).into_response())
    }};
}

async fn handle_request(
    State(state): State<Arc<CognitoState>>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<axum::response::Response, CognitoError> {
    let target = headers
        .get("x-amz-target")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| CognitoError::InvalidAction("Missing X-Amz-Target header".into()))?;

    let action = target
        .strip_prefix("AWSCognitoIdentityProviderService.")
        .ok_or_else(|| CognitoError::InvalidAction(format!("Invalid target: {target}")))?;

    match action {
        // User Pool
        "CreateUserPool" => dispatch!(state, body, CreateUserPoolRequest, create_user_pool),
        "DeleteUserPool" => {
            dispatch_empty!(state, body, DeleteUserPoolRequest, delete_user_pool)
        }
        "DescribeUserPool" => {
            dispatch!(state, body, DescribeUserPoolRequest, describe_user_pool)
        }
        "ListUserPools" => dispatch!(state, body, ListUserPoolsRequest, list_user_pools),
        "UpdateUserPool" => {
            dispatch_empty!(state, body, UpdateUserPoolRequest, update_user_pool)
        }
        // Users
        "AdminCreateUser" => {
            dispatch!(state, body, AdminCreateUserRequest, admin_create_user)
        }
        "AdminDeleteUser" => {
            dispatch_empty!(state, body, AdminDeleteUserRequest, admin_delete_user)
        }
        "AdminGetUser" => dispatch!(state, body, AdminGetUserRequest, admin_get_user),
        "AdminSetUserPassword" => {
            dispatch_empty!(
                state,
                body,
                AdminSetUserPasswordRequest,
                admin_set_user_password
            )
        }
        "AdminEnableUser" => {
            dispatch_empty!(state, body, AdminEnableUserRequest, admin_enable_user)
        }
        "AdminDisableUser" => {
            dispatch_empty!(state, body, AdminDisableUserRequest, admin_disable_user)
        }
        "AdminResetUserPassword" => {
            dispatch_empty!(
                state,
                body,
                AdminResetUserPasswordRequest,
                admin_reset_user_password
            )
        }
        "AdminUpdateUserAttributes" => {
            dispatch_empty!(
                state,
                body,
                AdminUpdateUserAttributesRequest,
                admin_update_user_attributes
            )
        }
        "ListUsers" => dispatch!(state, body, ListUsersRequest, list_users),
        // Clients
        "CreateUserPoolClient" => {
            dispatch!(
                state,
                body,
                CreateUserPoolClientRequest,
                create_user_pool_client
            )
        }
        "DeleteUserPoolClient" => {
            dispatch_empty!(
                state,
                body,
                DeleteUserPoolClientRequest,
                delete_user_pool_client
            )
        }
        "DescribeUserPoolClient" => {
            dispatch!(
                state,
                body,
                DescribeUserPoolClientRequest,
                describe_user_pool_client
            )
        }
        "ListUserPoolClients" => {
            dispatch!(
                state,
                body,
                ListUserPoolClientsRequest,
                list_user_pool_clients
            )
        }
        "UpdateUserPoolClient" => {
            dispatch!(
                state,
                body,
                UpdateUserPoolClientRequest,
                update_user_pool_client
            )
        }
        // Groups
        "CreateGroup" => dispatch!(state, body, CreateGroupRequest, create_group),
        "DeleteGroup" => dispatch_empty!(state, body, DeleteGroupRequest, delete_group),
        "GetGroup" => dispatch!(state, body, GetGroupRequest, get_group),
        "ListGroups" => dispatch!(state, body, ListGroupsRequest, list_groups),
        "AdminAddUserToGroup" => {
            dispatch_empty!(
                state,
                body,
                AdminAddUserToGroupRequest,
                admin_add_user_to_group
            )
        }
        "AdminRemoveUserFromGroup" => {
            dispatch_empty!(
                state,
                body,
                AdminRemoveUserFromGroupRequest,
                admin_remove_user_from_group
            )
        }
        "AdminListGroupsForUser" => {
            dispatch!(
                state,
                body,
                AdminListGroupsForUserRequest,
                admin_list_groups_for_user
            )
        }
        "ListUsersInGroup" => {
            dispatch!(state, body, ListUsersInGroupRequest, list_users_in_group)
        }
        // Auth
        "InitiateAuth" => dispatch!(state, body, InitiateAuthRequest, initiate_auth),
        "AdminInitiateAuth" => {
            dispatch!(state, body, AdminInitiateAuthRequest, admin_initiate_auth)
        }
        "SignUp" => dispatch!(state, body, SignUpRequest, sign_up),
        "ConfirmSignUp" => {
            dispatch_empty!(state, body, ConfirmSignUpRequest, confirm_sign_up)
        }
        "ForgotPassword" => dispatch!(state, body, ForgotPasswordRequest, forgot_password),
        "ConfirmForgotPassword" => {
            dispatch_empty!(
                state,
                body,
                ConfirmForgotPasswordRequest,
                confirm_forgot_password
            )
        }
        _ => Err(CognitoError::InvalidAction(format!(
            "Unknown action: {action}"
        ))),
    }
}

pub fn create_router(state: Arc<CognitoState>) -> Router {
    Router::new()
        .route("/", post(handle_request))
        .with_state(state)
}
