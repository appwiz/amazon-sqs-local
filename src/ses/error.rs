use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;

#[derive(Debug, Clone)]
pub enum SesError {
    NotFoundException(String),
    AlreadyExistsException(String),
    BadRequestException(String),
}

impl SesError {
    fn status_code(&self) -> StatusCode {
        match self {
            SesError::NotFoundException(_) => StatusCode::NOT_FOUND,
            SesError::AlreadyExistsException(_) => StatusCode::CONFLICT,
            SesError::BadRequestException(_) => StatusCode::BAD_REQUEST,
        }
    }

    fn message(&self) -> &str {
        match self {
            SesError::NotFoundException(m)
            | SesError::AlreadyExistsException(m)
            | SesError::BadRequestException(m) => m,
        }
    }
}

impl SesError {
    fn error_code(&self) -> &'static str {
        match self {
            SesError::NotFoundException(_) => "NotFoundException",
            SesError::AlreadyExistsException(_) => "AlreadyExistsException",
            SesError::BadRequestException(_) => "BadRequestException",
        }
    }
}

impl IntoResponse for SesError {
    fn into_response(self) -> Response {
        let error_code = self.error_code();
        let body = json!({
            "__type": error_code,
            "message": self.message(),
        });
        let mut resp = (self.status_code(), axum::Json(body)).into_response();
        resp.headers_mut().insert(
            "x-amzn-ErrorType",
            axum::http::HeaderValue::from_str(error_code).unwrap(),
        );
        resp
    }
}
