use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;

#[derive(Debug, Clone)]
pub enum CognitoError {
    ResourceNotFoundException(String),
    InvalidParameterException(String),
    UsernameExistsException(String),
    UserNotFoundException(String),
    GroupExistsException(String),
    NotAuthorizedException(String),
    InvalidAction(String),
}

impl CognitoError {
    fn error_code(&self) -> &str {
        match self {
            CognitoError::ResourceNotFoundException(_) => "ResourceNotFoundException",
            CognitoError::InvalidParameterException(_) => "InvalidParameterException",
            CognitoError::UsernameExistsException(_) => "UsernameExistsException",
            CognitoError::UserNotFoundException(_) => "UserNotFoundException",
            CognitoError::GroupExistsException(_) => "GroupExistsException",
            CognitoError::NotAuthorizedException(_) => "NotAuthorizedException",
            CognitoError::InvalidAction(_) => "InvalidAction",
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            CognitoError::ResourceNotFoundException(_) => StatusCode::BAD_REQUEST,
            CognitoError::UserNotFoundException(_) => StatusCode::BAD_REQUEST,
            CognitoError::UsernameExistsException(_) => StatusCode::BAD_REQUEST,
            CognitoError::GroupExistsException(_) => StatusCode::BAD_REQUEST,
            CognitoError::NotAuthorizedException(_) => StatusCode::BAD_REQUEST,
            CognitoError::InvalidParameterException(_) => StatusCode::BAD_REQUEST,
            CognitoError::InvalidAction(_) => StatusCode::BAD_REQUEST,
        }
    }

    fn message(&self) -> &str {
        match self {
            CognitoError::ResourceNotFoundException(m)
            | CognitoError::InvalidParameterException(m)
            | CognitoError::UsernameExistsException(m)
            | CognitoError::UserNotFoundException(m)
            | CognitoError::GroupExistsException(m)
            | CognitoError::NotAuthorizedException(m)
            | CognitoError::InvalidAction(m) => m,
        }
    }
}

impl IntoResponse for CognitoError {
    fn into_response(self) -> Response {
        let body = json!({
            "__type": format!("#{}", self.error_code()),
            "message": self.message(),
        });
        (self.status_code(), axum::Json(body)).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resourcenotfoundexception_error_code() {
        let err = CognitoError::ResourceNotFoundException("test".to_string());
        assert_eq!(err.error_code(), "ResourceNotFoundException");
    }
    #[test]
    fn test_invalidparameterexception_error_code() {
        let err = CognitoError::InvalidParameterException("test".to_string());
        assert_eq!(err.error_code(), "InvalidParameterException");
    }
    #[test]
    fn test_usernameexistsexception_error_code() {
        let err = CognitoError::UsernameExistsException("test".to_string());
        assert_eq!(err.error_code(), "UsernameExistsException");
    }
    #[test]
    fn test_usernotfoundexception_error_code() {
        let err = CognitoError::UserNotFoundException("test".to_string());
        assert_eq!(err.error_code(), "UserNotFoundException");
    }
    #[test]
    fn test_groupexistsexception_error_code() {
        let err = CognitoError::GroupExistsException("test".to_string());
        assert_eq!(err.error_code(), "GroupExistsException");
    }
    #[test]
    fn test_notauthorizedexception_error_code() {
        let err = CognitoError::NotAuthorizedException("test".to_string());
        assert_eq!(err.error_code(), "NotAuthorizedException");
    }
    #[test]
    fn test_invalidaction_error_code() {
        let err = CognitoError::InvalidAction("test".to_string());
        assert_eq!(err.error_code(), "InvalidAction");
    }
    #[test]
    fn test_message() {
        let err = CognitoError::ResourceNotFoundException("hello world".to_string());
        assert_eq!(err.message(), "hello world");
    }
    #[test]
    fn test_resourcenotfoundexception_status() {
        let err = CognitoError::ResourceNotFoundException("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_invalidparameterexception_status() {
        let err = CognitoError::InvalidParameterException("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_usernameexistsexception_status() {
        let err = CognitoError::UsernameExistsException("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_usernotfoundexception_status() {
        let err = CognitoError::UserNotFoundException("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_groupexistsexception_status() {
        let err = CognitoError::GroupExistsException("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_notauthorizedexception_status() {
        let err = CognitoError::NotAuthorizedException("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_invalidaction_status() {
        let err = CognitoError::InvalidAction("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_into_response() {
        let err = CognitoError::ResourceNotFoundException("test error".to_string());
        let resp = err.into_response();
        assert!(resp.status().is_client_error());
    }
}
