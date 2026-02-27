use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum PinpointError {
    ResourceNotFoundException(String),
    ResourceAlreadyExistsException(String),
    ValidationException(String),
    InvalidAction(String),
}

impl PinpointError {
    #[allow(dead_code)]
    fn error_code(&self) -> &str {
        match self {
            PinpointError::ResourceNotFoundException(_) => "ResourceNotFoundException",
            PinpointError::ResourceAlreadyExistsException(_) => "ResourceAlreadyExistsException",
            PinpointError::ValidationException(_) => "ValidationException",
            PinpointError::InvalidAction(_) => "InvalidAction",
        }
    }

    #[allow(dead_code)]
    fn status_code(&self) -> StatusCode {
        match self {
            PinpointError::ResourceNotFoundException(_) => StatusCode::NOT_FOUND,
            _ => StatusCode::BAD_REQUEST,
        }
    }

    #[allow(dead_code)]
    fn message(&self) -> &str {
        match self {
            PinpointError::ResourceNotFoundException(m)
            | PinpointError::ResourceAlreadyExistsException(m)
            | PinpointError::ValidationException(m)
            | PinpointError::InvalidAction(m) => m,
        }
    }
}

impl IntoResponse for PinpointError {
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
        let err = PinpointError::ResourceNotFoundException("test".to_string());
        assert_eq!(err.error_code(), "ResourceNotFoundException");
    }
    #[test]
    fn test_resourcealreadyexistsexception_error_code() {
        let err = PinpointError::ResourceAlreadyExistsException("test".to_string());
        assert_eq!(err.error_code(), "ResourceAlreadyExistsException");
    }
    #[test]
    fn test_validationexception_error_code() {
        let err = PinpointError::ValidationException("test".to_string());
        assert_eq!(err.error_code(), "ValidationException");
    }
    #[test]
    fn test_invalidaction_error_code() {
        let err = PinpointError::InvalidAction("test".to_string());
        assert_eq!(err.error_code(), "InvalidAction");
    }
    #[test]
    fn test_message() {
        let err = PinpointError::ResourceNotFoundException("hello world".to_string());
        assert_eq!(err.message(), "hello world");
    }
    #[test]
    fn test_resourcenotfoundexception_status() {
        let err = PinpointError::ResourceNotFoundException("test".to_string());
        assert_eq!(err.status_code(), StatusCode::NOT_FOUND);
    }
    #[test]
    fn test_resourcealreadyexistsexception_status() {
        let err = PinpointError::ResourceAlreadyExistsException("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_validationexception_status() {
        let err = PinpointError::ValidationException("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_invalidaction_status() {
        let err = PinpointError::InvalidAction("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_into_response() {
        let err = PinpointError::ResourceNotFoundException("test error".to_string());
        let resp = err.into_response();
        assert!(resp.status().is_client_error());
    }
}
