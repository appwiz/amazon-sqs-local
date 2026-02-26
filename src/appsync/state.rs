use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;
use uuid::Uuid;

use super::error::AppSyncError;
use super::types::*;

struct AppSyncStateInner {
    apis: HashMap<String, GraphqlApi>,
    api_keys: HashMap<String, Vec<ApiKey>>,
    data_sources: HashMap<String, HashMap<String, DataSource>>,
    schemas: HashMap<String, SchemaInfo>,
    tags: HashMap<String, HashMap<String, String>>,
    account_id: String,
    region: String,
}

pub struct AppSyncState {
    inner: Arc<Mutex<AppSyncStateInner>>,
}

impl AppSyncState {
    pub fn new(account_id: String, region: String) -> Self {
        AppSyncState {
            inner: Arc::new(Mutex::new(AppSyncStateInner {
                apis: HashMap::new(),
                api_keys: HashMap::new(),
                data_sources: HashMap::new(),
                schemas: HashMap::new(),
                tags: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    fn generate_id() -> String {
        let hex = Uuid::new_v4().to_string().replace('-', "");
        hex[..26].to_string()
    }

    fn generate_key_id() -> String {
        format!("da2-{}", &Uuid::new_v4().to_string().replace('-', "")[..26])
    }

    fn now_epoch() -> i64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64
    }

    // --- GraphQL API operations ---

    pub async fn create_graphql_api(
        &self,
        req: CreateGraphqlApiRequest,
    ) -> Result<GraphqlApi, AppSyncError> {
        let mut state = self.inner.lock().await;

        let api_id = Self::generate_id();
        let arn = format!(
            "arn:aws:appsync:{}:{}:apis/{}",
            state.region, state.account_id, api_id
        );

        let mut uris = HashMap::new();
        uris.insert(
            "GRAPHQL".to_string(),
            format!(
                "https://{}.appsync-api.{}.amazonaws.com/graphql",
                api_id, state.region
            ),
        );
        uris.insert(
            "REALTIME".to_string(),
            format!(
                "wss://{}.appsync-realtime-api.{}.amazonaws.com/graphql",
                api_id, state.region
            ),
        );

        let tags = req.tags.clone();
        let api = GraphqlApi {
            api_id: api_id.clone(),
            name: req.name,
            authentication_type: req
                .authentication_type
                .unwrap_or_else(|| "API_KEY".to_string()),
            arn: arn.clone(),
            uris,
            tags: tags.clone(),
            xray_enabled: req.xray_enabled.unwrap_or(false),
            api_type: req.api_type,
        };

        // Store tags by ARN
        if let Some(ref t) = tags {
            state.tags.insert(arn, t.clone());
        }

        state.api_keys.insert(api_id.clone(), Vec::new());
        state.data_sources.insert(api_id.clone(), HashMap::new());
        state.apis.insert(api_id, api.clone());
        Ok(api)
    }

    pub async fn get_graphql_api(&self, api_id: &str) -> Result<GraphqlApi, AppSyncError> {
        let state = self.inner.lock().await;
        state
            .apis
            .get(api_id)
            .cloned()
            .ok_or_else(|| {
                AppSyncError::NotFoundException(format!(
                    "GraphQL API '{}' not found.",
                    api_id
                ))
            })
    }

    pub async fn list_graphql_apis(&self) -> Result<Vec<GraphqlApi>, AppSyncError> {
        let state = self.inner.lock().await;
        Ok(state.apis.values().cloned().collect())
    }

    pub async fn update_graphql_api(
        &self,
        api_id: &str,
        req: UpdateGraphqlApiRequest,
    ) -> Result<GraphqlApi, AppSyncError> {
        let mut state = self.inner.lock().await;
        let api = state.apis.get_mut(api_id).ok_or_else(|| {
            AppSyncError::NotFoundException(format!(
                "GraphQL API '{}' not found.",
                api_id
            ))
        })?;

        api.name = req.name;
        if let Some(auth) = req.authentication_type {
            api.authentication_type = auth;
        }
        if let Some(xray) = req.xray_enabled {
            api.xray_enabled = xray;
        }

        Ok(api.clone())
    }

    pub async fn delete_graphql_api(&self, api_id: &str) -> Result<(), AppSyncError> {
        let mut state = self.inner.lock().await;
        let api = state.apis.remove(api_id).ok_or_else(|| {
            AppSyncError::NotFoundException(format!(
                "GraphQL API '{}' not found.",
                api_id
            ))
        })?;

        state.api_keys.remove(api_id);
        state.data_sources.remove(api_id);
        state.schemas.remove(api_id);
        state.tags.remove(&api.arn);
        Ok(())
    }

    // --- API Key operations ---

    pub async fn create_api_key(
        &self,
        api_id: &str,
        req: CreateApiKeyRequest,
    ) -> Result<ApiKey, AppSyncError> {
        let mut state = self.inner.lock().await;
        if !state.apis.contains_key(api_id) {
            return Err(AppSyncError::NotFoundException(format!(
                "GraphQL API '{}' not found.",
                api_id
            )));
        }

        let key = ApiKey {
            id: Self::generate_key_id(),
            description: req.description,
            expires: req.expires.unwrap_or_else(|| Self::now_epoch() + 604800),
            deletes: None,
        };

        state
            .api_keys
            .entry(api_id.to_string())
            .or_default()
            .push(key.clone());
        Ok(key)
    }

    pub async fn list_api_keys(&self, api_id: &str) -> Result<Vec<ApiKey>, AppSyncError> {
        let state = self.inner.lock().await;
        if !state.apis.contains_key(api_id) {
            return Err(AppSyncError::NotFoundException(format!(
                "GraphQL API '{}' not found.",
                api_id
            )));
        }
        Ok(state
            .api_keys
            .get(api_id)
            .cloned()
            .unwrap_or_default())
    }

    pub async fn update_api_key(
        &self,
        api_id: &str,
        key_id: &str,
        req: UpdateApiKeyRequest,
    ) -> Result<ApiKey, AppSyncError> {
        let mut state = self.inner.lock().await;
        if !state.apis.contains_key(api_id) {
            return Err(AppSyncError::NotFoundException(format!(
                "GraphQL API '{}' not found.",
                api_id
            )));
        }

        let keys = state.api_keys.entry(api_id.to_string()).or_default();
        let key = keys.iter_mut().find(|k| k.id == key_id).ok_or_else(|| {
            AppSyncError::NotFoundException(format!("API key '{}' not found.", key_id))
        })?;

        if let Some(desc) = req.description {
            key.description = Some(desc);
        }
        if let Some(exp) = req.expires {
            key.expires = exp;
        }

        Ok(key.clone())
    }

    pub async fn delete_api_key(
        &self,
        api_id: &str,
        key_id: &str,
    ) -> Result<(), AppSyncError> {
        let mut state = self.inner.lock().await;
        if !state.apis.contains_key(api_id) {
            return Err(AppSyncError::NotFoundException(format!(
                "GraphQL API '{}' not found.",
                api_id
            )));
        }

        let keys = state.api_keys.entry(api_id.to_string()).or_default();
        let len_before = keys.len();
        keys.retain(|k| k.id != key_id);
        if keys.len() == len_before {
            return Err(AppSyncError::NotFoundException(format!(
                "API key '{}' not found.",
                key_id
            )));
        }
        Ok(())
    }

    // --- Data Source operations ---

    pub async fn create_data_source(
        &self,
        api_id: &str,
        req: CreateDataSourceRequest,
    ) -> Result<DataSource, AppSyncError> {
        let mut state = self.inner.lock().await;
        if !state.apis.contains_key(api_id) {
            return Err(AppSyncError::NotFoundException(format!(
                "GraphQL API '{}' not found.",
                api_id
            )));
        }

        let ds_arn = format!(
            "arn:aws:appsync:{}:{}:apis/{}/datasources/{}",
            state.region, state.account_id, api_id, req.name
        );

        let sources = state.data_sources.entry(api_id.to_string()).or_default();
        if sources.contains_key(&req.name) {
            return Err(AppSyncError::ConcurrentModificationException(format!(
                "Data source '{}' already exists.",
                req.name
            )));
        }

        let ds = DataSource {
            data_source_arn: ds_arn,
            name: req.name.clone(),
            ds_type: req.ds_type,
            description: req.description,
            service_role_arn: req.service_role_arn,
        };

        sources.insert(req.name, ds.clone());
        Ok(ds)
    }

    pub async fn get_data_source(
        &self,
        api_id: &str,
        name: &str,
    ) -> Result<DataSource, AppSyncError> {
        let state = self.inner.lock().await;
        if !state.apis.contains_key(api_id) {
            return Err(AppSyncError::NotFoundException(format!(
                "GraphQL API '{}' not found.",
                api_id
            )));
        }

        state
            .data_sources
            .get(api_id)
            .and_then(|m| m.get(name))
            .cloned()
            .ok_or_else(|| {
                AppSyncError::NotFoundException(format!(
                    "Data source '{}' not found.",
                    name
                ))
            })
    }

    pub async fn list_data_sources(
        &self,
        api_id: &str,
    ) -> Result<Vec<DataSource>, AppSyncError> {
        let state = self.inner.lock().await;
        if !state.apis.contains_key(api_id) {
            return Err(AppSyncError::NotFoundException(format!(
                "GraphQL API '{}' not found.",
                api_id
            )));
        }
        Ok(state
            .data_sources
            .get(api_id)
            .map(|m| m.values().cloned().collect())
            .unwrap_or_default())
    }

    pub async fn update_data_source(
        &self,
        api_id: &str,
        name: &str,
        req: UpdateDataSourceRequest,
    ) -> Result<DataSource, AppSyncError> {
        let mut state = self.inner.lock().await;
        if !state.apis.contains_key(api_id) {
            return Err(AppSyncError::NotFoundException(format!(
                "GraphQL API '{}' not found.",
                api_id
            )));
        }

        let sources = state.data_sources.entry(api_id.to_string()).or_default();
        let ds = sources.get_mut(name).ok_or_else(|| {
            AppSyncError::NotFoundException(format!(
                "Data source '{}' not found.",
                name
            ))
        })?;

        ds.ds_type = req.ds_type;
        ds.description = req.description;
        if req.service_role_arn.is_some() {
            ds.service_role_arn = req.service_role_arn;
        }

        Ok(ds.clone())
    }

    pub async fn delete_data_source(
        &self,
        api_id: &str,
        name: &str,
    ) -> Result<(), AppSyncError> {
        let mut state = self.inner.lock().await;
        if !state.apis.contains_key(api_id) {
            return Err(AppSyncError::NotFoundException(format!(
                "GraphQL API '{}' not found.",
                api_id
            )));
        }

        let sources = state.data_sources.entry(api_id.to_string()).or_default();
        if sources.remove(name).is_none() {
            return Err(AppSyncError::NotFoundException(format!(
                "Data source '{}' not found.",
                name
            )));
        }
        Ok(())
    }

    // --- Schema operations ---

    pub async fn start_schema_creation(
        &self,
        api_id: &str,
        _req: StartSchemaCreationRequest,
    ) -> Result<SchemaInfo, AppSyncError> {
        let mut state = self.inner.lock().await;
        if !state.apis.contains_key(api_id) {
            return Err(AppSyncError::NotFoundException(format!(
                "GraphQL API '{}' not found.",
                api_id
            )));
        }

        let info = SchemaInfo {
            status: "SUCCESS".to_string(),
            details: None,
        };
        state.schemas.insert(api_id.to_string(), info.clone());
        Ok(info)
    }

    pub async fn get_schema_creation_status(
        &self,
        api_id: &str,
    ) -> Result<SchemaInfo, AppSyncError> {
        let state = self.inner.lock().await;
        if !state.apis.contains_key(api_id) {
            return Err(AppSyncError::NotFoundException(format!(
                "GraphQL API '{}' not found.",
                api_id
            )));
        }

        Ok(state
            .schemas
            .get(api_id)
            .cloned()
            .unwrap_or(SchemaInfo {
                status: "NOT_APPLICABLE".to_string(),
                details: None,
            }))
    }

    // --- Tag operations ---

    pub async fn tag_resource(
        &self,
        resource_arn: &str,
        req: TagResourceRequest,
    ) -> Result<(), AppSyncError> {
        let mut state = self.inner.lock().await;

        // Verify the ARN belongs to a known resource
        let api_exists = state.apis.values().any(|a| a.arn == resource_arn);
        if !api_exists {
            return Err(AppSyncError::NotFoundException(format!(
                "Resource '{}' not found.",
                resource_arn
            )));
        }

        let tags = state.tags.entry(resource_arn.to_string()).or_default();
        for (k, v) in req.tags {
            tags.insert(k, v);
        }

        // Copy tags to update the API's inline tags
        let current_tags = state
            .tags
            .get(resource_arn)
            .cloned()
            .unwrap_or_default();
        for api in state.apis.values_mut() {
            if api.arn == resource_arn {
                api.tags = Some(current_tags);
                break;
            }
        }

        Ok(())
    }

    pub async fn untag_resource(
        &self,
        resource_arn: &str,
        tag_keys: Vec<String>,
    ) -> Result<(), AppSyncError> {
        let mut state = self.inner.lock().await;

        let api_exists = state.apis.values().any(|a| a.arn == resource_arn);
        if !api_exists {
            return Err(AppSyncError::NotFoundException(format!(
                "Resource '{}' not found.",
                resource_arn
            )));
        }

        if let Some(tags) = state.tags.get_mut(resource_arn) {
            for key in &tag_keys {
                tags.remove(key);
            }
        }

        // Also update the API's inline tags
        for api in state.apis.values_mut() {
            if api.arn == resource_arn {
                if let Some(ref mut api_tags) = api.tags {
                    for key in &tag_keys {
                        api_tags.remove(key);
                    }
                }
                break;
            }
        }

        Ok(())
    }

    pub async fn list_tags_for_resource(
        &self,
        resource_arn: &str,
    ) -> Result<HashMap<String, String>, AppSyncError> {
        let state = self.inner.lock().await;

        let api_exists = state.apis.values().any(|a| a.arn == resource_arn);
        if !api_exists {
            return Err(AppSyncError::NotFoundException(format!(
                "Resource '{}' not found.",
                resource_arn
            )));
        }

        Ok(state
            .tags
            .get(resource_arn)
            .cloned()
            .unwrap_or_default())
    }
}
