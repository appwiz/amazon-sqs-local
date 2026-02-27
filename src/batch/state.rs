use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::BatchError;
use super::types::*;

#[allow(dead_code)]
struct BatchStateInner {
    compute_environments: HashMap<String, StoredComputeEnvironment>,
    job_queues: HashMap<String, StoredJobQueue>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredComputeEnvironment {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
struct StoredJobQueue {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct BatchState {
    inner: Arc<Mutex<BatchStateInner>>,
}

impl BatchState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        BatchState {
            inner: Arc::new(Mutex::new(BatchStateInner {
                compute_environments: HashMap::new(),
                job_queues: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_compute_environment(&self, req: CreateComputeEnvironmentRequest) -> Result<ComputeEnvironmentDetail, BatchError> {
        let mut state = self.inner.lock().await;
        let name = req.name.or(req.name_pascal).unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        if state.compute_environments.contains_key(&name) {
            return Err(BatchError::ResourceAlreadyExistsException(format!("ComputeEnvironment {} already exists", name)));
        }
        let arn = format!("arn:aws:batch:{}:{}:compute-environments/{}", state.region, state.account_id, name);
        let detail = ComputeEnvironmentDetail {
            name: name.clone(),
            arn: arn.clone(),
            status: "ACTIVE".to_string(),
            tags: req.tags.clone(),
        };
        state.compute_environments.insert(name, StoredComputeEnvironment {
            name: detail.name.clone(),
            arn,
            tags: req.tags.unwrap_or_default(),
        });
        Ok(detail)
    }

    #[allow(dead_code)]
    pub async fn get_compute_environment(&self, name: &str) -> Result<ComputeEnvironmentDetail, BatchError> {
        let state = self.inner.lock().await;
        let stored = state.compute_environments.get(name)
            .ok_or_else(|| BatchError::ResourceNotFoundException(format!("ComputeEnvironment {} not found", name)))?;
        Ok(ComputeEnvironmentDetail {
            name: stored.name.clone(),
            arn: stored.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(stored.tags.clone()),
        })
    }

    #[allow(dead_code)]
    pub async fn list_compute_environments(&self) -> Result<ListComputeEnvironmentsResponse, BatchError> {
        let state = self.inner.lock().await;
        let items: Vec<ComputeEnvironmentDetail> = state.compute_environments.values().map(|s| ComputeEnvironmentDetail {
            name: s.name.clone(),
            arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListComputeEnvironmentsResponse {
            items: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_compute_environment(&self, name: &str) -> Result<(), BatchError> {
        let mut state = self.inner.lock().await;
        state.compute_environments.remove(name)
            .ok_or_else(|| BatchError::ResourceNotFoundException(format!("ComputeEnvironment {} not found", name)))?;
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn create_job_queue(&self, req: CreateJobQueueRequest) -> Result<JobQueueDetail, BatchError> {
        let mut state = self.inner.lock().await;
        let name = req.name.or(req.name_pascal).unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        if state.job_queues.contains_key(&name) {
            return Err(BatchError::ResourceAlreadyExistsException(format!("JobQueue {} already exists", name)));
        }
        let arn = format!("arn:aws:batch:{}:{}:job-queues/{}", state.region, state.account_id, name);
        let detail = JobQueueDetail {
            name: name.clone(),
            arn: arn.clone(),
            status: "ACTIVE".to_string(),
            tags: req.tags.clone(),
        };
        state.job_queues.insert(name, StoredJobQueue {
            name: detail.name.clone(),
            arn,
            tags: req.tags.unwrap_or_default(),
        });
        Ok(detail)
    }

    #[allow(dead_code)]
    pub async fn get_job_queue(&self, name: &str) -> Result<JobQueueDetail, BatchError> {
        let state = self.inner.lock().await;
        let stored = state.job_queues.get(name)
            .ok_or_else(|| BatchError::ResourceNotFoundException(format!("JobQueue {} not found", name)))?;
        Ok(JobQueueDetail {
            name: stored.name.clone(),
            arn: stored.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(stored.tags.clone()),
        })
    }

    #[allow(dead_code)]
    pub async fn list_job_queues(&self) -> Result<ListJobQueuesResponse, BatchError> {
        let state = self.inner.lock().await;
        let items: Vec<JobQueueDetail> = state.job_queues.values().map(|s| JobQueueDetail {
            name: s.name.clone(),
            arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListJobQueuesResponse {
            items: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_job_queue(&self, name: &str) -> Result<(), BatchError> {
        let mut state = self.inner.lock().await;
        state.job_queues.remove(name)
            .ok_or_else(|| BatchError::ResourceNotFoundException(format!("JobQueue {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = BatchState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_compute_environment() {
        let state = BatchState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateComputeEnvironmentRequest::default();
        let result = state.create_compute_environment(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_get_compute_environment_not_found() {
        let state = BatchState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.get_compute_environment("nonexistent").await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_compute_environments() {
        let state = BatchState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.list_compute_environments().await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_compute_environment_not_found() {
        let state = BatchState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.delete_compute_environment("nonexistent").await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_create_job_queue() {
        let state = BatchState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateJobQueueRequest::default();
        let result = state.create_job_queue(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_get_job_queue_not_found() {
        let state = BatchState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.get_job_queue("nonexistent").await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_job_queues() {
        let state = BatchState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.list_job_queues().await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_job_queue_not_found() {
        let state = BatchState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.delete_job_queue("nonexistent").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_compute_environment_full_crud() {
        let state = BatchState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateComputeEnvironmentRequest::default();
        create_req.name = Some("test-crud-resource".to_string());
        let create_result = state.create_compute_environment(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let get_result = state.get_compute_environment("test-crud-resource").await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let del_result = state.delete_compute_environment("test-crud-resource").await;
        assert!(del_result.is_ok(), "delete should succeed");
    }

    #[tokio::test]
    async fn test_job_queue_full_crud() {
        let state = BatchState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateJobQueueRequest::default();
        create_req.name = Some("test-crud-resource".to_string());
        let create_result = state.create_job_queue(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let get_result = state.get_job_queue("test-crud-resource").await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let del_result = state.delete_job_queue("test-crud-resource").await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
