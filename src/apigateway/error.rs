use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum ApiGatewayError {
    NotFoundException(String),
    ConflictException(String),
    BadRequestException(String),
    TooManyRequestsException(String),
    UnauthorizedException(String),
    ServiceUnavailableException(String),
}

impl ApiGatewayError {
    fn error_type(&self) -> &str {
        match self {
            ApiGatewayError::NotFoundException(_) => "NotFoundException",
            ApiGatewayError::ConflictException(_) => "ConflictException",
            ApiGatewayError::BadRequestException(_) => "BadRequestException",
            ApiGatewayError::TooManyRequestsException(_) => "TooManyRequestsException",
            ApiGatewayError::UnauthorizedException(_) => "UnauthorizedException",
            ApiGatewayError::ServiceUnavailableException(_) => "ServiceUnavailableException",
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            ApiGatewayError::NotFoundException(_) => StatusCode::NOT_FOUND,
            ApiGatewayError::ConflictException(_) => StatusCode::CONFLICT,
            ApiGatewayError::BadRequestException(_) => StatusCode::BAD_REQUEST,
            ApiGatewayError::TooManyRequestsException(_) => StatusCode::TOO_MANY_REQUESTS,
            ApiGatewayError::UnauthorizedException(_) => StatusCode::UNAUTHORIZED,
            ApiGatewayError::ServiceUnavailableException(_) => StatusCode::SERVICE_UNAVAILABLE,
        }
    }

    fn message(&self) -> &str {
        match self {
            ApiGatewayError::NotFoundException(m)
            | ApiGatewayError::ConflictException(m)
            | ApiGatewayError::BadRequestException(m)
            | ApiGatewayError::TooManyRequestsException(m)
            | ApiGatewayError::UnauthorizedException(m)
            | ApiGatewayError::ServiceUnavailableException(m) => m,
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
