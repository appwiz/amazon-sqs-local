use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use tokio::sync::Mutex;
use uuid::Uuid;

use super::error::ServiceCatalogError;
use super::types::*;

struct Portfolio {
    id: String,
    arn: String,
    display_name: String,
    description: Option<String>,
    provider_name: String,
    tags: HashMap<String, String>,
    created_time: f64,
}

struct Product {
    id: String,
    arn: String,
    view_id: String,
    name: String,
    owner: String,
    product_type: String,
    description: Option<String>,
    distributor: Option<String>,
    support_description: Option<String>,
    support_email: Option<String>,
    support_url: Option<String>,
    tags: HashMap<String, String>,
    created_time: f64,
    provisioning_artifact: ProvisioningArtifactDetail,
}

struct ProvisionedProduct {
    id: String,
    arn: String,
    name: String,
    product_id: Option<String>,
    provisioning_artifact_id: Option<String>,
    status: String,
    status_message: Option<String>,
    pp_type: String,
    created_time: f64,
}

struct ServiceCatalogStateInner {
    portfolios: HashMap<String, Portfolio>,
    products: HashMap<String, Product>,
    provisioned_products: HashMap<String, ProvisionedProduct>,
    portfolio_product_associations: HashSet<(String, String)>,
    account_id: String,
    region: String,
}

pub struct ServiceCatalogState {
    inner: Arc<Mutex<ServiceCatalogStateInner>>,
}

impl ServiceCatalogState {
    pub fn new(account_id: String, region: String) -> Self {
        ServiceCatalogState {
            inner: Arc::new(Mutex::new(ServiceCatalogStateInner {
                portfolios: HashMap::new(),
                products: HashMap::new(),
                provisioned_products: HashMap::new(),
                portfolio_product_associations: HashSet::new(),
                account_id,
                region,
            })),
        }
    }

