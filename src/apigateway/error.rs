use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;

#[derive(Debug, Clone)]
pub enum ApiGatewayError {
    NotFoundException(String),
    ConflictException(String),
    BadRequestException(String),
}

impl ApiGatewayError {
    fn error_type(&self) -> &str {
        match self {
            ApiGatewayError::NotFoundException(_) => "NotFoundException",
            ApiGatewayError::ConflictException(_) => "ConflictException",
            ApiGatewayError::BadRequestException(_) => "BadRequestException",
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            ApiGatewayError::NotFoundException(_) => StatusCode::NOT_FOUND,
            ApiGatewayError::ConflictException(_) => StatusCode::CONFLICT,
            ApiGatewayError::BadRequestException(_) => StatusCode::BAD_REQUEST,
        }
    }

    fn message(&self) -> &str {
        match self {
            ApiGatewayError::NotFoundException(m)
            | ApiGatewayError::ConflictException(m)
            | ApiGatewayError::BadRequestException(m) => m,
        }
    }
}

impl IntoResponse for ApiGatewayError {
    fn into_response(self) -> Response {
        let body = json!({
            "message": self.message(),
        });
        (
            self.status_code(),
            [("x-amzn-ErrorType", self.error_type())],
            axum::Json(body),
        )
            .into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message() {
        let err = ApiGatewayError::NotFoundException("hello world".to_string());
        assert_eq!(err.message(), "hello world");
    }
    #[test]
    fn test_notfoundexception_status() {
        let err = ApiGatewayError::NotFoundException("test".to_string());
        assert_eq!(err.status_code(), StatusCode::NOT_FOUND);
    }
    #[test]
    fn test_conflictexception_status() {
        let err = ApiGatewayError::ConflictException("test".to_string());
        assert_eq!(err.status_code(), StatusCode::CONFLICT);
    }
    #[test]
    fn test_badrequestexception_status() {
        let err = ApiGatewayError::BadRequestException("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_into_response() {
        let err = ApiGatewayError::NotFoundException("test error".to_string());
        let resp = err.into_response();
        assert!(resp.status().is_client_error());
    }
}
