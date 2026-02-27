use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum DirectoryserviceError {
    ResourceNotFoundException(String),
    ResourceAlreadyExistsException(String),
    ValidationException(String),
    InvalidAction(String),
}

impl DirectoryserviceError {
    #[allow(dead_code)]
    fn error_code(&self) -> &str {
        match self {
            DirectoryserviceError::ResourceNotFoundException(_) => "ResourceNotFoundException",
            DirectoryserviceError::ResourceAlreadyExistsException(_) => "ResourceAlreadyExistsException",
            DirectoryserviceError::ValidationException(_) => "ValidationException",
            DirectoryserviceError::InvalidAction(_) => "InvalidAction",
        }
    }

    #[allow(dead_code)]
    fn status_code(&self) -> StatusCode {
        match self {
            DirectoryserviceError::ResourceNotFoundException(_) => StatusCode::NOT_FOUND,
            _ => StatusCode::BAD_REQUEST,
        }
    }

    #[allow(dead_code)]
    fn message(&self) -> &str {
        match self {
            DirectoryserviceError::ResourceNotFoundException(m)
            | DirectoryserviceError::ResourceAlreadyExistsException(m)
            | DirectoryserviceError::ValidationException(m)
            | DirectoryserviceError::InvalidAction(m) => m,
        }
    }
}

impl IntoResponse for DirectoryserviceError {
    #[allow(dead_code)]
    fn into_response(self) -> Response {
        let body = serde_json::json!({
            "__type": self.error_code(),
            "message": self.message(),
        });
        (self.status_code(), axum::Json(body)).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resourcenotfoundexception_error_code() {
        let err = DirectoryserviceError::ResourceNotFoundException("test".to_string());
        assert_eq!(err.error_code(), "ResourceNotFoundException");
    }
    #[test]
    fn test_resourcealreadyexistsexception_error_code() {
        let err = DirectoryserviceError::ResourceAlreadyExistsException("test".to_string());
        assert_eq!(err.error_code(), "ResourceAlreadyExistsException");
    }
    #[test]
    fn test_validationexception_error_code() {
        let err = DirectoryserviceError::ValidationException("test".to_string());
        assert_eq!(err.error_code(), "ValidationException");
    }
    #[test]
    fn test_invalidaction_error_code() {
        let err = DirectoryserviceError::InvalidAction("test".to_string());
        assert_eq!(err.error_code(), "InvalidAction");
    }
    #[test]
    fn test_message() {
        let err = DirectoryserviceError::ResourceNotFoundException("hello world".to_string());
        assert_eq!(err.message(), "hello world");
    }
    #[test]
    fn test_resourcenotfoundexception_status() {
        let err = DirectoryserviceError::ResourceNotFoundException("test".to_string());
        assert_eq!(err.status_code(), StatusCode::NOT_FOUND);
    }
    #[test]
    fn test_resourcealreadyexistsexception_status() {
        let err = DirectoryserviceError::ResourceAlreadyExistsException("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_validationexception_status() {
        let err = DirectoryserviceError::ValidationException("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_invalidaction_status() {
        let err = DirectoryserviceError::InvalidAction("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_into_response() {
        let err = DirectoryserviceError::ResourceNotFoundException("test error".to_string());
        let resp = err.into_response();
        assert!(resp.status().is_client_error());
    }
}
