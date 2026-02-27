mod _types {
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct CreatePricingPlanRequest {
    #[serde(rename = "PricingPlanName")]
    pub pricing_plan_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct CreatePricingPlanResponse {
    #[serde(rename = "PricingPlanArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pricing_plan_arn: Option<String>,
    #[serde(rename = "PricingPlanName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pricing_plan_name: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DescribePricingPlanRequest {
    #[serde(rename = "PricingPlanName")]
    pub pricing_plan_name: Option<String>,
    #[serde(rename = "PricingPlanArn")]
    pub pricing_plan_arn: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
#[allow(dead_code)]
pub struct PricingPlanDetail {
    #[serde(rename = "PricingPlanName")]
    pub pricing_plan_name: String,
    #[serde(rename = "PricingPlanArn")]
    pub pricing_plan_arn: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct DescribePricingPlanResponse {
    #[serde(rename = "PricingPlan")]
    pub pricing_plan: PricingPlanDetail,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct ListPricingPlansRequest {
    #[serde(rename = "MaxResults")]
    pub max_results: Option<usize>,
    #[serde(rename = "NextToken")]
    pub next_token: Option<String>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct ListPricingPlansResponse {
    #[serde(rename = "PricingPlans")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pricing_plans: Option<Vec<PricingPlanDetail>>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DeletePricingPlanRequest {
    #[serde(rename = "PricingPlanName")]
    pub pricing_plan_name: Option<String>,
    #[serde(rename = "PricingPlanArn")]
    pub pricing_plan_arn: Option<String>,
}

}
pub use _types::*;
