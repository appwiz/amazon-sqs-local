use std::collections::HashMap;

fn now() -> f64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs_f64()
}

#[derive(Debug, Clone)]
pub struct RestApi {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub created_date: f64,
    pub version: Option<String>,
    pub tags: HashMap<String, String>,
    pub resources: HashMap<String, Resource>,
    pub deployments: HashMap<String, Deployment>,
    pub stages: HashMap<String, Stage>,
    pub resource_counter: u64,
}

impl RestApi {
    pub fn new(id: String, name: String, description: Option<String>, version: Option<String>) -> Self {
        let ts = now();
        let root = Resource {
            id: "root".to_string(),
            parent_id: None,
            path_part: None,
            path: "/".to_string(),
            resource_methods: HashMap::new(),
        };
        let mut resources = HashMap::new();
        resources.insert("root".to_string(), root);
        RestApi {
            id,
            name,
            description,
            created_date: ts,
            version,
            tags: HashMap::new(),
            resources,
            deployments: HashMap::new(),
            stages: HashMap::new(),
            resource_counter: 1,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Resource {
    pub id: String,
    pub parent_id: Option<String>,
    pub path_part: Option<String>,
    pub path: String,
    pub resource_methods: HashMap<String, Method>,
}

#[derive(Debug, Clone)]
pub struct Method {
    pub http_method: String,
    pub authorization_type: String,
    pub authorizer_id: Option<String>,
    pub api_key_required: bool,
    pub request_parameters: HashMap<String, bool>,
    pub request_models: HashMap<String, String>,
    pub method_integration: Option<Integration>,
    pub method_responses: HashMap<String, MethodResponse>,
}

impl Method {
    pub fn new(http_method: String, authorization_type: String) -> Self {
        Method {
            http_method,
            authorization_type,
            authorizer_id: None,
            api_key_required: false,
            request_parameters: HashMap::new(),
            request_models: HashMap::new(),
            method_integration: None,
            method_responses: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Integration {
    pub integration_type: String,
    pub uri: Option<String>,
    pub http_method: Option<String>,
    pub passthrough_behavior: String,
    pub content_handling: Option<String>,
    pub request_parameters: HashMap<String, String>,
    pub request_templates: HashMap<String, String>,
    pub integration_responses: HashMap<String, IntegrationResponse>,
}

impl Integration {
    pub fn new(integration_type: String, uri: Option<String>, http_method: Option<String>) -> Self {
        Integration {
            integration_type,
            uri,
            http_method,
            passthrough_behavior: "WHEN_NO_MATCH".to_string(),
            content_handling: None,
            request_parameters: HashMap::new(),
            request_templates: HashMap::new(),
            integration_responses: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct IntegrationResponse {
    pub status_code: String,
    pub response_parameters: HashMap<String, String>,
    pub response_templates: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct MethodResponse {
    pub status_code: String,
    pub response_parameters: HashMap<String, bool>,
    pub response_models: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct Deployment {
    pub id: String,
    pub description: Option<String>,
    pub created_date: f64,
    pub stage_name: Option<String>,
}

impl Deployment {
    pub fn new(id: String, description: Option<String>, stage_name: Option<String>) -> Self {
        Deployment {
            id,
            description,
            created_date: now(),
            stage_name,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Stage {
    pub stage_name: String,
    pub deployment_id: Option<String>,
    pub description: Option<String>,
    pub created_date: f64,
    pub last_updated_date: f64,
    pub variables: HashMap<String, String>,
    pub tags: HashMap<String, String>,
}

impl Stage {
    pub fn new(stage_name: String, deployment_id: Option<String>, description: Option<String>) -> Self {
        let ts = now();
        Stage {
            stage_name,
            deployment_id,
            description,
            created_date: ts,
            last_updated_date: ts,
            variables: HashMap::new(),
            tags: HashMap::new(),
        }
    }
}
