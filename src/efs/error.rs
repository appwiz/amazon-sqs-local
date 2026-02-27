use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;

#[derive(Debug, Clone)]
pub enum EfsError {
    FileSystemNotFound(String),
    FileSystemInUse(String),
    MountTargetNotFound(String),
    MountTargetConflict(String),
    AccessPointNotFound(String),
    BadRequest(String),
}

impl EfsError {
    fn error_code(&self) -> &str {
        match self {
            EfsError::FileSystemNotFound(_) => "FileSystemNotFound",
            EfsError::FileSystemInUse(_) => "FileSystemInUse",
            EfsError::MountTargetNotFound(_) => "MountTargetNotFound",
            EfsError::MountTargetConflict(_) => "MountTargetConflict",
            EfsError::AccessPointNotFound(_) => "AccessPointNotFound",
            EfsError::BadRequest(_) => "BadRequest",
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            EfsError::FileSystemNotFound(_) => StatusCode::NOT_FOUND,
            EfsError::MountTargetNotFound(_) => StatusCode::NOT_FOUND,
            EfsError::AccessPointNotFound(_) => StatusCode::NOT_FOUND,
            EfsError::MountTargetConflict(_) => StatusCode::CONFLICT,
            EfsError::FileSystemInUse(_) => StatusCode::CONFLICT,
            EfsError::BadRequest(_) => StatusCode::BAD_REQUEST,
        }
    }

    fn message(&self) -> &str {
        match self {
            EfsError::FileSystemNotFound(m)
            | EfsError::FileSystemInUse(m)
            | EfsError::MountTargetNotFound(m)
            | EfsError::MountTargetConflict(m)
            | EfsError::AccessPointNotFound(m)
            | EfsError::BadRequest(m) => m,
        }
    }
}

impl IntoResponse for EfsError {
    fn into_response(self) -> Response {
        let body = json!({
            "ErrorCode": self.error_code(),
            "Message": self.message(),
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
    fn test_filesystemnotfound_error_code() {
        let err = EfsError::FileSystemNotFound("test".to_string());
        assert_eq!(err.error_code(), "FileSystemNotFound");
    }
    #[test]
    fn test_filesysteminuse_error_code() {
        let err = EfsError::FileSystemInUse("test".to_string());
        assert_eq!(err.error_code(), "FileSystemInUse");
    }
    #[test]
    fn test_mounttargetnotfound_error_code() {
        let err = EfsError::MountTargetNotFound("test".to_string());
        assert_eq!(err.error_code(), "MountTargetNotFound");
    }
    #[test]
    fn test_message() {
        let err = EfsError::FileSystemNotFound("hello".to_string());
        assert_eq!(err.message(), "hello");
    }
    #[test]
    fn test_filesystemnotfound_status() {
        let err = EfsError::FileSystemNotFound("test".to_string());
        assert_eq!(err.status_code(), StatusCode::NOT_FOUND);
    }
    #[test]
    fn test_filesysteminuse_status() {
        let err = EfsError::FileSystemInUse("test".to_string());
        assert_eq!(err.status_code(), StatusCode::CONFLICT);
    }
    #[test]
    fn test_into_response() {
        let err = EfsError::FileSystemNotFound("test".to_string());
        let resp = err.into_response();
        assert!(resp.status().is_client_error() || resp.status().is_server_error());
    }
}
