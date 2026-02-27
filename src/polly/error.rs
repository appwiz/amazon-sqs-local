use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum PollyError {
    ResourceNotFoundException(String),
    ResourceAlreadyExistsException(String),
    ValidationException(String),
    InvalidAction(String),
}

impl PollyError {
    #[allow(dead_code)]
    fn error_code(&self) -> &str {
        match self {
            PollyError::ResourceNotFoundException(_) => "ResourceNotFoundException",
            PollyError::ResourceAlreadyExistsException(_) => "ResourceAlreadyExistsException",
            PollyError::ValidationException(_) => "ValidationException",
            PollyError::InvalidAction(_) => "InvalidAction",
        }
    }

    #[allow(dead_code)]
    fn status_code(&self) -> StatusCode {
        match self {
            PollyError::ResourceNotFoundException(_) => StatusCode::NOT_FOUND,
            _ => StatusCode::BAD_REQUEST,
        }
    }

    #[allow(dead_code)]
    fn message(&self) -> &str {
        match self {
            PollyError::ResourceNotFoundException(m)
            | PollyError::ResourceAlreadyExistsException(m)
            | PollyError::ValidationException(m)
            | PollyError::InvalidAction(m) => m,
        }
    }
}

impl IntoResponse for PollyError {
    #[allow(dead_code)]
    fn into_response(self) -> Response {
        let body = serde_json::json!({
            "Message": self.message(),
        });
        (
            self.status_code(),
            [("x-amzn-ErrorType", self.error_code())],
            axum::Json(body),
        )
            .into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resourcenotfoundexception_error_code() {
        let err = PollyError::ResourceNotFoundException("test".to_string());
        assert_eq!(err.error_code(), "ResourceNotFoundException");
    }
    #[test]
    fn test_resourcealreadyexistsexception_error_code() {
        let err = PollyError::ResourceAlreadyExistsException("test".to_string());
        assert_eq!(err.error_code(), "ResourceAlreadyExistsException");
    }
    #[test]
    fn test_validationexception_error_code() {
        let err = PollyError::ValidationException("test".to_string());
        assert_eq!(err.error_code(), "ValidationException");
    }
    #[test]
    fn test_invalidaction_error_code() {
        let err = PollyError::InvalidAction("test".to_string());
        assert_eq!(err.error_code(), "InvalidAction");
    }
    #[test]
    fn test_message() {
        let err = PollyError::ResourceNotFoundException("hello world".to_string());
        assert_eq!(err.message(), "hello world");
    }
    #[test]
    fn test_resourcenotfoundexception_status() {
        let err = PollyError::ResourceNotFoundException("test".to_string());
        assert_eq!(err.status_code(), StatusCode::NOT_FOUND);
    }
    #[test]
    fn test_resourcealreadyexistsexception_status() {
        let err = PollyError::ResourceAlreadyExistsException("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_validationexception_status() {
        let err = PollyError::ValidationException("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_invalidaction_status() {
        let err = PollyError::InvalidAction("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_into_response() {
        let err = PollyError::ResourceNotFoundException("test error".to_string());
        let resp = err.into_response();
        assert!(resp.status().is_client_error());
    }
}
