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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notfoundexception_error_code() {
        let err = KmsError::NotFoundException("test".to_string());
        assert_eq!(err.error_code(), "NotFoundException");
    }
    #[test]
    fn test_invalidciphertextexception_error_code() {
        let err = KmsError::InvalidCiphertextException("test".to_string());
        assert_eq!(err.error_code(), "InvalidCiphertextException");
    }
    #[test]
    fn test_disabledexception_error_code() {
        let err = KmsError::DisabledException("test".to_string());
        assert_eq!(err.error_code(), "DisabledException");
    }
    #[test]
    fn test_invalidparameterexception_error_code() {
        let err = KmsError::InvalidParameterException("test".to_string());
        assert_eq!(err.error_code(), "InvalidParameterException");
    }
    #[test]
    fn test_invalidaction_error_code() {
        let err = KmsError::InvalidAction("test".to_string());
        assert_eq!(err.error_code(), "InvalidAction");
    }
    #[test]
    fn test_message() {
        let err = KmsError::NotFoundException("hello world".to_string());
        assert_eq!(err.message(), "hello world");
    }
    #[test]
    fn test_notfoundexception_status() {
        let err = KmsError::NotFoundException("test".to_string());
        assert_eq!(err.status_code(), StatusCode::NOT_FOUND);
    }
    #[test]
    fn test_invalidciphertextexception_status() {
        let err = KmsError::InvalidCiphertextException("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_disabledexception_status() {
        let err = KmsError::DisabledException("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_invalidparameterexception_status() {
        let err = KmsError::InvalidParameterException("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_invalidaction_status() {
        let err = KmsError::InvalidAction("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_into_response() {
        let err = KmsError::NotFoundException("test error".to_string());
        let resp = err.into_response();
        assert!(resp.status().is_client_error());
    }
}
