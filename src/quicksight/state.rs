use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::QuicksightError;
use super::types::*;

#[allow(dead_code)]
struct QuicksightStateInner {
    data_sets: HashMap<String, StoredDataSet>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredDataSet {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct QuicksightState {
    inner: Arc<Mutex<QuicksightStateInner>>,
}

impl QuicksightState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        QuicksightState {
            inner: Arc::new(Mutex::new(QuicksightStateInner {
                data_sets: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_data_set(&self, req: CreateDataSetRequest) -> Result<DataSetDetail, QuicksightError> {
        let mut state = self.inner.lock().await;
        let name = req.name.or(req.name_pascal).unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        if state.data_sets.contains_key(&name) {
            return Err(QuicksightError::ResourceAlreadyExistsException(format!("DataSet {} already exists", name)));
        }
        let arn = format!("arn:aws:quicksight:{}:{}:data-sets/{}", state.region, state.account_id, name);
        let detail = DataSetDetail {
            name: name.clone(),
            arn: arn.clone(),
            status: "ACTIVE".to_string(),
            tags: req.tags.clone(),
        };
        state.data_sets.insert(name, StoredDataSet {
            name: detail.name.clone(),
            arn,
            tags: req.tags.unwrap_or_default(),
        });
        Ok(detail)
    }

    #[allow(dead_code)]
    pub async fn get_data_set(&self, name: &str) -> Result<DataSetDetail, QuicksightError> {
        let state = self.inner.lock().await;
        let stored = state.data_sets.get(name)
            .ok_or_else(|| QuicksightError::ResourceNotFoundException(format!("DataSet {} not found", name)))?;
        Ok(DataSetDetail {
            name: stored.name.clone(),
            arn: stored.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(stored.tags.clone()),
        })
    }

    #[allow(dead_code)]
    pub async fn list_data_sets(&self) -> Result<ListDataSetsResponse, QuicksightError> {
        let state = self.inner.lock().await;
        let items: Vec<DataSetDetail> = state.data_sets.values().map(|s| DataSetDetail {
            name: s.name.clone(),
            arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListDataSetsResponse {
            items: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_data_set(&self, name: &str) -> Result<(), QuicksightError> {
        let mut state = self.inner.lock().await;
        state.data_sets.remove(name)
            .ok_or_else(|| QuicksightError::ResourceNotFoundException(format!("DataSet {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = QuicksightState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_data_set() {
        let state = QuicksightState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateDataSetRequest::default();
        let result = state.create_data_set(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_get_data_set_not_found() {
        let state = QuicksightState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.get_data_set("nonexistent").await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_data_sets() {
        let state = QuicksightState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.list_data_sets().await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_data_set_not_found() {
        let state = QuicksightState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.delete_data_set("nonexistent").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_data_set_full_crud() {
        let state = QuicksightState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateDataSetRequest::default();
        create_req.name = Some("test-crud-resource".to_string());
        let create_result = state.create_data_set(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let get_result = state.get_data_set("test-crud-resource").await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let del_result = state.delete_data_set("test-crud-resource").await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
