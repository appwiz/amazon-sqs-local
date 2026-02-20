use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;
use uuid::Uuid;

use super::api::{Deployment, Integration, IntegrationResponse, Method, MethodResponse, Resource, RestApi, Stage};
use super::error::ApiGatewayError;
use super::types::*;

fn now() -> f64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs_f64()
}

fn short_id() -> String {
    Uuid::new_v4()
        .to_string()
        .replace('-', "")
        .chars()
        .take(10)
        .collect()
}

struct ApiGatewayStateInner {
    apis: HashMap<String, RestApi>,
    _account_id: String,
    _region: String,
}

pub struct ApiGatewayState {
    inner: Arc<Mutex<ApiGatewayStateInner>>,
}

impl ApiGatewayState {
    pub fn new(account_id: String, region: String) -> Self {
        ApiGatewayState {
            inner: Arc::new(Mutex::new(ApiGatewayStateInner {
                apis: HashMap::new(),
                _account_id: account_id,
                _region: region,
            })),
        }
    }

    // --- REST APIs ---

    pub async fn create_rest_api(
        &self,
        req: CreateRestApiRequest,
    ) -> Result<RestApiOutput, ApiGatewayError> {
        let mut state = self.inner.lock().await;
        let id = short_id();
        let mut api = RestApi::new(id.clone(), req.name, req.description, req.version);
        api.tags = req.tags;
        let output = api_to_output(&api);
        state.apis.insert(id, api);
        Ok(output)
    }

    pub async fn delete_rest_api(&self, rest_api_id: &str) -> Result<(), ApiGatewayError> {
        let mut state = self.inner.lock().await;
        if state.apis.remove(rest_api_id).is_none() {
            return Err(ApiGatewayError::NotFoundException(format!(
                "Invalid REST API identifier specified: {}",
                rest_api_id
            )));
        }
        Ok(())
    }

    pub async fn get_rest_api(&self, rest_api_id: &str) -> Result<RestApiOutput, ApiGatewayError> {
        let state = self.inner.lock().await;
        let api = get_api(&state.apis, rest_api_id)?;
        Ok(api_to_output(api))
    }

    pub async fn get_rest_apis(&self) -> Result<RestApisOutput, ApiGatewayError> {
        let state = self.inner.lock().await;
        let mut apis: Vec<&RestApi> = state.apis.values().collect();
        apis.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(RestApisOutput {
            items: apis.into_iter().map(api_to_output).collect(),
            position: None,
        })
    }

    pub async fn update_rest_api(
        &self,
        rest_api_id: &str,
        req: UpdateRestApiRequest,
    ) -> Result<RestApiOutput, ApiGatewayError> {
        let mut state = self.inner.lock().await;
        let api = get_api_mut(&mut state.apis, rest_api_id)?;
        for op in req.patch_operations {
            match op.path.as_str() {
                "/name" => {
                    if op.op == "replace" {
                        if let Some(v) = op.value {
                            api.name = v;
                        }
                    }
                }
                "/description" => {
                    if op.op == "replace" {
                        api.description = op.value;
                    }
                }
                "/version" => {
                    if op.op == "replace" {
                        api.version = op.value;
                    }
                }
                _ => {}
            }
        }
        Ok(api_to_output(api))
    }

    // --- Resources ---

    pub async fn get_resources(&self, rest_api_id: &str) -> Result<ResourcesOutput, ApiGatewayError> {
        let state = self.inner.lock().await;
        let api = get_api(&state.apis, rest_api_id)?;
        let mut resources: Vec<&Resource> = api.resources.values().collect();
        resources.sort_by(|a, b| a.path.cmp(&b.path));
        Ok(ResourcesOutput {
            items: resources.into_iter().map(resource_to_output).collect(),
            position: None,
        })
    }

    pub async fn get_resource(
        &self,
        rest_api_id: &str,
        resource_id: &str,
    ) -> Result<ResourceOutput, ApiGatewayError> {
        let state = self.inner.lock().await;
        let api = get_api(&state.apis, rest_api_id)?;
        let resource = get_resource(&api.resources, resource_id)?;
        Ok(resource_to_output(resource))
    }

