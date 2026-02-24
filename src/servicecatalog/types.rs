use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// --- Shared types ---

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct PortfolioDetail {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "ARN")]
    pub arn: String,
    #[serde(rename = "DisplayName")]
    pub display_name: String,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ProviderName")]
    pub provider_name: String,
    #[serde(rename = "CreatedTime")]
    pub created_time: f64,
}

#[derive(Debug, Serialize, Clone)]
pub struct ProductViewSummary {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "ProductId")]
    pub product_id: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Owner")]
    pub owner: String,
    #[serde(rename = "ShortDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_description: Option<String>,
    #[serde(rename = "Type")]
    pub product_type: String,
    #[serde(rename = "HasDefaultPath")]
    pub has_default_path: bool,
    #[serde(rename = "Distributor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distributor: Option<String>,
    #[serde(rename = "SupportDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_description: Option<String>,
    #[serde(rename = "SupportEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_email: Option<String>,
    #[serde(rename = "SupportUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_url: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct ProductViewDetail {
    #[serde(rename = "ProductViewSummary")]
    pub product_view_summary: ProductViewSummary,
    #[serde(rename = "ProductARN")]
    pub product_arn: String,
    #[serde(rename = "CreatedTime")]
    pub created_time: f64,
    #[serde(rename = "Status")]
    pub status: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct ProvisioningArtifactDetail {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Active")]
    pub active: bool,
    #[serde(rename = "CreatedTime")]
    pub created_time: f64,
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_type: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct ProvisionedProductDetail {
    #[serde(rename = "Arn")]
    pub arn: String,
    #[serde(rename = "CreatedTime")]
    pub created_time: f64,
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "ProductId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(rename = "ProvisioningArtifactId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifact_id: Option<String>,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "StatusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[serde(rename = "Type")]
    pub pp_type: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct RecordDetail {
    #[serde(rename = "RecordId")]
    pub record_id: String,
    #[serde(rename = "CreatedTime")]
    pub created_time: f64,
    #[serde(rename = "ProductId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(rename = "ProvisionedProductId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_product_id: Option<String>,
    #[serde(rename = "ProvisionedProductName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_product_name: Option<String>,
    #[serde(rename = "ProvisioningArtifactId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifact_id: Option<String>,
    #[serde(rename = "RecordType")]
    pub record_type: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "UpdatedTime")]
    pub updated_time: f64,
}

// --- CreatePortfolio ---

#[derive(Debug, Deserialize)]
pub struct CreatePortfolioRequest {
    #[serde(rename = "DisplayName")]
    pub display_name: String,
    #[serde(rename = "ProviderName")]
    pub provider_name: String,
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "IdempotencyToken")]
    pub idempotency_token: String,
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Serialize)]
pub struct CreatePortfolioResponse {
    #[serde(rename = "PortfolioDetail")]
    pub portfolio_detail: PortfolioDetail,
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

// --- DeletePortfolio ---

#[derive(Debug, Deserialize)]
pub struct DeletePortfolioRequest {
    #[serde(rename = "Id")]
    pub id: String,
}

// --- DescribePortfolio ---

#[derive(Debug, Deserialize)]
pub struct DescribePortfolioRequest {
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Debug, Serialize)]
pub struct DescribePortfolioResponse {
    #[serde(rename = "PortfolioDetail")]
    pub portfolio_detail: PortfolioDetail,
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
    #[serde(rename = "Budgets")]
    pub budgets: Vec<serde_json::Value>,
    #[serde(rename = "TagOptions")]
    pub tag_options: Vec<serde_json::Value>,
}

// --- ListPortfolios ---

#[derive(Debug, Deserialize, Default)]
pub struct ListPortfoliosRequest {
    #[serde(rename = "PageSize")]
    #[serde(default)]
    pub page_size: Option<usize>,
    #[serde(rename = "PageToken")]
    #[serde(default)]
    pub page_token: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ListPortfoliosResponse {
    #[serde(rename = "PortfolioDetails")]
    pub portfolio_details: Vec<PortfolioDetail>,
    #[serde(rename = "NextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

// --- UpdatePortfolio ---

#[derive(Debug, Deserialize)]
pub struct UpdatePortfolioRequest {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "DisplayName")]
    pub display_name: Option<String>,
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "ProviderName")]
    pub provider_name: Option<String>,
    #[serde(rename = "AddTags")]
    pub add_tags: Option<Vec<Tag>>,
    #[serde(rename = "RemoveTags")]
    pub remove_tags: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
