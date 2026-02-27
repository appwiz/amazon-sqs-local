use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;

#[derive(Debug, Clone)]
pub enum FirehoseError {
    ResourceNotFoundException(String),
    ResourceInUseException(String),
    InvalidArgumentException(String),
    LimitExceededException(String),
    ConcurrentModificationException(String),
    InvalidAction(String),
}

impl FirehoseError {
    fn error_code(&self) -> &str {
        match self {
            FirehoseError::ResourceNotFoundException(_) => "ResourceNotFoundException",
            FirehoseError::ResourceInUseException(_) => "ResourceInUseException",
            FirehoseError::InvalidArgumentException(_) => "InvalidArgumentException",
            FirehoseError::LimitExceededException(_) => "LimitExceededException",
            FirehoseError::ConcurrentModificationException(_) => "ConcurrentModificationException",
            FirehoseError::InvalidAction(_) => "InvalidAction",
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            FirehoseError::ResourceNotFoundException(_) => StatusCode::BAD_REQUEST,
            FirehoseError::ResourceInUseException(_) => StatusCode::BAD_REQUEST,
            FirehoseError::LimitExceededException(_) => StatusCode::BAD_REQUEST,
            FirehoseError::ConcurrentModificationException(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::BAD_REQUEST,
        }
    }

    fn message(&self) -> &str {
        match self {
            FirehoseError::ResourceNotFoundException(m)
            | FirehoseError::ResourceInUseException(m)
            | FirehoseError::InvalidArgumentException(m)
            | FirehoseError::LimitExceededException(m)
            | FirehoseError::ConcurrentModificationException(m)
            | FirehoseError::InvalidAction(m) => m,
        }
    }
}

impl IntoResponse for FirehoseError {
    fn into_response(self) -> Response {
        let body = json!({
            "__type": format!("#{}", self.error_code()),
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
        let err = FirehoseError::ResourceNotFoundException("test".to_string());
        assert_eq!(err.error_code(), "ResourceNotFoundException");
    }
    #[test]
    fn test_resourceinuseexception_error_code() {
        let err = FirehoseError::ResourceInUseException("test".to_string());
        assert_eq!(err.error_code(), "ResourceInUseException");
    }
    #[test]
    fn test_invalidargumentexception_error_code() {
        let err = FirehoseError::InvalidArgumentException("test".to_string());
        assert_eq!(err.error_code(), "InvalidArgumentException");
    }
    #[test]
    fn test_limitexceededexception_error_code() {
        let err = FirehoseError::LimitExceededException("test".to_string());
        assert_eq!(err.error_code(), "LimitExceededException");
    }
    #[test]
    fn test_concurrentmodificationexception_error_code() {
        let err = FirehoseError::ConcurrentModificationException("test".to_string());
        assert_eq!(err.error_code(), "ConcurrentModificationException");
    }
    #[test]
    fn test_invalidaction_error_code() {
        let err = FirehoseError::InvalidAction("test".to_string());
        assert_eq!(err.error_code(), "InvalidAction");
    }
    #[test]
    fn test_message() {
        let err = FirehoseError::ResourceNotFoundException("hello world".to_string());
        assert_eq!(err.message(), "hello world");
    }
    #[test]
    fn test_resourcenotfoundexception_status() {
        let err = FirehoseError::ResourceNotFoundException("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_resourceinuseexception_status() {
        let err = FirehoseError::ResourceInUseException("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_invalidargumentexception_status() {
        let err = FirehoseError::InvalidArgumentException("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_limitexceededexception_status() {
        let err = FirehoseError::LimitExceededException("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_concurrentmodificationexception_status() {
        let err = FirehoseError::ConcurrentModificationException("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_invalidaction_status() {
        let err = FirehoseError::InvalidAction("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_into_response() {
        let err = FirehoseError::ResourceNotFoundException("test error".to_string());
        let resp = err.into_response();
        assert!(resp.status().is_client_error());
    }
}
