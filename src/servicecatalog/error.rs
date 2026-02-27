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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resourcenotfoundexception_error_code() {
        let err = ServiceCatalogError::ResourceNotFoundException("test".to_string());
        assert_eq!(err.error_code(), "ResourceNotFoundException");
    }
    #[test]
    fn test_invalidparametersexception_error_code() {
        let err = ServiceCatalogError::InvalidParametersException("test".to_string());
        assert_eq!(err.error_code(), "InvalidParametersException");
    }
    #[test]
    fn test_resourceinuseexception_error_code() {
        let err = ServiceCatalogError::ResourceInUseException("test".to_string());
        assert_eq!(err.error_code(), "ResourceInUseException");
    }
    #[test]
    fn test_duplicateresourceexception_error_code() {
        let err = ServiceCatalogError::DuplicateResourceException("test".to_string());
        assert_eq!(err.error_code(), "DuplicateResourceException");
    }
    #[test]
    fn test_invalidaction_error_code() {
        let err = ServiceCatalogError::InvalidAction("test".to_string());
        assert_eq!(err.error_code(), "InvalidAction");
    }
    #[test]
    fn test_message() {
        let err = ServiceCatalogError::ResourceNotFoundException("hello world".to_string());
        assert_eq!(err.message(), "hello world");
    }
    #[test]
    fn test_resourcenotfoundexception_status() {
        let err = ServiceCatalogError::ResourceNotFoundException("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_invalidparametersexception_status() {
        let err = ServiceCatalogError::InvalidParametersException("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_resourceinuseexception_status() {
        let err = ServiceCatalogError::ResourceInUseException("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_duplicateresourceexception_status() {
        let err = ServiceCatalogError::DuplicateResourceException("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_invalidaction_status() {
        let err = ServiceCatalogError::InvalidAction("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_into_response() {
        let err = ServiceCatalogError::ResourceNotFoundException("test error".to_string());
        let resp = err.into_response();
        assert!(resp.status().is_client_error());
    }
}
