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
