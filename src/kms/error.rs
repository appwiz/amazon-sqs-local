use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;

#[derive(Debug, Clone)]
pub enum KmsError {
    NotFoundException(String),
    InvalidCiphertextException(String),
    DisabledException(String),
    InvalidParameterException(String),
    InvalidAction(String),
}

impl KmsError {
    fn error_code(&self) -> &str {
        match self {
            KmsError::NotFoundException(_) => "NotFoundException",
            KmsError::InvalidCiphertextException(_) => "InvalidCiphertextException",
            KmsError::DisabledException(_) => "DisabledException",
            KmsError::InvalidParameterException(_) => "InvalidParameterException",
            KmsError::InvalidAction(_) => "InvalidAction",
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            KmsError::NotFoundException(_) => StatusCode::NOT_FOUND,
            KmsError::InvalidCiphertextException(_) => StatusCode::BAD_REQUEST,
            KmsError::DisabledException(_) => StatusCode::BAD_REQUEST,
            KmsError::InvalidParameterException(_) => StatusCode::BAD_REQUEST,
            KmsError::InvalidAction(_) => StatusCode::BAD_REQUEST,
        }
    }

    fn message(&self) -> &str {
        match self {
            KmsError::NotFoundException(m)
            | KmsError::InvalidCiphertextException(m)
            | KmsError::DisabledException(m)
            | KmsError::InvalidParameterException(m)
            | KmsError::InvalidAction(m) => m,
        }
    }
}

impl IntoResponse for KmsError {
    fn into_response(self) -> Response {
        let body = json!({
            "__type": self.error_code(),
            "message": self.message(),
        });
        (self.status_code(), axum::Json(body)).into_response()
    }
}