    pub async fn create_resource(
        &self,
        rest_api_id: &str,
        parent_id: &str,
        req: CreateResourceRequest,
    ) -> Result<ResourceOutput, ApiGatewayError> {
        let mut state = self.inner.lock().await;
        let api = get_api_mut(&mut state.apis, rest_api_id)?;

        let parent = get_resource(&api.resources, parent_id)?;
        let parent_path = parent.path.clone();

        let path = if parent_path == "/" {
            format!("/{}", req.path_part)
        } else {
            format!("{}/{}", parent_path, req.path_part)
        };

        api.resource_counter += 1;
        let id = format!("{:010}", api.resource_counter);
        let resource = Resource {
            id: id.clone(),
            parent_id: Some(parent_id.to_string()),
            path_part: Some(req.path_part),
            path,
            resource_methods: HashMap::new(),
        };
        let output = resource_to_output(&resource);
        api.resources.insert(id, resource);
        Ok(output)
    }

    pub async fn delete_resource(
        &self,
        rest_api_id: &str,
        resource_id: &str,
    ) -> Result<(), ApiGatewayError> {
        let mut state = self.inner.lock().await;
        let api = get_api_mut(&mut state.apis, rest_api_id)?;
        if api.resources.remove(resource_id).is_none() {
            return Err(ApiGatewayError::NotFoundException(format!(
                "Invalid Resource identifier specified: {}",
                resource_id
            )));
        }
        Ok(())
    }

    // --- Methods ---

    pub async fn put_method(
        &self,
        rest_api_id: &str,
        resource_id: &str,
        http_method: &str,
        req: PutMethodRequest,
    ) -> Result<MethodOutput, ApiGatewayError> {
        let mut state = self.inner.lock().await;
        let api = get_api_mut(&mut state.apis, rest_api_id)?;
        let resource = get_resource_mut(&mut api.resources, resource_id)?;

        let mut method = Method::new(http_method.to_uppercase(), req.authorization_type);
        method.authorizer_id = req.authorizer_id;
        method.api_key_required = req.api_key_required;
        method.request_parameters = req.request_parameters;
        method.request_models = req.request_models;

        let output = method_to_output(&method);
        resource
            .resource_methods
            .insert(http_method.to_uppercase(), method);
        Ok(output)
    }

    pub async fn get_method(
        &self,
        rest_api_id: &str,
        resource_id: &str,
        http_method: &str,
    ) -> Result<MethodOutput, ApiGatewayError> {
        let state = self.inner.lock().await;
        let api = get_api(&state.apis, rest_api_id)?;
        let resource = get_resource(&api.resources, resource_id)?;
        let method = get_method(&resource.resource_methods, http_method)?;
        Ok(method_to_output(method))
    }

    pub async fn delete_method(
        &self,
        rest_api_id: &str,
        resource_id: &str,
        http_method: &str,
    ) -> Result<(), ApiGatewayError> {
        let mut state = self.inner.lock().await;
        let api = get_api_mut(&mut state.apis, rest_api_id)?;
        let resource = get_resource_mut(&mut api.resources, resource_id)?;
        if resource.resource_methods.remove(&http_method.to_uppercase()).is_none() {
            return Err(ApiGatewayError::NotFoundException(format!(
                "Invalid Method identifier specified: {}",
                http_method
            )));
        }
        Ok(())
    }

    // --- Integration ---

    pub async fn put_integration(
        &self,
        rest_api_id: &str,
        resource_id: &str,
        http_method: &str,
        req: PutIntegrationRequest,
    ) -> Result<IntegrationOutput, ApiGatewayError> {
        let mut state = self.inner.lock().await;
        let api = get_api_mut(&mut state.apis, rest_api_id)?;
        let resource = get_resource_mut(&mut api.resources, resource_id)?;
        let method = get_method_mut(&mut resource.resource_methods, http_method)?;

        let mut integration = Integration::new(req.integration_type, req.uri, req.http_method);
        if let Some(pb) = req.passthrough_behavior {
            integration.passthrough_behavior = pb;
        }
        integration.content_handling = req.content_handling;
        integration.request_parameters = req.request_parameters;
        integration.request_templates = req.request_templates;

        let output = integration_to_output(&integration);
        method.method_integration = Some(integration);
        Ok(output)
    }

