use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;

#[derive(Debug, Clone)]
pub enum LambdaError {
    ResourceNotFoundException(String),
    ResourceConflictException(String),
    InvalidParameterValueException(String),
    ServiceException(String),
}

impl LambdaError {
    fn error_type(&self) -> &str {
        match self {
            LambdaError::ResourceNotFoundException(_) => "ResourceNotFoundException",
            LambdaError::ResourceConflictException(_) => "ResourceConflictException",
            LambdaError::InvalidParameterValueException(_) => "InvalidParameterValueException",
            LambdaError::ServiceException(_) => "ServiceException",
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            LambdaError::ResourceNotFoundException(_) => StatusCode::NOT_FOUND,
            LambdaError::ResourceConflictException(_) => StatusCode::CONFLICT,
            LambdaError::InvalidParameterValueException(_) => StatusCode::BAD_REQUEST,
            LambdaError::ServiceException(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn message(&self) -> &str {
        match self {
            LambdaError::ResourceNotFoundException(m)
            | LambdaError::ResourceConflictException(m)
            | LambdaError::InvalidParameterValueException(m)
            | LambdaError::ServiceException(m) => m,
        }
    }
}

impl IntoResponse for LambdaError {
    fn into_response(self) -> Response {
        let body = json!({
            "Message": self.message(),
        });
        (
            self.status_code(),
            [("x-amzn-ErrorType", self.error_type())],
            axum::Json(body),
        )
            .into_response()
    }
}
