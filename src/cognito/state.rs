use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;
use uuid::Uuid;

use super::error::CognitoError;
use super::pool::{Group, User, UserPool, UserPoolClient};
use super::types::*;

fn now() -> f64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs_f64()
}

fn fake_token(kind: &str, username: &str, pool_id: &str) -> String {
    format!("{}-{}-{}-{}", kind, username, pool_id, Uuid::new_v4())
}

struct CognitoStateInner {
    pools: HashMap<String, UserPool>,
    account_id: String,
    region: String,
    pool_counter: u64,
}

pub struct CognitoState {
    inner: Arc<Mutex<CognitoStateInner>>,
}

impl CognitoState {
    pub fn new(account_id: String, region: String) -> Self {
        CognitoState {
            inner: Arc::new(Mutex::new(CognitoStateInner {
                pools: HashMap::new(),
                account_id,
                region,
                pool_counter: 0,
            })),
        }
    }

    // --- User Pool ---

    pub async fn create_user_pool(
        &self,
        req: CreateUserPoolRequest,
    ) -> Result<CreateUserPoolResponse, CognitoError> {
        let mut state = self.inner.lock().await;
        state.pool_counter += 1;
        let suffix = Uuid::new_v4()
            .to_string()
            .replace('-', "")
            .chars()
            .take(9)
            .collect::<String>();
        let id = format!("{}_{}", state.region, suffix);
        let mut pool = UserPool::new(id.clone(), req.pool_name, &state.region, &state.account_id);
        pool.auto_verified_attributes = req.auto_verified_attributes;
        pool.username_attributes = req.username_attributes;
        if let Some(tags) = req.user_pool_tags {
            pool.tags = tags;
        }
        let resp = CreateUserPoolResponse {
            user_pool: pool_to_type(&pool),
        };
        state.pools.insert(id, pool);
        Ok(resp)
    }

    pub async fn delete_user_pool(&self, req: DeleteUserPoolRequest) -> Result<(), CognitoError> {
        let mut state = self.inner.lock().await;
        if state.pools.remove(&req.user_pool_id).is_none() {
            return Err(CognitoError::ResourceNotFoundException(format!(
                "User pool {} not found.",
                req.user_pool_id
            )));
        }
        Ok(())
    }

    pub async fn describe_user_pool(
        &self,
        req: DescribeUserPoolRequest,
    ) -> Result<DescribeUserPoolResponse, CognitoError> {
        let state = self.inner.lock().await;
        let pool = get_pool(&state.pools, &req.user_pool_id)?;
        Ok(DescribeUserPoolResponse {
            user_pool: pool_to_type(pool),
        })
    }

    pub async fn list_user_pools(
        &self,
        req: ListUserPoolsRequest,
    ) -> Result<ListUserPoolsResponse, CognitoError> {
        let state = self.inner.lock().await;
        let mut pools: Vec<&UserPool> = state.pools.values().collect();
        pools.sort_by(|a, b| a.name.cmp(&b.name));

        let limit = req.max_results.unwrap_or(60).min(60);
        let start = req
            .next_token
            .as_deref()
            .and_then(|t| pools.iter().position(|p| p.id == t).map(|i| i + 1))
            .unwrap_or(0);

        let page: Vec<&UserPool> = pools.iter().skip(start).take(limit).copied().collect();
        let next_token = if start + limit < pools.len() {
            page.last().map(|p| p.id.clone())
        } else {
            None
        };

        Ok(ListUserPoolsResponse {
            user_pools: page
                .into_iter()
                .map(|p| UserPoolDescriptionType {
                    id: p.id.clone(),
                    name: p.name.clone(),
                    status: p.status.clone(),
                    creation_date: p.creation_date,
                    last_modified_date: p.last_modified_date,
                })
                .collect(),
            next_token,
        })
    }

    pub async fn update_user_pool(&self, req: UpdateUserPoolRequest) -> Result<(), CognitoError> {
        let mut state = self.inner.lock().await;
        let pool = get_pool_mut(&mut state.pools, &req.user_pool_id)?;
        if let Some(tags) = req.user_pool_tags {
            pool.tags = tags;
        }
        if let Some(attrs) = req.auto_verified_attributes {
            pool.auto_verified_attributes = attrs;
        }
        pool.last_modified_date = now();
        Ok(())
    }