pub struct UpdatePortfolioResponse {
    #[serde(rename = "PortfolioDetail")]
    pub portfolio_detail: PortfolioDetail,
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

// --- CreateProduct ---

#[derive(Debug, Deserialize)]
pub struct ProvisioningArtifactParameters {
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "Info")]
    pub info: Option<HashMap<String, String>>,
    #[serde(rename = "Type")]
    pub artifact_type: Option<String>,
    #[serde(rename = "DisableTemplateValidation")]
    pub disable_template_validation: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct CreateProductRequest {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Owner")]
    pub owner: String,
    #[serde(rename = "ProductType")]
    pub product_type: String,
    #[serde(rename = "IdempotencyToken")]
    pub idempotency_token: String,
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "Distributor")]
    pub distributor: Option<String>,
    #[serde(rename = "SupportDescription")]
    pub support_description: Option<String>,
    #[serde(rename = "SupportEmail")]
    pub support_email: Option<String>,
    #[serde(rename = "SupportUrl")]
    pub support_url: Option<String>,
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "ProvisioningArtifactParameters")]
    pub provisioning_artifact_parameters: Option<ProvisioningArtifactParameters>,
}

#[derive(Debug, Serialize)]
pub struct CreateProductResponse {
    #[serde(rename = "ProductViewDetail")]
    pub product_view_detail: ProductViewDetail,
    #[serde(rename = "ProvisioningArtifactDetail")]
    pub provisioning_artifact_detail: ProvisioningArtifactDetail,
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

// --- DeleteProduct ---

#[derive(Debug, Deserialize)]
pub struct DeleteProductRequest {
    #[serde(rename = "Id")]
    pub id: String,
}

// --- DescribeProduct ---

#[derive(Debug, Deserialize)]
pub struct DescribeProductRequest {
    #[serde(rename = "Id")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct DescribeProductResponse {
    #[serde(rename = "ProductViewSummary")]
    pub product_view_summary: ProductViewSummary,
    #[serde(rename = "ProvisioningArtifacts")]
    pub provisioning_artifacts: Vec<ProvisioningArtifactDetail>,
    #[serde(rename = "Budgets")]
    pub budgets: Vec<serde_json::Value>,
    #[serde(rename = "LaunchPaths")]
    pub launch_paths: Vec<serde_json::Value>,
}

// --- UpdateProduct ---

#[derive(Debug, Deserialize)]
pub struct UpdateProductRequest {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Owner")]
    pub owner: Option<String>,
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "Distributor")]
    pub distributor: Option<String>,
    #[serde(rename = "SupportDescription")]
    pub support_description: Option<String>,
    #[serde(rename = "SupportEmail")]
    pub support_email: Option<String>,
    #[serde(rename = "SupportUrl")]
    pub support_url: Option<String>,
    #[serde(rename = "AddTags")]
    pub add_tags: Option<Vec<Tag>>,
    #[serde(rename = "RemoveTags")]
    pub remove_tags: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
