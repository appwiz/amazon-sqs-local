use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::DevopsguruError;
use super::types::*;

#[allow(dead_code)]
struct DevopsguruStateInner {
    insights: HashMap<String, StoredInsight>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredInsight {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct DevopsguruState {
    inner: Arc<Mutex<DevopsguruStateInner>>,
}

impl DevopsguruState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        DevopsguruState {
            inner: Arc::new(Mutex::new(DevopsguruStateInner {
                insights: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_insight(&self, req: CreateInsightRequest) -> Result<InsightDetail, DevopsguruError> {
        let mut state = self.inner.lock().await;
        let name = req.name.or(req.name_pascal).unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        if state.insights.contains_key(&name) {
            return Err(DevopsguruError::ResourceAlreadyExistsException(format!("Insight {} already exists", name)));
        }
        let arn = format!("arn:aws:devops-guru:{}:{}:insights/{}", state.region, state.account_id, name);
        let detail = InsightDetail {
            name: name.clone(),
            arn: arn.clone(),
            status: "ACTIVE".to_string(),
            tags: req.tags.clone(),
        };
        state.insights.insert(name, StoredInsight {
            name: detail.name.clone(),
            arn,
            tags: req.tags.unwrap_or_default(),
        });
        Ok(detail)
    }

    #[allow(dead_code)]
    pub async fn get_insight(&self, name: &str) -> Result<InsightDetail, DevopsguruError> {
        let state = self.inner.lock().await;
        let stored = state.insights.get(name)
            .ok_or_else(|| DevopsguruError::ResourceNotFoundException(format!("Insight {} not found", name)))?;
        Ok(InsightDetail {
            name: stored.name.clone(),
            arn: stored.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(stored.tags.clone()),
        })
    }

    #[allow(dead_code)]
    pub async fn list_insights(&self) -> Result<ListInsightsResponse, DevopsguruError> {
        let state = self.inner.lock().await;
        let items: Vec<InsightDetail> = state.insights.values().map(|s| InsightDetail {
            name: s.name.clone(),
            arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListInsightsResponse {
            items: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_insight(&self, name: &str) -> Result<(), DevopsguruError> {
        let mut state = self.inner.lock().await;
        state.insights.remove(name)
            .ok_or_else(|| DevopsguruError::ResourceNotFoundException(format!("Insight {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = DevopsguruState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_insight() {
        let state = DevopsguruState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateInsightRequest::default();
        let result = state.create_insight(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_get_insight_not_found() {
        let state = DevopsguruState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.get_insight("nonexistent").await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_insights() {
        let state = DevopsguruState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.list_insights().await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_insight_not_found() {
        let state = DevopsguruState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.delete_insight("nonexistent").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_insight_full_crud() {
        let state = DevopsguruState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateInsightRequest::default();
        create_req.name = Some("test-crud-resource".to_string());
        let create_result = state.create_insight(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let get_result = state.get_insight("test-crud-resource").await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let del_result = state.delete_insight("test-crud-resource").await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
