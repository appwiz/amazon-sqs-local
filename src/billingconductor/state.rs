use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::BillingconductorError;
use super::types::*;

#[allow(dead_code)]
struct BillingconductorStateInner {
    pricing_plans: HashMap<String, StoredPricingPlan>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredPricingPlan {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct BillingconductorState {
    inner: Arc<Mutex<BillingconductorStateInner>>,
}

impl BillingconductorState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        BillingconductorState {
            inner: Arc::new(Mutex::new(BillingconductorStateInner {
                pricing_plans: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_pricing_plan(&self, req: CreatePricingPlanRequest) -> Result<CreatePricingPlanResponse, BillingconductorError> {
        let mut state = self.inner.lock().await;
        let name = req.pricing_plan_name.clone();
        if state.pricing_plans.contains_key(&name) {
            return Err(BillingconductorError::ResourceAlreadyExistsException(format!("PricingPlan {} already exists", name)));
        }
        let arn = format!("arn:aws:billingconductor:{}:{}:pricing-plans/{}", state.region, state.account_id, name);
        state.pricing_plans.insert(name.clone(), StoredPricingPlan {
            name: name.clone(),
            arn: arn.clone(),
            tags: req.tags.unwrap_or_default(),
        });
        Ok(CreatePricingPlanResponse {
            pricing_plan_arn: Some(arn),
            pricing_plan_name: Some(name),
        })
    }

    #[allow(dead_code)]
    pub async fn describe_pricing_plan(&self, req: DescribePricingPlanRequest) -> Result<DescribePricingPlanResponse, BillingconductorError> {
        let state = self.inner.lock().await;
        let name = req.pricing_plan_name.or_else(|| req.pricing_plan_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| BillingconductorError::ValidationException("Name or ARN required".to_string()))?;
        let stored = state.pricing_plans.get(&name)
            .ok_or_else(|| BillingconductorError::ResourceNotFoundException(format!("PricingPlan {} not found", name)))?;
        Ok(DescribePricingPlanResponse {
            pricing_plan: PricingPlanDetail {
                pricing_plan_name: stored.name.clone(),
                pricing_plan_arn: stored.arn.clone(),
                status: "ACTIVE".to_string(),
                tags: Some(stored.tags.clone()),
            },
        })
    }

    #[allow(dead_code)]
    pub async fn list_pricing_plans(&self, _req: ListPricingPlansRequest) -> Result<ListPricingPlansResponse, BillingconductorError> {
        let state = self.inner.lock().await;
        let items: Vec<PricingPlanDetail> = state.pricing_plans.values().map(|s| PricingPlanDetail {
            pricing_plan_name: s.name.clone(),
            pricing_plan_arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListPricingPlansResponse {
            pricing_plans: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_pricing_plan(&self, req: DeletePricingPlanRequest) -> Result<(), BillingconductorError> {
        let mut state = self.inner.lock().await;
        let name = req.pricing_plan_name.or_else(|| req.pricing_plan_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| BillingconductorError::ValidationException("Name or ARN required".to_string()))?;
        state.pricing_plans.remove(&name)
            .ok_or_else(|| BillingconductorError::ResourceNotFoundException(format!("PricingPlan {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = BillingconductorState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_pricing_plan() {
        let state = BillingconductorState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreatePricingPlanRequest::default();
        let result = state.create_pricing_plan(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_describe_pricing_plan_not_found() {
        let state = BillingconductorState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DescribePricingPlanRequest::default();
        let result = state.describe_pricing_plan(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_pricing_plans_empty() {
        let state = BillingconductorState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = ListPricingPlansRequest::default();
        let result = state.list_pricing_plans(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_pricing_plan_not_found() {
        let state = BillingconductorState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DeletePricingPlanRequest::default();
        let result = state.delete_pricing_plan(req).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_pricing_plan_create_and_list() {
        let state = BillingconductorState::new("123456789012".to_string(), "us-east-1".to_string());
        let create_req = CreatePricingPlanRequest::default();
        let _created = state.create_pricing_plan(create_req).await.unwrap();
        let list_req = ListPricingPlansRequest::default();
        let listed = state.list_pricing_plans(list_req).await.unwrap();
        let _ = listed;
    }

    #[tokio::test]
    async fn test_pricing_plan_full_crud() {
        let state = BillingconductorState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreatePricingPlanRequest::default();
        create_req.pricing_plan_name = "test-crud-resource".to_string();
        let create_result = state.create_pricing_plan(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let mut get_req = DescribePricingPlanRequest::default();
        get_req.pricing_plan_name = Some("test-crud-resource".to_string());
        let get_result = state.describe_pricing_plan(get_req).await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let mut del_req = DeletePricingPlanRequest::default();
        del_req.pricing_plan_name = Some("test-crud-resource".to_string());
        let del_result = state.delete_pricing_plan(del_req).await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