    pub async fn get_integration(
        &self,
        rest_api_id: &str,
        resource_id: &str,
        http_method: &str,
    ) -> Result<IntegrationOutput, ApiGatewayError> {
        let state = self.inner.lock().await;
        let api = get_api(&state.apis, rest_api_id)?;
        let resource = get_resource(&api.resources, resource_id)?;
        let method = get_method(&resource.resource_methods, http_method)?;
        let integration =
            method
                .method_integration
                .as_ref()
                .ok_or_else(|| ApiGatewayError::NotFoundException(
                    "Integration not found.".to_string(),
                ))?;
        Ok(integration_to_output(integration))
    }

    pub async fn delete_integration(
        &self,
        rest_api_id: &str,
        resource_id: &str,
        http_method: &str,
    ) -> Result<(), ApiGatewayError> {
        let mut state = self.inner.lock().await;
        let api = get_api_mut(&mut state.apis, rest_api_id)?;
        let resource = get_resource_mut(&mut api.resources, resource_id)?;
        let method = get_method_mut(&mut resource.resource_methods, http_method)?;
        if method.method_integration.take().is_none() {
            return Err(ApiGatewayError::NotFoundException(
                "Integration not found.".to_string(),
            ));
        }
        Ok(())
    }

    // --- MethodResponse ---

    pub async fn put_method_response(
        &self,
        rest_api_id: &str,
        resource_id: &str,
        http_method: &str,
        status_code: &str,
        req: PutMethodResponseRequest,
    ) -> Result<MethodResponseOutput, ApiGatewayError> {
        let mut state = self.inner.lock().await;
        let api = get_api_mut(&mut state.apis, rest_api_id)?;
        let resource = get_resource_mut(&mut api.resources, resource_id)?;
        let method = get_method_mut(&mut resource.resource_methods, http_method)?;

        let mr = MethodResponse {
            status_code: status_code.to_string(),
            response_parameters: req.response_parameters,
            response_models: req.response_models,
        };
        let output = MethodResponseOutput {
            status_code: mr.status_code.clone(),
            response_parameters: mr.response_parameters.clone(),
            response_models: mr.response_models.clone(),
        };
        method.method_responses.insert(status_code.to_string(), mr);
        Ok(output)
    }

    pub async fn get_method_response(
        &self,
        rest_api_id: &str,
        resource_id: &str,
        http_method: &str,
        status_code: &str,
    ) -> Result<MethodResponseOutput, ApiGatewayError> {
        let state = self.inner.lock().await;
        let api = get_api(&state.apis, rest_api_id)?;
        let resource = get_resource(&api.resources, resource_id)?;
        let method = get_method(&resource.resource_methods, http_method)?;
        let mr = method.method_responses.get(status_code).ok_or_else(|| {
            ApiGatewayError::NotFoundException(format!(
                "Invalid Response status code specified: {}",
                status_code
            ))
        })?;
        Ok(MethodResponseOutput {
            status_code: mr.status_code.clone(),
            response_parameters: mr.response_parameters.clone(),
            response_models: mr.response_models.clone(),
        })
    }

    pub async fn delete_method_response(
        &self,
        rest_api_id: &str,
        resource_id: &str,
        http_method: &str,
        status_code: &str,
    ) -> Result<(), ApiGatewayError> {
        let mut state = self.inner.lock().await;
        let api = get_api_mut(&mut state.apis, rest_api_id)?;
        let resource = get_resource_mut(&mut api.resources, resource_id)?;
        let method = get_method_mut(&mut resource.resource_methods, http_method)?;
        if method.method_responses.remove(status_code).is_none() {
            return Err(ApiGatewayError::NotFoundException(format!(
                "Invalid Response status code specified: {}",
                status_code
            )));
        }
        Ok(())
    }

    // --- IntegrationResponse ---