pub struct UpdateProductResponse {
    #[serde(rename = "ProductViewDetail")]
    pub product_view_detail: ProductViewDetail,
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

// --- SearchProducts ---

#[derive(Debug, Deserialize, Default)]
pub struct SearchProductsRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    pub filters: Option<HashMap<String, Vec<String>>>,
    #[serde(rename = "PageSize")]
    #[serde(default)]
    pub page_size: Option<usize>,
    #[serde(rename = "PageToken")]
    #[serde(default)]
    pub page_token: Option<String>,
    #[serde(rename = "SortBy")]
    #[serde(default)]
    pub sort_by: Option<String>,
    #[serde(rename = "SortOrder")]
    #[serde(default)]
    pub sort_order: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SearchProductsResponse {
    #[serde(rename = "ProductViewSummaries")]
    pub product_view_summaries: Vec<ProductViewSummary>,
    #[serde(rename = "NextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

// --- AssociateProductWithPortfolio ---

#[derive(Debug, Deserialize)]
pub struct AssociateProductWithPortfolioRequest {
    #[serde(rename = "ProductId")]
    pub product_id: String,
    #[serde(rename = "PortfolioId")]
    pub portfolio_id: String,
}

// --- DisassociateProductFromPortfolio ---

#[derive(Debug, Deserialize)]
pub struct DisassociateProductFromPortfolioRequest {
    #[serde(rename = "ProductId")]
    pub product_id: String,
    #[serde(rename = "PortfolioId")]
    pub portfolio_id: String,
}

// --- ProvisionProduct ---

#[derive(Debug, Deserialize)]
pub struct ProvisioningParameter {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,
}

#[derive(Debug, Deserialize)]
pub struct ProvisionProductRequest {
    #[serde(rename = "ProvisionedProductName")]
    pub provisioned_product_name: String,
    #[serde(rename = "ProvisionToken")]
    pub provision_token: String,
    #[serde(rename = "ProductId")]
    pub product_id: Option<String>,
    #[serde(rename = "ProductName")]
    pub product_name: Option<String>,
    #[serde(rename = "ProvisioningArtifactId")]
    pub provisioning_artifact_id: Option<String>,
    #[serde(rename = "ProvisioningArtifactName")]
    pub provisioning_artifact_name: Option<String>,
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "ProvisioningParameters")]
    pub provisioning_parameters: Option<Vec<ProvisioningParameter>>,
}

#[derive(Debug, Serialize)]
pub struct ProvisionProductResponse {
    #[serde(rename = "RecordDetail")]
    pub record_detail: RecordDetail,
}

// --- DescribeProvisionedProduct ---

#[derive(Debug, Deserialize)]
pub struct DescribeProvisionedProductRequest {
    #[serde(rename = "Id")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct DescribeProvisionedProductResponse {
    #[serde(rename = "ProvisionedProductDetail")]
    pub provisioned_product_detail: ProvisionedProductDetail,
    #[serde(rename = "CloudWatchDashboards")]
    pub cloud_watch_dashboards: Vec<serde_json::Value>,
}

// --- SearchProvisionedProducts ---

#[derive(Debug, Deserialize, Default)]
pub struct SearchProvisionedProductsRequest {
    #[serde(rename = "PageSize")]
    #[serde(default)]
    pub page_size: Option<usize>,
    #[serde(rename = "PageToken")]
    #[serde(default)]
    pub page_token: Option<String>,
    #[serde(rename = "SortBy")]
    #[serde(default)]
    pub sort_by: Option<String>,
    #[serde(rename = "SortOrder")]
    #[serde(default)]
    pub sort_order: Option<String>,
    #[serde(rename = "Filters")]
    #[serde(default)]
    pub filters: Option<HashMap<String, Vec<String>>>,
    #[serde(rename = "AccessLevelFilter")]
    #[serde(default)]
    pub access_level_filter: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub struct SearchProvisionedProductsResponse {
    #[serde(rename = "ProvisionedProducts")]
    pub provisioned_products: Vec<ProvisionedProductDetail>,
    #[serde(rename = "TotalResultsCount")]
    pub total_results_count: usize,
    #[serde(rename = "NextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

// --- TerminateProvisionedProduct ---

#[derive(Debug, Deserialize)]
pub struct TerminateProvisionedProductRequest {
    #[serde(rename = "TerminateToken")]
    pub terminate_token: String,
    #[serde(rename = "ProvisionedProductId")]
    pub provisioned_product_id: Option<String>,
    #[serde(rename = "ProvisionedProductName")]
    pub provisioned_product_name: Option<String>,
    #[serde(rename = "IgnoreErrors")]
    pub ignore_errors: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct TerminateProvisionedProductResponse {
    #[serde(rename = "RecordDetail")]
    pub record_detail: RecordDetail,
}
