use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::CloudtrailError;
use super::types::*;

#[allow(dead_code)]
struct CloudtrailStateInner {
    trails: HashMap<String, StoredTrail>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredTrail {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct CloudtrailState {
    inner: Arc<Mutex<CloudtrailStateInner>>,
}

impl CloudtrailState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        CloudtrailState {
            inner: Arc::new(Mutex::new(CloudtrailStateInner {
                trails: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_trail(&self, req: CreateTrailRequest) -> Result<CreateTrailResponse, CloudtrailError> {
        let mut state = self.inner.lock().await;
        let name = req.trail_name.clone();
        if state.trails.contains_key(&name) {
            return Err(CloudtrailError::ResourceAlreadyExistsException(format!("Trail {} already exists", name)));
        }
        let arn = format!("arn:aws:cloudtrail:{}:{}:trails/{}", state.region, state.account_id, name);
        state.trails.insert(name.clone(), StoredTrail {
            name: name.clone(),
            arn: arn.clone(),
            tags: req.tags.unwrap_or_default(),
        });
        Ok(CreateTrailResponse {
            trail_arn: Some(arn),
            trail_name: Some(name),
        })
    }

    #[allow(dead_code)]
    pub async fn describe_trail(&self, req: DescribeTrailRequest) -> Result<DescribeTrailResponse, CloudtrailError> {
        let state = self.inner.lock().await;
        let name = req.trail_name.or_else(|| req.trail_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| CloudtrailError::ValidationException("Name or ARN required".to_string()))?;
        let stored = state.trails.get(&name)
            .ok_or_else(|| CloudtrailError::ResourceNotFoundException(format!("Trail {} not found", name)))?;
        Ok(DescribeTrailResponse {
            trail: TrailDetail {
                trail_name: stored.name.clone(),
                trail_arn: stored.arn.clone(),
                status: "ACTIVE".to_string(),
                tags: Some(stored.tags.clone()),
            },
        })
    }

    #[allow(dead_code)]
    pub async fn list_trails(&self, _req: ListTrailsRequest) -> Result<ListTrailsResponse, CloudtrailError> {
        let state = self.inner.lock().await;
        let items: Vec<TrailDetail> = state.trails.values().map(|s| TrailDetail {
            trail_name: s.name.clone(),
            trail_arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListTrailsResponse {
            trails: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_trail(&self, req: DeleteTrailRequest) -> Result<(), CloudtrailError> {
        let mut state = self.inner.lock().await;
        let name = req.trail_name.or_else(|| req.trail_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| CloudtrailError::ValidationException("Name or ARN required".to_string()))?;
        state.trails.remove(&name)
            .ok_or_else(|| CloudtrailError::ResourceNotFoundException(format!("Trail {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = CloudtrailState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_trail() {
        let state = CloudtrailState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateTrailRequest::default();
        let result = state.create_trail(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_describe_trail_not_found() {
        let state = CloudtrailState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DescribeTrailRequest::default();
        let result = state.describe_trail(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_trails_empty() {
        let state = CloudtrailState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = ListTrailsRequest::default();
        let result = state.list_trails(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_trail_not_found() {
        let state = CloudtrailState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DeleteTrailRequest::default();
        let result = state.delete_trail(req).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_trail_create_and_list() {
        let state = CloudtrailState::new("123456789012".to_string(), "us-east-1".to_string());
        let create_req = CreateTrailRequest::default();
        let _created = state.create_trail(create_req).await.unwrap();
        let list_req = ListTrailsRequest::default();
        let listed = state.list_trails(list_req).await.unwrap();
        let _ = listed;
    }

    #[tokio::test]
    async fn test_trail_full_crud() {
        let state = CloudtrailState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateTrailRequest::default();
        create_req.trail_name = "test-crud-resource".to_string();
        let create_result = state.create_trail(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let mut get_req = DescribeTrailRequest::default();
        get_req.trail_name = Some("test-crud-resource".to_string());
        let get_result = state.describe_trail(get_req).await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let mut del_req = DeleteTrailRequest::default();
        del_req.trail_name = Some("test-crud-resource".to_string());
        let del_result = state.delete_trail(del_req).await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
