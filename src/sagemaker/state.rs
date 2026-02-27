use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::SagemakerError;
use super::types::*;

#[allow(dead_code)]
struct SagemakerStateInner {
    notebook_instances: HashMap<String, StoredNotebookInstance>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredNotebookInstance {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct SagemakerState {
    inner: Arc<Mutex<SagemakerStateInner>>,
}

impl SagemakerState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        SagemakerState {
            inner: Arc::new(Mutex::new(SagemakerStateInner {
                notebook_instances: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_notebook_instance(&self, req: CreateNotebookInstanceRequest) -> Result<CreateNotebookInstanceResponse, SagemakerError> {
        let mut state = self.inner.lock().await;
        let name = req.notebook_instance_name.clone();
        if state.notebook_instances.contains_key(&name) {
            return Err(SagemakerError::ResourceAlreadyExistsException(format!("NotebookInstance {} already exists", name)));
        }
        let arn = format!("arn:aws:sagemaker:{}:{}:notebook-instances/{}", state.region, state.account_id, name);
        state.notebook_instances.insert(name.clone(), StoredNotebookInstance {
            name: name.clone(),
            arn: arn.clone(),
            tags: req.tags.unwrap_or_default(),
        });
        Ok(CreateNotebookInstanceResponse {
            notebook_instance_arn: Some(arn),
            notebook_instance_name: Some(name),
        })
    }

    #[allow(dead_code)]
    pub async fn describe_notebook_instance(&self, req: DescribeNotebookInstanceRequest) -> Result<DescribeNotebookInstanceResponse, SagemakerError> {
        let state = self.inner.lock().await;
        let name = req.notebook_instance_name.or_else(|| req.notebook_instance_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| SagemakerError::ValidationException("Name or ARN required".to_string()))?;
        let stored = state.notebook_instances.get(&name)
            .ok_or_else(|| SagemakerError::ResourceNotFoundException(format!("NotebookInstance {} not found", name)))?;
        Ok(DescribeNotebookInstanceResponse {
            notebook_instance: NotebookInstanceDetail {
                notebook_instance_name: stored.name.clone(),
                notebook_instance_arn: stored.arn.clone(),
                status: "ACTIVE".to_string(),
                tags: Some(stored.tags.clone()),
            },
        })
    }

    #[allow(dead_code)]
    pub async fn list_notebook_instances(&self, _req: ListNotebookInstancesRequest) -> Result<ListNotebookInstancesResponse, SagemakerError> {
        let state = self.inner.lock().await;
        let items: Vec<NotebookInstanceDetail> = state.notebook_instances.values().map(|s| NotebookInstanceDetail {
            notebook_instance_name: s.name.clone(),
            notebook_instance_arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListNotebookInstancesResponse {
            notebook_instances: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_notebook_instance(&self, req: DeleteNotebookInstanceRequest) -> Result<(), SagemakerError> {
        let mut state = self.inner.lock().await;
        let name = req.notebook_instance_name.or_else(|| req.notebook_instance_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| SagemakerError::ValidationException("Name or ARN required".to_string()))?;
        state.notebook_instances.remove(&name)
            .ok_or_else(|| SagemakerError::ResourceNotFoundException(format!("NotebookInstance {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = SagemakerState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_notebook_instance() {
        let state = SagemakerState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateNotebookInstanceRequest::default();
        let result = state.create_notebook_instance(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_describe_notebook_instance_not_found() {
        let state = SagemakerState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DescribeNotebookInstanceRequest::default();
        let result = state.describe_notebook_instance(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_notebook_instances_empty() {
        let state = SagemakerState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = ListNotebookInstancesRequest::default();
        let result = state.list_notebook_instances(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_notebook_instance_not_found() {
        let state = SagemakerState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DeleteNotebookInstanceRequest::default();
        let result = state.delete_notebook_instance(req).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_notebook_instance_create_and_list() {
        let state = SagemakerState::new("123456789012".to_string(), "us-east-1".to_string());
        let create_req = CreateNotebookInstanceRequest::default();
        let _created = state.create_notebook_instance(create_req).await.unwrap();
        let list_req = ListNotebookInstancesRequest::default();
        let listed = state.list_notebook_instances(list_req).await.unwrap();
        let _ = listed;
    }

    #[tokio::test]
    async fn test_notebook_instance_full_crud() {
        let state = SagemakerState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateNotebookInstanceRequest::default();
        create_req.notebook_instance_name = "test-crud-resource".to_string();
        let create_result = state.create_notebook_instance(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let mut get_req = DescribeNotebookInstanceRequest::default();
        get_req.notebook_instance_name = Some("test-crud-resource".to_string());
        let get_result = state.describe_notebook_instance(get_req).await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let mut del_req = DeleteNotebookInstanceRequest::default();
        del_req.notebook_instance_name = Some("test-crud-resource".to_string());
        let del_result = state.delete_notebook_instance(del_req).await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