    // --- Users ---

    pub async fn admin_create_user(
        &self,
        req: AdminCreateUserRequest,
    ) -> Result<AdminCreateUserResponse, CognitoError> {
        let mut state = self.inner.lock().await;
        let pool = get_pool_mut(&mut state.pools, &req.user_pool_id)?;

        if pool.users.contains_key(&req.username) {
            return Err(CognitoError::UsernameExistsException(format!(
                "User already exists: {}",
                req.username
            )));
        }

        let attrs: Vec<AttributeType> = req
            .user_attributes
            .into_iter()
            .map(|a| AttributeType {
                name: a.name,
                value: a.value,
            })
            .collect();

        let user = User::new(req.username.clone(), attrs.clone(), req.temporary_password);
        let resp = AdminCreateUserResponse {
            user: UserType {
                username: user.username.clone(),
                attributes: user
                    .attributes
                    .iter()
                    .map(|a| AttributeType {
                        name: a.name.clone(),
                        value: a.value.clone(),
                    })
                    .collect(),
                user_create_date: user.user_create_date,
                user_last_modified_date: user.user_last_modified_date,
                enabled: user.enabled,
                user_status: user.user_status.clone(),
            },
        };
        pool.users.insert(req.username, user);
        pool.estimated_number_of_users = pool.users.len() as i64;
        Ok(resp)
    }

    pub async fn admin_delete_user(
        &self,
        req: AdminDeleteUserRequest,
    ) -> Result<(), CognitoError> {
        let mut state = self.inner.lock().await;
        let pool = get_pool_mut(&mut state.pools, &req.user_pool_id)?;
        if pool.users.remove(&req.username).is_none() {
            return Err(CognitoError::UserNotFoundException(format!(
                "User does not exist: {}",
                req.username
            )));
        }
        pool.estimated_number_of_users = pool.users.len() as i64;
        Ok(())
    }

    pub async fn admin_get_user(
        &self,
        req: AdminGetUserRequest,
    ) -> Result<AdminGetUserResponse, CognitoError> {
        let state = self.inner.lock().await;
        let pool = get_pool(&state.pools, &req.user_pool_id)?;
        let user = pool
            .users
            .get(&req.username)
            .ok_or_else(|| CognitoError::UserNotFoundException(format!(
                "User does not exist: {}",
                req.username
            )))?;
        Ok(AdminGetUserResponse {
            username: user.username.clone(),
            user_attributes: user
                .attributes
                .iter()
                .map(|a| AttributeType {
                    name: a.name.clone(),
                    value: a.value.clone(),
                })
                .collect(),
            user_create_date: user.user_create_date,
            user_last_modified_date: user.user_last_modified_date,
            enabled: user.enabled,
            user_status: user.user_status.clone(),
            user_mfa_setting_list: Vec::new(),
        })
    }

    pub async fn admin_set_user_password(
        &self,
        req: AdminSetUserPasswordRequest,
    ) -> Result<(), CognitoError> {
        let mut state = self.inner.lock().await;
        let pool = get_pool_mut(&mut state.pools, &req.user_pool_id)?;
        let user = pool
            .users
            .get_mut(&req.username)
            .ok_or_else(|| CognitoError::UserNotFoundException(format!(
                "User does not exist: {}",
                req.username
            )))?;
        user.password = Some(req.password);
        user.user_last_modified_date = now();
        if req.permanent {
            user.user_status = "CONFIRMED".to_string();
        }
        Ok(())
    }

    pub async fn admin_enable_user(
        &self,
        req: AdminEnableUserRequest,
    ) -> Result<(), CognitoError> {
        let mut state = self.inner.lock().await;
        let pool = get_pool_mut(&mut state.pools, &req.user_pool_id)?;
        let user = pool
            .users
            .get_mut(&req.username)
            .ok_or_else(|| CognitoError::UserNotFoundException(format!(
                "User does not exist: {}",
                req.username
            )))?;
        user.enabled = true;
        user.user_last_modified_date = now();
        Ok(())
    }

