use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum CwlError {
    ResourceNotFoundException(String),
    ResourceAlreadyExistsException(String),
    InvalidParameterException(String),
    LimitExceededException(String),
    ServiceUnavailableException(String),
    InvalidSequenceTokenException(String),
    DataAlreadyAcceptedException(String),
    InvalidAction(String),
}

impl CwlError {
    fn error_code(&self) -> &str {
        match self {
            CwlError::ResourceNotFoundException(_) => "ResourceNotFoundException",
            CwlError::ResourceAlreadyExistsException(_) => "ResourceAlreadyExistsException",
            CwlError::InvalidParameterException(_) => "InvalidParameterException",
            CwlError::LimitExceededException(_) => "LimitExceededException",
            CwlError::ServiceUnavailableException(_) => "ServiceUnavailableException",
            CwlError::InvalidSequenceTokenException(_) => "InvalidSequenceTokenException",
            CwlError::DataAlreadyAcceptedException(_) => "DataAlreadyAcceptedException",
            CwlError::InvalidAction(_) => "InvalidAction",
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            CwlError::ResourceNotFoundException(_) => StatusCode::BAD_REQUEST,
            CwlError::ServiceUnavailableException(_) => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::BAD_REQUEST,
        }
    }

    fn message(&self) -> &str {
        match self {
            CwlError::ResourceNotFoundException(m)
            | CwlError::ResourceAlreadyExistsException(m)
            | CwlError::InvalidParameterException(m)
            | CwlError::LimitExceededException(m)
            | CwlError::ServiceUnavailableException(m)
            | CwlError::InvalidSequenceTokenException(m)
            | CwlError::DataAlreadyAcceptedException(m)
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
