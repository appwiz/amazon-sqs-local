use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::BudgetsError;
use super::types::*;

#[allow(dead_code)]
struct BudgetsStateInner {
    budgets: HashMap<String, StoredBudget>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredBudget {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct BudgetsState {
    inner: Arc<Mutex<BudgetsStateInner>>,
}

impl BudgetsState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        BudgetsState {
            inner: Arc::new(Mutex::new(BudgetsStateInner {
                budgets: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_budget(&self, req: CreateBudgetRequest) -> Result<CreateBudgetResponse, BudgetsError> {
        let mut state = self.inner.lock().await;
        let name = req.budget_name.clone();
        if state.budgets.contains_key(&name) {
            return Err(BudgetsError::ResourceAlreadyExistsException(format!("Budget {} already exists", name)));
        }
        let arn = format!("arn:aws:budgets:{}:{}:budgets/{}", state.region, state.account_id, name);
        state.budgets.insert(name.clone(), StoredBudget {
            name: name.clone(),
            arn: arn.clone(),
            tags: req.tags.unwrap_or_default(),
        });
        Ok(CreateBudgetResponse {
            budget_arn: Some(arn),
            budget_name: Some(name),
        })
    }

    #[allow(dead_code)]
    pub async fn describe_budget(&self, req: DescribeBudgetRequest) -> Result<DescribeBudgetResponse, BudgetsError> {
        let state = self.inner.lock().await;
        let name = req.budget_name.or_else(|| req.budget_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| BudgetsError::ValidationException("Name or ARN required".to_string()))?;
        let stored = state.budgets.get(&name)
            .ok_or_else(|| BudgetsError::ResourceNotFoundException(format!("Budget {} not found", name)))?;
        Ok(DescribeBudgetResponse {
            budget: BudgetDetail {
                budget_name: stored.name.clone(),
                budget_arn: stored.arn.clone(),
                status: "ACTIVE".to_string(),
                tags: Some(stored.tags.clone()),
            },
        })
    }

    #[allow(dead_code)]
    pub async fn list_budgets(&self, _req: ListBudgetsRequest) -> Result<ListBudgetsResponse, BudgetsError> {
        let state = self.inner.lock().await;
        let items: Vec<BudgetDetail> = state.budgets.values().map(|s| BudgetDetail {
            budget_name: s.name.clone(),
            budget_arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListBudgetsResponse {
            budgets: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_budget(&self, req: DeleteBudgetRequest) -> Result<(), BudgetsError> {
        let mut state = self.inner.lock().await;
        let name = req.budget_name.or_else(|| req.budget_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| BudgetsError::ValidationException("Name or ARN required".to_string()))?;
        state.budgets.remove(&name)
            .ok_or_else(|| BudgetsError::ResourceNotFoundException(format!("Budget {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = BudgetsState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_budget() {
        let state = BudgetsState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateBudgetRequest::default();
        let result = state.create_budget(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_describe_budget_not_found() {
        let state = BudgetsState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DescribeBudgetRequest::default();
        let result = state.describe_budget(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_budgets_empty() {
        let state = BudgetsState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = ListBudgetsRequest::default();
        let result = state.list_budgets(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_budget_not_found() {
        let state = BudgetsState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DeleteBudgetRequest::default();
        let result = state.delete_budget(req).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_budget_create_and_list() {
        let state = BudgetsState::new("123456789012".to_string(), "us-east-1".to_string());
        let create_req = CreateBudgetRequest::default();
        let _created = state.create_budget(create_req).await.unwrap();
        let list_req = ListBudgetsRequest::default();
        let listed = state.list_budgets(list_req).await.unwrap();
        let _ = listed;
    }

    #[tokio::test]
    async fn test_budget_full_crud() {
        let state = BudgetsState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateBudgetRequest::default();
        create_req.budget_name = "test-crud-resource".to_string();
        let create_result = state.create_budget(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let mut get_req = DescribeBudgetRequest::default();
        get_req.budget_name = Some("test-crud-resource".to_string());
        let get_result = state.describe_budget(get_req).await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let mut del_req = DeleteBudgetRequest::default();
        del_req.budget_name = Some("test-crud-resource".to_string());
        let del_result = state.delete_budget(del_req).await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
