use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;

#[derive(Debug, Clone)]
pub enum AppSyncError {
    NotFoundException(String),
    BadRequestException(String),
    ConcurrentModificationException(String),
}

impl AppSyncError {
    fn error_code(&self) -> &str {
        match self {
            AppSyncError::NotFoundException(_) => "NotFoundException",
            AppSyncError::BadRequestException(_) => "BadRequestException",
            AppSyncError::ConcurrentModificationException(_) => {
                "ConcurrentModificationException"
            }
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            AppSyncError::NotFoundException(_) => StatusCode::NOT_FOUND,
            AppSyncError::BadRequestException(_) => StatusCode::BAD_REQUEST,
            AppSyncError::ConcurrentModificationException(_) => StatusCode::CONFLICT,
        }
    }

    fn message(&self) -> &str {
        match self {
            AppSyncError::NotFoundException(m)
            | AppSyncError::BadRequestException(m)
            | AppSyncError::ConcurrentModificationException(m) => m,
        }
    }
}

impl IntoResponse for AppSyncError {
    fn into_response(self) -> Response {
        let body = json!({
            "message": self.message(),
        });
        let mut resp = (self.status_code(), axum::Json(body)).into_response();
        resp.headers_mut().insert(
            "x-amzn-ErrorType",
            axum::http::HeaderValue::from_str(self.error_code()).unwrap(),
        );
        resp
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notfoundexception_error_code() {
        let err = AppSyncError::NotFoundException("test".to_string());
        assert_eq!(err.error_code(), "NotFoundException");
    }
    #[test]
    fn test_badrequestexception_error_code() {
        let err = AppSyncError::BadRequestException("test".to_string());
        assert_eq!(err.error_code(), "BadRequestException");
    }
    #[test]
    fn test_concurrentmodificationexception_error_code() {
        let err = AppSyncError::ConcurrentModificationException("test".to_string());
        assert_eq!(err.error_code(), "ConcurrentModificationException");
    }
    #[test]
    fn test_message() {
        let err = AppSyncError::NotFoundException("hello world".to_string());
        assert_eq!(err.message(), "hello world");
    }
    #[test]
    fn test_notfoundexception_status() {
        let err = AppSyncError::NotFoundException("test".to_string());
        assert_eq!(err.status_code(), StatusCode::NOT_FOUND);
    }
    #[test]
    fn test_badrequestexception_status() {
        let err = AppSyncError::BadRequestException("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_concurrentmodificationexception_status() {
        let err = AppSyncError::ConcurrentModificationException("test".to_string());
        assert_eq!(err.status_code(), StatusCode::CONFLICT);
    }
    #[test]
    fn test_into_response() {
        let err = AppSyncError::NotFoundException("test error".to_string());
        let resp = err.into_response();
        assert!(resp.status().is_client_error());
    }
}