    pub async fn admin_disable_user(
        &self,
        req: AdminDisableUserRequest,
    ) -> Result<(), CognitoError> {
        let mut state = self.inner.lock().await;
        let pool = get_pool_mut(&mut state.pools, &req.user_pool_id)?;
        let user = pool
            .users
            .get_mut(&req.username)
            .ok_or_else(|| CognitoError::UserNotFoundException(format!(
                "User does not exist: {}",
                req.username
            )))?;
        user.enabled = false;
        user.user_last_modified_date = now();
        Ok(())
    }

    pub async fn admin_reset_user_password(
        &self,
        req: AdminResetUserPasswordRequest,
    ) -> Result<(), CognitoError> {
        let mut state = self.inner.lock().await;
        let pool = get_pool_mut(&mut state.pools, &req.user_pool_id)?;
        let user = pool
            .users
            .get_mut(&req.username)
            .ok_or_else(|| CognitoError::UserNotFoundException(format!(
                "User does not exist: {}",
                req.username
            )))?;
        user.user_status = "RESET_REQUIRED".to_string();
        user.user_last_modified_date = now();
        Ok(())
    }

    pub async fn admin_update_user_attributes(
        &self,
        req: AdminUpdateUserAttributesRequest,
    ) -> Result<(), CognitoError> {
        let mut state = self.inner.lock().await;
        let pool = get_pool_mut(&mut state.pools, &req.user_pool_id)?;
        let user = pool
            .users
            .get_mut(&req.username)
            .ok_or_else(|| CognitoError::UserNotFoundException(format!(
                "User does not exist: {}",
                req.username
            )))?;
        for new_attr in req.user_attributes {
            if let Some(existing) = user.attributes.iter_mut().find(|a| a.name == new_attr.name) {
                existing.value = new_attr.value;
            } else {
                user.attributes.push(AttributeType {
                    name: new_attr.name,
                    value: new_attr.value,
                });
            }
        }
        user.user_last_modified_date = now();
        Ok(())
    }

    pub async fn list_users(
        &self,
        req: ListUsersRequest,
    ) -> Result<ListUsersResponse, CognitoError> {
        let state = self.inner.lock().await;
        let pool = get_pool(&state.pools, &req.user_pool_id)?;

        let mut users: Vec<&User> = pool.users.values().collect();
        users.sort_by(|a, b| a.username.cmp(&b.username));

        let limit = req.limit.unwrap_or(60).min(60);
        let start = req
            .pagination_token
            .as_deref()
            .and_then(|t| users.iter().position(|u| u.username == t).map(|i| i + 1))
            .unwrap_or(0);

        let page: Vec<&User> = users.iter().skip(start).take(limit).copied().collect();
        let pagination_token = if start + limit < users.len() {
            page.last().map(|u| u.username.clone())
        } else {
            None
        };

        Ok(ListUsersResponse {
            users: page.into_iter().map(user_to_type).collect(),
            pagination_token,
        })
    }

    // --- Clients ---

    pub async fn create_user_pool_client(
        &self,
        req: CreateUserPoolClientRequest,
    ) -> Result<CreateUserPoolClientResponse, CognitoError> {
        let mut state = self.inner.lock().await;
        let pool = get_pool_mut(&mut state.pools, &req.user_pool_id)?;

        let client_id = Uuid::new_v4()
            .to_string()
            .replace('-', "")
            .chars()
            .take(26)
            .collect::<String>();

        let mut client = UserPoolClient::new(
            client_id.clone(),
            req.client_name,
            req.user_pool_id.clone(),
        );
        if req.generate_secret {
            client.client_secret = Some(
                Uuid::new_v4()
                    .to_string()
                    .replace('-', "")
                    .chars()
                    .take(40)
                    .collect(),
            );
        }
        client.explicit_auth_flows = req.explicit_auth_flows;
        client.allowed_o_auth_flows = req.allowed_o_auth_flows;
        client.allowed_o_auth_scopes = req.allowed_o_auth_scopes;
        client.callback_urls = req.callback_ur_ls;
        client.logout_urls = req.logout_ur_ls;
        client.supported_identity_providers = req.supported_identity_providers;
        if let Some(p) = req.prevent_user_existence_errors {
            client.prevent_user_existence_errors = p;
        }
        if let Some(e) = req.enable_token_revocation {
            client.enable_token_revocation = e;
        }

        let resp = CreateUserPoolClientResponse {
            user_pool_client: client_to_type(&client),
        };
        pool.clients.insert(client_id, client);
        Ok(resp)
    }

