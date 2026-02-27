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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notfoundexception_error_code() {
        let err = SesError::NotFoundException("test".to_string());
        assert_eq!(err.error_code(), "NotFoundException");
    }
    #[test]
    fn test_alreadyexistsexception_error_code() {
        let err = SesError::AlreadyExistsException("test".to_string());
        assert_eq!(err.error_code(), "AlreadyExistsException");
    }
    #[test]
    fn test_badrequestexception_error_code() {
        let err = SesError::BadRequestException("test".to_string());
        assert_eq!(err.error_code(), "BadRequestException");
    }
    #[test]
    fn test_message() {
        let err = SesError::NotFoundException("hello".to_string());
        assert_eq!(err.message(), "hello");
    }
    #[test]
    fn test_notfoundexception_status() {
        let err = SesError::NotFoundException("test".to_string());
        assert_eq!(err.status_code(), StatusCode::NOT_FOUND);
    }
    #[test]
    fn test_alreadyexistsexception_status() {
        let err = SesError::AlreadyExistsException("test".to_string());
        assert_eq!(err.status_code(), StatusCode::CONFLICT);
    }
    #[test]
    fn test_into_response() {
        let err = SesError::NotFoundException("test".to_string());
        let resp = err.into_response();
        assert!(resp.status().is_client_error() || resp.status().is_server_error());
    }
}
