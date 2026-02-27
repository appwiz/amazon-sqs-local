use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;

#[derive(Debug, Clone)]
pub enum SsmError {
    ParameterNotFound(String),
    ParameterAlreadyExists(String),
    InvalidAction(String),
}

impl SsmError {
    fn error_code(&self) -> &str {
        match self {
            SsmError::ParameterNotFound(_) => "ParameterNotFound",
            SsmError::ParameterAlreadyExists(_) => "ParameterAlreadyExists",
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parameternotfound_error_code() {
        let err = SsmError::ParameterNotFound("test".to_string());
        assert_eq!(err.error_code(), "ParameterNotFound");
    }
    #[test]
    fn test_parameteralreadyexists_error_code() {
        let err = SsmError::ParameterAlreadyExists("test".to_string());
        assert_eq!(err.error_code(), "ParameterAlreadyExists");
    }
    #[test]
    fn test_invalidaction_error_code() {
        let err = SsmError::InvalidAction("test".to_string());
        assert_eq!(err.error_code(), "InvalidAction");
    }
    #[test]
    fn test_message() {
        let err = SsmError::ParameterNotFound("hello world".to_string());
        assert_eq!(err.message(), "hello world");
    }
    #[test]
    fn test_parameternotfound_status() {
        let err = SsmError::ParameterNotFound("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_parameteralreadyexists_status() {
        let err = SsmError::ParameterAlreadyExists("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_invalidaction_status() {
        let err = SsmError::InvalidAction("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_into_response() {
        let err = SsmError::ParameterNotFound("test error".to_string());
        let resp = err.into_response();
        assert!(resp.status().is_client_error());
    }
}
