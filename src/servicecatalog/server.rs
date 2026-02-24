use std::sync::Arc;

use axum::body::Bytes;
use axum::extract::State;
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};

use super::error::ServiceCatalogError;
use super::state::ServiceCatalogState;
use super::types::*;

macro_rules! dispatch {
    ($state:expr, $body:expr, $req_type:ty, $method:ident) => {{
        let req: $req_type = serde_json::from_slice(&$body)
            .map_err(|e| ServiceCatalogError::InvalidParametersException(e.to_string()))?;
        let resp = $state.$method(req).await?;
        Ok(Json(serde_json::to_value(resp).unwrap()).into_response())
    }};
}

macro_rules! dispatch_empty {
    ($state:expr, $body:expr, $req_type:ty, $method:ident) => {{
        let req: $req_type = serde_json::from_slice(&$body)
            .map_err(|e| ServiceCatalogError::InvalidParametersException(e.to_string()))?;
        $state.$method(req).await?;
        Ok(Json(serde_json::json!({})).into_response())
    }};
}

async fn handle_request(
    State(state): State<Arc<ServiceCatalogState>>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<axum::response::Response, ServiceCatalogError> {
    let target = headers
        .get("x-amz-target")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| {
            ServiceCatalogError::InvalidAction("Missing X-Amz-Target header".into())
        })?;

    let action = target
        .strip_prefix("AWS242ServiceCatalogService.")
        .ok_or_else(|| {
            ServiceCatalogError::InvalidAction(format!("Invalid target: {target}"))
        })?;

    match action {
        "CreatePortfolio" => dispatch!(state, body, CreatePortfolioRequest, create_portfolio),
        "DeletePortfolio" => {
            dispatch_empty!(state, body, DeletePortfolioRequest, delete_portfolio)
        }
        "DescribePortfolio" => {
            dispatch!(state, body, DescribePortfolioRequest, describe_portfolio)
        }
        "ListPortfolios" => dispatch!(state, body, ListPortfoliosRequest, list_portfolios),
        "UpdatePortfolio" => dispatch!(state, body, UpdatePortfolioRequest, update_portfolio),
        "CreateProduct" => dispatch!(state, body, CreateProductRequest, create_product),
        "DeleteProduct" => dispatch_empty!(state, body, DeleteProductRequest, delete_product),
        "DescribeProduct" => dispatch!(state, body, DescribeProductRequest, describe_product),
        "UpdateProduct" => dispatch!(state, body, UpdateProductRequest, update_product),
        "SearchProducts" => dispatch!(state, body, SearchProductsRequest, search_products),
        "AssociateProductWithPortfolio" => {
            dispatch_empty!(
                state,
                body,
                AssociateProductWithPortfolioRequest,
                associate_product_with_portfolio
            )
        }
        "DisassociateProductFromPortfolio" => {
            dispatch_empty!(
                state,
                body,
                DisassociateProductFromPortfolioRequest,
                disassociate_product_from_portfolio
            )
        }
        "ProvisionProduct" => {
            dispatch!(state, body, ProvisionProductRequest, provision_product)
        }
        "DescribeProvisionedProduct" => {
            dispatch!(
                state,
                body,
                DescribeProvisionedProductRequest,
                describe_provisioned_product
            )
        }
        "SearchProvisionedProducts" => {
            dispatch!(
                state,
                body,
                SearchProvisionedProductsRequest,
                search_provisioned_products
            )
        }
        "TerminateProvisionedProduct" => {
            dispatch!(
                state,
                body,
                TerminateProvisionedProductRequest,
                terminate_provisioned_product
            )
        }
        _ => Err(ServiceCatalogError::InvalidAction(format!(
            "Unknown action: {action}"
        ))),
    }
}

pub fn create_router(state: Arc<ServiceCatalogState>) -> Router {
    Router::new()
        .route("/", post(handle_request))
        .with_state(state)
}
