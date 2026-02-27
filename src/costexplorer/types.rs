mod _types {
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct CreateCostCategoryRequest {
    #[serde(rename = "CostCategoryName")]
    pub cost_category_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct CreateCostCategoryResponse {
    #[serde(rename = "CostCategoryArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_category_arn: Option<String>,
    #[serde(rename = "CostCategoryName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_category_name: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DescribeCostCategoryRequest {
    #[serde(rename = "CostCategoryName")]
    pub cost_category_name: Option<String>,
    #[serde(rename = "CostCategoryArn")]
    pub cost_category_arn: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
#[allow(dead_code)]
pub struct CostCategoryDetail {
    #[serde(rename = "CostCategoryName")]
    pub cost_category_name: String,
    #[serde(rename = "CostCategoryArn")]
    pub cost_category_arn: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct DescribeCostCategoryResponse {
    #[serde(rename = "CostCategory")]
    pub cost_category: CostCategoryDetail,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct ListCostCategorysRequest {
    #[serde(rename = "MaxResults")]
    pub max_results: Option<usize>,
    #[serde(rename = "NextToken")]
    pub next_token: Option<String>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct ListCostCategorysResponse {
    #[serde(rename = "CostCategorys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_categorys: Option<Vec<CostCategoryDetail>>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DeleteCostCategoryRequest {
    #[serde(rename = "CostCategoryName")]
    pub cost_category_name: Option<String>,
    #[serde(rename = "CostCategoryArn")]
    pub cost_category_arn: Option<String>,
}

}
pub use _types::*;
