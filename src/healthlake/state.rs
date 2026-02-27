use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::HealthlakeError;
use super::types::*;

#[allow(dead_code)]
struct HealthlakeStateInner {
    f_h_i_r_datastores: HashMap<String, StoredFHIRDatastore>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredFHIRDatastore {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct HealthlakeState {
    inner: Arc<Mutex<HealthlakeStateInner>>,
}

impl HealthlakeState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        HealthlakeState {
            inner: Arc::new(Mutex::new(HealthlakeStateInner {
                f_h_i_r_datastores: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_f_h_i_r_datastore(&self, req: CreateFHIRDatastoreRequest) -> Result<CreateFHIRDatastoreResponse, HealthlakeError> {
        let mut state = self.inner.lock().await;
        let name = req.f_h_i_r_datastore_name.clone();
        if state.f_h_i_r_datastores.contains_key(&name) {
            return Err(HealthlakeError::ResourceAlreadyExistsException(format!("FHIRDatastore {} already exists", name)));
        }
        let arn = format!("arn:aws:healthlake:{}:{}:fhir-datastores/{}", state.region, state.account_id, name);
        state.f_h_i_r_datastores.insert(name.clone(), StoredFHIRDatastore {
            name: name.clone(),
            arn: arn.clone(),
            tags: req.tags.unwrap_or_default(),
        });
        Ok(CreateFHIRDatastoreResponse {
            f_h_i_r_datastore_arn: Some(arn),
            f_h_i_r_datastore_name: Some(name),
        })
    }

    #[allow(dead_code)]
    pub async fn describe_f_h_i_r_datastore(&self, req: DescribeFHIRDatastoreRequest) -> Result<DescribeFHIRDatastoreResponse, HealthlakeError> {
        let state = self.inner.lock().await;
        let name = req.f_h_i_r_datastore_name.or_else(|| req.f_h_i_r_datastore_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| HealthlakeError::ValidationException("Name or ARN required".to_string()))?;
        let stored = state.f_h_i_r_datastores.get(&name)
            .ok_or_else(|| HealthlakeError::ResourceNotFoundException(format!("FHIRDatastore {} not found", name)))?;
        Ok(DescribeFHIRDatastoreResponse {
            f_h_i_r_datastore: FHIRDatastoreDetail {
                f_h_i_r_datastore_name: stored.name.clone(),
                f_h_i_r_datastore_arn: stored.arn.clone(),
                status: "ACTIVE".to_string(),
                tags: Some(stored.tags.clone()),
            },
        })
    }

    #[allow(dead_code)]
    pub async fn list_f_h_i_r_datastores(&self, _req: ListFHIRDatastoresRequest) -> Result<ListFHIRDatastoresResponse, HealthlakeError> {
        let state = self.inner.lock().await;
        let items: Vec<FHIRDatastoreDetail> = state.f_h_i_r_datastores.values().map(|s| FHIRDatastoreDetail {
            f_h_i_r_datastore_name: s.name.clone(),
            f_h_i_r_datastore_arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListFHIRDatastoresResponse {
            f_h_i_r_datastores: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_f_h_i_r_datastore(&self, req: DeleteFHIRDatastoreRequest) -> Result<(), HealthlakeError> {
        let mut state = self.inner.lock().await;
        let name = req.f_h_i_r_datastore_name.or_else(|| req.f_h_i_r_datastore_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| HealthlakeError::ValidationException("Name or ARN required".to_string()))?;
        state.f_h_i_r_datastores.remove(&name)
            .ok_or_else(|| HealthlakeError::ResourceNotFoundException(format!("FHIRDatastore {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = HealthlakeState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_f_h_i_r_datastore() {
        let state = HealthlakeState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateFHIRDatastoreRequest::default();
        let result = state.create_f_h_i_r_datastore(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_describe_f_h_i_r_datastore_not_found() {
        let state = HealthlakeState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DescribeFHIRDatastoreRequest::default();
        let result = state.describe_f_h_i_r_datastore(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_f_h_i_r_datastores_empty() {
        let state = HealthlakeState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = ListFHIRDatastoresRequest::default();
        let result = state.list_f_h_i_r_datastores(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_f_h_i_r_datastore_not_found() {
        let state = HealthlakeState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DeleteFHIRDatastoreRequest::default();
        let result = state.delete_f_h_i_r_datastore(req).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_f_h_i_r_datastore_create_and_list() {
        let state = HealthlakeState::new("123456789012".to_string(), "us-east-1".to_string());
        let create_req = CreateFHIRDatastoreRequest::default();
        let _created = state.create_f_h_i_r_datastore(create_req).await.unwrap();
        let list_req = ListFHIRDatastoresRequest::default();
        let listed = state.list_f_h_i_r_datastores(list_req).await.unwrap();
        let _ = listed;
    }

    #[tokio::test]
    async fn test_f_h_i_r_datastore_full_crud() {
        let state = HealthlakeState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateFHIRDatastoreRequest::default();
        create_req.f_h_i_r_datastore_name = "test-crud-resource".to_string();
        let create_result = state.create_f_h_i_r_datastore(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let mut get_req = DescribeFHIRDatastoreRequest::default();
        get_req.f_h_i_r_datastore_name = Some("test-crud-resource".to_string());
        let get_result = state.describe_f_h_i_r_datastore(get_req).await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let mut del_req = DeleteFHIRDatastoreRequest::default();
        del_req.f_h_i_r_datastore_name = Some("test-crud-resource".to_string());
        let del_result = state.delete_f_h_i_r_datastore(del_req).await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
