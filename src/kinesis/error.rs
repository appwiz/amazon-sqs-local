use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;

#[derive(Debug, Clone)]
pub enum KinesisError {
    ResourceNotFoundException(String),
    ResourceInUseException(String),
    InvalidArgumentException(String),
    ExpiredIteratorException(String),
    InvalidAction(String),
}

impl KinesisError {
    fn error_code(&self) -> &str {
        match self {
            KinesisError::ResourceNotFoundException(_) => "ResourceNotFoundException",
            KinesisError::ResourceInUseException(_) => "ResourceInUseException",
            KinesisError::InvalidArgumentException(_) => "InvalidArgumentException",
            KinesisError::ExpiredIteratorException(_) => "ExpiredIteratorException",
            KinesisError::InvalidAction(_) => "InvalidAction",
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            KinesisError::ResourceNotFoundException(_) => StatusCode::BAD_REQUEST,
            KinesisError::ExpiredIteratorException(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::BAD_REQUEST,
        }
    }

    fn message(&self) -> &str {
        match self {
            KinesisError::ResourceNotFoundException(m)
            | KinesisError::ResourceInUseException(m)
            | KinesisError::InvalidArgumentException(m)
            | KinesisError::ExpiredIteratorException(m)
            | KinesisError::InvalidAction(m) => m,
        }
    }
}

impl IntoResponse for KinesisError {
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
        let err = KinesisError::ResourceNotFoundException("test".to_string());
        assert_eq!(err.error_code(), "ResourceNotFoundException");
    }
    #[test]
    fn test_resourceinuseexception_error_code() {
        let err = KinesisError::ResourceInUseException("test".to_string());
        assert_eq!(err.error_code(), "ResourceInUseException");
    }
    #[test]
    fn test_invalidargumentexception_error_code() {
        let err = KinesisError::InvalidArgumentException("test".to_string());
        assert_eq!(err.error_code(), "InvalidArgumentException");
    }
    #[test]
    fn test_expirediteratorexception_error_code() {
        let err = KinesisError::ExpiredIteratorException("test".to_string());
        assert_eq!(err.error_code(), "ExpiredIteratorException");
    }
    #[test]
    fn test_invalidaction_error_code() {
        let err = KinesisError::InvalidAction("test".to_string());
        assert_eq!(err.error_code(), "InvalidAction");
    }
    #[test]
    fn test_message() {
        let err = KinesisError::ResourceNotFoundException("hello world".to_string());
        assert_eq!(err.message(), "hello world");
    }
    #[test]
    fn test_resourcenotfoundexception_status() {
        let err = KinesisError::ResourceNotFoundException("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_resourceinuseexception_status() {
        let err = KinesisError::ResourceInUseException("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_invalidargumentexception_status() {
        let err = KinesisError::InvalidArgumentException("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_expirediteratorexception_status() {
        let err = KinesisError::ExpiredIteratorException("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_invalidaction_status() {
        let err = KinesisError::InvalidAction("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_into_response() {
        let err = KinesisError::ResourceNotFoundException("test error".to_string());
        let resp = err.into_response();
        assert!(resp.status().is_client_error());
    }
}
