use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum SecretsManagerError {
    ResourceNotFoundException(String),
    ResourceExistsException(String),
    InvalidParameterException(String),
    InvalidRequestException(String),
    LimitExceededException(String),
    InvalidAction(String),
}

impl SecretsManagerError {
    fn error_code(&self) -> &str {
        match self {
            SecretsManagerError::ResourceNotFoundException(_) => "ResourceNotFoundException",
            SecretsManagerError::ResourceExistsException(_) => "ResourceExistsException",
            SecretsManagerError::InvalidParameterException(_) => "InvalidParameterException",
            SecretsManagerError::InvalidRequestException(_) => "InvalidRequestException",
            SecretsManagerError::LimitExceededException(_) => "LimitExceededException",
            SecretsManagerError::InvalidAction(_) => "InvalidAction",
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            SecretsManagerError::ResourceNotFoundException(_) => StatusCode::BAD_REQUEST,
            SecretsManagerError::ResourceExistsException(_) => StatusCode::BAD_REQUEST,
            SecretsManagerError::InvalidParameterException(_) => StatusCode::BAD_REQUEST,
            SecretsManagerError::InvalidRequestException(_) => StatusCode::BAD_REQUEST,
            SecretsManagerError::LimitExceededException(_) => StatusCode::BAD_REQUEST,
            SecretsManagerError::InvalidAction(_) => StatusCode::BAD_REQUEST,
        }
    }

    fn message(&self) -> &str {
        match self {
            SecretsManagerError::ResourceNotFoundException(m)
            | SecretsManagerError::ResourceExistsException(m)
            | SecretsManagerError::InvalidParameterException(m)
            | SecretsManagerError::InvalidRequestException(m)
            | SecretsManagerError::LimitExceededException(m)
            | SecretsManagerError::InvalidAction(m) => m,
        }
    }
}

impl IntoResponse for SecretsManagerError {
    fn into_response(self) -> Response {
        let body = json!({
            "__type": self.error_code(),
            "Message": self.message(),
        });
        (self.status_code(), axum::Json(body)).into_response()
    }
}
