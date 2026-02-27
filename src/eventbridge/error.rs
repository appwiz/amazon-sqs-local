use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;

#[derive(Debug, Clone)]
pub enum EventBridgeError {
    ResourceNotFoundException(String),
    ResourceAlreadyExistsException(String),
    InvalidAction(String),
}

impl EventBridgeError {
    fn error_code(&self) -> &str {
        match self {
            EventBridgeError::ResourceNotFoundException(_) => "ResourceNotFoundException",
            EventBridgeError::ResourceAlreadyExistsException(_) => "ResourceAlreadyExistsException",
            EventBridgeError::InvalidAction(_) => "InvalidAction",
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            EventBridgeError::ResourceNotFoundException(_) => StatusCode::NOT_FOUND,
            _ => StatusCode::BAD_REQUEST,
        }
    }

    fn message(&self) -> &str {
        match self {
            EventBridgeError::ResourceNotFoundException(m)
            | EventBridgeError::ResourceAlreadyExistsException(m)
            | EventBridgeError::InvalidAction(m) => m,
        }
    }
}

impl IntoResponse for EventBridgeError {
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
        let err = EventBridgeError::ResourceNotFoundException("test".to_string());
        assert_eq!(err.error_code(), "ResourceNotFoundException");
    }
    #[test]
    fn test_resourcealreadyexistsexception_error_code() {
        let err = EventBridgeError::ResourceAlreadyExistsException("test".to_string());
        assert_eq!(err.error_code(), "ResourceAlreadyExistsException");
    }
    #[test]
    fn test_invalidaction_error_code() {
        let err = EventBridgeError::InvalidAction("test".to_string());
        assert_eq!(err.error_code(), "InvalidAction");
    }
    #[test]
    fn test_message() {
        let err = EventBridgeError::ResourceNotFoundException("hello".to_string());
        assert_eq!(err.message(), "hello");
    }
    #[test]
    fn test_resourcenotfoundexception_status() {
        let err = EventBridgeError::ResourceNotFoundException("test".to_string());
        assert_eq!(err.status_code(), StatusCode::NOT_FOUND);
    }
    #[test]
    fn test_resourcealreadyexistsexception_status() {
        let err = EventBridgeError::ResourceAlreadyExistsException("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_into_response() {
        let err = EventBridgeError::ResourceNotFoundException("test".to_string());
        let resp = err.into_response();
        assert!(resp.status().is_client_error() || resp.status().is_server_error());
    }
}
