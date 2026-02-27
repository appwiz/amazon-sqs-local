use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;

#[derive(Debug, Clone)]
pub enum ServiceCatalogError {
    ResourceNotFoundException(String),
    InvalidParametersException(String),
    ResourceInUseException(String),
    DuplicateResourceException(String),
    InvalidAction(String),
}

impl ServiceCatalogError {
    fn error_code(&self) -> &str {
        match self {
            ServiceCatalogError::ResourceNotFoundException(_) => "ResourceNotFoundException",
            ServiceCatalogError::InvalidParametersException(_) => "InvalidParametersException",
            ServiceCatalogError::ResourceInUseException(_) => "ResourceInUseException",
            ServiceCatalogError::DuplicateResourceException(_) => "DuplicateResourceException",
            ServiceCatalogError::InvalidAction(_) => "InvalidAction",
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            ServiceCatalogError::ResourceNotFoundException(_) => StatusCode::BAD_REQUEST,
            ServiceCatalogError::InvalidParametersException(_) => StatusCode::BAD_REQUEST,
            ServiceCatalogError::ResourceInUseException(_) => StatusCode::BAD_REQUEST,
            ServiceCatalogError::DuplicateResourceException(_) => StatusCode::BAD_REQUEST,
            ServiceCatalogError::InvalidAction(_) => StatusCode::BAD_REQUEST,
        }
    }

    fn message(&self) -> &str {
        match self {
            ServiceCatalogError::ResourceNotFoundException(m)
            | ServiceCatalogError::InvalidParametersException(m)
            | ServiceCatalogError::ResourceInUseException(m)
            | ServiceCatalogError::DuplicateResourceException(m)
            | ServiceCatalogError::InvalidAction(m) => m,
        }
    }
}

impl IntoResponse for ServiceCatalogError {
    fn into_response(self) -> Response {
        let body = json!({
            "__type": self.error_code(),
            "Message": self.message(),
        });
        (self.status_code(), axum::Json(body)).into_response()
    }
}