    pub async fn delete_user_pool_client(
        &self,
        req: DeleteUserPoolClientRequest,
    ) -> Result<(), CognitoError> {
        let mut state = self.inner.lock().await;
        let pool = get_pool_mut(&mut state.pools, &req.user_pool_id)?;
        if pool.clients.remove(&req.client_id).is_none() {
            return Err(CognitoError::ResourceNotFoundException(format!(
                "User pool client {} not found.",
                req.client_id
            )));
        }
        Ok(())
    }

    pub async fn describe_user_pool_client(
        &self,
        req: DescribeUserPoolClientRequest,
    ) -> Result<DescribeUserPoolClientResponse, CognitoError> {
        let state = self.inner.lock().await;
        let pool = get_pool(&state.pools, &req.user_pool_id)?;
        let client = pool
            .clients
            .get(&req.client_id)
            .ok_or_else(|| CognitoError::ResourceNotFoundException(format!(
                "User pool client {} not found.",
                req.client_id
            )))?;
        Ok(DescribeUserPoolClientResponse {
            user_pool_client: client_to_type(client),
        })
    }

    pub async fn list_user_pool_clients(
        &self,
        req: ListUserPoolClientsRequest,
    ) -> Result<ListUserPoolClientsResponse, CognitoError> {
        let state = self.inner.lock().await;
        let pool = get_pool(&state.pools, &req.user_pool_id)?;

        let mut clients: Vec<&UserPoolClient> = pool.clients.values().collect();
        clients.sort_by(|a, b| a.client_name.cmp(&b.client_name));

        let limit = req.max_results.unwrap_or(60).min(60);
        let start = req
            .next_token
            .as_deref()
            .and_then(|t| clients.iter().position(|c| c.client_id == t).map(|i| i + 1))
            .unwrap_or(0);

        let page: Vec<&UserPoolClient> =
            clients.iter().skip(start).take(limit).copied().collect();
        let next_token = if start + limit < clients.len() {
            page.last().map(|c| c.client_id.clone())
        } else {
            None
        };

        Ok(ListUserPoolClientsResponse {
            user_pool_clients: page
                .into_iter()
                .map(|c| UserPoolClientDescription {
                    client_id: c.client_id.clone(),
                    client_name: c.client_name.clone(),
                    user_pool_id: c.user_pool_id.clone(),
                })
                .collect(),
            next_token,
        })
    }

    pub async fn update_user_pool_client(
        &self,
        req: UpdateUserPoolClientRequest,
    ) -> Result<UpdateUserPoolClientResponse, CognitoError> {
        let mut state = self.inner.lock().await;
        let pool = get_pool_mut(&mut state.pools, &req.user_pool_id)?;
        let client = pool
            .clients
            .get_mut(&req.client_id)
            .ok_or_else(|| CognitoError::ResourceNotFoundException(format!(
                "User pool client {} not found.",
                req.client_id
            )))?;
        if let Some(name) = req.client_name {
            client.client_name = name;
        }
        if let Some(flows) = req.explicit_auth_flows {
            client.explicit_auth_flows = flows;
        }
        if let Some(flows) = req.allowed_o_auth_flows {
            client.allowed_o_auth_flows = flows;
        }
        if let Some(scopes) = req.allowed_o_auth_scopes {
            client.allowed_o_auth_scopes = scopes;
        }
        if let Some(urls) = req.callback_ur_ls {
            client.callback_urls = urls;
        }
        if let Some(urls) = req.logout_ur_ls {
            client.logout_urls = urls;
        }
        if let Some(p) = req.prevent_user_existence_errors {
            client.prevent_user_existence_errors = p;
        }
        if let Some(e) = req.enable_token_revocation {
            client.enable_token_revocation = e;
        }
        client.last_modified_date = now();
        Ok(UpdateUserPoolClientResponse {
            user_pool_client: client_to_type(client),
        })
    }

    // --- Groups ---

