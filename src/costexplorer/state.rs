use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::CostexplorerError;
use super::types::*;

#[allow(dead_code)]
struct CostexplorerStateInner {
    cost_categorys: HashMap<String, StoredCostCategory>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredCostCategory {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct CostexplorerState {
    inner: Arc<Mutex<CostexplorerStateInner>>,
}

impl CostexplorerState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        CostexplorerState {
            inner: Arc::new(Mutex::new(CostexplorerStateInner {
                cost_categorys: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_cost_category(&self, req: CreateCostCategoryRequest) -> Result<CreateCostCategoryResponse, CostexplorerError> {
        let mut state = self.inner.lock().await;
        let name = req.cost_category_name.clone();
        if state.cost_categorys.contains_key(&name) {
            return Err(CostexplorerError::ResourceAlreadyExistsException(format!("CostCategory {} already exists", name)));
        }
        let arn = format!("arn:aws:ce:{}:{}:cost-categories/{}", state.region, state.account_id, name);
        state.cost_categorys.insert(name.clone(), StoredCostCategory {
            name: name.clone(),
            arn: arn.clone(),
            tags: req.tags.unwrap_or_default(),
        });
        Ok(CreateCostCategoryResponse {
            cost_category_arn: Some(arn),
            cost_category_name: Some(name),
        })
    }

    #[allow(dead_code)]
    pub async fn describe_cost_category(&self, req: DescribeCostCategoryRequest) -> Result<DescribeCostCategoryResponse, CostexplorerError> {
        let state = self.inner.lock().await;
        let name = req.cost_category_name.or_else(|| req.cost_category_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| CostexplorerError::ValidationException("Name or ARN required".to_string()))?;
        let stored = state.cost_categorys.get(&name)
            .ok_or_else(|| CostexplorerError::ResourceNotFoundException(format!("CostCategory {} not found", name)))?;
        Ok(DescribeCostCategoryResponse {
            cost_category: CostCategoryDetail {
                cost_category_name: stored.name.clone(),
                cost_category_arn: stored.arn.clone(),
                status: "ACTIVE".to_string(),
                tags: Some(stored.tags.clone()),
            },
        })
    }

    #[allow(dead_code)]
    pub async fn list_cost_categorys(&self, _req: ListCostCategorysRequest) -> Result<ListCostCategorysResponse, CostexplorerError> {
        let state = self.inner.lock().await;
        let items: Vec<CostCategoryDetail> = state.cost_categorys.values().map(|s| CostCategoryDetail {
            cost_category_name: s.name.clone(),
            cost_category_arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListCostCategorysResponse {
            cost_categorys: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_cost_category(&self, req: DeleteCostCategoryRequest) -> Result<(), CostexplorerError> {
        let mut state = self.inner.lock().await;
        let name = req.cost_category_name.or_else(|| req.cost_category_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| CostexplorerError::ValidationException("Name or ARN required".to_string()))?;
        state.cost_categorys.remove(&name)
            .ok_or_else(|| CostexplorerError::ResourceNotFoundException(format!("CostCategory {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = CostexplorerState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_cost_category() {
        let state = CostexplorerState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateCostCategoryRequest::default();
        let result = state.create_cost_category(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_describe_cost_category_not_found() {
        let state = CostexplorerState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DescribeCostCategoryRequest::default();
        let result = state.describe_cost_category(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_cost_categorys_empty() {
        let state = CostexplorerState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = ListCostCategorysRequest::default();
        let result = state.list_cost_categorys(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_cost_category_not_found() {
        let state = CostexplorerState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DeleteCostCategoryRequest::default();
        let result = state.delete_cost_category(req).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_cost_category_create_and_list() {
        let state = CostexplorerState::new("123456789012".to_string(), "us-east-1".to_string());
        let create_req = CreateCostCategoryRequest::default();
        let _created = state.create_cost_category(create_req).await.unwrap();
        let list_req = ListCostCategorysRequest::default();
        let listed = state.list_cost_categorys(list_req).await.unwrap();
        let _ = listed;
    }

    #[tokio::test]
    async fn test_cost_category_full_crud() {
        let state = CostexplorerState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateCostCategoryRequest::default();
        create_req.cost_category_name = "test-crud-resource".to_string();
        let create_result = state.create_cost_category(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let mut get_req = DescribeCostCategoryRequest::default();
        get_req.cost_category_name = Some("test-crud-resource".to_string());
        let get_result = state.describe_cost_category(get_req).await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let mut del_req = DeleteCostCategoryRequest::default();
        del_req.cost_category_name = Some("test-crud-resource".to_string());
        let del_result = state.delete_cost_category(del_req).await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
