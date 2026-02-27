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