    pub async fn create_group(
        &self,
        req: CreateGroupRequest,
    ) -> Result<CreateGroupResponse, CognitoError> {
        let mut state = self.inner.lock().await;
        let pool = get_pool_mut(&mut state.pools, &req.user_pool_id)?;

        if pool.groups.contains_key(&req.group_name) {
            return Err(CognitoError::GroupExistsException(format!(
                "Group already exists: {}",
                req.group_name
            )));
        }

        let group = Group::new(
            req.group_name.clone(),
            req.user_pool_id,
            req.description,
            req.role_arn,
            req.precedence,
        );
        let resp = CreateGroupResponse {
            group: group_to_type(&group),
        };
        pool.groups.insert(req.group_name, group);
        Ok(resp)
    }

    pub async fn delete_group(&self, req: DeleteGroupRequest) -> Result<(), CognitoError> {
        let mut state = self.inner.lock().await;
        let pool = get_pool_mut(&mut state.pools, &req.user_pool_id)?;
        if pool.groups.remove(&req.group_name).is_none() {
            return Err(CognitoError::ResourceNotFoundException(format!(
                "Group does not exist: {}",
                req.group_name
            )));
        }
        Ok(())
    }

    pub async fn get_group(&self, req: GetGroupRequest) -> Result<GetGroupResponse, CognitoError> {
        let state = self.inner.lock().await;
        let pool = get_pool(&state.pools, &req.user_pool_id)?;
        let group = pool
            .groups
            .get(&req.group_name)
            .ok_or_else(|| CognitoError::ResourceNotFoundException(format!(
                "Group does not exist: {}",
                req.group_name
            )))?;
        Ok(GetGroupResponse {
            group: group_to_type(group),
        })
    }

    pub async fn list_groups(
        &self,
        req: ListGroupsRequest,
    ) -> Result<ListGroupsResponse, CognitoError> {
        let state = self.inner.lock().await;
        let pool = get_pool(&state.pools, &req.user_pool_id)?;

        let mut groups: Vec<&Group> = pool.groups.values().collect();
        groups.sort_by(|a, b| a.group_name.cmp(&b.group_name));

        let limit = req.limit.unwrap_or(60).min(60);
        let start = req
            .next_token
            .as_deref()
            .and_then(|t| groups.iter().position(|g| g.group_name == t).map(|i| i + 1))
            .unwrap_or(0);

        let page: Vec<&Group> = groups.iter().skip(start).take(limit).copied().collect();
        let next_token = if start + limit < groups.len() {
            page.last().map(|g| g.group_name.clone())
        } else {
            None
        };

        Ok(ListGroupsResponse {
            groups: page.into_iter().map(group_to_type).collect(),
            next_token,
        })
    }

    pub async fn admin_add_user_to_group(
        &self,
        req: AdminAddUserToGroupRequest,
    ) -> Result<(), CognitoError> {
        let mut state = self.inner.lock().await;
        let pool = get_pool_mut(&mut state.pools, &req.user_pool_id)?;

        if !pool.groups.contains_key(&req.group_name) {
            return Err(CognitoError::ResourceNotFoundException(format!(
                "Group does not exist: {}",
                req.group_name
            )));
        }
        let user = pool
            .users
            .get_mut(&req.username)
            .ok_or_else(|| CognitoError::UserNotFoundException(format!(
                "User does not exist: {}",
                req.username
            )))?;
        if !user.groups.contains(&req.group_name) {
            user.groups.push(req.group_name);
        }
        Ok(())
    }

    pub async fn admin_remove_user_from_group(
        &self,
        req: AdminRemoveUserFromGroupRequest,
    ) -> Result<(), CognitoError> {
        let mut state = self.inner.lock().await;
        let pool = get_pool_mut(&mut state.pools, &req.user_pool_id)?;

        if !pool.groups.contains_key(&req.group_name) {
            return Err(CognitoError::ResourceNotFoundException(format!(
                "Group does not exist: {}",
                req.group_name
            )));
        }
        let user = pool
            .users
            .get_mut(&req.username)
            .ok_or_else(|| CognitoError::UserNotFoundException(format!(
                "User does not exist: {}",
                req.username
            )))?;
        user.groups.retain(|g| g != &req.group_name);
        Ok(())
    }

