use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::TranscribeError;
use super::types::*;

#[allow(dead_code)]
struct TranscribeStateInner {
    transcription_jobs: HashMap<String, StoredTranscriptionJob>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredTranscriptionJob {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct TranscribeState {
    inner: Arc<Mutex<TranscribeStateInner>>,
}

impl TranscribeState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        TranscribeState {
            inner: Arc::new(Mutex::new(TranscribeStateInner {
                transcription_jobs: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_transcription_job(&self, req: CreateTranscriptionJobRequest) -> Result<CreateTranscriptionJobResponse, TranscribeError> {
        let mut state = self.inner.lock().await;
        let name = req.transcription_job_name.clone();
        if state.transcription_jobs.contains_key(&name) {
            return Err(TranscribeError::ResourceAlreadyExistsException(format!("TranscriptionJob {} already exists", name)));
        }
        let arn = format!("arn:aws:transcribe:{}:{}:transcription-jobs/{}", state.region, state.account_id, name);
        state.transcription_jobs.insert(name.clone(), StoredTranscriptionJob {
            name: name.clone(),
            arn: arn.clone(),
            tags: req.tags.unwrap_or_default(),
        });
        Ok(CreateTranscriptionJobResponse {
            transcription_job_arn: Some(arn),
            transcription_job_name: Some(name),
        })
    }

    #[allow(dead_code)]
    pub async fn describe_transcription_job(&self, req: DescribeTranscriptionJobRequest) -> Result<DescribeTranscriptionJobResponse, TranscribeError> {
        let state = self.inner.lock().await;
        let name = req.transcription_job_name.or_else(|| req.transcription_job_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| TranscribeError::ValidationException("Name or ARN required".to_string()))?;
        let stored = state.transcription_jobs.get(&name)
            .ok_or_else(|| TranscribeError::ResourceNotFoundException(format!("TranscriptionJob {} not found", name)))?;
        Ok(DescribeTranscriptionJobResponse {
            transcription_job: TranscriptionJobDetail {
                transcription_job_name: stored.name.clone(),
                transcription_job_arn: stored.arn.clone(),
                status: "ACTIVE".to_string(),
                tags: Some(stored.tags.clone()),
            },
        })
    }

    #[allow(dead_code)]
    pub async fn list_transcription_jobs(&self, _req: ListTranscriptionJobsRequest) -> Result<ListTranscriptionJobsResponse, TranscribeError> {
        let state = self.inner.lock().await;
        let items: Vec<TranscriptionJobDetail> = state.transcription_jobs.values().map(|s| TranscriptionJobDetail {
            transcription_job_name: s.name.clone(),
            transcription_job_arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListTranscriptionJobsResponse {
            transcription_jobs: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_transcription_job(&self, req: DeleteTranscriptionJobRequest) -> Result<(), TranscribeError> {
        let mut state = self.inner.lock().await;
        let name = req.transcription_job_name.or_else(|| req.transcription_job_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| TranscribeError::ValidationException("Name or ARN required".to_string()))?;
        state.transcription_jobs.remove(&name)
            .ok_or_else(|| TranscribeError::ResourceNotFoundException(format!("TranscriptionJob {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = TranscribeState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_transcription_job() {
        let state = TranscribeState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateTranscriptionJobRequest::default();
        let result = state.create_transcription_job(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_describe_transcription_job_not_found() {
        let state = TranscribeState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DescribeTranscriptionJobRequest::default();
        let result = state.describe_transcription_job(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_transcription_jobs_empty() {
        let state = TranscribeState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = ListTranscriptionJobsRequest::default();
        let result = state.list_transcription_jobs(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_transcription_job_not_found() {
        let state = TranscribeState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DeleteTranscriptionJobRequest::default();
        let result = state.delete_transcription_job(req).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_transcription_job_create_and_list() {
        let state = TranscribeState::new("123456789012".to_string(), "us-east-1".to_string());
        let create_req = CreateTranscriptionJobRequest::default();
        let _created = state.create_transcription_job(create_req).await.unwrap();
        let list_req = ListTranscriptionJobsRequest::default();
        let listed = state.list_transcription_jobs(list_req).await.unwrap();
        let _ = listed;
    }

    #[tokio::test]
    async fn test_transcription_job_full_crud() {
        let state = TranscribeState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateTranscriptionJobRequest::default();
        create_req.transcription_job_name = "test-crud-resource".to_string();
        let create_result = state.create_transcription_job(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let mut get_req = DescribeTranscriptionJobRequest::default();
        get_req.transcription_job_name = Some("test-crud-resource".to_string());
        let get_result = state.describe_transcription_job(get_req).await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let mut del_req = DeleteTranscriptionJobRequest::default();
        del_req.transcription_job_name = Some("test-crud-resource".to_string());
        let del_result = state.delete_transcription_job(del_req).await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
