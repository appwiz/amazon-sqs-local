use std::collections::HashMap;

use super::types::AttributeType;

fn now() -> f64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs_f64()
}

#[derive(Debug, Clone)]
pub struct UserPool {
    pub id: String,
    pub name: String,
    pub arn: String,
    pub status: String,
    pub creation_date: f64,
    pub last_modified_date: f64,
    pub auto_verified_attributes: Vec<String>,
    pub username_attributes: Vec<String>,
    pub tags: HashMap<String, String>,
    pub estimated_number_of_users: i64,
    pub users: HashMap<String, User>,
    pub clients: HashMap<String, UserPoolClient>,
    pub groups: HashMap<String, Group>,
}

impl UserPool {
    pub fn new(id: String, name: String, region: &str, account_id: &str) -> Self {
        let ts = now();
        UserPool {
            arn: format!(
                "arn:aws:cognito-idp:{}:{}:userpool/{}",
                region, account_id, id
            ),
            id,
            name,
            status: "Active".to_string(),
            creation_date: ts,
            last_modified_date: ts,
            auto_verified_attributes: Vec::new(),
            username_attributes: Vec::new(),
            tags: HashMap::new(),
            estimated_number_of_users: 0,
            users: HashMap::new(),
            clients: HashMap::new(),
            groups: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct User {
    pub username: String,
    pub attributes: Vec<AttributeType>,
    pub user_create_date: f64,
    pub user_last_modified_date: f64,
    pub enabled: bool,
    pub user_status: String,
    pub password: Option<String>,
    pub groups: Vec<String>,
}

impl User {
    pub fn new(username: String, attributes: Vec<AttributeType>, temporary_password: Option<String>) -> Self {
        let ts = now();
        let status = if temporary_password.is_some() {
            "FORCE_CHANGE_PASSWORD".to_string()
        } else {
            "CONFIRMED".to_string()
        };
        User {
            username,
            attributes,
            user_create_date: ts,
            user_last_modified_date: ts,
            enabled: true,
            user_status: status,
            password: temporary_password,
            groups: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct UserPoolClient {
    pub client_id: String,
    pub client_name: String,
    pub user_pool_id: String,
    pub client_secret: Option<String>,
    pub creation_date: f64,
    pub last_modified_date: f64,
    pub explicit_auth_flows: Vec<String>,
    pub allowed_o_auth_flows: Vec<String>,
    pub allowed_o_auth_scopes: Vec<String>,
    pub callback_urls: Vec<String>,
    pub logout_urls: Vec<String>,
    pub supported_identity_providers: Vec<String>,
    pub prevent_user_existence_errors: String,
    pub enable_token_revocation: bool,
}

impl UserPoolClient {
    pub fn new(client_id: String, client_name: String, user_pool_id: String) -> Self {
        let ts = now();
        UserPoolClient {
            client_id,
            client_name,
            user_pool_id,
            client_secret: None,
            creation_date: ts,
            last_modified_date: ts,
            explicit_auth_flows: Vec::new(),
            allowed_o_auth_flows: Vec::new(),
            allowed_o_auth_scopes: Vec::new(),
            callback_urls: Vec::new(),
            logout_urls: Vec::new(),
            supported_identity_providers: Vec::new(),
            prevent_user_existence_errors: "LEGACY".to_string(),
            enable_token_revocation: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Group {
    pub group_name: String,
    pub user_pool_id: String,
    pub description: Option<String>,
    pub role_arn: Option<String>,
    pub precedence: Option<i64>,
    pub creation_date: f64,
    pub last_modified_date: f64,
}

impl Group {
    pub fn new(group_name: String, user_pool_id: String, description: Option<String>, role_arn: Option<String>, precedence: Option<i64>) -> Self {
        let ts = now();
        Group {
            group_name,
            user_pool_id,
            description,
            role_arn,
            precedence,
            creation_date: ts,
            last_modified_date: ts,
        }
    }
}
