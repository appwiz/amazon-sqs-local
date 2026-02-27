use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::SecuritylakeError;
use super::types::*;

#[allow(dead_code)]
struct SecuritylakeStateInner {
    data_lakes: HashMap<String, StoredDataLake>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredDataLake {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct SecuritylakeState {
    inner: Arc<Mutex<SecuritylakeStateInner>>,
}

impl SecuritylakeState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        SecuritylakeState {
            inner: Arc::new(Mutex::new(SecuritylakeStateInner {
                data_lakes: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_data_lake(&self, req: CreateDataLakeRequest) -> Result<DataLakeDetail, SecuritylakeError> {
        let mut state = self.inner.lock().await;
        let name = req.name.or(req.name_pascal).unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        if state.data_lakes.contains_key(&name) {
            return Err(SecuritylakeError::ResourceAlreadyExistsException(format!("DataLake {} already exists", name)));
        }
        let arn = format!("arn:aws:securitylake:{}:{}:data-lakes/{}", state.region, state.account_id, name);
        let detail = DataLakeDetail {
            name: name.clone(),
            arn: arn.clone(),
            status: "ACTIVE".to_string(),
            tags: req.tags.clone(),
        };
        state.data_lakes.insert(name, StoredDataLake {
            name: detail.name.clone(),
            arn,
            tags: req.tags.unwrap_or_default(),
        });
        Ok(detail)
    }

    #[allow(dead_code)]
    pub async fn get_data_lake(&self, name: &str) -> Result<DataLakeDetail, SecuritylakeError> {
        let state = self.inner.lock().await;
        let stored = state.data_lakes.get(name)
            .ok_or_else(|| SecuritylakeError::ResourceNotFoundException(format!("DataLake {} not found", name)))?;
        Ok(DataLakeDetail {
            name: stored.name.clone(),
            arn: stored.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(stored.tags.clone()),
        })
    }

    #[allow(dead_code)]
    pub async fn list_data_lakes(&self) -> Result<ListDataLakesResponse, SecuritylakeError> {
        let state = self.inner.lock().await;
        let items: Vec<DataLakeDetail> = state.data_lakes.values().map(|s| DataLakeDetail {
            name: s.name.clone(),
            arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListDataLakesResponse {
            items: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_data_lake(&self, name: &str) -> Result<(), SecuritylakeError> {
        let mut state = self.inner.lock().await;
        state.data_lakes.remove(name)
            .ok_or_else(|| SecuritylakeError::ResourceNotFoundException(format!("DataLake {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = SecuritylakeState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_data_lake() {
        let state = SecuritylakeState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateDataLakeRequest::default();
        let result = state.create_data_lake(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_get_data_lake_not_found() {
        let state = SecuritylakeState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.get_data_lake("nonexistent").await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_data_lakes() {
        let state = SecuritylakeState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.list_data_lakes().await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_data_lake_not_found() {
        let state = SecuritylakeState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.delete_data_lake("nonexistent").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_data_lake_full_crud() {
        let state = SecuritylakeState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateDataLakeRequest::default();
        create_req.name = Some("test-crud-resource".to_string());
        let create_result = state.create_data_lake(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let get_result = state.get_data_lake("test-crud-resource").await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let del_result = state.delete_data_lake("test-crud-resource").await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