    pub async fn admin_list_groups_for_user(
        &self,
        req: AdminListGroupsForUserRequest,
    ) -> Result<AdminListGroupsForUserResponse, CognitoError> {
        let state = self.inner.lock().await;
        let pool = get_pool(&state.pools, &req.user_pool_id)?;
        let user = pool
            .users
            .get(&req.username)
            .ok_or_else(|| CognitoError::UserNotFoundException(format!(
                "User does not exist: {}",
                req.username
            )))?;

        let mut groups: Vec<GroupType> = user
            .groups
            .iter()
            .filter_map(|g| pool.groups.get(g))
            .map(group_to_type)
            .collect();
        groups.sort_by(|a, b| a.group_name.cmp(&b.group_name));

        Ok(AdminListGroupsForUserResponse {
            groups,
            next_token: None,
        })
    }

    pub async fn list_users_in_group(
        &self,
        req: ListUsersInGroupRequest,
    ) -> Result<ListUsersInGroupResponse, CognitoError> {
        let state = self.inner.lock().await;
        let pool = get_pool(&state.pools, &req.user_pool_id)?;

        if !pool.groups.contains_key(&req.group_name) {
            return Err(CognitoError::ResourceNotFoundException(format!(
                "Group does not exist: {}",
                req.group_name
            )));
        }

        let mut users: Vec<&User> = pool
            .users
            .values()
            .filter(|u| u.groups.contains(&req.group_name))
            .collect();
        users.sort_by(|a, b| a.username.cmp(&b.username));

        Ok(ListUsersInGroupResponse {
            users: users.into_iter().map(user_to_type).collect(),
            next_token: None,
        })
    }

    // --- Auth ---

    pub async fn initiate_auth(
        &self,
        req: InitiateAuthRequest,
    ) -> Result<InitiateAuthResponse, CognitoError> {
        let state = self.inner.lock().await;

        // Find pool that owns this client
        let (pool, _client) = state
            .pools
            .values()
            .find_map(|p| p.clients.get(&req.client_id).map(|c| (p, c)))
            .ok_or_else(|| {
                CognitoError::ResourceNotFoundException(format!(
                    "User pool client {} not found.",
                    req.client_id
                ))
            })?;

        match req.auth_flow.as_str() {
            "USER_PASSWORD_AUTH" | "USER_SRP_AUTH" => {
                let params = req.auth_parameters.unwrap_or_default();
                let username = params.get("USERNAME").cloned().unwrap_or_default();
                let _password = params.get("PASSWORD").cloned().unwrap_or_default();

                let user = pool
                    .users
                    .get(&username)
                    .ok_or_else(|| CognitoError::UserNotFoundException(format!(
                        "User does not exist: {}",
                        username
                    )))?;

                if !user.enabled {
                    return Err(CognitoError::NotAuthorizedException(
                        "User is disabled.".to_string(),
                    ));
                }

                Ok(InitiateAuthResponse {
                    authentication_result: Some(AuthenticationResultType {
                        access_token: fake_token("access", &username, &pool.id),
                        expires_in: 3600,
                        token_type: "Bearer".to_string(),
                        refresh_token: fake_token("refresh", &username, &pool.id),
                        id_token: fake_token("id", &username, &pool.id),
                    }),
                    challenge_name: None,
                    session: None,
                })
            }
            "REFRESH_TOKEN_AUTH" | "REFRESH_TOKEN" => Ok(InitiateAuthResponse {
                authentication_result: Some(AuthenticationResultType {
                    access_token: fake_token("access", "refresh-user", &pool.id),
                    expires_in: 3600,
                    token_type: "Bearer".to_string(),
                    refresh_token: fake_token("refresh", "refresh-user", &pool.id),
                    id_token: fake_token("id", "refresh-user", &pool.id),
                }),
                challenge_name: None,
                session: None,
            }),
            other => Err(CognitoError::InvalidParameterException(format!(
                "Unsupported auth flow: {}",
                other
            ))),
        }
    }

