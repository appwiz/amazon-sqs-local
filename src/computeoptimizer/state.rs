use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::ComputeoptimizerError;
use super::types::*;

#[allow(dead_code)]
struct ComputeoptimizerStateInner {
    recommendations: HashMap<String, StoredRecommendation>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredRecommendation {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct ComputeoptimizerState {
    inner: Arc<Mutex<ComputeoptimizerStateInner>>,
}

impl ComputeoptimizerState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        ComputeoptimizerState {
            inner: Arc::new(Mutex::new(ComputeoptimizerStateInner {
                recommendations: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_recommendation(&self, req: CreateRecommendationRequest) -> Result<CreateRecommendationResponse, ComputeoptimizerError> {
        let mut state = self.inner.lock().await;
        let name = req.recommendation_name.clone();
        if state.recommendations.contains_key(&name) {
            return Err(ComputeoptimizerError::ResourceAlreadyExistsException(format!("Recommendation {} already exists", name)));
        }
        let arn = format!("arn:aws:compute-optimizer:{}:{}:recommendations/{}", state.region, state.account_id, name);
        state.recommendations.insert(name.clone(), StoredRecommendation {
            name: name.clone(),
            arn: arn.clone(),
            tags: req.tags.unwrap_or_default(),
        });
        Ok(CreateRecommendationResponse {
            recommendation_arn: Some(arn),
            recommendation_name: Some(name),
        })
    }

    #[allow(dead_code)]
    pub async fn describe_recommendation(&self, req: DescribeRecommendationRequest) -> Result<DescribeRecommendationResponse, ComputeoptimizerError> {
        let state = self.inner.lock().await;
        let name = req.recommendation_name.or_else(|| req.recommendation_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| ComputeoptimizerError::ValidationException("Name or ARN required".to_string()))?;
        let stored = state.recommendations.get(&name)
            .ok_or_else(|| ComputeoptimizerError::ResourceNotFoundException(format!("Recommendation {} not found", name)))?;
        Ok(DescribeRecommendationResponse {
            recommendation: RecommendationDetail {
                recommendation_name: stored.name.clone(),
                recommendation_arn: stored.arn.clone(),
                status: "ACTIVE".to_string(),
                tags: Some(stored.tags.clone()),
            },
        })
    }

    #[allow(dead_code)]
    pub async fn list_recommendations(&self, _req: ListRecommendationsRequest) -> Result<ListRecommendationsResponse, ComputeoptimizerError> {
        let state = self.inner.lock().await;
        let items: Vec<RecommendationDetail> = state.recommendations.values().map(|s| RecommendationDetail {
            recommendation_name: s.name.clone(),
            recommendation_arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListRecommendationsResponse {
            recommendations: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_recommendation(&self, req: DeleteRecommendationRequest) -> Result<(), ComputeoptimizerError> {
        let mut state = self.inner.lock().await;
        let name = req.recommendation_name.or_else(|| req.recommendation_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| ComputeoptimizerError::ValidationException("Name or ARN required".to_string()))?;
        state.recommendations.remove(&name)
            .ok_or_else(|| ComputeoptimizerError::ResourceNotFoundException(format!("Recommendation {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = ComputeoptimizerState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_recommendation() {
        let state = ComputeoptimizerState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateRecommendationRequest::default();
        let result = state.create_recommendation(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_describe_recommendation_not_found() {
        let state = ComputeoptimizerState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DescribeRecommendationRequest::default();
        let result = state.describe_recommendation(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_recommendations_empty() {
        let state = ComputeoptimizerState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = ListRecommendationsRequest::default();
        let result = state.list_recommendations(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_recommendation_not_found() {
        let state = ComputeoptimizerState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DeleteRecommendationRequest::default();
        let result = state.delete_recommendation(req).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_recommendation_create_and_list() {
        let state = ComputeoptimizerState::new("123456789012".to_string(), "us-east-1".to_string());
        let create_req = CreateRecommendationRequest::default();
        let _created = state.create_recommendation(create_req).await.unwrap();
        let list_req = ListRecommendationsRequest::default();
        let listed = state.list_recommendations(list_req).await.unwrap();
        let _ = listed;
    }

    #[tokio::test]
    async fn test_recommendation_full_crud() {
        let state = ComputeoptimizerState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateRecommendationRequest::default();
        create_req.recommendation_name = "test-crud-resource".to_string();
        let create_result = state.create_recommendation(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let mut get_req = DescribeRecommendationRequest::default();
        get_req.recommendation_name = Some("test-crud-resource".to_string());
        let get_result = state.describe_recommendation(get_req).await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let mut del_req = DeleteRecommendationRequest::default();
        del_req.recommendation_name = Some("test-crud-resource".to_string());
        let del_result = state.delete_recommendation(del_req).await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