    pub async fn put_integration_response(
        &self,
        rest_api_id: &str,
        resource_id: &str,
        http_method: &str,
        status_code: &str,
        req: PutIntegrationResponseRequest,
    ) -> Result<IntegrationResponseOutput, ApiGatewayError> {
        let mut state = self.inner.lock().await;
        let api = get_api_mut(&mut state.apis, rest_api_id)?;
        let resource = get_resource_mut(&mut api.resources, resource_id)?;
        let method = get_method_mut(&mut resource.resource_methods, http_method)?;
        let integration = method.method_integration.as_mut().ok_or_else(|| {
            ApiGatewayError::NotFoundException("Integration not found.".to_string())
        })?;

        let ir = IntegrationResponse {
            status_code: status_code.to_string(),
            response_parameters: req.response_parameters,
            response_templates: req.response_templates,
        };
        let output = IntegrationResponseOutput {
            status_code: ir.status_code.clone(),
            response_parameters: ir.response_parameters.clone(),
            response_templates: ir.response_templates.clone(),
        };
        integration
            .integration_responses
            .insert(status_code.to_string(), ir);
        Ok(output)
    }

    // --- Deployments ---

    pub async fn create_deployment(
        &self,
        rest_api_id: &str,
        req: CreateDeploymentRequest,
    ) -> Result<DeploymentOutput, ApiGatewayError> {
        let mut state = self.inner.lock().await;
        let api = get_api_mut(&mut state.apis, rest_api_id)?;

        let id = short_id();
        let deployment = Deployment::new(id.clone(), req.description, req.stage_name.clone());

        // If stage_name provided, create/update the stage
        if let Some(ref stage_name) = req.stage_name {
            let stage = api
                .stages
                .entry(stage_name.clone())
                .or_insert_with(|| Stage::new(stage_name.clone(), None, req.stage_description));
            stage.deployment_id = Some(id.clone());
            stage.last_updated_date = now();
            if !req.variables.is_empty() {
                stage.variables = req.variables;
            }
        }

        let output = DeploymentOutput {
            id: deployment.id.clone(),
            description: deployment.description.clone(),
            created_date: deployment.created_date,
        };
        api.deployments.insert(id, deployment);
        Ok(output)
    }

    pub async fn get_deployment(
        &self,
        rest_api_id: &str,
        deployment_id: &str,
    ) -> Result<DeploymentOutput, ApiGatewayError> {
        let state = self.inner.lock().await;
        let api = get_api(&state.apis, rest_api_id)?;
        let d = api.deployments.get(deployment_id).ok_or_else(|| {
            ApiGatewayError::NotFoundException(format!(
                "Invalid Deployment identifier specified: {}",
                deployment_id
            ))
        })?;
        Ok(DeploymentOutput {
            id: d.id.clone(),
            description: d.description.clone(),
            created_date: d.created_date,
        })
    }

