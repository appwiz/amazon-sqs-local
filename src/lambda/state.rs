use std::collections::HashMap;
use std::sync::Arc;

use base64::Engine;
use sha2::{Digest, Sha256};
use tokio::sync::Mutex;
use uuid::Uuid;

use super::error::LambdaError;
use super::function::{
    Alias, AliasRoutingConfig, EventSourceMapping, LambdaFunction, PolicyStatement,
    PublishedVersion,
};
use super::types::*;

struct LambdaStateInner {
    functions: HashMap<String, LambdaFunction>,
    event_source_mappings: HashMap<String, EventSourceMapping>,
    account_id: String,
    region: String,
}

pub struct LambdaState {
    inner: Arc<Mutex<LambdaStateInner>>,
}

fn not_found_err(account_id: &str, region: &str, function_name: &str) -> LambdaError {
    LambdaError::ResourceNotFoundException(format!(
        "Function not found: arn:aws:lambda:{}:{}:function:{}",
        region, account_id, function_name
    ))
}

impl LambdaState {
    pub fn new(account_id: String, region: String) -> Self {
        LambdaState {
            inner: Arc::new(Mutex::new(LambdaStateInner {
                functions: HashMap::new(),
                event_source_mappings: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    fn compute_sha256(data: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        let result = hasher.finalize();
        base64::engine::general_purpose::STANDARD.encode(result)
    }

    fn now_iso() -> String {
        chrono::Utc::now()
            .format("%Y-%m-%dT%H:%M:%S%.3f+0000")
            .to_string()
    }

    fn make_arn(account_id: &str, region: &str, function_name: &str) -> String {
        format!(
            "arn:aws:lambda:{}:{}:function:{}",
            region, account_id, function_name
        )
    }

    fn function_to_config(func: &LambdaFunction) -> FunctionConfiguration {
        FunctionConfiguration {
            function_name: func.function_name.clone(),
            function_arn: func.function_arn.clone(),
            runtime: func.runtime.clone(),
            role: func.role.clone(),
            handler: func.handler.clone(),
            code_size: func.code_size,
            description: func.description.clone(),
            timeout: func.timeout,
            memory_size: func.memory_size,
            last_modified: func.last_modified.clone(),
            code_sha256: func.code_sha256.clone(),
            version: func.version.clone(),
            state: func.state.clone(),
            package_type: func.package_type.clone(),
            environment: if func.environment.is_empty() {
                None
            } else {
                Some(Environment {
                    variables: Some(func.environment.clone()),
                })
            },
            architectures: func.architectures.clone(),
        }
    }

    fn published_version_to_config(
        pv: &PublishedVersion,
        func: &LambdaFunction,
    ) -> FunctionConfiguration {
        FunctionConfiguration {
            function_name: func.function_name.clone(),
            function_arn: format!("{}:{}", func.function_arn, pv.version),
            runtime: pv.runtime.clone(),
            role: pv.role.clone(),
            handler: pv.handler.clone(),
            code_size: pv.code_size,
            description: pv.description.clone(),
            timeout: pv.timeout,
            memory_size: pv.memory_size,
            last_modified: pv.last_modified.clone(),
            code_sha256: pv.code_sha256.clone(),
            version: pv.version.clone(),
            state: "Active".to_string(),
            package_type: func.package_type.clone(),
            environment: if pv.environment.is_empty() {
                None
            } else {
                Some(Environment {
                    variables: Some(pv.environment.clone()),
                })
            },
            architectures: pv.architectures.clone(),
        }
    }

    pub async fn create_function(
        &self,
        req: CreateFunctionRequest,
    ) -> Result<FunctionConfiguration, LambdaError> {
        let mut inner = self.inner.lock().await;

        if inner.functions.contains_key(&req.function_name) {
            return Err(LambdaError::ResourceConflictException(format!(
                "Function already exist: {}",
                req.function_name
            )));
        }

        let code_bytes = if let Some(ref zip_file) = req.code.zip_file {
            base64::engine::general_purpose::STANDARD
                .decode(zip_file)
                .unwrap_or_default()
        } else {
            Vec::new()
        };

        let code_sha256 = Self::compute_sha256(&code_bytes);
        let code_size = code_bytes.len() as i64;
        let arn = Self::make_arn(&inner.account_id, &inner.region, &req.function_name);
        let now = Self::now_iso();

        let env_vars = req
            .environment
            .and_then(|e| e.variables)
            .unwrap_or_default();

        let func = LambdaFunction {
            function_name: req.function_name.clone(),
            function_arn: arn,
            runtime: req.runtime,
            role: req.role,
            handler: req.handler,
            code_size,
            code_sha256,
            description: req.description.unwrap_or_default(),
            timeout: req.timeout.unwrap_or(3),
            memory_size: req.memory_size.unwrap_or(128),
            last_modified: now,
            version: "$LATEST".to_string(),
            state: "Active".to_string(),
            package_type: req.package_type.unwrap_or_else(|| "Zip".to_string()),
            environment: env_vars,
            tags: req.tags.unwrap_or_default(),
            code: code_bytes,
            architectures: req
                .architectures
                .unwrap_or_else(|| vec!["x86_64".to_string()]),
            versions: Vec::new(),
            aliases: HashMap::new(),
            policy_statements: Vec::new(),
        };

        let config = Self::function_to_config(&func);
        inner.functions.insert(req.function_name, func);
        Ok(config)
    }

    pub async fn get_function(
        &self,
        function_name: &str,
    ) -> Result<GetFunctionResponse, LambdaError> {
        let inner = self.inner.lock().await;
        let err = not_found_err(&inner.account_id, &inner.region, function_name);
        let func = inner.functions.get(function_name).ok_or(err)?;

        let config = Self::function_to_config(func);
        let tags = if func.tags.is_empty() {
            None
        } else {
            Some(func.tags.clone())
        };

        Ok(GetFunctionResponse {
            configuration: config,
            code: FunctionCodeLocation {
                location: String::new(),
                repository_type: "S3".to_string(),
            },
            tags,
        })
    }

    pub async fn list_functions(&self) -> Result<ListFunctionsResponse, LambdaError> {
        let inner = self.inner.lock().await;
        let functions: Vec<FunctionConfiguration> = inner
            .functions
            .values()
            .map(Self::function_to_config)
            .collect();
        Ok(ListFunctionsResponse { functions })
    }

    pub async fn delete_function(&self, function_name: &str) -> Result<(), LambdaError> {
        let mut inner = self.inner.lock().await;
        let err = not_found_err(&inner.account_id, &inner.region, function_name);
        inner.functions.remove(function_name).ok_or(err)?;
        Ok(())
    }

    pub async fn update_function_code(
        &self,
        function_name: &str,
        req: UpdateFunctionCodeRequest,
    ) -> Result<FunctionConfiguration, LambdaError> {
        let mut inner = self.inner.lock().await;
        let err = not_found_err(&inner.account_id, &inner.region, function_name);
        let func = inner.functions.get_mut(function_name).ok_or(err)?;

        let code_bytes = if let Some(ref zip_file) = req.zip_file {
            base64::engine::general_purpose::STANDARD
                .decode(zip_file)
                .unwrap_or_default()
        } else {
            Vec::new()
        };

        func.code_sha256 = Self::compute_sha256(&code_bytes);
        func.code_size = code_bytes.len() as i64;
        func.code = code_bytes;
        func.last_modified = Self::now_iso();

        Ok(Self::function_to_config(func))
    }

    pub async fn update_function_configuration(
        &self,
        function_name: &str,
        req: UpdateFunctionConfigurationRequest,
    ) -> Result<FunctionConfiguration, LambdaError> {
        let mut inner = self.inner.lock().await;
        let err = not_found_err(&inner.account_id, &inner.region, function_name);
        let func = inner.functions.get_mut(function_name).ok_or(err)?;

        if let Some(runtime) = req.runtime {
            func.runtime = Some(runtime);
        }
        if let Some(role) = req.role {
            func.role = role;
        }
        if let Some(handler) = req.handler {
            func.handler = Some(handler);
        }
        if let Some(description) = req.description {
            func.description = description;
        }
        if let Some(timeout) = req.timeout {
            func.timeout = timeout;
        }
        if let Some(memory_size) = req.memory_size {
            func.memory_size = memory_size;
        }
        if let Some(environment) = req.environment {
            func.environment = environment.variables.unwrap_or_default();
        }
        func.last_modified = Self::now_iso();

        Ok(Self::function_to_config(func))
    }

    pub async fn invoke(
        &self,
        function_name: &str,
        invocation_type: Option<&str>,
    ) -> Result<(axum::http::StatusCode, String), LambdaError> {
        let inner = self.inner.lock().await;
        if !inner.functions.contains_key(function_name) {
            return Err(not_found_err(
                &inner.account_id,
                &inner.region,
                function_name,
            ));
        }

        match invocation_type {
            Some("Event") => Ok((axum::http::StatusCode::ACCEPTED, String::new())),
            Some("DryRun") => Ok((axum::http::StatusCode::NO_CONTENT, String::new())),
            _ => Ok((axum::http::StatusCode::OK, "null".to_string())),
        }
    }

    pub async fn add_permission(
        &self,
        function_name: &str,
        req: AddPermissionRequest,
    ) -> Result<AddPermissionResponse, LambdaError> {
        let mut inner = self.inner.lock().await;
        let err = not_found_err(&inner.account_id, &inner.region, function_name);
        let func = inner.functions.get_mut(function_name).ok_or(err)?;

        let statement = PolicyStatement {
            sid: req.statement_id.clone(),
            effect: "Allow".to_string(),
            principal: serde_json::json!({ "Service": req.principal }),
            action: req.action.clone(),
            resource: func.function_arn.clone(),
        };

        func.policy_statements.push(statement.clone());

        let statement_json = serde_json::json!({
            "Sid": statement.sid,
            "Effect": statement.effect,
            "Principal": statement.principal,
            "Action": statement.action,
            "Resource": statement.resource,
        });

        Ok(AddPermissionResponse {
            statement: serde_json::to_string(&statement_json).unwrap(),
        })
    }

    pub async fn remove_permission(
        &self,
        function_name: &str,
        statement_id: &str,
    ) -> Result<(), LambdaError> {
        let mut inner = self.inner.lock().await;
        let err = not_found_err(&inner.account_id, &inner.region, function_name);
        let func = inner.functions.get_mut(function_name).ok_or(err)?;

        let len_before = func.policy_statements.len();
        func.policy_statements.retain(|s| s.sid != statement_id);
        if func.policy_statements.len() == len_before {
            return Err(LambdaError::ResourceNotFoundException(format!(
                "Statement not found: {}",
                statement_id
            )));
        }
        Ok(())
    }

    pub async fn get_policy(
        &self,
        function_name: &str,
    ) -> Result<GetPolicyResponse, LambdaError> {
        let inner = self.inner.lock().await;
        let err = not_found_err(&inner.account_id, &inner.region, function_name);
        let func = inner.functions.get(function_name).ok_or(err)?;

        if func.policy_statements.is_empty() {
            return Err(LambdaError::ResourceNotFoundException(
                "No policy is associated with the given resource.".to_string(),
            ));
        }

        let statements: Vec<serde_json::Value> = func
            .policy_statements
            .iter()
            .map(|s| {
                serde_json::json!({
                    "Sid": s.sid,
                    "Effect": s.effect,
                    "Principal": s.principal,
                    "Action": s.action,
                    "Resource": s.resource,
                })
            })
            .collect();

        let policy = serde_json::json!({
            "Version": "2012-10-17",
            "Id": "default",
            "Statement": statements,
        });

        Ok(GetPolicyResponse {
            policy: serde_json::to_string(&policy).unwrap(),
            revision_id: Uuid::new_v4().to_string(),
        })
    }

    pub async fn publish_version(
        &self,
        function_name: &str,
        req: PublishVersionRequest,
    ) -> Result<FunctionConfiguration, LambdaError> {
        let mut inner = self.inner.lock().await;
        let err = not_found_err(&inner.account_id, &inner.region, function_name);
        let func = inner.functions.get_mut(function_name).ok_or(err)?;

        let version_number = (func.versions.len() + 1).to_string();
        let now = Self::now_iso();

        let pv = PublishedVersion {
            version: version_number,
            code_sha256: func.code_sha256.clone(),
            code_size: func.code_size,
            description: req.description.unwrap_or_else(|| func.description.clone()),
            runtime: func.runtime.clone(),
            role: func.role.clone(),
            handler: func.handler.clone(),
            timeout: func.timeout,
            memory_size: func.memory_size,
            last_modified: now,
            environment: func.environment.clone(),
            architectures: func.architectures.clone(),
        };

        let config = Self::published_version_to_config(&pv, func);
        func.versions.push(pv);
        Ok(config)
    }

    pub async fn list_versions_by_function(
        &self,
        function_name: &str,
    ) -> Result<ListVersionsResponse, LambdaError> {
        let inner = self.inner.lock().await;
        let err = not_found_err(&inner.account_id, &inner.region, function_name);
        let func = inner.functions.get(function_name).ok_or(err)?;

        let mut versions = vec![Self::function_to_config(func)];
        for pv in &func.versions {
            versions.push(Self::published_version_to_config(pv, func));
        }

        Ok(ListVersionsResponse { versions })
    }

    pub async fn create_alias(
        &self,
        function_name: &str,
        req: CreateAliasRequest,
    ) -> Result<AliasResponse, LambdaError> {
        let mut inner = self.inner.lock().await;
        let err = not_found_err(&inner.account_id, &inner.region, function_name);
        let func = inner.functions.get_mut(function_name).ok_or(err)?;

        if func.aliases.contains_key(&req.name) {
            return Err(LambdaError::ResourceConflictException(format!(
                "Alias already exists: {}",
                req.name
            )));
        }

        let routing_config = req.routing_config.and_then(|rc| {
            rc.additional_version_weights.map(|w| AliasRoutingConfig {
                additional_version_weights: w,
            })
        });

        let alias = Alias {
            name: req.name.clone(),
            function_version: req.function_version.clone(),
            description: req.description.unwrap_or_default(),
            routing_config,
        };

        let response = AliasResponse {
            alias_arn: format!("{}:{}", func.function_arn, alias.name),
            name: alias.name.clone(),
            function_version: alias.function_version.clone(),
            description: alias.description.clone(),
            routing_config: alias.routing_config.as_ref().map(|rc| {
                AliasRoutingConfigResponse {
                    additional_version_weights: rc.additional_version_weights.clone(),
                }
            }),
        };

        func.aliases.insert(req.name, alias);
        Ok(response)
    }

    pub async fn delete_alias(
        &self,
        function_name: &str,
        alias_name: &str,
    ) -> Result<(), LambdaError> {
        let mut inner = self.inner.lock().await;
        let err = not_found_err(&inner.account_id, &inner.region, function_name);
        let func = inner.functions.get_mut(function_name).ok_or(err)?;

        func.aliases.remove(alias_name).ok_or_else(|| {
            LambdaError::ResourceNotFoundException(format!("Alias not found: {}", alias_name))
        })?;
        Ok(())
    }

    pub async fn get_alias(
        &self,
        function_name: &str,
        alias_name: &str,
    ) -> Result<AliasResponse, LambdaError> {
        let inner = self.inner.lock().await;
        let err = not_found_err(&inner.account_id, &inner.region, function_name);
        let func = inner.functions.get(function_name).ok_or(err)?;

        let alias = func.aliases.get(alias_name).ok_or_else(|| {
            LambdaError::ResourceNotFoundException(format!("Alias not found: {}", alias_name))
        })?;

        Ok(AliasResponse {
            alias_arn: format!("{}:{}", func.function_arn, alias.name),
            name: alias.name.clone(),
            function_version: alias.function_version.clone(),
            description: alias.description.clone(),
            routing_config: alias.routing_config.as_ref().map(|rc| {
                AliasRoutingConfigResponse {
                    additional_version_weights: rc.additional_version_weights.clone(),
                }
            }),
        })
    }

    pub async fn list_aliases(
        &self,
        function_name: &str,
    ) -> Result<ListAliasesResponse, LambdaError> {
        let inner = self.inner.lock().await;
        let err = not_found_err(&inner.account_id, &inner.region, function_name);
        let func = inner.functions.get(function_name).ok_or(err)?;

        let aliases: Vec<AliasResponse> = func
            .aliases
            .values()
            .map(|alias| AliasResponse {
                alias_arn: format!("{}:{}", func.function_arn, alias.name),
                name: alias.name.clone(),
                function_version: alias.function_version.clone(),
                description: alias.description.clone(),
                routing_config: alias.routing_config.as_ref().map(|rc| {
                    AliasRoutingConfigResponse {
                        additional_version_weights: rc.additional_version_weights.clone(),
                    }
                }),
            })
            .collect();

        Ok(ListAliasesResponse { aliases })
    }

    pub async fn create_event_source_mapping(
        &self,
        req: CreateEventSourceMappingRequest,
    ) -> Result<EventSourceMappingResponse, LambdaError> {
        let mut inner = self.inner.lock().await;
        let err = not_found_err(&inner.account_id, &inner.region, &req.function_name);
        let func = inner.functions.get(&req.function_name).ok_or(err)?;

        let uuid = Uuid::new_v4().to_string();
        let now = Self::now_iso();
        let function_arn = func.function_arn.clone();

        let mapping = EventSourceMapping {
            uuid: uuid.clone(),
            event_source_arn: req.event_source_arn.clone(),
            function_arn: function_arn.clone(),
            state: if req.enabled.unwrap_or(true) {
                "Enabled".to_string()
            } else {
                "Disabled".to_string()
            },
            batch_size: req.batch_size.unwrap_or(10),
            last_modified: now,
        };

        let response = EventSourceMappingResponse {
            uuid: mapping.uuid.clone(),
            event_source_arn: mapping.event_source_arn.clone(),
            function_arn: mapping.function_arn.clone(),
            state: mapping.state.clone(),
            batch_size: mapping.batch_size,
            last_modified: mapping.last_modified.clone(),
        };

        inner.event_source_mappings.insert(uuid, mapping);
        Ok(response)
    }

    pub async fn delete_event_source_mapping(
        &self,
        uuid: &str,
    ) -> Result<EventSourceMappingResponse, LambdaError> {
        let mut inner = self.inner.lock().await;
        let mapping =
            inner
                .event_source_mappings
                .remove(uuid)
                .ok_or_else(|| {
                    LambdaError::ResourceNotFoundException(format!(
                        "Event source mapping not found: {}",
                        uuid
                    ))
                })?;

        Ok(EventSourceMappingResponse {
            uuid: mapping.uuid,
            event_source_arn: mapping.event_source_arn,
            function_arn: mapping.function_arn,
            state: "Deleting".to_string(),
            batch_size: mapping.batch_size,
            last_modified: mapping.last_modified,
        })
    }

    pub async fn list_event_source_mappings(
        &self,
    ) -> Result<ListEventSourceMappingsResponse, LambdaError> {
        let inner = self.inner.lock().await;
        let mappings: Vec<EventSourceMappingResponse> = inner
            .event_source_mappings
            .values()
            .map(|m| EventSourceMappingResponse {
                uuid: m.uuid.clone(),
                event_source_arn: m.event_source_arn.clone(),
                function_arn: m.function_arn.clone(),
                state: m.state.clone(),
                batch_size: m.batch_size,
                last_modified: m.last_modified.clone(),
            })
            .collect();

        Ok(ListEventSourceMappingsResponse {
            event_source_mappings: mappings,
        })
    }

    pub async fn tag_resource(
        &self,
        arn: &str,
        tags: HashMap<String, String>,
    ) -> Result<(), LambdaError> {
        let mut inner = self.inner.lock().await;
        let func = inner
            .functions
            .values_mut()
            .find(|f| f.function_arn == arn)
            .ok_or_else(|| {
                LambdaError::ResourceNotFoundException(format!("Function not found: {}", arn))
            })?;

        func.tags.extend(tags);
        Ok(())
    }

    pub async fn untag_resource(
        &self,
        arn: &str,
        tag_keys: &[String],
    ) -> Result<(), LambdaError> {
        let mut inner = self.inner.lock().await;
        let func = inner
            .functions
            .values_mut()
            .find(|f| f.function_arn == arn)
            .ok_or_else(|| {
                LambdaError::ResourceNotFoundException(format!("Function not found: {}", arn))
            })?;

        for key in tag_keys {
            func.tags.remove(key);
        }
        Ok(())
    }

    pub async fn list_tags(&self, arn: &str) -> Result<ListTagsResponse, LambdaError> {
        let inner = self.inner.lock().await;
        let func = inner
            .functions
            .values()
            .find(|f| f.function_arn == arn)
            .ok_or_else(|| {
                LambdaError::ResourceNotFoundException(format!("Function not found: {}", arn))
            })?;

        Ok(ListTagsResponse {
            tags: func.tags.clone(),
        })
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = LambdaState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_list_functions() {
        let state = LambdaState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.list_functions().await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_function_not_found() {
        let state = LambdaState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.delete_function("nonexistent").await;
        assert!(result.is_err());
    }

    fn make_state() -> LambdaState {
        LambdaState::new("123456789012".to_string(), "us-east-1".to_string())
    }

    fn make_create_fn_req(name: &str) -> CreateFunctionRequest {
        CreateFunctionRequest {
            function_name: name.to_string(),
            role: "arn:aws:iam::123456789012:role/lambda-role".to_string(),
            code: FunctionCode { zip_file: Some("base64data".to_string()) },
            runtime: Some("python3.9".to_string()),
            handler: Some("index.handler".to_string()),
            ..Default::default()
        }
    }

    #[tokio::test]
    async fn test_create_function() {
        let state = make_state();
        let result = state.create_function(make_create_fn_req("my-fn")).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().function_name, "my-fn");
    }

    #[tokio::test]
    async fn test_create_function_duplicate() {
        let state = make_state();
        state.create_function(make_create_fn_req("dup")).await.unwrap();
        assert!(state.create_function(make_create_fn_req("dup")).await.is_err());
    }

    #[tokio::test]
    async fn test_get_function() {
        let state = make_state();
        state.create_function(make_create_fn_req("fn1")).await.unwrap();
        let result = state.get_function("fn1").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_function_not_found() {
        let state = make_state();
        assert!(state.get_function("nope").await.is_err());
    }

    #[tokio::test]
    async fn test_delete_function_success() {
        let state = make_state();
        state.create_function(make_create_fn_req("fn1")).await.unwrap();
        assert!(state.delete_function("fn1").await.is_ok());
    }

    #[tokio::test]
    async fn test_list_functions_with_items() {
        let state = make_state();
        state.create_function(make_create_fn_req("fn1")).await.unwrap();
        state.create_function(make_create_fn_req("fn2")).await.unwrap();
        let result = state.list_functions().await.unwrap();
        assert_eq!(result.functions.len(), 2);
    }

    #[tokio::test]
    async fn test_update_function_code() {
        let state = make_state();
        state.create_function(make_create_fn_req("fn1")).await.unwrap();
        let result = state.update_function_code("fn1", UpdateFunctionCodeRequest::default()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_function_configuration() {
        let state = make_state();
        state.create_function(make_create_fn_req("fn1")).await.unwrap();
        let req = UpdateFunctionConfigurationRequest {
            description: Some("updated desc".to_string()),
            ..Default::default()
        };
        let result = state.update_function_configuration("fn1", req).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_invoke() {
        let state = make_state();
        state.create_function(make_create_fn_req("fn1")).await.unwrap();
        let result = state.invoke("fn1", None).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_publish_version() {
        let state = make_state();
        state.create_function(make_create_fn_req("fn1")).await.unwrap();
        let result = state.publish_version("fn1", PublishVersionRequest::default()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_and_list_aliases() {
        let state = make_state();
        state.create_function(make_create_fn_req("fn1")).await.unwrap();
        let version = state.publish_version("fn1", PublishVersionRequest::default()).await.unwrap();
        let req = CreateAliasRequest {
            name: "prod".to_string(),
            function_version: version.version,
            ..Default::default()
        };
        assert!(state.create_alias("fn1", req).await.is_ok());
        let aliases = state.list_aliases("fn1").await.unwrap();
        assert_eq!(aliases.aliases.len(), 1);
    }

    #[tokio::test]
    async fn test_tag_and_list_tags() {
        let state = make_state();
        let config = state.create_function(make_create_fn_req("fn1")).await.unwrap();
        let arn = config.function_arn;
        let mut tags = std::collections::HashMap::new();
        tags.insert("env".to_string(), "test".to_string());
        state.tag_resource(&arn, tags).await.unwrap();
        let result = state.list_tags(&arn).await.unwrap();
        assert!(result.tags.contains_key("env"));
    }

    #[tokio::test]
    async fn test_add_and_get_policy() {
        let state = make_state();
        state.create_function(make_create_fn_req("fn1")).await.unwrap();
        let req = AddPermissionRequest {
            statement_id: "stmt1".to_string(),
            action: "lambda:InvokeFunction".to_string(),
            principal: "s3.amazonaws.com".to_string(),
            ..Default::default()
        };
        assert!(state.add_permission("fn1", req).await.is_ok());
        let policy = state.get_policy("fn1").await;
        assert!(policy.is_ok());
    }
}
