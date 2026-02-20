use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum SesError {
    NotFoundException(String),
    AlreadyExistsException(String),
    SendingPausedException(String),
    MessageRejected(String),
    InvalidParameterException(String),
    TooManyRequestsException(String),
    BadRequestException(String),
}

impl SesError {
    fn error_code(&self) -> &str {
        match self {
            SesError::NotFoundException(_) => "NotFoundException",
            SesError::AlreadyExistsException(_) => "AlreadyExistsException",
            SesError::SendingPausedException(_) => "SendingPausedException",
            SesError::MessageRejected(_) => "MessageRejected",
            SesError::InvalidParameterException(_) => "InvalidParameterException",
            SesError::TooManyRequestsException(_) => "TooManyRequestsException",
            SesError::BadRequestException(_) => "BadRequestException",
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            SesError::NotFoundException(_) => StatusCode::NOT_FOUND,
            SesError::AlreadyExistsException(_) => StatusCode::CONFLICT,
            SesError::TooManyRequestsException(_) => StatusCode::TOO_MANY_REQUESTS,
            _ => StatusCode::BAD_REQUEST,
        }
    }

    fn message(&self) -> &str {
        match self {
            SesError::NotFoundException(m)
            | SesError::AlreadyExistsException(m)
            | SesError::SendingPausedException(m)
            | SesError::MessageRejected(m)
            | SesError::InvalidParameterException(m)
            | SesError::TooManyRequestsException(m)
            | SesError::BadRequestException(m) => m,
        }
    }
}

impl IntoResponse for SesError {
    fn into_response(self) -> Response {
        let body = json!({
            "message": self.message(),
        });
        let mut resp = (self.status_code(), axum::Json(body)).into_response();
        resp.headers_mut().insert(
            "x-amzn-ErrorType",
            axum::http::HeaderValue::from_static("SesError"),
        );
        resp
    }
}