    pub async fn admin_initiate_auth(
        &self,
        req: AdminInitiateAuthRequest,
    ) -> Result<InitiateAuthResponse, CognitoError> {
        let state = self.inner.lock().await;
        let pool = get_pool(&state.pools, &req.user_pool_id)?;

        if !pool.clients.contains_key(&req.client_id) {
            return Err(CognitoError::ResourceNotFoundException(format!(
                "User pool client {} not found.",
                req.client_id
            )));
        }

        match req.auth_flow.as_str() {
            "ADMIN_USER_PASSWORD_AUTH" | "ADMIN_NO_SRP_AUTH" => {
                let params = req.auth_parameters.unwrap_or_default();
                let username = params.get("USERNAME").cloned().unwrap_or_default();

                let user = pool
                    .users
                    .get(&username)
                    .ok_or_else(|| CognitoError::UserNotFoundException(format!(
                        "User does not exist: {}",
                        username
                    )))?;

                if !user.enabled {
                    return Err(CognitoError::NotAuthorizedException(
                        "User is disabled.".to_string(),
                    ));
                }

                Ok(InitiateAuthResponse {
                    authentication_result: Some(AuthenticationResultType {
                        access_token: fake_token("access", &username, &pool.id),
                        expires_in: 3600,
                        token_type: "Bearer".to_string(),
                        refresh_token: fake_token("refresh", &username, &pool.id),
                        id_token: fake_token("id", &username, &pool.id),
                    }),
                    challenge_name: None,
                    session: None,
                })
            }
            other => Err(CognitoError::InvalidParameterException(format!(
                "Unsupported auth flow: {}",
                other
            ))),
        }
    }

    pub async fn sign_up(&self, req: SignUpRequest) -> Result<SignUpResponse, CognitoError> {
        let mut state = self.inner.lock().await;

        // Find pool by client id
        let pool_id = state
            .pools
            .values()
            .find(|p| p.clients.contains_key(&req.client_id))
            .map(|p| p.id.clone())
            .ok_or_else(|| {
                CognitoError::ResourceNotFoundException(format!(
                    "User pool client {} not found.",
                    req.client_id
                ))
            })?;

        let pool = state.pools.get_mut(&pool_id).unwrap();

        if pool.users.contains_key(&req.username) {
            return Err(CognitoError::UsernameExistsException(format!(
                "User already exists: {}",
                req.username
            )));
        }

        let sub = Uuid::new_v4().to_string();
        let mut attrs: Vec<AttributeType> = req
            .user_attributes
            .into_iter()
            .map(|a| AttributeType {
                name: a.name,
                value: a.value,
            })
            .collect();
        attrs.push(AttributeType {
            name: "sub".to_string(),
            value: sub.clone(),
        });

        let user = User::new(req.username.clone(), attrs, Some(req.password));
        let mut user = user;
        user.user_status = "UNCONFIRMED".to_string();
        pool.users.insert(req.username, user);
        pool.estimated_number_of_users = pool.users.len() as i64;

        Ok(SignUpResponse {
            user_confirmed: false,
            user_sub: sub,
        })
    }

    pub async fn confirm_sign_up(
        &self,
        req: ConfirmSignUpRequest,
    ) -> Result<(), CognitoError> {
        let mut state = self.inner.lock().await;

        let pool_id = state
            .pools
            .values()
            .find(|p| p.clients.contains_key(&req.client_id))
            .map(|p| p.id.clone())
            .ok_or_else(|| {
                CognitoError::ResourceNotFoundException(format!(
                    "User pool client {} not found.",
                    req.client_id
                ))
            })?;

        let pool = state.pools.get_mut(&pool_id).unwrap();
        let user = pool
            .users
            .get_mut(&req.username)
            .ok_or_else(|| CognitoError::UserNotFoundException(format!(
                "User does not exist: {}",
                req.username
            )))?;

        // Accept any code in in-memory mode
        user.user_status = "CONFIRMED".to_string();
        user.user_last_modified_date = now();
        Ok(())
    }

    pub async fn forgot_password(
        &self,
        req: ForgotPasswordRequest,
    ) -> Result<ForgotPasswordResponse, CognitoError> {
        let state = self.inner.lock().await;

        let pool = state
            .pools
            .values()
            .find(|p| p.clients.contains_key(&req.client_id))
            .ok_or_else(|| {
                CognitoError::ResourceNotFoundException(format!(
                    "User pool client {} not found.",
                    req.client_id
                ))
            })?;

        if !pool.users.contains_key(&req.username) {
            return Err(CognitoError::UserNotFoundException(format!(
                "User does not exist: {}",
                req.username
            )));
        }

        Ok(ForgotPasswordResponse {
            code_delivery_details: CodeDeliveryDetailsType {
                destination: "test@example.com".to_string(),
                delivery_medium: "EMAIL".to_string(),
                attribute_name: "email".to_string(),
            },
        })
    }

