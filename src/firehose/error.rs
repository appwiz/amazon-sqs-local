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
