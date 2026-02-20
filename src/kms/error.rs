use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum KmsError {
    NotFoundException(String),
    AlreadyExistsException(String),
    InvalidKeyUsageException(String),
    InvalidCiphertextException(String),
    DisabledException(String),
    KMSInvalidStateException(String),
    InvalidParameterException(String),
    LimitExceededException(String),
    InvalidAction(String),
}

impl KmsError {
    fn error_code(&self) -> &str {
        match self {
            KmsError::NotFoundException(_) => "NotFoundException",
            KmsError::AlreadyExistsException(_) => "AlreadyExistsException",
            KmsError::InvalidKeyUsageException(_) => "InvalidKeyUsageException",
            KmsError::InvalidCiphertextException(_) => "InvalidCiphertextException",
            KmsError::DisabledException(_) => "DisabledException",
            KmsError::KMSInvalidStateException(_) => "KMSInvalidStateException",
            KmsError::InvalidParameterException(_) => "InvalidParameterException",
            KmsError::LimitExceededException(_) => "LimitExceededException",
            KmsError::InvalidAction(_) => "InvalidAction",
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            KmsError::NotFoundException(_) => StatusCode::NOT_FOUND,
            KmsError::AlreadyExistsException(_) => StatusCode::BAD_REQUEST,
            KmsError::InvalidKeyUsageException(_) => StatusCode::BAD_REQUEST,
            KmsError::InvalidCiphertextException(_) => StatusCode::BAD_REQUEST,
            KmsError::DisabledException(_) => StatusCode::BAD_REQUEST,
            KmsError::KMSInvalidStateException(_) => StatusCode::BAD_REQUEST,
            KmsError::InvalidParameterException(_) => StatusCode::BAD_REQUEST,
            KmsError::LimitExceededException(_) => StatusCode::BAD_REQUEST,
            KmsError::InvalidAction(_) => StatusCode::BAD_REQUEST,
        }
    }

    fn message(&self) -> &str {
        match self {
            KmsError::NotFoundException(m)
            | KmsError::AlreadyExistsException(m)
            | KmsError::InvalidKeyUsageException(m)
            | KmsError::InvalidCiphertextException(m)
            | KmsError::DisabledException(m)
            | KmsError::KMSInvalidStateException(m)
            | KmsError::InvalidParameterException(m)
            | KmsError::LimitExceededException(m)
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