    pub async fn get_deployments(
        &self,
        rest_api_id: &str,
    ) -> Result<DeploymentsOutput, ApiGatewayError> {
        let state = self.inner.lock().await;
        let api = get_api(&state.apis, rest_api_id)?;
        let mut deployments: Vec<&Deployment> = api.deployments.values().collect();
        deployments.sort_by(|a, b| {
            b.created_date
                .partial_cmp(&a.created_date)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        Ok(DeploymentsOutput {
            items: deployments
                .into_iter()
                .map(|d| DeploymentOutput {
                    id: d.id.clone(),
                    description: d.description.clone(),
                    created_date: d.created_date,
                })
                .collect(),
            position: None,
        })
    }

    // --- Stages ---

    pub async fn create_stage(
        &self,
        rest_api_id: &str,
        req: CreateStageRequest,
    ) -> Result<StageOutput, ApiGatewayError> {
        let mut state = self.inner.lock().await;
        let api = get_api_mut(&mut state.apis, rest_api_id)?;

        if api.stages.contains_key(&req.stage_name) {
            return Err(ApiGatewayError::ConflictException(format!(
                "Stage already exists: {}",
                req.stage_name
            )));
        }

        if !api.deployments.contains_key(&req.deployment_id) {
            return Err(ApiGatewayError::NotFoundException(format!(
                "Invalid Deployment identifier specified: {}",
                req.deployment_id
            )));
        }

        let mut stage = Stage::new(
            req.stage_name.clone(),
            Some(req.deployment_id),
            req.description,
        );
        stage.variables = req.variables;
        stage.tags = req.tags;

        let output = stage_to_output(&stage);
        api.stages.insert(req.stage_name, stage);
        Ok(output)
    }

    pub async fn get_stage(
        &self,
        rest_api_id: &str,
        stage_name: &str,
    ) -> Result<StageOutput, ApiGatewayError> {
        let state = self.inner.lock().await;
        let api = get_api(&state.apis, rest_api_id)?;
        let stage = api.stages.get(stage_name).ok_or_else(|| {
            ApiGatewayError::NotFoundException(format!(
                "Invalid Stage identifier specified: {}",
                stage_name
            ))
        })?;
        Ok(stage_to_output(stage))
    }

    pub async fn get_stages(&self, rest_api_id: &str) -> Result<StagesOutput, ApiGatewayError> {
        let state = self.inner.lock().await;
        let api = get_api(&state.apis, rest_api_id)?;
        let mut stages: Vec<&Stage> = api.stages.values().collect();
        stages.sort_by(|a, b| a.stage_name.cmp(&b.stage_name));
        Ok(StagesOutput {
            item: stages.into_iter().map(stage_to_output).collect(),
        })
    }

    pub async fn update_stage(
        &self,
        rest_api_id: &str,
        stage_name: &str,
        req: UpdateStageRequest,
    ) -> Result<StageOutput, ApiGatewayError> {
        let mut state = self.inner.lock().await;
        let api = get_api_mut(&mut state.apis, rest_api_id)?;
        let stage = api.stages.get_mut(stage_name).ok_or_else(|| {
            ApiGatewayError::NotFoundException(format!(
                "Invalid Stage identifier specified: {}",
                stage_name
            ))
        })?;

        for op in req.patch_operations {
            match (op.op.as_str(), op.path.as_str()) {
                ("replace", "/description") => {
                    stage.description = op.value;
                }
                ("replace", "/deploymentId") => {
                    stage.deployment_id = op.value;
                }
                ("replace", path) if path.starts_with("/variables/") => {
                    let key = path.trim_start_matches("/variables/").to_string();
                    if let Some(v) = op.value {
                        stage.variables.insert(key, v);
                    }
                }
                ("remove", path) if path.starts_with("/variables/") => {
                    let key = path.trim_start_matches("/variables/").to_string();
                    stage.variables.remove(&key);
                }
                _ => {}
            }
        }
        stage.last_updated_date = now();
        Ok(stage_to_output(stage))
    }

    pub async fn delete_stage(
        &self,
        rest_api_id: &str,
        stage_name: &str,
    ) -> Result<(), ApiGatewayError> {
        let mut state = self.inner.lock().await;
        let api = get_api_mut(&mut state.apis, rest_api_id)?;
        if api.stages.remove(stage_name).is_none() {
            return Err(ApiGatewayError::NotFoundException(format!(
                "Invalid Stage identifier specified: {}",
                stage_name
            )));
        }
        Ok(())
    }

    // --- Tags ---

    pub async fn tag_resource(
        &self,
        rest_api_id: &str,
        req: TagResourceRequest,
    ) -> Result<(), ApiGatewayError> {
        let mut state = self.inner.lock().await;
        let api = get_api_mut(&mut state.apis, rest_api_id)?;
        api.tags.extend(req.tags);
        Ok(())
    }

    pub async fn untag_resource(
        &self,
        rest_api_id: &str,
        tag_keys: Vec<String>,
    ) -> Result<(), ApiGatewayError> {
        let mut state = self.inner.lock().await;
        let api = get_api_mut(&mut state.apis, rest_api_id)?;
        for key in tag_keys {
            api.tags.remove(&key);
        }
        Ok(())
    }

    pub async fn get_tags(&self, rest_api_id: &str) -> Result<HashMap<String, String>, ApiGatewayError> {
        let state = self.inner.lock().await;
        let api = get_api(&state.apis, rest_api_id)?;
        Ok(api.tags.clone())
    }
}

// --- Helpers ---

fn get_api<'a>(
    apis: &'a HashMap<String, RestApi>,
    api_id: &str,
) -> Result<&'a RestApi, ApiGatewayError> {
    apis.get(api_id).ok_or_else(|| {
        ApiGatewayError::NotFoundException(format!(
            "Invalid REST API identifier specified: {}",
            api_id
        ))
    })
}

fn get_api_mut<'a>(
    apis: &'a mut HashMap<String, RestApi>,
    api_id: &str,
) -> Result<&'a mut RestApi, ApiGatewayError> {
    apis.get_mut(api_id).ok_or_else(|| {
        ApiGatewayError::NotFoundException(format!(
            "Invalid REST API identifier specified: {}",
            api_id
        ))
    })
}

