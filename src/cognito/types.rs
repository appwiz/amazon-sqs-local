use serde::{Deserialize, Serialize};

// --- Shared types ---

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AttributeType {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Tag {
    pub key: String,
    #[serde(default)]
    pub value: Option<String>,
}

// --- UserPool types ---

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct UserPoolType {
    pub id: String,
    pub name: String,
    pub arn: String,
    pub status: String,
    pub creation_date: f64,
    pub last_modified_date: f64,
    pub estimated_number_of_users: i64,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub auto_verified_attributes: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub username_attributes: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct UserPoolDescriptionType {
    pub id: String,
    pub name: String,
    pub status: String,
    pub creation_date: f64,
    pub last_modified_date: f64,
}

// --- CreateUserPool ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateUserPoolRequest {
    pub pool_name: String,
    #[serde(default)]
    pub auto_verified_attributes: Vec<String>,
    #[serde(default)]
    pub username_attributes: Vec<String>,
    #[serde(default)]
    pub user_pool_tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateUserPoolResponse {
    pub user_pool: UserPoolType,
}

// --- DeleteUserPool ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteUserPoolRequest {
    pub user_pool_id: String,
}

// --- DescribeUserPool ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DescribeUserPoolRequest {
    pub user_pool_id: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct DescribeUserPoolResponse {
    pub user_pool: UserPoolType,
}

// --- ListUserPools ---

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ListUserPoolsRequest {
    #[serde(default)]
    pub max_results: Option<usize>,
    #[serde(default)]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListUserPoolsResponse {
    pub user_pools: Vec<UserPoolDescriptionType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

// --- UpdateUserPool ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UpdateUserPoolRequest {
    pub user_pool_id: String,
    #[serde(default)]
    pub user_pool_tags: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    pub auto_verified_attributes: Option<Vec<String>>,
}

// --- User types ---

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct UserType {
    pub username: String,
    pub attributes: Vec<AttributeType>,
    pub user_create_date: f64,
    pub user_last_modified_date: f64,
    pub enabled: bool,
    pub user_status: String,
}

// --- AdminCreateUser ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AdminCreateUserRequest {
    pub user_pool_id: String,
    pub username: String,
    #[serde(default)]
    pub user_attributes: Vec<AttributeType>,
    #[serde(default)]
    pub temporary_password: Option<String>,
    #[serde(default)]
    pub force_alias_creation: bool,
    #[serde(default)]
    pub message_action: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct AdminCreateUserResponse {
    pub user: UserType,
}

// --- AdminDeleteUser ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AdminDeleteUserRequest {
    pub user_pool_id: String,
    pub username: String,
}

// --- AdminGetUser ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AdminGetUserRequest {
    pub user_pool_id: String,
    pub username: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct AdminGetUserResponse {
    pub username: String,
    pub user_attributes: Vec<AttributeType>,
    pub user_create_date: f64,
    pub user_last_modified_date: f64,
    pub enabled: bool,
    pub user_status: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub user_mfa_setting_list: Vec<String>,
}

// --- AdminSetUserPassword ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AdminSetUserPasswordRequest {
    pub user_pool_id: String,
    pub username: String,
    pub password: String,
    #[serde(default)]
    pub permanent: bool,
}

// --- AdminEnableUser / AdminDisableUser ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AdminEnableUserRequest {
    pub user_pool_id: String,
    pub username: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AdminDisableUserRequest {
    pub user_pool_id: String,
    pub username: String,
}

// --- AdminResetUserPassword ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AdminResetUserPasswordRequest {
    pub user_pool_id: String,
    pub username: String,
}

