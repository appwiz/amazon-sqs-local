use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::CloudfrontError;
use super::types::*;

#[allow(dead_code)]
struct CloudfrontStateInner {
    distributions: HashMap<String, StoredDistribution>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredDistribution {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct CloudfrontState {
    inner: Arc<Mutex<CloudfrontStateInner>>,
}

impl CloudfrontState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        CloudfrontState {
            inner: Arc::new(Mutex::new(CloudfrontStateInner {
                distributions: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_distribution(&self, req: CreateDistributionRequest) -> Result<DistributionDetail, CloudfrontError> {
        let mut state = self.inner.lock().await;
        let name = req.name.or(req.name_pascal).unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        if state.distributions.contains_key(&name) {
            return Err(CloudfrontError::ResourceAlreadyExistsException(format!("Distribution {} already exists", name)));
        }
        let arn = format!("arn:aws:cloudfront:{}:{}:distributions/{}", state.region, state.account_id, name);
        let detail = DistributionDetail {
            name: name.clone(),
            arn: arn.clone(),
            status: "ACTIVE".to_string(),
            tags: req.tags.clone(),
        };
        state.distributions.insert(name, StoredDistribution {
            name: detail.name.clone(),
            arn,
            tags: req.tags.unwrap_or_default(),
        });
        Ok(detail)
    }

    #[allow(dead_code)]
    pub async fn get_distribution(&self, name: &str) -> Result<DistributionDetail, CloudfrontError> {
        let state = self.inner.lock().await;
        let stored = state.distributions.get(name)
            .ok_or_else(|| CloudfrontError::ResourceNotFoundException(format!("Distribution {} not found", name)))?;
        Ok(DistributionDetail {
            name: stored.name.clone(),
            arn: stored.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(stored.tags.clone()),
        })
    }

    #[allow(dead_code)]
    pub async fn list_distributions(&self) -> Result<ListDistributionsResponse, CloudfrontError> {
        let state = self.inner.lock().await;
        let items: Vec<DistributionDetail> = state.distributions.values().map(|s| DistributionDetail {
            name: s.name.clone(),
            arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListDistributionsResponse {
            items: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_distribution(&self, name: &str) -> Result<(), CloudfrontError> {
        let mut state = self.inner.lock().await;
        state.distributions.remove(name)
            .ok_or_else(|| CloudfrontError::ResourceNotFoundException(format!("Distribution {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = CloudfrontState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_distribution() {
        let state = CloudfrontState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateDistributionRequest::default();
        let result = state.create_distribution(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_get_distribution_not_found() {
        let state = CloudfrontState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.get_distribution("nonexistent").await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_distributions() {
        let state = CloudfrontState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.list_distributions().await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_distribution_not_found() {
        let state = CloudfrontState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.delete_distribution("nonexistent").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_distribution_full_crud() {
        let state = CloudfrontState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateDistributionRequest::default();
        create_req.name = Some("test-crud-resource".to_string());
        let create_result = state.create_distribution(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let get_result = state.get_distribution("test-crud-resource").await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let del_result = state.delete_distribution("test-crud-resource").await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
