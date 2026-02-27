use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::ForecastError;
use super::types::*;

#[allow(dead_code)]
struct ForecastStateInner {
    datasets: HashMap<String, StoredDataset>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredDataset {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct ForecastState {
    inner: Arc<Mutex<ForecastStateInner>>,
}

impl ForecastState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        ForecastState {
            inner: Arc::new(Mutex::new(ForecastStateInner {
                datasets: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_dataset(&self, req: CreateDatasetRequest) -> Result<CreateDatasetResponse, ForecastError> {
        let mut state = self.inner.lock().await;
        let name = req.dataset_name.clone();
        if state.datasets.contains_key(&name) {
            return Err(ForecastError::ResourceAlreadyExistsException(format!("Dataset {} already exists", name)));
        }
        let arn = format!("arn:aws:forecast:{}:{}:datasets/{}", state.region, state.account_id, name);
        state.datasets.insert(name.clone(), StoredDataset {
            name: name.clone(),
            arn: arn.clone(),
            tags: req.tags.unwrap_or_default(),
        });
        Ok(CreateDatasetResponse {
            dataset_arn: Some(arn),
            dataset_name: Some(name),
        })
    }

    #[allow(dead_code)]
    pub async fn describe_dataset(&self, req: DescribeDatasetRequest) -> Result<DescribeDatasetResponse, ForecastError> {
        let state = self.inner.lock().await;
        let name = req.dataset_name.or_else(|| req.dataset_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| ForecastError::ValidationException("Name or ARN required".to_string()))?;
        let stored = state.datasets.get(&name)
            .ok_or_else(|| ForecastError::ResourceNotFoundException(format!("Dataset {} not found", name)))?;
        Ok(DescribeDatasetResponse {
            dataset: DatasetDetail {
                dataset_name: stored.name.clone(),
                dataset_arn: stored.arn.clone(),
                status: "ACTIVE".to_string(),
                tags: Some(stored.tags.clone()),
            },
        })
    }

    #[allow(dead_code)]
    pub async fn list_datasets(&self, _req: ListDatasetsRequest) -> Result<ListDatasetsResponse, ForecastError> {
        let state = self.inner.lock().await;
        let items: Vec<DatasetDetail> = state.datasets.values().map(|s| DatasetDetail {
            dataset_name: s.name.clone(),
            dataset_arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListDatasetsResponse {
            datasets: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_dataset(&self, req: DeleteDatasetRequest) -> Result<(), ForecastError> {
        let mut state = self.inner.lock().await;
        let name = req.dataset_name.or_else(|| req.dataset_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| ForecastError::ValidationException("Name or ARN required".to_string()))?;
        state.datasets.remove(&name)
            .ok_or_else(|| ForecastError::ResourceNotFoundException(format!("Dataset {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = ForecastState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_dataset() {
        let state = ForecastState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateDatasetRequest::default();
        let result = state.create_dataset(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_describe_dataset_not_found() {
        let state = ForecastState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DescribeDatasetRequest::default();
        let result = state.describe_dataset(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_datasets_empty() {
        let state = ForecastState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = ListDatasetsRequest::default();
        let result = state.list_datasets(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_dataset_not_found() {
        let state = ForecastState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DeleteDatasetRequest::default();
        let result = state.delete_dataset(req).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_dataset_create_and_list() {
        let state = ForecastState::new("123456789012".to_string(), "us-east-1".to_string());
        let create_req = CreateDatasetRequest::default();
        let _created = state.create_dataset(create_req).await.unwrap();
        let list_req = ListDatasetsRequest::default();
        let listed = state.list_datasets(list_req).await.unwrap();
        let _ = listed;
    }

    #[tokio::test]
    async fn test_dataset_full_crud() {
        let state = ForecastState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateDatasetRequest::default();
        create_req.dataset_name = "test-crud-resource".to_string();
        let create_result = state.create_dataset(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let mut get_req = DescribeDatasetRequest::default();
        get_req.dataset_name = Some("test-crud-resource".to_string());
        let get_result = state.describe_dataset(get_req).await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let mut del_req = DeleteDatasetRequest::default();
        del_req.dataset_name = Some("test-crud-resource".to_string());
        let del_result = state.delete_dataset(del_req).await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
