use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::ApprunnerError;
use super::types::*;

#[allow(dead_code)]
struct ApprunnerStateInner {
    services: HashMap<String, StoredService>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredService {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct ApprunnerState {
    inner: Arc<Mutex<ApprunnerStateInner>>,
}

impl ApprunnerState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        ApprunnerState {
            inner: Arc::new(Mutex::new(ApprunnerStateInner {
                services: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_service(&self, req: CreateServiceRequest) -> Result<CreateServiceResponse, ApprunnerError> {
        let mut state = self.inner.lock().await;
        let name = req.service_name.clone();
        if state.services.contains_key(&name) {
            return Err(ApprunnerError::ResourceAlreadyExistsException(format!("Service {} already exists", name)));
        }
        let arn = format!("arn:aws:apprunner:{}:{}:services/{}", state.region, state.account_id, name);
        state.services.insert(name.clone(), StoredService {
            name: name.clone(),
            arn: arn.clone(),
            tags: req.tags.unwrap_or_default(),
        });
        Ok(CreateServiceResponse {
            service_arn: Some(arn),
            service_name: Some(name),
        })
    }

    #[allow(dead_code)]
    pub async fn describe_service(&self, req: DescribeServiceRequest) -> Result<DescribeServiceResponse, ApprunnerError> {
        let state = self.inner.lock().await;
        let name = req.service_name.or_else(|| req.service_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| ApprunnerError::ValidationException("Name or ARN required".to_string()))?;
        let stored = state.services.get(&name)
            .ok_or_else(|| ApprunnerError::ResourceNotFoundException(format!("Service {} not found", name)))?;
        Ok(DescribeServiceResponse {
            service: ServiceDetail {
                service_name: stored.name.clone(),
                service_arn: stored.arn.clone(),
                status: "ACTIVE".to_string(),
                tags: Some(stored.tags.clone()),
            },
        })
    }

    #[allow(dead_code)]
    pub async fn list_services(&self, _req: ListServicesRequest) -> Result<ListServicesResponse, ApprunnerError> {
        let state = self.inner.lock().await;
        let items: Vec<ServiceDetail> = state.services.values().map(|s| ServiceDetail {
            service_name: s.name.clone(),
            service_arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListServicesResponse {
            services: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_service(&self, req: DeleteServiceRequest) -> Result<(), ApprunnerError> {
        let mut state = self.inner.lock().await;
        let name = req.service_name.or_else(|| req.service_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| ApprunnerError::ValidationException("Name or ARN required".to_string()))?;
        state.services.remove(&name)
            .ok_or_else(|| ApprunnerError::ResourceNotFoundException(format!("Service {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = ApprunnerState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_service() {
        let state = ApprunnerState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateServiceRequest::default();
        let result = state.create_service(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_describe_service_not_found() {
        let state = ApprunnerState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DescribeServiceRequest::default();
        let result = state.describe_service(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_services_empty() {
        let state = ApprunnerState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = ListServicesRequest::default();
        let result = state.list_services(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_service_not_found() {
        let state = ApprunnerState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DeleteServiceRequest::default();
        let result = state.delete_service(req).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_service_create_and_list() {
        let state = ApprunnerState::new("123456789012".to_string(), "us-east-1".to_string());
        let create_req = CreateServiceRequest::default();
        let _created = state.create_service(create_req).await.unwrap();
        let list_req = ListServicesRequest::default();
        let listed = state.list_services(list_req).await.unwrap();
        let _ = listed;
    }

    #[tokio::test]
    async fn test_service_full_crud() {
        let state = ApprunnerState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateServiceRequest::default();
        create_req.service_name = "test-crud-resource".to_string();
        let create_result = state.create_service(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let mut get_req = DescribeServiceRequest::default();
        get_req.service_name = Some("test-crud-resource".to_string());
        let get_result = state.describe_service(get_req).await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let mut del_req = DeleteServiceRequest::default();
        del_req.service_name = Some("test-crud-resource".to_string());
        let del_result = state.delete_service(del_req).await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