fn get_resource<'a>(
    resources: &'a HashMap<String, Resource>,
    resource_id: &str,
) -> Result<&'a Resource, ApiGatewayError> {
    resources.get(resource_id).ok_or_else(|| {
        ApiGatewayError::NotFoundException(format!(
            "Invalid Resource identifier specified: {}",
            resource_id
        ))
    })
}

fn get_resource_mut<'a>(
    resources: &'a mut HashMap<String, Resource>,
    resource_id: &str,
) -> Result<&'a mut Resource, ApiGatewayError> {
    resources.get_mut(resource_id).ok_or_else(|| {
        ApiGatewayError::NotFoundException(format!(
            "Invalid Resource identifier specified: {}",
            resource_id
        ))
    })
}

fn get_method<'a>(
    methods: &'a HashMap<String, Method>,
    http_method: &str,
) -> Result<&'a Method, ApiGatewayError> {
    methods.get(&http_method.to_uppercase()).ok_or_else(|| {
        ApiGatewayError::NotFoundException(format!(
            "Invalid Method identifier specified: {}",
            http_method
        ))
    })
}

fn get_method_mut<'a>(
    methods: &'a mut HashMap<String, Method>,
    http_method: &str,
) -> Result<&'a mut Method, ApiGatewayError> {
    methods
        .get_mut(&http_method.to_uppercase())
        .ok_or_else(|| {
            ApiGatewayError::NotFoundException(format!(
                "Invalid Method identifier specified: {}",
                http_method
            ))
        })
}

// --- Conversion helpers ---

fn api_to_output(api: &RestApi) -> RestApiOutput {
    RestApiOutput {
        id: api.id.clone(),
        name: api.name.clone(),
        description: api.description.clone(),
        created_date: api.created_date,
        version: api.version.clone(),
        tags: api.tags.clone(),
    }
}

fn resource_to_output(resource: &Resource) -> ResourceOutput {
    ResourceOutput {
        id: resource.id.clone(),
        parent_id: resource.parent_id.clone(),
        path_part: resource.path_part.clone(),
        path: resource.path.clone(),
        resource_methods: resource
            .resource_methods
            .iter()
            .map(|(k, v)| (k.clone(), method_to_output(v)))
            .collect(),
    }
}

fn method_to_output(method: &Method) -> MethodOutput {
    MethodOutput {
        http_method: method.http_method.clone(),
        authorization_type: method.authorization_type.clone(),
        authorizer_id: method.authorizer_id.clone(),
        api_key_required: method.api_key_required,
        request_parameters: method.request_parameters.clone(),
        request_models: method.request_models.clone(),
        method_integration: method.method_integration.as_ref().map(integration_to_output),
        method_responses: method
            .method_responses
            .iter()
            .map(|(k, v)| {
                (
                    k.clone(),
                    MethodResponseOutput {
                        status_code: v.status_code.clone(),
                        response_parameters: v.response_parameters.clone(),
                        response_models: v.response_models.clone(),
                    },
                )
            })
            .collect(),
    }
}

fn integration_to_output(integration: &Integration) -> IntegrationOutput {
    IntegrationOutput {
        integration_type: integration.integration_type.clone(),
        uri: integration.uri.clone(),
        http_method: integration.http_method.clone(),
        passthrough_behavior: integration.passthrough_behavior.clone(),
        content_handling: integration.content_handling.clone(),
        request_parameters: integration.request_parameters.clone(),
        request_templates: integration.request_templates.clone(),
        integration_responses: integration
            .integration_responses
            .iter()
            .map(|(k, v)| {
                (
                    k.clone(),
                    IntegrationResponseOutput {
                        status_code: v.status_code.clone(),
                        response_parameters: v.response_parameters.clone(),
                        response_templates: v.response_templates.clone(),
                    },
                )
            })
            .collect(),
    }
}

fn stage_to_output(stage: &Stage) -> StageOutput {
    StageOutput {
        stage_name: stage.stage_name.clone(),
        deployment_id: stage.deployment_id.clone(),
        description: stage.description.clone(),
        created_date: stage.created_date,
        last_updated_date: stage.last_updated_date,
        variables: stage.variables.clone(),
        tags: stage.tags.clone(),
    }
}