    fn now() -> f64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64()
    }

    fn make_tags_vec(tags: &HashMap<String, String>) -> Vec<Tag> {
        tags.iter()
            .map(|(k, v)| Tag {
                key: k.clone(),
                value: v.clone(),
            })
            .collect()
    }

    fn parse_tags(input: Option<Vec<Tag>>) -> HashMap<String, String> {
        let mut map = HashMap::new();
        if let Some(tags) = input {
            for tag in tags {
                map.insert(tag.key, tag.value);
            }
        }
        map
    }

    fn build_product_view_summary(product: &Product) -> ProductViewSummary {
        ProductViewSummary {
            id: product.view_id.clone(),
            product_id: product.id.clone(),
            name: product.name.clone(),
            owner: product.owner.clone(),
            short_description: product.description.clone(),
            product_type: product.product_type.clone(),
            has_default_path: false,
            distributor: product.distributor.clone(),
            support_description: product.support_description.clone(),
            support_email: product.support_email.clone(),
            support_url: product.support_url.clone(),
        }
    }

    fn build_product_view_detail(product: &Product) -> ProductViewDetail {
        ProductViewDetail {
            product_view_summary: Self::build_product_view_summary(product),
            product_arn: product.arn.clone(),
            created_time: product.created_time,
            status: "AVAILABLE".to_string(),
        }
    }

    // --- Portfolio operations ---

    pub async fn create_portfolio(
        &self,
        req: CreatePortfolioRequest,
    ) -> Result<CreatePortfolioResponse, ServiceCatalogError> {
        let mut state = self.inner.lock().await;
        let id = format!("port-{}", Uuid::new_v4());
        let arn = format!(
            "arn:aws:catalog:{}:{}:portfolio/{}",
            state.region, state.account_id, id
        );
        let now = Self::now();
        let tags = Self::parse_tags(req.tags);

        let portfolio = Portfolio {
            id: id.clone(),
            arn: arn.clone(),
            display_name: req.display_name.clone(),
            description: req.description.clone(),
            provider_name: req.provider_name.clone(),
            tags: tags.clone(),
            created_time: now,
        };

        let detail = PortfolioDetail {
            id: id.clone(),
            arn,
            display_name: req.display_name,
            description: req.description,
            provider_name: req.provider_name,
            created_time: now,
        };

        state.portfolios.insert(id, portfolio);

        Ok(CreatePortfolioResponse {
            portfolio_detail: detail,
            tags: Self::make_tags_vec(&tags),
        })
    }

    pub async fn delete_portfolio(
        &self,
        req: DeletePortfolioRequest,
    ) -> Result<(), ServiceCatalogError> {
        let mut state = self.inner.lock().await;
        if !state.portfolios.contains_key(&req.id) {
            return Err(ServiceCatalogError::ResourceNotFoundException(format!(
                "Portfolio {} not found",
                req.id
            )));
        }
        // Check if any associations exist
        let has_associations = state
            .portfolio_product_associations
            .iter()
            .any(|(pid, _)| pid == &req.id);
        if has_associations {
            return Err(ServiceCatalogError::ResourceInUseException(format!(
                "Portfolio {} has associated products",
                req.id
            )));
        }
        state.portfolios.remove(&req.id);
        Ok(())
    }

    pub async fn describe_portfolio(
        &self,
        req: DescribePortfolioRequest,
    ) -> Result<DescribePortfolioResponse, ServiceCatalogError> {
        let state = self.inner.lock().await;
        let portfolio = state.portfolios.get(&req.id).ok_or_else(|| {
            ServiceCatalogError::ResourceNotFoundException(format!(
                "Portfolio {} not found",
                req.id
            ))
        })?;

        Ok(DescribePortfolioResponse {
            portfolio_detail: PortfolioDetail {
                id: portfolio.id.clone(),
                arn: portfolio.arn.clone(),
                display_name: portfolio.display_name.clone(),
                description: portfolio.description.clone(),
                provider_name: portfolio.provider_name.clone(),
                created_time: portfolio.created_time,
            },
            tags: Self::make_tags_vec(&portfolio.tags),
            budgets: vec![],
            tag_options: vec![],
        })
    }

    pub async fn list_portfolios(
        &self,
        req: ListPortfoliosRequest,
    ) -> Result<ListPortfoliosResponse, ServiceCatalogError> {
        let state = self.inner.lock().await;
        let mut details: Vec<PortfolioDetail> = state
            .portfolios
            .values()
            .map(|p| PortfolioDetail {
                id: p.id.clone(),
                arn: p.arn.clone(),
                display_name: p.display_name.clone(),
                description: p.description.clone(),
                provider_name: p.provider_name.clone(),
                created_time: p.created_time,
            })
            .collect();
        details.sort_by(|a, b| a.display_name.cmp(&b.display_name));

        let limit = req.page_size.unwrap_or(20);
        let truncated = details.len() > limit;
        details.truncate(limit);

        Ok(ListPortfoliosResponse {
            portfolio_details: details,
            next_page_token: if truncated {
                Some("next".to_string())
            } else {
                None
            },
        })
    }

    pub async fn update_portfolio(
        &self,
        req: UpdatePortfolioRequest,
    ) -> Result<UpdatePortfolioResponse, ServiceCatalogError> {
        let mut state = self.inner.lock().await;
        let portfolio = state.portfolios.get_mut(&req.id).ok_or_else(|| {
            ServiceCatalogError::ResourceNotFoundException(format!(
                "Portfolio {} not found",
                req.id
            ))
        })?;

        if let Some(name) = req.display_name {
            portfolio.display_name = name;
        }
        if let Some(desc) = req.description {
            portfolio.description = Some(desc);
        }
        if let Some(provider) = req.provider_name {
            portfolio.provider_name = provider;
        }
        if let Some(add_tags) = req.add_tags {
            for tag in add_tags {
                portfolio.tags.insert(tag.key, tag.value);
            }
        }
        if let Some(remove_tags) = req.remove_tags {
            for key in remove_tags {
                portfolio.tags.remove(&key);
            }
        }

        let detail = PortfolioDetail {
            id: portfolio.id.clone(),
            arn: portfolio.arn.clone(),
            display_name: portfolio.display_name.clone(),
            description: portfolio.description.clone(),
            provider_name: portfolio.provider_name.clone(),
            created_time: portfolio.created_time,
        };
        let tags = Self::make_tags_vec(&portfolio.tags);

        Ok(UpdatePortfolioResponse {
            portfolio_detail: detail,
            tags,
        })
    }

    // --- Product operations ---

    pub async fn create_product(
        &self,
        req: CreateProductRequest,
    ) -> Result<CreateProductResponse, ServiceCatalogError> {
        let mut state = self.inner.lock().await;
        let product_id = format!("prod-{}", Uuid::new_v4());
        let view_id = format!("prodview-{}", Uuid::new_v4());
        let artifact_id = format!("pa-{}", Uuid::new_v4());
        let arn = format!(
            "arn:aws:catalog:{}:{}:product/{}",
            state.region, state.account_id, product_id
        );
        let now = Self::now();
        let tags = Self::parse_tags(req.tags);

        let artifact_detail = ProvisioningArtifactDetail {
            id: artifact_id.clone(),
            name: req
                .provisioning_artifact_parameters
                .as_ref()
                .and_then(|p| p.name.clone()),
            description: req
                .provisioning_artifact_parameters
                .as_ref()
                .and_then(|p| p.description.clone()),
            active: true,
            created_time: now,
            artifact_type: req
                .provisioning_artifact_parameters
                .as_ref()
                .and_then(|p| p.artifact_type.clone()),
        };

        let product = Product {
            id: product_id.clone(),
            arn: arn.clone(),
            view_id: view_id.clone(),
            name: req.name.clone(),
            owner: req.owner.clone(),
            product_type: req.product_type.clone(),
            description: req.description.clone(),
            distributor: req.distributor.clone(),
            support_description: req.support_description.clone(),
            support_email: req.support_email.clone(),
            support_url: req.support_url.clone(),
            tags: tags.clone(),
            created_time: now,
            provisioning_artifact: artifact_detail.clone(),
        };

        let view_detail = Self::build_product_view_detail(&product);

        state.products.insert(product_id, product);

        Ok(CreateProductResponse {
            product_view_detail: view_detail,
            provisioning_artifact_detail: artifact_detail,
            tags: Self::make_tags_vec(&tags),
        })
    }

    pub async fn delete_product(
        &self,
        req: DeleteProductRequest,
    ) -> Result<(), ServiceCatalogError> {
        let mut state = self.inner.lock().await;
        if !state.products.contains_key(&req.id) {
            return Err(ServiceCatalogError::ResourceNotFoundException(format!(
                "Product {} not found",
                req.id
            )));
        }
        // Remove any associations
        state
            .portfolio_product_associations
            .retain(|(_, prod_id)| prod_id != &req.id);
        state.products.remove(&req.id);
        Ok(())
    }

    pub async fn describe_product(
        &self,
        req: DescribeProductRequest,
    ) -> Result<DescribeProductResponse, ServiceCatalogError> {
        let state = self.inner.lock().await;
        let product = if let Some(ref id) = req.id {
            state.products.get(id).ok_or_else(|| {
                ServiceCatalogError::ResourceNotFoundException(format!(
                    "Product {} not found",
                    id
                ))
            })?
        } else if let Some(ref name) = req.name {
            state
                .products
                .values()
                .find(|p| p.name == *name)
                .ok_or_else(|| {
                    ServiceCatalogError::ResourceNotFoundException(format!(
                        "Product with name {} not found",
                        name
                    ))
                })?
        } else {
            return Err(ServiceCatalogError::InvalidParametersException(
                "Either Id or Name must be provided".to_string(),
            ));
        };

        Ok(DescribeProductResponse {
            product_view_summary: Self::build_product_view_summary(product),
            provisioning_artifacts: vec![product.provisioning_artifact.clone()],
            budgets: vec![],
            launch_paths: vec![],
        })
    }

    pub async fn update_product(
        &self,
        req: UpdateProductRequest,
    ) -> Result<UpdateProductResponse, ServiceCatalogError> {
        let mut state = self.inner.lock().await;
        let product = state.products.get_mut(&req.id).ok_or_else(|| {
            ServiceCatalogError::ResourceNotFoundException(format!(
                "Product {} not found",
                req.id
            ))
        })?;

        if let Some(name) = req.name {
            product.name = name;
        }
        if let Some(owner) = req.owner {
            product.owner = owner;
        }
        if let Some(desc) = req.description {
            product.description = Some(desc);
        }
        if let Some(dist) = req.distributor {
            product.distributor = Some(dist);
        }
        if let Some(sd) = req.support_description {
            product.support_description = Some(sd);
        }
        if let Some(se) = req.support_email {
            product.support_email = Some(se);
        }
        if let Some(su) = req.support_url {
            product.support_url = Some(su);
        }
        if let Some(add_tags) = req.add_tags {
            for tag in add_tags {
                product.tags.insert(tag.key, tag.value);
            }
        }
        if let Some(remove_tags) = req.remove_tags {
            for key in remove_tags {
                product.tags.remove(&key);
            }
        }

        let view_detail = Self::build_product_view_detail(product);
        let tags = Self::make_tags_vec(&product.tags);

        Ok(UpdateProductResponse {
            product_view_detail: view_detail,
            tags,
        })
    }

    pub async fn search_products(
        &self,
        req: SearchProductsRequest,
    ) -> Result<SearchProductsResponse, ServiceCatalogError> {
        let state = self.inner.lock().await;
        let mut summaries: Vec<ProductViewSummary> = state
            .products
            .values()
            .filter(|p| {
                if let Some(ref filters) = req.filters {
                    if let Some(name_filters) = filters.get("FullTextSearch") {
                        return name_filters
                            .iter()
                            .any(|f| p.name.to_lowercase().contains(&f.to_lowercase()));
                    }
                }
                true
            })
            .map(|p| Self::build_product_view_summary(p))
            .collect();

        summaries.sort_by(|a, b| a.name.cmp(&b.name));

        let limit = req.page_size.unwrap_or(20);
        let truncated = summaries.len() > limit;
        summaries.truncate(limit);

        Ok(SearchProductsResponse {
            product_view_summaries: summaries,
            next_page_token: if truncated {
                Some("next".to_string())
            } else {
                None
            },
        })
    }

    // --- Association operations ---

    pub async fn associate_product_with_portfolio(
        &self,
        req: AssociateProductWithPortfolioRequest,
    ) -> Result<(), ServiceCatalogError> {
        let mut state = self.inner.lock().await;
        if !state.portfolios.contains_key(&req.portfolio_id) {
            return Err(ServiceCatalogError::ResourceNotFoundException(format!(
                "Portfolio {} not found",
                req.portfolio_id
            )));
        }
        if !state.products.contains_key(&req.product_id) {
            return Err(ServiceCatalogError::ResourceNotFoundException(format!(
                "Product {} not found",
                req.product_id
            )));
        }
        let pair = (req.portfolio_id, req.product_id);
        if state.portfolio_product_associations.contains(&pair) {
            return Err(ServiceCatalogError::DuplicateResourceException(
                "Association already exists".to_string(),
            ));
        }
        state.portfolio_product_associations.insert(pair);
        Ok(())
    }

    pub async fn disassociate_product_from_portfolio(
        &self,
        req: DisassociateProductFromPortfolioRequest,
    ) -> Result<(), ServiceCatalogError> {
        let mut state = self.inner.lock().await;
        let pair = (req.portfolio_id.clone(), req.product_id.clone());
        if !state.portfolio_product_associations.remove(&pair) {
            return Err(ServiceCatalogError::ResourceNotFoundException(format!(
                "Association between portfolio {} and product {} not found",
                req.portfolio_id, req.product_id
            )));
        }
        Ok(())
    }

    // --- Provisioned product operations ---

    pub async fn provision_product(
        &self,
        req: ProvisionProductRequest,
    ) -> Result<ProvisionProductResponse, ServiceCatalogError> {
        let mut state = self.inner.lock().await;

        // Resolve product ID
        let product_id = if let Some(ref id) = req.product_id {
            if !state.products.contains_key(id) {
                return Err(ServiceCatalogError::ResourceNotFoundException(format!(
                    "Product {} not found",
                    id
                )));
            }
            Some(id.clone())
        } else if let Some(ref name) = req.product_name {
            let found = state.products.values().find(|p| p.name == *name);
            match found {
                Some(p) => Some(p.id.clone()),
                None => {
                    return Err(ServiceCatalogError::ResourceNotFoundException(format!(
                        "Product with name {} not found",
                        name
                    )))
                }
            }
        } else {
            None
        };

        let pp_id = format!("pp-{}", Uuid::new_v4());
        let record_id = format!("rec-{}", Uuid::new_v4());
        let now = Self::now();
        let pp_arn = format!(
            "arn:aws:servicecatalog:{}:{}:stack/{}/{}",
            state.region, state.account_id, req.provisioned_product_name, pp_id
        );

        let provisioned_product = ProvisionedProduct {
            id: pp_id.clone(),
            arn: pp_arn.clone(),
            name: req.provisioned_product_name.clone(),
            product_id: product_id.clone(),
            provisioning_artifact_id: req.provisioning_artifact_id.clone(),
            status: "AVAILABLE".to_string(),
            status_message: None,
            pp_type: "CFN_STACK".to_string(),
            created_time: now,
        };

        state
            .provisioned_products
            .insert(pp_id.clone(), provisioned_product);

        let record_detail = RecordDetail {
            record_id,
            created_time: now,
            product_id,
            provisioned_product_id: Some(pp_id),
            provisioned_product_name: Some(req.provisioned_product_name),
            provisioning_artifact_id: req.provisioning_artifact_id,
            record_type: "PROVISION_PRODUCT".to_string(),
            status: "SUCCEEDED".to_string(),
            updated_time: now,
        };

        Ok(ProvisionProductResponse { record_detail })
    }

    pub async fn describe_provisioned_product(
        &self,
        req: DescribeProvisionedProductRequest,
    ) -> Result<DescribeProvisionedProductResponse, ServiceCatalogError> {
        let state = self.inner.lock().await;
        let pp = if let Some(ref id) = req.id {
            state.provisioned_products.get(id).ok_or_else(|| {
                ServiceCatalogError::ResourceNotFoundException(format!(
                    "Provisioned product {} not found",
                    id
                ))
            })?
        } else if let Some(ref name) = req.name {
            state
                .provisioned_products
                .values()
                .find(|p| p.name == *name)
                .ok_or_else(|| {
                    ServiceCatalogError::ResourceNotFoundException(format!(
                        "Provisioned product with name {} not found",
                        name
                    ))
                })?
        } else {
            return Err(ServiceCatalogError::InvalidParametersException(
                "Either Id or Name must be provided".to_string(),
            ));
        };

        Ok(DescribeProvisionedProductResponse {
            provisioned_product_detail: ProvisionedProductDetail {
                arn: pp.arn.clone(),
                created_time: pp.created_time,
                id: pp.id.clone(),
                name: pp.name.clone(),
                product_id: pp.product_id.clone(),
                provisioning_artifact_id: pp.provisioning_artifact_id.clone(),
                status: pp.status.clone(),
                status_message: pp.status_message.clone(),
                pp_type: pp.pp_type.clone(),
            },
            cloud_watch_dashboards: vec![],
        })
    }

    pub async fn search_provisioned_products(
        &self,
        req: SearchProvisionedProductsRequest,
    ) -> Result<SearchProvisionedProductsResponse, ServiceCatalogError> {
        let state = self.inner.lock().await;
        let mut products: Vec<ProvisionedProductDetail> = state
            .provisioned_products
            .values()
            .map(|pp| ProvisionedProductDetail {
                arn: pp.arn.clone(),
                created_time: pp.created_time,
                id: pp.id.clone(),
                name: pp.name.clone(),
                product_id: pp.product_id.clone(),
                provisioning_artifact_id: pp.provisioning_artifact_id.clone(),
                status: pp.status.clone(),
                status_message: pp.status_message.clone(),
                pp_type: pp.pp_type.clone(),
            })
            .collect();

        products.sort_by(|a, b| a.name.cmp(&b.name));

        let total = products.len();
        let limit = req.page_size.unwrap_or(20);
        let truncated = products.len() > limit;
        products.truncate(limit);

        Ok(SearchProvisionedProductsResponse {
            provisioned_products: products,
            total_results_count: total,
            next_page_token: if truncated {
                Some("next".to_string())
            } else {
                None
            },
        })
    }

    pub async fn terminate_provisioned_product(
        &self,
        req: TerminateProvisionedProductRequest,
    ) -> Result<TerminateProvisionedProductResponse, ServiceCatalogError> {
        let mut state = self.inner.lock().await;

        let pp_id = if let Some(ref id) = req.provisioned_product_id {
            id.clone()
        } else if let Some(ref name) = req.provisioned_product_name {
            let found = state.provisioned_products.values().find(|p| p.name == *name);
            match found {
                Some(p) => p.id.clone(),
                None => {
                    return Err(ServiceCatalogError::ResourceNotFoundException(format!(
                        "Provisioned product with name {} not found",
                        name
                    )))
                }
            }
        } else {
            return Err(ServiceCatalogError::InvalidParametersException(
                "Either ProvisionedProductId or ProvisionedProductName must be provided"
                    .to_string(),
            ));
        };

        let pp = state.provisioned_products.remove(&pp_id).ok_or_else(|| {
            ServiceCatalogError::ResourceNotFoundException(format!(
                "Provisioned product {} not found",
                pp_id
            ))
        })?;

        let now = Self::now();
        let record_id = format!("rec-{}", Uuid::new_v4());

        let record_detail = RecordDetail {
            record_id,
            created_time: now,
            product_id: pp.product_id,
            provisioned_product_id: Some(pp.id),
            provisioned_product_name: Some(pp.name),
            provisioning_artifact_id: pp.provisioning_artifact_id,
            record_type: "TERMINATE_PROVISIONED_PRODUCT".to_string(),
            status: "SUCCEEDED".to_string(),
            updated_time: now,
        };

        Ok(TerminateProvisionedProductResponse { record_detail })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_state() -> ServiceCatalogState {
        ServiceCatalogState::new("123456789012".to_string(), "us-east-1".to_string())
    }

    #[tokio::test]
    async fn test_new_state() {
        let _state = make_state();
    }

    #[tokio::test]
    async fn test_create_portfolio() {
        let state = make_state();
        let req = CreatePortfolioRequest {
            display_name: "My Portfolio".to_string(),
            provider_name: "My Provider".to_string(),
            ..Default::default()
        };
        let result = state.create_portfolio(req).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().portfolio_detail.display_name, "My Portfolio");
    }

    #[tokio::test]
    async fn test_describe_portfolio() {
        let state = make_state();
        let resp = state.create_portfolio(CreatePortfolioRequest {
            display_name: "p1".to_string(),
            provider_name: "prov".to_string(),
            ..Default::default()
        }).await.unwrap();
        let id = resp.portfolio_detail.id;
        let result = state.describe_portfolio(DescribePortfolioRequest { id }).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_describe_portfolio_not_found() {
        let state = make_state();
        assert!(state.describe_portfolio(DescribePortfolioRequest { id: "nope".to_string() }).await.is_err());
    }

    #[tokio::test]
    async fn test_list_portfolios() {
        let state = make_state();
        state.create_portfolio(CreatePortfolioRequest {
            display_name: "p1".to_string(),
            provider_name: "prov".to_string(),
            ..Default::default()
        }).await.unwrap();
        let result = state.list_portfolios(ListPortfoliosRequest::default()).await.unwrap();
        assert_eq!(result.portfolio_details.len(), 1);
    }

    #[tokio::test]
    async fn test_delete_portfolio() {
        let state = make_state();
        let resp = state.create_portfolio(CreatePortfolioRequest {
            display_name: "p1".to_string(),
            provider_name: "prov".to_string(),
            ..Default::default()
        }).await.unwrap();
        assert!(state.delete_portfolio(DeletePortfolioRequest { id: resp.portfolio_detail.id }).await.is_ok());
    }

    #[tokio::test]
    async fn test_update_portfolio() {
        let state = make_state();
        let resp = state.create_portfolio(CreatePortfolioRequest {
            display_name: "p1".to_string(),
            provider_name: "prov".to_string(),
            ..Default::default()
        }).await.unwrap();
        let req = UpdatePortfolioRequest {
            id: resp.portfolio_detail.id,
            display_name: Some("updated".to_string()),
            ..Default::default()
        };
        let result = state.update_portfolio(req).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_product() {
        let state = make_state();
        let req = CreateProductRequest {
            name: "My Product".to_string(),
            owner: "Me".to_string(),
            product_type: "CLOUD_FORMATION_TEMPLATE".to_string(),
            ..Default::default()
        };
        let result = state.create_product(req).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_describe_product() {
        let state = make_state();
        let resp = state.create_product(CreateProductRequest {
            name: "prod1".to_string(),
            owner: "owner".to_string(),
            product_type: "CLOUD_FORMATION_TEMPLATE".to_string(),
            ..Default::default()
        }).await.unwrap();
        let product_id = resp.product_view_detail.product_view_summary.product_id;
        let result = state.describe_product(DescribeProductRequest { id: Some(product_id), ..Default::default() }).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_product() {
        let state = make_state();
        let resp = state.create_product(CreateProductRequest {
            name: "prod1".to_string(),
            owner: "owner".to_string(),
            product_type: "CLOUD_FORMATION_TEMPLATE".to_string(),
            ..Default::default()
        }).await.unwrap();
        let product_id = resp.product_view_detail.product_view_summary.product_id;
        assert!(state.delete_product(DeleteProductRequest { id: product_id }).await.is_ok());
    }

    #[tokio::test]
    async fn test_search_products() {
        let state = make_state();
        state.create_product(CreateProductRequest {
            name: "prod1".to_string(),
            owner: "owner".to_string(),
            product_type: "CLOUD_FORMATION_TEMPLATE".to_string(),
            ..Default::default()
        }).await.unwrap();
        let result = state.search_products(SearchProductsRequest::default()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_associate_product_with_portfolio() {
        let state = make_state();
        let portfolio = state.create_portfolio(CreatePortfolioRequest {
            display_name: "p1".to_string(),
            provider_name: "prov".to_string(),
            ..Default::default()
        }).await.unwrap();
        let product = state.create_product(CreateProductRequest {
            name: "prod1".to_string(),
            owner: "owner".to_string(),
            product_type: "CLOUD_FORMATION_TEMPLATE".to_string(),
            ..Default::default()
        }).await.unwrap();
        let req = AssociateProductWithPortfolioRequest {
            product_id: product.product_view_detail.product_view_summary.product_id,
            portfolio_id: portfolio.portfolio_detail.id,
        };
        assert!(state.associate_product_with_portfolio(req).await.is_ok());
    }

    // --- Extended coverage: portfolio operations ---

    #[tokio::test]
    async fn test_delete_portfolio_not_found() {
        let state = make_state();
        let result = state.delete_portfolio(DeletePortfolioRequest { id: "nope".to_string() }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_portfolio_with_associations_fails() {
        let state = make_state();
        let portfolio = state.create_portfolio(CreatePortfolioRequest {
            display_name: "p1".to_string(),
            provider_name: "prov".to_string(),
            ..Default::default()
        }).await.unwrap();
        let product = state.create_product(CreateProductRequest {
            name: "prod1".to_string(),
            owner: "owner".to_string(),
            product_type: "CLOUD_FORMATION_TEMPLATE".to_string(),
            ..Default::default()
        }).await.unwrap();
        state.associate_product_with_portfolio(AssociateProductWithPortfolioRequest {
            product_id: product.product_view_detail.product_view_summary.product_id,
            portfolio_id: portfolio.portfolio_detail.id.clone(),
        }).await.unwrap();
        let result = state.delete_portfolio(DeletePortfolioRequest { id: portfolio.portfolio_detail.id }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_portfolio_with_tags() {
        let state = make_state();
        let result = state.create_portfolio(CreatePortfolioRequest {
            display_name: "p1".to_string(),
            provider_name: "prov".to_string(),
            tags: Some(vec![Tag { key: "env".to_string(), value: "prod".to_string() }]),
            ..Default::default()
        }).await.unwrap();
        assert_eq!(result.tags.len(), 1);
        assert_eq!(result.tags[0].key, "env");
    }

    #[tokio::test]
    async fn test_list_portfolios_empty() {
        let state = make_state();
        let result = state.list_portfolios(ListPortfoliosRequest::default()).await.unwrap();
        assert!(result.portfolio_details.is_empty());
        assert!(result.next_page_token.is_none());
    }

    #[tokio::test]
    async fn test_list_portfolios_pagination() {
        let state = make_state();
        for i in 0..3 {
            state.create_portfolio(CreatePortfolioRequest {
                display_name: format!("p{}", i),
                provider_name: "prov".to_string(),
                ..Default::default()
            }).await.unwrap();
        }
        let result = state.list_portfolios(ListPortfoliosRequest { page_size: Some(2) }).await.unwrap();
        assert_eq!(result.portfolio_details.len(), 2);
        assert!(result.next_page_token.is_some());
    }

    #[tokio::test]
    async fn test_update_portfolio_not_found() {
        let state = make_state();
        let result = state.update_portfolio(UpdatePortfolioRequest {
            id: "nope".to_string(),
            ..Default::default()
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_portfolio_all_fields() {
        let state = make_state();
        let resp = state.create_portfolio(CreatePortfolioRequest {
            display_name: "p1".to_string(),
            provider_name: "prov".to_string(),
            ..Default::default()
        }).await.unwrap();
        let result = state.update_portfolio(UpdatePortfolioRequest {
            id: resp.portfolio_detail.id,
            display_name: Some("updated".to_string()),
            description: Some("desc".to_string()),
            provider_name: Some("new-prov".to_string()),
            add_tags: Some(vec![Tag { key: "k".to_string(), value: "v".to_string() }]),
            remove_tags: None,
        }).await.unwrap();
        assert_eq!(result.portfolio_detail.display_name, "updated");
        assert_eq!(result.portfolio_detail.description.as_deref(), Some("desc"));
        assert_eq!(result.portfolio_detail.provider_name, "new-prov");
        assert_eq!(result.tags.len(), 1);
    }

    #[tokio::test]
    async fn test_update_portfolio_remove_tags() {
        let state = make_state();
        let resp = state.create_portfolio(CreatePortfolioRequest {
            display_name: "p1".to_string(),
            provider_name: "prov".to_string(),
            tags: Some(vec![Tag { key: "env".to_string(), value: "prod".to_string() }]),
            ..Default::default()
        }).await.unwrap();
        let result = state.update_portfolio(UpdatePortfolioRequest {
            id: resp.portfolio_detail.id,
            remove_tags: Some(vec!["env".to_string()]),
            ..Default::default()
        }).await.unwrap();
        assert!(result.tags.is_empty());
    }

    // --- Extended coverage: product operations ---

    #[tokio::test]
    async fn test_delete_product_not_found() {
        let state = make_state();
        let result = state.delete_product(DeleteProductRequest { id: "nope".to_string() }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_product_removes_associations() {
        let state = make_state();
        let portfolio = state.create_portfolio(CreatePortfolioRequest {
            display_name: "p1".to_string(),
            provider_name: "prov".to_string(),
            ..Default::default()
        }).await.unwrap();
        let product = state.create_product(CreateProductRequest {
            name: "prod1".to_string(),
            owner: "owner".to_string(),
            product_type: "CLOUD_FORMATION_TEMPLATE".to_string(),
            ..Default::default()
        }).await.unwrap();
        let pid = product.product_view_detail.product_view_summary.product_id.clone();
        state.associate_product_with_portfolio(AssociateProductWithPortfolioRequest {
            product_id: pid.clone(),
            portfolio_id: portfolio.portfolio_detail.id.clone(),
        }).await.unwrap();
        state.delete_product(DeleteProductRequest { id: pid }).await.unwrap();
        // Portfolio should be deletable now since associations were cleaned up
        assert!(state.delete_portfolio(DeletePortfolioRequest { id: portfolio.portfolio_detail.id }).await.is_ok());
    }

    #[tokio::test]
    async fn test_describe_product_by_name() {
        let state = make_state();
        state.create_product(CreateProductRequest {
            name: "my-product".to_string(),
            owner: "owner".to_string(),
            product_type: "CLOUD_FORMATION_TEMPLATE".to_string(),
            ..Default::default()
        }).await.unwrap();
        let result = state.describe_product(DescribeProductRequest {
            id: None,
            name: Some("my-product".to_string()),
        }).await.unwrap();
        assert_eq!(result.product_view_summary.name, "my-product");
    }

    #[tokio::test]
    async fn test_describe_product_by_name_not_found() {
        let state = make_state();
        let result = state.describe_product(DescribeProductRequest {
            id: None,
            name: Some("nonexistent".to_string()),
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_describe_product_no_id_or_name() {
        let state = make_state();
        let result = state.describe_product(DescribeProductRequest {
            id: None,
            name: None,
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_describe_product_not_found() {
        let state = make_state();
        let result = state.describe_product(DescribeProductRequest {
            id: Some("nope".to_string()),
            name: None,
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_product_with_provisioning_artifact() {
        let state = make_state();
        let result = state.create_product(CreateProductRequest {
            name: "prod1".to_string(),
            owner: "owner".to_string(),
            product_type: "CLOUD_FORMATION_TEMPLATE".to_string(),
            provisioning_artifact_parameters: Some(ProvisioningArtifactParameters {
                name: Some("v1".to_string()),
                description: Some("first".to_string()),
                artifact_type: Some("CLOUD_FORMATION_TEMPLATE".to_string()),
            }),
            ..Default::default()
        }).await.unwrap();
        assert_eq!(result.provisioning_artifact_detail.name.as_deref(), Some("v1"));
        assert_eq!(result.provisioning_artifact_detail.description.as_deref(), Some("first"));
    }

    #[tokio::test]
    async fn test_update_product_not_found() {
        let state = make_state();
        let result = state.update_product(UpdateProductRequest {
            id: "nope".to_string(),
            ..Default::default()
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_product_all_fields() {
        let state = make_state();
        let resp = state.create_product(CreateProductRequest {
            name: "prod1".to_string(),
            owner: "owner".to_string(),
            product_type: "CLOUD_FORMATION_TEMPLATE".to_string(),
            ..Default::default()
        }).await.unwrap();
        let pid = resp.product_view_detail.product_view_summary.product_id;
        let result = state.update_product(UpdateProductRequest {
            id: pid,
            name: Some("updated".to_string()),
            owner: Some("new-owner".to_string()),
            description: Some("desc".to_string()),
            distributor: Some("dist".to_string()),
            support_description: Some("sd".to_string()),
            support_email: Some("e@e.com".to_string()),
            support_url: Some("https://e.com".to_string()),
            add_tags: Some(vec![Tag { key: "k".to_string(), value: "v".to_string() }]),
            remove_tags: None,
        }).await.unwrap();
        assert_eq!(result.product_view_detail.product_view_summary.name, "updated");
        assert_eq!(result.product_view_detail.product_view_summary.owner, "new-owner");
        assert_eq!(result.tags.len(), 1);
    }

    #[tokio::test]
    async fn test_update_product_remove_tags() {
        let state = make_state();
        let resp = state.create_product(CreateProductRequest {
            name: "prod1".to_string(),
            owner: "owner".to_string(),
            product_type: "CLOUD_FORMATION_TEMPLATE".to_string(),
            tags: Some(vec![Tag { key: "env".to_string(), value: "prod".to_string() }]),
            ..Default::default()
        }).await.unwrap();
        let pid = resp.product_view_detail.product_view_summary.product_id;
        let result = state.update_product(UpdateProductRequest {
            id: pid,
            remove_tags: Some(vec!["env".to_string()]),
            ..Default::default()
        }).await.unwrap();
        assert!(result.tags.is_empty());
    }

    #[tokio::test]
    async fn test_search_products_with_filter() {
        let state = make_state();
        state.create_product(CreateProductRequest {
            name: "alpha-product".to_string(),
            owner: "owner".to_string(),
            product_type: "CLOUD_FORMATION_TEMPLATE".to_string(),
            ..Default::default()
        }).await.unwrap();
        state.create_product(CreateProductRequest {
            name: "beta-service".to_string(),
            owner: "owner".to_string(),
            product_type: "CLOUD_FORMATION_TEMPLATE".to_string(),
            ..Default::default()
        }).await.unwrap();
        let mut filters = HashMap::new();
        filters.insert("FullTextSearch".to_string(), vec!["alpha".to_string()]);
        let result = state.search_products(SearchProductsRequest {
            filters: Some(filters),
            page_size: None,
        }).await.unwrap();
        assert_eq!(result.product_view_summaries.len(), 1);
        assert_eq!(result.product_view_summaries[0].name, "alpha-product");
    }

    #[tokio::test]
    async fn test_search_products_pagination() {
        let state = make_state();
        for i in 0..3 {
            state.create_product(CreateProductRequest {
                name: format!("prod{}", i),
                owner: "owner".to_string(),
                product_type: "CLOUD_FORMATION_TEMPLATE".to_string(),
                ..Default::default()
            }).await.unwrap();
        }
        let result = state.search_products(SearchProductsRequest {
            filters: None,
            page_size: Some(2),
        }).await.unwrap();
        assert_eq!(result.product_view_summaries.len(), 2);
        assert!(result.next_page_token.is_some());
    }

    // --- Extended coverage: association operations ---

    #[tokio::test]
    async fn test_associate_portfolio_not_found() {
        let state = make_state();
        let product = state.create_product(CreateProductRequest {
            name: "p".to_string(),
            owner: "o".to_string(),
            product_type: "CLOUD_FORMATION_TEMPLATE".to_string(),
            ..Default::default()
        }).await.unwrap();
        let result = state.associate_product_with_portfolio(AssociateProductWithPortfolioRequest {
            product_id: product.product_view_detail.product_view_summary.product_id,
            portfolio_id: "nope".to_string(),
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_associate_product_not_found() {
        let state = make_state();
        let portfolio = state.create_portfolio(CreatePortfolioRequest {
            display_name: "p".to_string(),
            provider_name: "prov".to_string(),
            ..Default::default()
        }).await.unwrap();
        let result = state.associate_product_with_portfolio(AssociateProductWithPortfolioRequest {
            product_id: "nope".to_string(),
            portfolio_id: portfolio.portfolio_detail.id,
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_associate_duplicate() {
        let state = make_state();
        let portfolio = state.create_portfolio(CreatePortfolioRequest {
            display_name: "p".to_string(),
            provider_name: "prov".to_string(),
            ..Default::default()
        }).await.unwrap();
        let product = state.create_product(CreateProductRequest {
            name: "prod".to_string(),
            owner: "o".to_string(),
            product_type: "CLOUD_FORMATION_TEMPLATE".to_string(),
            ..Default::default()
        }).await.unwrap();
        let pid = product.product_view_detail.product_view_summary.product_id;
        let pfid = portfolio.portfolio_detail.id;
        state.associate_product_with_portfolio(AssociateProductWithPortfolioRequest {
            product_id: pid.clone(),
            portfolio_id: pfid.clone(),
        }).await.unwrap();
        let result = state.associate_product_with_portfolio(AssociateProductWithPortfolioRequest {
            product_id: pid,
            portfolio_id: pfid,
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_disassociate_product_from_portfolio() {
        let state = make_state();
        let portfolio = state.create_portfolio(CreatePortfolioRequest {
            display_name: "p".to_string(),
            provider_name: "prov".to_string(),
            ..Default::default()
        }).await.unwrap();
        let product = state.create_product(CreateProductRequest {
            name: "prod".to_string(),
            owner: "o".to_string(),
            product_type: "CLOUD_FORMATION_TEMPLATE".to_string(),
            ..Default::default()
        }).await.unwrap();
        let pid = product.product_view_detail.product_view_summary.product_id;
        let pfid = portfolio.portfolio_detail.id;
        state.associate_product_with_portfolio(AssociateProductWithPortfolioRequest {
            product_id: pid.clone(),
            portfolio_id: pfid.clone(),
        }).await.unwrap();
        assert!(state.disassociate_product_from_portfolio(DisassociateProductFromPortfolioRequest {
            product_id: pid,
            portfolio_id: pfid,
        }).await.is_ok());
    }

    #[tokio::test]
    async fn test_disassociate_not_found() {
        let state = make_state();
        let result = state.disassociate_product_from_portfolio(DisassociateProductFromPortfolioRequest {
            product_id: "a".to_string(),
            portfolio_id: "b".to_string(),
        }).await;
        assert!(result.is_err());
    }

    // --- Extended coverage: provisioned product operations ---

    #[tokio::test]
    async fn test_provision_product() {
        let state = make_state();
        let product = state.create_product(CreateProductRequest {
            name: "prod1".to_string(),
            owner: "owner".to_string(),
            product_type: "CLOUD_FORMATION_TEMPLATE".to_string(),
            ..Default::default()
        }).await.unwrap();
        let pid = product.product_view_detail.product_view_summary.product_id;
        let result = state.provision_product(ProvisionProductRequest {
            provisioned_product_name: "my-pp".to_string(),
            product_id: Some(pid),
            product_name: None,
            provisioning_artifact_id: None,
        }).await.unwrap();
        assert_eq!(result.record_detail.record_type, "PROVISION_PRODUCT");
        assert_eq!(result.record_detail.status, "SUCCEEDED");
    }

    #[tokio::test]
    async fn test_provision_product_by_name() {
        let state = make_state();
        state.create_product(CreateProductRequest {
            name: "my-prod".to_string(),
            owner: "owner".to_string(),
            product_type: "CLOUD_FORMATION_TEMPLATE".to_string(),
            ..Default::default()
        }).await.unwrap();
        let result = state.provision_product(ProvisionProductRequest {
            provisioned_product_name: "my-pp".to_string(),
            product_id: None,
            product_name: Some("my-prod".to_string()),
            provisioning_artifact_id: None,
        }).await.unwrap();
        assert!(result.record_detail.product_id.is_some());
    }

    #[tokio::test]
    async fn test_provision_product_not_found() {
        let state = make_state();
        let result = state.provision_product(ProvisionProductRequest {
            provisioned_product_name: "pp".to_string(),
            product_id: Some("nope".to_string()),
            product_name: None,
            provisioning_artifact_id: None,
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_provision_product_by_name_not_found() {
        let state = make_state();
        let result = state.provision_product(ProvisionProductRequest {
            provisioned_product_name: "pp".to_string(),
            product_id: None,
            product_name: Some("nope".to_string()),
            provisioning_artifact_id: None,
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_provision_product_no_product_ref() {
        let state = make_state();
        let result = state.provision_product(ProvisionProductRequest {
            provisioned_product_name: "pp".to_string(),
            product_id: None,
            product_name: None,
            provisioning_artifact_id: None,
        }).await;
        assert!(result.is_ok()); // product_id is optional
    }

    #[tokio::test]
    async fn test_describe_provisioned_product_by_id() {
        let state = make_state();
        let pp = state.provision_product(ProvisionProductRequest {
            provisioned_product_name: "my-pp".to_string(),
            ..Default::default()
        }).await.unwrap();
        let pp_id = pp.record_detail.provisioned_product_id.unwrap();
        let result = state.describe_provisioned_product(DescribeProvisionedProductRequest {
            id: Some(pp_id),
            name: None,
        }).await.unwrap();
        assert_eq!(result.provisioned_product_detail.name, "my-pp");
    }

    #[tokio::test]
    async fn test_describe_provisioned_product_by_name() {
        let state = make_state();
        state.provision_product(ProvisionProductRequest {
            provisioned_product_name: "my-pp".to_string(),
            ..Default::default()
        }).await.unwrap();
        let result = state.describe_provisioned_product(DescribeProvisionedProductRequest {
            id: None,
            name: Some("my-pp".to_string()),
        }).await.unwrap();
        assert_eq!(result.provisioned_product_detail.name, "my-pp");
    }

    #[tokio::test]
    async fn test_describe_provisioned_product_not_found() {
        let state = make_state();
        let result = state.describe_provisioned_product(DescribeProvisionedProductRequest {
            id: Some("nope".to_string()),
            name: None,
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_describe_provisioned_product_by_name_not_found() {
        let state = make_state();
        let result = state.describe_provisioned_product(DescribeProvisionedProductRequest {
            id: None,
            name: Some("nope".to_string()),
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_describe_provisioned_product_no_id_or_name() {
        let state = make_state();
        let result = state.describe_provisioned_product(DescribeProvisionedProductRequest {
            id: None,
            name: None,
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_search_provisioned_products() {
        let state = make_state();
        state.provision_product(ProvisionProductRequest {
            provisioned_product_name: "pp1".to_string(),
            ..Default::default()
        }).await.unwrap();
        state.provision_product(ProvisionProductRequest {
            provisioned_product_name: "pp2".to_string(),
            ..Default::default()
        }).await.unwrap();
        let result = state.search_provisioned_products(SearchProvisionedProductsRequest::default()).await.unwrap();
        assert_eq!(result.provisioned_products.len(), 2);
        assert_eq!(result.total_results_count, 2);
    }

    #[tokio::test]
    async fn test_search_provisioned_products_pagination() {
        let state = make_state();
        for i in 0..3 {
            state.provision_product(ProvisionProductRequest {
                provisioned_product_name: format!("pp{}", i),
                ..Default::default()
            }).await.unwrap();
        }
        let result = state.search_provisioned_products(SearchProvisionedProductsRequest { page_size: Some(2) }).await.unwrap();
        assert_eq!(result.provisioned_products.len(), 2);
        assert!(result.next_page_token.is_some());
    }

    #[tokio::test]
    async fn test_terminate_provisioned_product_by_id() {
        let state = make_state();
        let pp = state.provision_product(ProvisionProductRequest {
            provisioned_product_name: "pp1".to_string(),
            ..Default::default()
        }).await.unwrap();
        let pp_id = pp.record_detail.provisioned_product_id.unwrap();
        let result = state.terminate_provisioned_product(TerminateProvisionedProductRequest {
            provisioned_product_id: Some(pp_id),
            provisioned_product_name: None,
        }).await.unwrap();
        assert_eq!(result.record_detail.record_type, "TERMINATE_PROVISIONED_PRODUCT");
    }

    #[tokio::test]
    async fn test_terminate_provisioned_product_by_name() {
        let state = make_state();
        state.provision_product(ProvisionProductRequest {
            provisioned_product_name: "pp1".to_string(),
            ..Default::default()
        }).await.unwrap();
        let result = state.terminate_provisioned_product(TerminateProvisionedProductRequest {
            provisioned_product_id: None,
            provisioned_product_name: Some("pp1".to_string()),
        }).await.unwrap();
        assert_eq!(result.record_detail.record_type, "TERMINATE_PROVISIONED_PRODUCT");
    }

    #[tokio::test]
    async fn test_terminate_provisioned_product_not_found_by_id() {
        let state = make_state();
        let result = state.terminate_provisioned_product(TerminateProvisionedProductRequest {
            provisioned_product_id: Some("nope".to_string()),
            provisioned_product_name: None,
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_terminate_provisioned_product_not_found_by_name() {
        let state = make_state();
        let result = state.terminate_provisioned_product(TerminateProvisionedProductRequest {
            provisioned_product_id: None,
            provisioned_product_name: Some("nope".to_string()),
        }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_terminate_provisioned_product_no_id_or_name() {
        let state = make_state();
        let result = state.terminate_provisioned_product(TerminateProvisionedProductRequest {
            provisioned_product_id: None,
            provisioned_product_name: None,
        }).await;
        assert!(result.is_err());
    }
}
