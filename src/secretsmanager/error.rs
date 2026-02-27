use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;

#[derive(Debug, Clone)]
pub enum SecretsManagerError {
    ResourceNotFoundException(String),
    ResourceExistsException(String),
    InvalidParameterException(String),
    InvalidRequestException(String),
    InvalidAction(String),
}

impl SecretsManagerError {
    fn error_code(&self) -> &str {
        match self {
            SecretsManagerError::ResourceNotFoundException(_) => "ResourceNotFoundException",
            SecretsManagerError::ResourceExistsException(_) => "ResourceExistsException",
            SecretsManagerError::InvalidParameterException(_) => "InvalidParameterException",
            SecretsManagerError::InvalidRequestException(_) => "InvalidRequestException",
            SecretsManagerError::InvalidAction(_) => "InvalidAction",
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            SecretsManagerError::ResourceNotFoundException(_) => StatusCode::BAD_REQUEST,
            SecretsManagerError::ResourceExistsException(_) => StatusCode::BAD_REQUEST,
            SecretsManagerError::InvalidParameterException(_) => StatusCode::BAD_REQUEST,
            SecretsManagerError::InvalidRequestException(_) => StatusCode::BAD_REQUEST,
            SecretsManagerError::InvalidAction(_) => StatusCode::BAD_REQUEST,
        }
    }

    fn message(&self) -> &str {
        match self {
            SecretsManagerError::ResourceNotFoundException(m)
            | SecretsManagerError::ResourceExistsException(m)
            | SecretsManagerError::InvalidParameterException(m)
            | SecretsManagerError::InvalidRequestException(m)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resourcenotfoundexception_error_code() {
        let err = SecretsManagerError::ResourceNotFoundException("test".to_string());
        assert_eq!(err.error_code(), "ResourceNotFoundException");
    }
    #[test]
    fn test_resourceexistsexception_error_code() {
        let err = SecretsManagerError::ResourceExistsException("test".to_string());
        assert_eq!(err.error_code(), "ResourceExistsException");
    }
    #[test]
    fn test_invalidparameterexception_error_code() {
        let err = SecretsManagerError::InvalidParameterException("test".to_string());
        assert_eq!(err.error_code(), "InvalidParameterException");
    }
    #[test]
    fn test_invalidrequestexception_error_code() {
        let err = SecretsManagerError::InvalidRequestException("test".to_string());
        assert_eq!(err.error_code(), "InvalidRequestException");
    }
    #[test]
    fn test_invalidaction_error_code() {
        let err = SecretsManagerError::InvalidAction("test".to_string());
        assert_eq!(err.error_code(), "InvalidAction");
    }
    #[test]
    fn test_message() {
        let err = SecretsManagerError::ResourceNotFoundException("hello world".to_string());
        assert_eq!(err.message(), "hello world");
    }
    #[test]
    fn test_resourcenotfoundexception_status() {
        let err = SecretsManagerError::ResourceNotFoundException("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_resourceexistsexception_status() {
        let err = SecretsManagerError::ResourceExistsException("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_invalidparameterexception_status() {
        let err = SecretsManagerError::InvalidParameterException("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_invalidrequestexception_status() {
        let err = SecretsManagerError::InvalidRequestException("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_invalidaction_status() {
        let err = SecretsManagerError::InvalidAction("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_into_response() {
        let err = SecretsManagerError::ResourceNotFoundException("test error".to_string());
        let resp = err.into_response();
        assert!(resp.status().is_client_error());
    }
}