    pub async fn confirm_forgot_password(
        &self,
        req: ConfirmForgotPasswordRequest,
    ) -> Result<(), CognitoError> {
        let mut state = self.inner.lock().await;

        let pool_id = state
            .pools
            .values()
            .find(|p| p.clients.contains_key(&req.client_id))
            .map(|p| p.id.clone())
            .ok_or_else(|| {
                CognitoError::ResourceNotFoundException(format!(
                    "User pool client {} not found.",
                    req.client_id
                ))
            })?;

        let pool = state.pools.get_mut(&pool_id).unwrap();
        let user = pool
            .users
            .get_mut(&req.username)
            .ok_or_else(|| CognitoError::UserNotFoundException(format!(
                "User does not exist: {}",
                req.username
            )))?;

        user.password = Some(req.password);
        user.user_status = "CONFIRMED".to_string();
        user.user_last_modified_date = now();
        Ok(())
    }
}

// --- Helpers ---

fn get_pool<'a>(
    pools: &'a HashMap<String, UserPool>,
    pool_id: &str,
) -> Result<&'a UserPool, CognitoError> {
    pools
        .get(pool_id)
        .ok_or_else(|| CognitoError::ResourceNotFoundException(format!(
            "User pool {} not found.",
            pool_id
        )))
}

fn get_pool_mut<'a>(
    pools: &'a mut HashMap<String, UserPool>,
    pool_id: &str,
) -> Result<&'a mut UserPool, CognitoError> {
    pools
        .get_mut(pool_id)
        .ok_or_else(|| CognitoError::ResourceNotFoundException(format!(
            "User pool {} not found.",
            pool_id
        )))
}

fn pool_to_type(pool: &UserPool) -> UserPoolType {
    UserPoolType {
        id: pool.id.clone(),
        name: pool.name.clone(),
        arn: pool.arn.clone(),
        status: pool.status.clone(),
        creation_date: pool.creation_date,
        last_modified_date: pool.last_modified_date,
        estimated_number_of_users: pool.estimated_number_of_users,
        auto_verified_attributes: pool.auto_verified_attributes.clone(),
        username_attributes: pool.username_attributes.clone(),
    }
}

fn user_to_type(user: &User) -> UserType {
    UserType {
        username: user.username.clone(),
        attributes: user
            .attributes
            .iter()
            .map(|a| AttributeType {
                name: a.name.clone(),
                value: a.value.clone(),
            })
            .collect(),
        user_create_date: user.user_create_date,
        user_last_modified_date: user.user_last_modified_date,
        enabled: user.enabled,
        user_status: user.user_status.clone(),
    }
}

fn client_to_type(client: &UserPoolClient) -> UserPoolClientType {
    UserPoolClientType {
        client_id: client.client_id.clone(),
        client_name: client.client_name.clone(),
        user_pool_id: client.user_pool_id.clone(),
        client_secret: client.client_secret.clone(),
        creation_date: client.creation_date,
        last_modified_date: client.last_modified_date,
        explicit_auth_flows: client.explicit_auth_flows.clone(),
        allowed_o_auth_flows: client.allowed_o_auth_flows.clone(),
        allowed_o_auth_scopes: client.allowed_o_auth_scopes.clone(),
        callback_ur_ls: client.callback_urls.clone(),
        logout_ur_ls: client.logout_urls.clone(),
        prevent_user_existence_errors: client.prevent_user_existence_errors.clone(),
        enable_token_revocation: client.enable_token_revocation,
    }
}

fn group_to_type(group: &Group) -> GroupType {
    GroupType {
        group_name: group.group_name.clone(),
        user_pool_id: group.user_pool_id.clone(),
        description: group.description.clone(),
        role_arn: group.role_arn.clone(),
        precedence: group.precedence,
        creation_date: group.creation_date,
        last_modified_date: group.last_modified_date,
    }
}
