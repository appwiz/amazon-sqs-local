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
        Ok(Json(resp).into_response())
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


#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_missing_target_header() {
        let state = Arc::new(CognitoState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/")
            .header("content-type", "application/x-amz-json-1.1")
            .body(Body::from("{}"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_unknown_action() {
        let state = Arc::new(CognitoState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/")
            .header("content-type", "application/x-amz-json-1.1")
            .header("x-amz-target", "AWSCognitoIdentityProviderService.FakeAction")
            .body(Body::from("{}"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }
    #[tokio::test]
    async fn test_listuserpools_ok() {
        let state = Arc::new(CognitoState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/")
            .header("content-type", "application/x-amz-json-1.1")
            .header("x-amz-target", "AWSCognitoIdentityProviderService.ListUserPools")
            .body(Body::from("{}"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_createuserpool_action() {
        let state = Arc::new(CognitoState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/")
            .header("content-type", "application/x-amz-json-1.1")
            .header("x-amz-target", "AWSCognitoIdentityProviderService.CreateUserPool")
            .body(Body::from("{}"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert!(resp.status().is_success() || resp.status().is_client_error());
    }
    #[tokio::test]
    async fn test_creategroup_action() {
        let state = Arc::new(CognitoState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/")
            .header("content-type", "application/x-amz-json-1.1")
            .header("x-amz-target", "AWSCognitoIdentityProviderService.CreateGroup")
            .body(Body::from("{}"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert!(resp.status().is_success() || resp.status().is_client_error());
    }
    #[tokio::test]
    async fn test_listuserpools_action() {
        let state = Arc::new(CognitoState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/")
            .header("content-type", "application/x-amz-json-1.1")
            .header("x-amz-target", "AWSCognitoIdentityProviderService.ListUserPools")
            .body(Body::from("{}"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[tokio::test]
    async fn test_listusers_action() {
        let state = Arc::new(CognitoState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/")
            .header("content-type", "application/x-amz-json-1.1")
            .header("x-amz-target", "AWSCognitoIdentityProviderService.ListUsers")
            .body(Body::from("{}"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert!(resp.status().is_client_error());
    }
    #[tokio::test]
    async fn test_listgroups_action() {
        let state = Arc::new(CognitoState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/")
            .header("content-type", "application/x-amz-json-1.1")
            .header("x-amz-target", "AWSCognitoIdentityProviderService.ListGroups")
            .body(Body::from("{}"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert!(resp.status().is_client_error());
    }

    fn new_state() -> Arc<CognitoState> {
        Arc::new(CognitoState::new("123456789012".to_string(), "us-east-1".to_string()))
    }

    fn cognito_req(action: &str, body: &str) -> Request<Body> {
        Request::builder()
            .method("POST")
            .uri("/")
            .header("content-type", "application/x-amz-json-1.1")
            .header("x-amz-target", format!("AWSCognitoIdentityProviderService.{}", action))
            .body(Body::from(body.to_string()))
            .unwrap()
    }

    async fn extract_body(resp: axum::response::Response) -> serde_json::Value {
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX).await.unwrap();
        serde_json::from_slice(&body).unwrap()
    }

    #[tokio::test]
    async fn test_create_and_describe_user_pool() {
        let state = new_state();
        let app = create_router(state.clone());
        let resp = app
            .oneshot(cognito_req("CreateUserPool", r#"{"PoolName": "test-pool"}"#))
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let json = extract_body(resp).await;
        let pool_id = json["UserPool"]["Id"].as_str().unwrap().to_string();

        // Describe
        let app = create_router(state);
        let resp = app
            .oneshot(cognito_req("DescribeUserPool", &format!(r#"{{"UserPoolId": "{}"}}"#, pool_id)))
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let json = extract_body(resp).await;
        assert_eq!(json["UserPool"]["Name"], "test-pool");
    }

    #[tokio::test]
    async fn test_delete_user_pool() {
        let state = new_state();
        let app = create_router(state.clone());
        let resp = app
            .oneshot(cognito_req("CreateUserPool", r#"{"PoolName": "del-pool"}"#))
            .await
            .unwrap();
        let json = extract_body(resp).await;
        let pool_id = json["UserPool"]["Id"].as_str().unwrap().to_string();

        let app = create_router(state.clone());
        let resp = app
            .oneshot(cognito_req("DeleteUserPool", &format!(r#"{{"UserPoolId": "{}"}}"#, pool_id)))
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        // Verify deleted
        let app = create_router(state);
        let resp = app
            .oneshot(cognito_req("DescribeUserPool", &format!(r#"{{"UserPoolId": "{}"}}"#, pool_id)))
            .await
            .unwrap();
        assert_ne!(resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_admin_create_and_get_user() {
        let state = new_state();
        let app = create_router(state.clone());
        let resp = app
            .oneshot(cognito_req("CreateUserPool", r#"{"PoolName": "user-pool"}"#))
            .await
            .unwrap();
        let json = extract_body(resp).await;
        let pool_id = json["UserPool"]["Id"].as_str().unwrap().to_string();

        // Create user
        let app = create_router(state.clone());
        let resp = app
            .oneshot(cognito_req(
                "AdminCreateUser",
                &format!(r#"{{"UserPoolId": "{}", "Username": "testuser"}}"#, pool_id),
            ))
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        // Get user
        let app = create_router(state.clone());
        let resp = app
            .oneshot(cognito_req(
                "AdminGetUser",
                &format!(r#"{{"UserPoolId": "{}", "Username": "testuser"}}"#, pool_id),
            ))
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        // List users
        let app = create_router(state.clone());
        let resp = app
            .oneshot(cognito_req(
                "ListUsers",
                &format!(r#"{{"UserPoolId": "{}"}}"#, pool_id),
            ))
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        // Delete user
        let app = create_router(state);
        let resp = app
            .oneshot(cognito_req(
                "AdminDeleteUser",
                &format!(r#"{{"UserPoolId": "{}", "Username": "testuser"}}"#, pool_id),
            ))
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_create_and_describe_user_pool_client() {
        let state = new_state();
        let app = create_router(state.clone());
        let resp = app
            .oneshot(cognito_req("CreateUserPool", r#"{"PoolName": "client-pool"}"#))
            .await
            .unwrap();
        let json = extract_body(resp).await;
        let pool_id = json["UserPool"]["Id"].as_str().unwrap().to_string();

        // Create client
        let app = create_router(state.clone());
        let resp = app
            .oneshot(cognito_req(
                "CreateUserPoolClient",
                &format!(r#"{{"UserPoolId": "{}", "ClientName": "my-client"}}"#, pool_id),
            ))
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let json = extract_body(resp).await;
        let client_id = json["UserPoolClient"]["ClientId"].as_str().unwrap().to_string();

        // Describe client
        let app = create_router(state.clone());
        let resp = app
            .oneshot(cognito_req(
                "DescribeUserPoolClient",
                &format!(r#"{{"UserPoolId": "{}", "ClientId": "{}"}}"#, pool_id, client_id),
            ))
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        // List clients
        let app = create_router(state.clone());
        let resp = app
            .oneshot(cognito_req(
                "ListUserPoolClients",
                &format!(r#"{{"UserPoolId": "{}"}}"#, pool_id),
            ))
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        // Delete client
        let app = create_router(state);
        let resp = app
            .oneshot(cognito_req(
                "DeleteUserPoolClient",
                &format!(r#"{{"UserPoolId": "{}", "ClientId": "{}"}}"#, pool_id, client_id),
            ))
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_create_and_get_group() {
        let state = new_state();
        let app = create_router(state.clone());
        let resp = app
            .oneshot(cognito_req("CreateUserPool", r#"{"PoolName": "grp-pool"}"#))
            .await
            .unwrap();
        let json = extract_body(resp).await;
        let pool_id = json["UserPool"]["Id"].as_str().unwrap().to_string();

        // Create group
        let app = create_router(state.clone());
        let resp = app
            .oneshot(cognito_req(
                "CreateGroup",
                &format!(r#"{{"UserPoolId": "{}", "GroupName": "admins"}}"#, pool_id),
            ))
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        // Get group
        let app = create_router(state.clone());
        let resp = app
            .oneshot(cognito_req(
                "GetGroup",
                &format!(r#"{{"UserPoolId": "{}", "GroupName": "admins"}}"#, pool_id),
            ))
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        // List groups
        let app = create_router(state.clone());
        let resp = app
            .oneshot(cognito_req(
                "ListGroups",
                &format!(r#"{{"UserPoolId": "{}"}}"#, pool_id),
            ))
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        // Delete group
        let app = create_router(state);
        let resp = app
            .oneshot(cognito_req(
                "DeleteGroup",
                &format!(r#"{{"UserPoolId": "{}", "GroupName": "admins"}}"#, pool_id),
            ))
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_admin_add_user_to_group() {
        let state = new_state();
        let app = create_router(state.clone());
        let resp = app
            .oneshot(cognito_req("CreateUserPool", r#"{"PoolName": "aug-pool"}"#))
            .await
            .unwrap();
        let json = extract_body(resp).await;
        let pool_id = json["UserPool"]["Id"].as_str().unwrap().to_string();

        // Create user
        let app = create_router(state.clone());
        app.oneshot(cognito_req(
            "AdminCreateUser",
            &format!(r#"{{"UserPoolId": "{}", "Username": "user1"}}"#, pool_id),
        ))
        .await
        .unwrap();

        // Create group
        let app = create_router(state.clone());
        app.oneshot(cognito_req(
            "CreateGroup",
            &format!(r#"{{"UserPoolId": "{}", "GroupName": "devs"}}"#, pool_id),
        ))
        .await
        .unwrap();

        // Add user to group
        let app = create_router(state.clone());
        let resp = app
            .oneshot(cognito_req(
                "AdminAddUserToGroup",
                &format!(r#"{{"UserPoolId": "{}", "Username": "user1", "GroupName": "devs"}}"#, pool_id),
            ))
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        // List groups for user
        let app = create_router(state.clone());
        let resp = app
            .oneshot(cognito_req(
                "AdminListGroupsForUser",
                &format!(r#"{{"UserPoolId": "{}", "Username": "user1"}}"#, pool_id),
            ))
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        // List users in group
        let app = create_router(state.clone());
        let resp = app
            .oneshot(cognito_req(
                "ListUsersInGroup",
                &format!(r#"{{"UserPoolId": "{}", "GroupName": "devs"}}"#, pool_id),
            ))
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        // Remove user from group
        let app = create_router(state);
        let resp = app
            .oneshot(cognito_req(
                "AdminRemoveUserFromGroup",
                &format!(r#"{{"UserPoolId": "{}", "Username": "user1", "GroupName": "devs"}}"#, pool_id),
            ))
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_admin_set_user_password() {
        let state = new_state();
        let app = create_router(state.clone());
        let resp = app
            .oneshot(cognito_req("CreateUserPool", r#"{"PoolName": "pwd-pool"}"#))
            .await
            .unwrap();
        let json = extract_body(resp).await;
        let pool_id = json["UserPool"]["Id"].as_str().unwrap().to_string();

        let app = create_router(state.clone());
        app.oneshot(cognito_req(
            "AdminCreateUser",
            &format!(r#"{{"UserPoolId": "{}", "Username": "pwduser"}}"#, pool_id),
        ))
        .await
        .unwrap();

        let app = create_router(state);
        let resp = app
            .oneshot(cognito_req(
                "AdminSetUserPassword",
                &format!(r#"{{"UserPoolId": "{}", "Username": "pwduser", "Password": "NewP@ss1!", "Permanent": true}}"#, pool_id),
            ))
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_admin_enable_disable_user() {
        let state = new_state();
        let app = create_router(state.clone());
        let resp = app
            .oneshot(cognito_req("CreateUserPool", r#"{"PoolName": "ed-pool"}"#))
            .await
            .unwrap();
        let json = extract_body(resp).await;
        let pool_id = json["UserPool"]["Id"].as_str().unwrap().to_string();

        let app = create_router(state.clone());
        app.oneshot(cognito_req(
            "AdminCreateUser",
            &format!(r#"{{"UserPoolId": "{}", "Username": "eduser"}}"#, pool_id),
        ))
        .await
        .unwrap();

        // Disable
        let app = create_router(state.clone());
        let resp = app
            .oneshot(cognito_req(
                "AdminDisableUser",
                &format!(r#"{{"UserPoolId": "{}", "Username": "eduser"}}"#, pool_id),
            ))
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        // Enable
        let app = create_router(state);
        let resp = app
            .oneshot(cognito_req(
                "AdminEnableUser",
                &format!(r#"{{"UserPoolId": "{}", "Username": "eduser"}}"#, pool_id),
            ))
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_update_user_pool() {
        let state = new_state();
        let app = create_router(state.clone());
        let resp = app
            .oneshot(cognito_req("CreateUserPool", r#"{"PoolName": "upd-pool"}"#))
            .await
            .unwrap();
        let json = extract_body(resp).await;
        let pool_id = json["UserPool"]["Id"].as_str().unwrap().to_string();

        let app = create_router(state);
        let resp = app
            .oneshot(cognito_req(
                "UpdateUserPool",
                &format!(r#"{{"UserPoolId": "{}", "AutoVerifiedAttributes": ["email"]}}"#, pool_id),
            ))
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
