use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;

#[derive(Debug, Clone)]
pub enum CwlError {
    ResourceNotFoundException(String),
    ResourceAlreadyExistsException(String),
    InvalidParameterException(String),
    InvalidAction(String),
}

impl CwlError {
    fn error_code(&self) -> &str {
        match self {
            CwlError::ResourceNotFoundException(_) => "ResourceNotFoundException",
            CwlError::ResourceAlreadyExistsException(_) => "ResourceAlreadyExistsException",
            CwlError::InvalidParameterException(_) => "InvalidParameterException",
            CwlError::InvalidAction(_) => "InvalidAction",
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            CwlError::ResourceNotFoundException(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::BAD_REQUEST,
        }
    }

    fn message(&self) -> &str {
        match self {
            CwlError::ResourceNotFoundException(m)
            | CwlError::ResourceAlreadyExistsException(m)
            | CwlError::InvalidParameterException(m)
            | CwlError::InvalidAction(m) => m,
        }
    }
}

impl IntoResponse for CwlError {
    fn into_response(self) -> Response {
        let body = json!({
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
        let err = CwlError::ResourceNotFoundException("test".to_string());
        assert_eq!(err.error_code(), "ResourceNotFoundException");
    }
    #[test]
    fn test_resourcealreadyexistsexception_error_code() {
        let err = CwlError::ResourceAlreadyExistsException("test".to_string());
        assert_eq!(err.error_code(), "ResourceAlreadyExistsException");
    }
    #[test]
    fn test_invalidparameterexception_error_code() {
        let err = CwlError::InvalidParameterException("test".to_string());
        assert_eq!(err.error_code(), "InvalidParameterException");
    }
    #[test]
    fn test_invalidaction_error_code() {
        let err = CwlError::InvalidAction("test".to_string());
        assert_eq!(err.error_code(), "InvalidAction");
    }
    #[test]
    fn test_message() {
        let err = CwlError::ResourceNotFoundException("hello world".to_string());
        assert_eq!(err.message(), "hello world");
    }
    #[test]
    fn test_resourcenotfoundexception_status() {
        let err = CwlError::ResourceNotFoundException("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_resourcealreadyexistsexception_status() {
        let err = CwlError::ResourceAlreadyExistsException("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_invalidparameterexception_status() {
        let err = CwlError::InvalidParameterException("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_invalidaction_status() {
        let err = CwlError::InvalidAction("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_into_response() {
        let err = CwlError::ResourceNotFoundException("test error".to_string());
        let resp = err.into_response();
        assert!(resp.status().is_client_error());
    }
}
