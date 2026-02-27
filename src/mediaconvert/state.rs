use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::MediaconvertError;
use super::types::*;

#[allow(dead_code)]
struct MediaconvertStateInner {
    jobs: HashMap<String, StoredJob>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredJob {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct MediaconvertState {
    inner: Arc<Mutex<MediaconvertStateInner>>,
}

impl MediaconvertState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        MediaconvertState {
            inner: Arc::new(Mutex::new(MediaconvertStateInner {
                jobs: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_job(&self, req: CreateJobRequest) -> Result<JobDetail, MediaconvertError> {
        let mut state = self.inner.lock().await;
        let name = req.name.or(req.name_pascal).unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        if state.jobs.contains_key(&name) {
            return Err(MediaconvertError::ResourceAlreadyExistsException(format!("Job {} already exists", name)));
        }
        let arn = format!("arn:aws:mediaconvert:{}:{}:jobs/{}", state.region, state.account_id, name);
        let detail = JobDetail {
            name: name.clone(),
            arn: arn.clone(),
            status: "ACTIVE".to_string(),
            tags: req.tags.clone(),
        };
        state.jobs.insert(name, StoredJob {
            name: detail.name.clone(),
            arn,
            tags: req.tags.unwrap_or_default(),
        });
        Ok(detail)
    }

    #[allow(dead_code)]
    pub async fn get_job(&self, name: &str) -> Result<JobDetail, MediaconvertError> {
        let state = self.inner.lock().await;
        let stored = state.jobs.get(name)
            .ok_or_else(|| MediaconvertError::ResourceNotFoundException(format!("Job {} not found", name)))?;
        Ok(JobDetail {
            name: stored.name.clone(),
            arn: stored.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(stored.tags.clone()),
        })
    }

    #[allow(dead_code)]
    pub async fn list_jobs(&self) -> Result<ListJobsResponse, MediaconvertError> {
        let state = self.inner.lock().await;
        let items: Vec<JobDetail> = state.jobs.values().map(|s| JobDetail {
            name: s.name.clone(),
            arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListJobsResponse {
            items: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_job(&self, name: &str) -> Result<(), MediaconvertError> {
        let mut state = self.inner.lock().await;
        state.jobs.remove(name)
            .ok_or_else(|| MediaconvertError::ResourceNotFoundException(format!("Job {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = MediaconvertState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_job() {
        let state = MediaconvertState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateJobRequest::default();
        let result = state.create_job(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_get_job_not_found() {
        let state = MediaconvertState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.get_job("nonexistent").await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_jobs() {
        let state = MediaconvertState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.list_jobs().await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_job_not_found() {
        let state = MediaconvertState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.delete_job("nonexistent").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_job_full_crud() {
        let state = MediaconvertState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateJobRequest::default();
        create_req.name = Some("test-crud-resource".to_string());
        let create_result = state.create_job(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let get_result = state.get_job("test-crud-resource").await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let del_result = state.delete_job("test-crud-resource").await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
