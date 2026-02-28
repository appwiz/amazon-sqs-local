use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::SsmError;
use super::types::*;

struct SsmParameter {
    name: String,
    value: String,
    param_type: String,
    description: Option<String>,
    version: i64,
    arn: String,
    last_modified_date: f64,
    tags: HashMap<String, String>,
    tier: String,
    data_type: String,
}

struct SsmStateInner {
    parameters: HashMap<String, SsmParameter>,
    account_id: String,
    region: String,
}

pub struct SsmState {
    inner: Arc<Mutex<SsmStateInner>>,
}

impl SsmState {
    pub fn new(account_id: String, region: String) -> Self {
        SsmState {
            inner: Arc::new(Mutex::new(SsmStateInner {
                parameters: HashMap::new(),
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

    fn make_arn(account_id: &str, region: &str, name: &str) -> String {
        format!("arn:aws:ssm:{}:{}:parameter{}", region, account_id,
            if name.starts_with('/') { name.to_string() } else { format!("/{}", name) })
    }

    pub async fn put_parameter(&self, req: PutParameterRequest) -> Result<PutParameterResponse, SsmError> {
        let mut state = self.inner.lock().await;
        let overwrite = req.overwrite.unwrap_or(false);
        if state.parameters.contains_key(&req.name) && !overwrite {
            return Err(SsmError::ParameterAlreadyExists(format!(
                "Parameter {} already exists", req.name
            )));
        }
        let arn = Self::make_arn(&state.account_id, &state.region, &req.name);
        let version = state.parameters.get(&req.name).map(|p| p.version + 1).unwrap_or(1);
        let now = Self::now();
        let mut tags = state.parameters.get(&req.name)
            .map(|p| p.tags.clone())
            .unwrap_or_default();
        if let Some(t) = req.tags {
            for tag in t { tags.insert(tag.key, tag.value); }
        }
        let tier = req.tier.unwrap_or_else(|| "Standard".to_string());
        let data_type = req.data_type.unwrap_or_else(|| "text".to_string());
        state.parameters.insert(req.name.clone(), SsmParameter {
            name: req.name,
            value: req.value,
            param_type: req.param_type.unwrap_or_else(|| "String".to_string()),
            description: req.description,
            version,
            arn,
            last_modified_date: now,
            tags,
            tier: tier.clone(),
            data_type,
        });
        Ok(PutParameterResponse { version, tier })
    }

    pub async fn get_parameter(&self, req: GetParameterRequest) -> Result<GetParameterResponse, SsmError> {
        let state = self.inner.lock().await;
        let param = state.parameters.get(&req.name)
            .ok_or_else(|| SsmError::ParameterNotFound(format!(
                "Parameter {} not found", req.name
            )))?;
        Ok(GetParameterResponse {
            parameter: Parameter {
                name: param.name.clone(),
                param_type: param.param_type.clone(),
                value: param.value.clone(),
                version: param.version,
                arn: param.arn.clone(),
                last_modified_date: param.last_modified_date,
                data_type: param.data_type.clone(),
            },
        })
    }

    pub async fn get_parameters(&self, req: GetParametersRequest) -> Result<GetParametersResponse, SsmError> {
        let state = self.inner.lock().await;
        let mut parameters = Vec::new();
        let mut invalid = Vec::new();
        for name in &req.names {
            if let Some(param) = state.parameters.get(name) {
                parameters.push(Parameter {
                    name: param.name.clone(),
                    param_type: param.param_type.clone(),
                    value: param.value.clone(),
                    version: param.version,
                    arn: param.arn.clone(),
                    last_modified_date: param.last_modified_date,
                    data_type: param.data_type.clone(),
                });
            } else {
                invalid.push(name.clone());
            }
        }
        Ok(GetParametersResponse { parameters, invalid_parameters: invalid })
    }

    pub async fn get_parameters_by_path(
        &self,
        req: GetParametersByPathRequest,
    ) -> Result<GetParametersByPathResponse, SsmError> {
        let state = self.inner.lock().await;
        let recursive = req.recursive.unwrap_or(false);
        let path = if req.path.ends_with('/') {
            req.path.clone()
        } else {
            format!("{}/", req.path)
        };

        let mut params: Vec<Parameter> = state.parameters.values()
            .filter(|p| {
                let pname = if p.name.starts_with('/') { p.name.as_str() } else { &p.name };
                if recursive {
                    pname.starts_with(&path) || pname == req.path.trim_end_matches('/')
                } else {
                    // Non-recursive: only direct children
                    if pname.starts_with(&path) {
                        let rest = &pname[path.len()..];
                        !rest.contains('/')
                    } else {
                        false
                    }
                }
            })
            .map(|p| Parameter {
                name: p.name.clone(),
                param_type: p.param_type.clone(),
                value: p.value.clone(),
                version: p.version,
                arn: p.arn.clone(),
                last_modified_date: p.last_modified_date,
                data_type: p.data_type.clone(),
            })
            .collect();

        params.sort_by(|a, b| a.name.cmp(&b.name));
        let limit = req.max_results.unwrap_or(10);
        let has_more = params.len() > limit;
        params.truncate(limit);

        Ok(GetParametersByPathResponse {
            parameters: params,
            next_token: if has_more { Some("next".to_string()) } else { None },
        })
    }

    pub async fn delete_parameter(&self, req: DeleteParameterRequest) -> Result<(), SsmError> {
        let mut state = self.inner.lock().await;
        if state.parameters.remove(&req.name).is_none() {
            return Err(SsmError::ParameterNotFound(format!(
                "Parameter {} not found", req.name
            )));
        }
        Ok(())
    }

    pub async fn delete_parameters(&self, req: DeleteParametersRequest) -> Result<DeleteParametersResponse, SsmError> {
        let mut state = self.inner.lock().await;
        let mut deleted = Vec::new();
        let mut invalid = Vec::new();
        for name in &req.names {
            if state.parameters.remove(name).is_some() {
                deleted.push(name.clone());
            } else {
                invalid.push(name.clone());
            }
        }
        Ok(DeleteParametersResponse { deleted_parameters: deleted, invalid_parameters: invalid })
    }

    pub async fn describe_parameters(&self, req: DescribeParametersRequest) -> Result<DescribeParametersResponse, SsmError> {
        let state = self.inner.lock().await;
        let mut params: Vec<ParameterMetadata> = state.parameters.values()
            .map(|p| ParameterMetadata {
                name: p.name.clone(),
                param_type: p.param_type.clone(),
                version: p.version,
                last_modified_date: p.last_modified_date,
                arn: p.arn.clone(),
                description: p.description.clone(),
                tier: p.tier.clone(),
                data_type: p.data_type.clone(),
            })
            .collect();
        params.sort_by(|a, b| a.name.cmp(&b.name));
        let limit = req.max_results.unwrap_or(50);
        let has_more = params.len() > limit;
        params.truncate(limit);
        Ok(DescribeParametersResponse {
            parameters: params,
            next_token: if has_more { Some("next".to_string()) } else { None },
        })
    }

    pub async fn add_tags_to_resource(&self, req: AddTagsToResourceRequest) -> Result<(), SsmError> {
        let mut state = self.inner.lock().await;
        // For Parameter resource type, ResourceId is the parameter name
        let param = state.parameters.get_mut(&req.resource_id)
            .ok_or_else(|| SsmError::ParameterNotFound(format!(
                "Parameter {} not found", req.resource_id
            )))?;
        for tag in req.tags {
            param.tags.insert(tag.key, tag.value);
        }
        Ok(())
    }

    pub async fn remove_tags_from_resource(&self, req: RemoveTagsFromResourceRequest) -> Result<(), SsmError> {
        let mut state = self.inner.lock().await;
        let param = state.parameters.get_mut(&req.resource_id)
            .ok_or_else(|| SsmError::ParameterNotFound(format!(
                "Parameter {} not found", req.resource_id
            )))?;
        for key in &req.tag_keys {
            param.tags.remove(key);
        }
        Ok(())
    }

    pub async fn list_tags_for_resource(&self, req: ListTagsForResourceRequest) -> Result<ListTagsForResourceResponse, SsmError> {
        let state = self.inner.lock().await;
        let param = state.parameters.get(&req.resource_id)
            .ok_or_else(|| SsmError::ParameterNotFound(format!(
                "Parameter {} not found", req.resource_id
            )))?;
        let mut tag_list: Vec<Tag> = param.tags.iter().map(|(k, v)| Tag {
            key: k.clone(),
            value: v.clone(),
        }).collect();
        tag_list.sort_by(|a, b| a.key.cmp(&b.key));
        Ok(ListTagsForResourceResponse { tag_list })
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = SsmState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_put_parameter() {
        let state = SsmState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = PutParameterRequest::default();
        let _ = state.put_parameter(req).await;
    }
    #[tokio::test]
    async fn test_get_parameter_not_found() {
        let state = SsmState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = GetParameterRequest::default();
        let result = state.get_parameter(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_get_parameters_not_found() {
        let state = SsmState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = GetParametersRequest::default();
        let result = state.get_parameters(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_parameter_not_found() {
        let state = SsmState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DeleteParameterRequest::default();
        let result = state.delete_parameter(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_delete_parameters_not_found() {
        let state = SsmState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DeleteParametersRequest::default();
        let result = state.delete_parameters(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_describe_parameters_not_found() {
        let state = SsmState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DescribeParametersRequest::default();
        let result = state.describe_parameters(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_add_tags_to_resource() {
        let state = SsmState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = AddTagsToResourceRequest::default();
        let _ = state.add_tags_to_resource(req).await;
    }
    #[tokio::test]
    async fn test_remove_tags_from_resource() {
        let state = SsmState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = RemoveTagsFromResourceRequest::default();
        let _ = state.remove_tags_from_resource(req).await;
    }
    #[tokio::test]
    async fn test_list_tags_for_resource_empty() {
        let state = SsmState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = ListTagsForResourceRequest::default();
        let result = state.list_tags_for_resource(req).await;
        assert!(result.is_err());
    }

    fn make_state() -> SsmState {
        SsmState::new("123456789012".to_string(), "us-east-1".to_string())
    }

    async fn put_param(state: &SsmState, name: &str, value: &str) {
        state.put_parameter(PutParameterRequest {
            name: name.to_string(),
            value: value.to_string(),
            ..Default::default()
        }).await.unwrap();
    }

    // --- Extended coverage: put_parameter ---

    #[tokio::test]
    async fn test_put_parameter_with_all_fields() {
        let state = make_state();
        let result = state.put_parameter(PutParameterRequest {
            name: "/app/key".to_string(),
            value: "secret".to_string(),
            param_type: Some("SecureString".to_string()),
            description: Some("my param".to_string()),
            tier: Some("Advanced".to_string()),
            data_type: Some("text".to_string()),
            overwrite: None,
            tags: Some(vec![Tag { key: "env".to_string(), value: "prod".to_string() }]),
        }).await.unwrap();
        assert_eq!(result.version, 1);
        assert_eq!(result.tier, "Advanced");
    }

    #[tokio::test]
    async fn test_put_parameter_duplicate_no_overwrite() {
        let state = make_state();
        put_param(&state, "my-param", "v1").await;
        let result = state.put_parameter(PutParameterRequest {
            name: "my-param".to_string(),
            value: "v2".to_string(),
            overwrite: Some(false),
            ..Default::default()
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_put_parameter_overwrite() {
        let state = make_state();
        put_param(&state, "my-param", "v1").await;
        let result = state.put_parameter(PutParameterRequest {
            name: "my-param".to_string(),
            value: "v2".to_string(),
            overwrite: Some(true),
            ..Default::default()
        }).await.unwrap();
        assert_eq!(result.version, 2);
    }

    #[tokio::test]
    async fn test_put_parameter_overwrite_preserves_tags() {
        let state = make_state();
        state.put_parameter(PutParameterRequest {
            name: "my-param".to_string(),
            value: "v1".to_string(),
            tags: Some(vec![Tag { key: "env".to_string(), value: "prod".to_string() }]),
            ..Default::default()
        }).await.unwrap();
        state.put_parameter(PutParameterRequest {
            name: "my-param".to_string(),
            value: "v2".to_string(),
            overwrite: Some(true),
            ..Default::default()
        }).await.unwrap();
        let tags = state.list_tags_for_resource(ListTagsForResourceRequest {
            resource_id: "my-param".to_string(),
        }).await.unwrap();
        assert_eq!(tags.tag_list.len(), 1);
    }

    // --- Extended coverage: get_parameter ---

    #[tokio::test]
    async fn test_get_parameter_success() {
        let state = make_state();
        put_param(&state, "/app/db-host", "localhost").await;
        let result = state.get_parameter(GetParameterRequest {
            name: "/app/db-host".to_string(),
        }).await.unwrap();
        assert_eq!(result.parameter.name, "/app/db-host");
        assert_eq!(result.parameter.value, "localhost");
        assert_eq!(result.parameter.version, 1);
        assert!(result.parameter.arn.contains("/app/db-host"));
    }

    // --- Extended coverage: get_parameters ---

    #[tokio::test]
    async fn test_get_parameters_mixed() {
        let state = make_state();
        put_param(&state, "p1", "v1").await;
        put_param(&state, "p2", "v2").await;
        let result = state.get_parameters(GetParametersRequest {
            names: vec!["p1".to_string(), "p2".to_string(), "missing".to_string()],
        }).await.unwrap();
        assert_eq!(result.parameters.len(), 2);
        assert_eq!(result.invalid_parameters, vec!["missing".to_string()]);
    }

    // --- Extended coverage: get_parameters_by_path ---

    #[tokio::test]
    async fn test_get_parameters_by_path_non_recursive() {
        let state = make_state();
        put_param(&state, "/app/db-host", "localhost").await;
        put_param(&state, "/app/db-port", "5432").await;
        put_param(&state, "/app/nested/key", "val").await;
        let result = state.get_parameters_by_path(GetParametersByPathRequest {
            path: "/app".to_string(),
            recursive: Some(false),
            max_results: None,
        }).await.unwrap();
        assert_eq!(result.parameters.len(), 2);
    }

    #[tokio::test]
    async fn test_get_parameters_by_path_recursive() {
        let state = make_state();
        put_param(&state, "/app/db-host", "localhost").await;
        put_param(&state, "/app/nested/key", "val").await;
        let result = state.get_parameters_by_path(GetParametersByPathRequest {
            path: "/app".to_string(),
            recursive: Some(true),
            max_results: None,
        }).await.unwrap();
        assert_eq!(result.parameters.len(), 2);
    }

    #[tokio::test]
    async fn test_get_parameters_by_path_with_limit() {
        let state = make_state();
        put_param(&state, "/app/a", "1").await;
        put_param(&state, "/app/b", "2").await;
        put_param(&state, "/app/c", "3").await;
        let result = state.get_parameters_by_path(GetParametersByPathRequest {
            path: "/app".to_string(),
            recursive: Some(false),
            max_results: Some(2),
        }).await.unwrap();
        assert_eq!(result.parameters.len(), 2);
        assert!(result.next_token.is_some());
    }

    #[tokio::test]
    async fn test_get_parameters_by_path_trailing_slash() {
        let state = make_state();
        put_param(&state, "/app/key", "val").await;
        let result = state.get_parameters_by_path(GetParametersByPathRequest {
            path: "/app/".to_string(),
            recursive: Some(false),
            max_results: None,
        }).await.unwrap();
        assert_eq!(result.parameters.len(), 1);
    }

    // --- Extended coverage: delete_parameter ---

    #[tokio::test]
    async fn test_delete_parameter_success() {
        let state = make_state();
        put_param(&state, "my-param", "val").await;
        assert!(state.delete_parameter(DeleteParameterRequest { name: "my-param".to_string() }).await.is_ok());
        assert!(state.get_parameter(GetParameterRequest { name: "my-param".to_string() }).await.is_err());
    }

    // --- Extended coverage: delete_parameters ---

    #[tokio::test]
    async fn test_delete_parameters_mixed() {
        let state = make_state();
        put_param(&state, "p1", "v1").await;
        put_param(&state, "p2", "v2").await;
        let result = state.delete_parameters(DeleteParametersRequest {
            names: vec!["p1".to_string(), "missing".to_string()],
        }).await.unwrap();
        assert_eq!(result.deleted_parameters, vec!["p1".to_string()]);
        assert_eq!(result.invalid_parameters, vec!["missing".to_string()]);
    }

    // --- Extended coverage: describe_parameters ---

    #[tokio::test]
    async fn test_describe_parameters_with_data() {
        let state = make_state();
        state.put_parameter(PutParameterRequest {
            name: "/app/key".to_string(),
            value: "val".to_string(),
            description: Some("my desc".to_string()),
            ..Default::default()
        }).await.unwrap();
        let result = state.describe_parameters(DescribeParametersRequest { max_results: None }).await.unwrap();
        assert_eq!(result.parameters.len(), 1);
        assert_eq!(result.parameters[0].name, "/app/key");
        assert_eq!(result.parameters[0].description.as_deref(), Some("my desc"));
    }

    #[tokio::test]
    async fn test_describe_parameters_with_limit() {
        let state = make_state();
        put_param(&state, "p1", "v1").await;
        put_param(&state, "p2", "v2").await;
        put_param(&state, "p3", "v3").await;
        let result = state.describe_parameters(DescribeParametersRequest { max_results: Some(2) }).await.unwrap();
        assert_eq!(result.parameters.len(), 2);
        assert!(result.next_token.is_some());
    }

    // --- Extended coverage: tag operations ---

    #[tokio::test]
    async fn test_add_tags_to_resource_success() {
        let state = make_state();
        put_param(&state, "my-param", "val").await;
        state.add_tags_to_resource(AddTagsToResourceRequest {
            resource_id: "my-param".to_string(),
            tags: vec![
                Tag { key: "env".to_string(), value: "prod".to_string() },
                Tag { key: "team".to_string(), value: "infra".to_string() },
            ],
        }).await.unwrap();
        let tags = state.list_tags_for_resource(ListTagsForResourceRequest {
            resource_id: "my-param".to_string(),
        }).await.unwrap();
        assert_eq!(tags.tag_list.len(), 2);
    }

    #[tokio::test]
    async fn test_add_tags_not_found() {
        let state = make_state();
        let result = state.add_tags_to_resource(AddTagsToResourceRequest {
            resource_id: "nope".to_string(),
            tags: vec![Tag { key: "k".to_string(), value: "v".to_string() }],
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_remove_tags_from_resource_success() {
        let state = make_state();
        state.put_parameter(PutParameterRequest {
            name: "my-param".to_string(),
            value: "val".to_string(),
            tags: Some(vec![
                Tag { key: "env".to_string(), value: "prod".to_string() },
                Tag { key: "team".to_string(), value: "infra".to_string() },
            ]),
            ..Default::default()
        }).await.unwrap();
        state.remove_tags_from_resource(RemoveTagsFromResourceRequest {
            resource_id: "my-param".to_string(),
            tag_keys: vec!["team".to_string()],
        }).await.unwrap();
        let tags = state.list_tags_for_resource(ListTagsForResourceRequest {
            resource_id: "my-param".to_string(),
        }).await.unwrap();
        assert_eq!(tags.tag_list.len(), 1);
        assert_eq!(tags.tag_list[0].key, "env");
    }

    #[tokio::test]
    async fn test_remove_tags_not_found() {
        let state = make_state();
        let result = state.remove_tags_from_resource(RemoveTagsFromResourceRequest {
            resource_id: "nope".to_string(),
            tag_keys: vec!["k".to_string()],
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_tags_for_resource_success() {
        let state = make_state();
        state.put_parameter(PutParameterRequest {
            name: "my-param".to_string(),
            value: "val".to_string(),
            tags: Some(vec![Tag { key: "env".to_string(), value: "prod".to_string() }]),
            ..Default::default()
        }).await.unwrap();
        let tags = state.list_tags_for_resource(ListTagsForResourceRequest {
            resource_id: "my-param".to_string(),
        }).await.unwrap();
        assert_eq!(tags.tag_list.len(), 1);
        assert_eq!(tags.tag_list[0].key, "env");
        assert_eq!(tags.tag_list[0].value, "prod");
    }
}