// --- ListUsers ---

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ListUsersRequest {
    pub user_pool_id: String,
    #[serde(default)]
    pub limit: Option<usize>,
    #[serde(default)]
    pub pagination_token: Option<String>,
    #[serde(default)]
    pub filter: Option<String>,
    #[serde(default)]
    pub attributes_to_get: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListUsersResponse {
    pub users: Vec<UserType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination_token: Option<String>,
}

// --- UserPoolClient types ---

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct UserPoolClientType {
    pub client_id: String,
    pub client_name: String,
    pub user_pool_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    pub creation_date: f64,
    pub last_modified_date: f64,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub explicit_auth_flows: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub allowed_o_auth_flows: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub allowed_o_auth_scopes: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub callback_ur_ls: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub logout_ur_ls: Vec<String>,
    pub prevent_user_existence_errors: String,
    pub enable_token_revocation: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct UserPoolClientDescription {
    pub client_id: String,
    pub client_name: String,
    pub user_pool_id: String,
}

// --- CreateUserPoolClient ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateUserPoolClientRequest {
    pub user_pool_id: String,
    pub client_name: String,
    #[serde(default)]
    pub generate_secret: bool,
    #[serde(default)]
    pub explicit_auth_flows: Vec<String>,
    #[serde(default)]
    pub allowed_o_auth_flows: Vec<String>,
    #[serde(default)]
    pub allowed_o_auth_scopes: Vec<String>,
    #[serde(default)]
    pub callback_ur_ls: Vec<String>,
    #[serde(default)]
    pub logout_ur_ls: Vec<String>,
    #[serde(default)]
    pub supported_identity_providers: Vec<String>,
    #[serde(default)]
    pub prevent_user_existence_errors: Option<String>,
    #[serde(default)]
    pub enable_token_revocation: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateUserPoolClientResponse {
    pub user_pool_client: UserPoolClientType,
}

// --- DeleteUserPoolClient ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteUserPoolClientRequest {
    pub user_pool_id: String,
    pub client_id: String,
}

// --- DescribeUserPoolClient ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DescribeUserPoolClientRequest {
    pub user_pool_id: String,
    pub client_id: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct DescribeUserPoolClientResponse {
    pub user_pool_client: UserPoolClientType,
}

// --- ListUserPoolClients ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListUserPoolClientsRequest {
    pub user_pool_id: String,
    #[serde(default)]
    pub max_results: Option<usize>,
    #[serde(default)]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListUserPoolClientsResponse {
    pub user_pool_clients: Vec<UserPoolClientDescription>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

// --- UpdateUserPoolClient ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UpdateUserPoolClientRequest {
    pub user_pool_id: String,
    pub client_id: String,
    #[serde(default)]
    pub client_name: Option<String>,
    #[serde(default)]
    pub explicit_auth_flows: Option<Vec<String>>,
    #[serde(default)]
    pub allowed_o_auth_flows: Option<Vec<String>>,
    #[serde(default)]
    pub allowed_o_auth_scopes: Option<Vec<String>>,
    #[serde(default)]
    pub callback_ur_ls: Option<Vec<String>>,
    #[serde(default)]
    pub logout_ur_ls: Option<Vec<String>>,
    #[serde(default)]
    pub prevent_user_existence_errors: Option<String>,
    #[serde(default)]
    pub enable_token_revocation: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct UpdateUserPoolClientResponse {
    pub user_pool_client: UserPoolClientType,
}

// --- Group types ---

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct GroupType {
    pub group_name: String,
    pub user_pool_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precedence: Option<i64>,
    pub creation_date: f64,
    pub last_modified_date: f64,
}

// --- CreateGroup ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateGroupRequest {
    pub user_pool_id: String,
    pub group_name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub role_arn: Option<String>,
    #[serde(default)]
    pub precedence: Option<i64>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateGroupResponse {
    pub group: GroupType,
}

// --- DeleteGroup ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteGroupRequest {
    pub user_pool_id: String,
    pub group_name: String,
}

// --- GetGroup ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetGroupRequest {
    pub user_pool_id: String,
    pub group_name: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetGroupResponse {
    pub group: GroupType,
}

// --- ListGroups ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListGroupsRequest {
    pub user_pool_id: String,
    #[serde(default)]
    pub limit: Option<usize>,
    #[serde(default)]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListGroupsResponse {
    pub groups: Vec<GroupType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

// --- AdminAddUserToGroup ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AdminAddUserToGroupRequest {
    pub user_pool_id: String,
    pub username: String,
    pub group_name: String,
}

// --- AdminRemoveUserFromGroup ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AdminRemoveUserFromGroupRequest {
    pub user_pool_id: String,
    pub username: String,
    pub group_name: String,
}

// --- AdminListGroupsForUser ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AdminListGroupsForUserRequest {
    pub user_pool_id: String,
    pub username: String,
    #[serde(default)]
    pub limit: Option<usize>,
    #[serde(default)]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct AdminListGroupsForUserResponse {
    pub groups: Vec<GroupType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

// --- ListUsersInGroup ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListUsersInGroupRequest {
    pub user_pool_id: String,
    pub group_name: String,
    #[serde(default)]
    pub limit: Option<usize>,
    #[serde(default)]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListUsersInGroupResponse {
    pub users: Vec<UserType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

// --- Auth types ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct InitiateAuthRequest {
    pub auth_flow: String,
    pub client_id: String,
    #[serde(default)]
    pub auth_parameters: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AdminInitiateAuthRequest {
    pub user_pool_id: String,
    pub client_id: String,
    pub auth_flow: String,
    #[serde(default)]
    pub auth_parameters: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct AuthenticationResultType {
    pub access_token: String,
    pub expires_in: i64,
    pub token_type: String,
    pub refresh_token: String,
    pub id_token: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct InitiateAuthResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_result: Option<AuthenticationResultType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenge_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<String>,
}

// --- SignUp ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SignUpRequest {
    pub client_id: String,
    pub username: String,
    pub password: String,
    #[serde(default)]
    pub user_attributes: Vec<AttributeType>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct SignUpResponse {
    pub user_confirmed: bool,
    pub user_sub: String,
}

// --- ConfirmSignUp ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfirmSignUpRequest {
    pub client_id: String,
    pub username: String,
    pub confirmation_code: String,
}

// --- ForgotPassword ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ForgotPasswordRequest {
    pub client_id: String,
    pub username: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ForgotPasswordResponse {
    pub code_delivery_details: CodeDeliveryDetailsType,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CodeDeliveryDetailsType {
    pub destination: String,
    pub delivery_medium: String,
    pub attribute_name: String,
}

// --- ConfirmForgotPassword ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfirmForgotPasswordRequest {
    pub client_id: String,
    pub username: String,
    pub confirmation_code: String,
    pub password: String,
}

// --- ChangePassword ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ChangePasswordRequest {
    pub access_token: String,
    pub previous_password: String,
    pub proposed_password: String,
}

// --- AdminUpdateUserAttributes ---

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AdminUpdateUserAttributesRequest {
    pub user_pool_id: String,
    pub username: String,
    pub user_attributes: Vec<AttributeType>,
}
