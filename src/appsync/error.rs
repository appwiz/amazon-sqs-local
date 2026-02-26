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
