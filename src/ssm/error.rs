use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum SsmError {
    ParameterNotFound(String),
    ParameterAlreadyExists(String),
    InvalidKeyId(String),
    InvalidFilterValue(String),
    TooManyUpdates(String),
    InvalidAction(String),
}

impl SsmError {
    fn error_code(&self) -> &str {
        match self {
            SsmError::ParameterNotFound(_) => "ParameterNotFound",
            SsmError::ParameterAlreadyExists(_) => "ParameterAlreadyExists",
            SsmError::InvalidKeyId(_) => "InvalidKeyId",
            SsmError::InvalidFilterValue(_) => "InvalidFilterValue",
            SsmError::TooManyUpdates(_) => "TooManyUpdates",
            SsmError::InvalidAction(_) => "InvalidAction",
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            SsmError::ParameterNotFound(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::BAD_REQUEST,
        }
    }

    fn message(&self) -> &str {
        match self {
            SsmError::ParameterNotFound(m)
            | SsmError::ParameterAlreadyExists(m)
            | SsmError::InvalidKeyId(m)
            | SsmError::InvalidFilterValue(m)
            | SsmError::TooManyUpdates(m)
            | SsmError::InvalidAction(m) => m,
        }
    }
}

impl IntoResponse for SsmError {
    fn into_response(self) -> Response {
        let body = json!({
            "__type": self.error_code(),
            "message": self.message(),
        });
        (self.status_code(), axum::Json(body)).into_response()
    }
}
