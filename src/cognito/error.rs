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
