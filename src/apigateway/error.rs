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
