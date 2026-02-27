mod _types {
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct CreateBudgetRequest {
    #[serde(rename = "BudgetName")]
    pub budget_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct CreateBudgetResponse {
    #[serde(rename = "BudgetArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget_arn: Option<String>,
    #[serde(rename = "BudgetName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget_name: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DescribeBudgetRequest {
    #[serde(rename = "BudgetName")]
    pub budget_name: Option<String>,
    #[serde(rename = "BudgetArn")]
    pub budget_arn: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
#[allow(dead_code)]
pub struct BudgetDetail {
    #[serde(rename = "BudgetName")]
    pub budget_name: String,
    #[serde(rename = "BudgetArn")]
    pub budget_arn: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct DescribeBudgetResponse {
    #[serde(rename = "Budget")]
    pub budget: BudgetDetail,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct ListBudgetsRequest {
    #[serde(rename = "MaxResults")]
    pub max_results: Option<usize>,
    #[serde(rename = "NextToken")]
    pub next_token: Option<String>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct ListBudgetsResponse {
    #[serde(rename = "Budgets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budgets: Option<Vec<BudgetDetail>>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DeleteBudgetRequest {
    #[serde(rename = "BudgetName")]
    pub budget_name: Option<String>,
    #[serde(rename = "BudgetArn")]
    pub budget_arn: Option<String>,
}

}
pub use _types::*;
