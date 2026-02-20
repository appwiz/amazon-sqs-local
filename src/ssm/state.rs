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
            .unwrap()
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
