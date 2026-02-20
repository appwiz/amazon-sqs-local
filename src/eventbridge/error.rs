use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum EventBridgeError {
    ResourceNotFoundException(String),
    ResourceAlreadyExistsException(String),
    InvalidEventPatternException(String),
    LimitExceededException(String),
    InvalidAction(String),
}

impl EventBridgeError {
    fn error_code(&self) -> &str {
        match self {
            EventBridgeError::ResourceNotFoundException(_) => "ResourceNotFoundException",
            EventBridgeError::ResourceAlreadyExistsException(_) => "ResourceAlreadyExistsException",
            EventBridgeError::InvalidEventPatternException(_) => "InvalidEventPatternException",
            EventBridgeError::LimitExceededException(_) => "LimitExceededException",
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
            | EventBridgeError::InvalidEventPatternException(m)
            | EventBridgeError::LimitExceededException(m)
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
