use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;

#[derive(Debug, Clone)]
pub enum SfnError {
    StateMachineAlreadyExists(String),
    StateMachineDoesNotExist(String),
    ExecutionDoesNotExist(String),
    ExecutionAlreadyExists(String),
    InvalidArn(String),
    InvalidAction(String),
}

impl SfnError {
    fn error_code(&self) -> &str {
        match self {
            SfnError::StateMachineAlreadyExists(_) => "StateMachineAlreadyExists",
            SfnError::StateMachineDoesNotExist(_) => "StateMachineDoesNotExist",
            SfnError::ExecutionDoesNotExist(_) => "ExecutionDoesNotExist",
            SfnError::ExecutionAlreadyExists(_) => "ExecutionAlreadyExists",
            SfnError::InvalidArn(_) => "InvalidArn",
            SfnError::InvalidAction(_) => "InvalidAction",
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            SfnError::StateMachineDoesNotExist(_) => StatusCode::BAD_REQUEST,
            SfnError::ExecutionDoesNotExist(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::BAD_REQUEST,
        }
    }

    fn message(&self) -> &str {
        match self {
            SfnError::StateMachineAlreadyExists(m)
            | SfnError::StateMachineDoesNotExist(m)
            | SfnError::ExecutionDoesNotExist(m)
            | SfnError::ExecutionAlreadyExists(m)
            | SfnError::InvalidArn(m)
            | SfnError::InvalidAction(m) => m,
        }
    }
}

impl IntoResponse for SfnError {
    fn into_response(self) -> Response {
        let body = json!({
            "__type": self.error_code(),
            "message": self.message(),
        });
        (self.status_code(), axum::Json(body)).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_statemachinealreadyexists_error_code() {
        let err = SfnError::StateMachineAlreadyExists("test".to_string());
        assert_eq!(err.error_code(), "StateMachineAlreadyExists");
    }
    #[test]
    fn test_statemachinedoesnotexist_error_code() {
        let err = SfnError::StateMachineDoesNotExist("test".to_string());
        assert_eq!(err.error_code(), "StateMachineDoesNotExist");
    }
    #[test]
    fn test_executiondoesnotexist_error_code() {
        let err = SfnError::ExecutionDoesNotExist("test".to_string());
        assert_eq!(err.error_code(), "ExecutionDoesNotExist");
    }
    #[test]
    fn test_executionalreadyexists_error_code() {
        let err = SfnError::ExecutionAlreadyExists("test".to_string());
        assert_eq!(err.error_code(), "ExecutionAlreadyExists");
    }
    #[test]
    fn test_invalidarn_error_code() {
        let err = SfnError::InvalidArn("test".to_string());
        assert_eq!(err.error_code(), "InvalidArn");
    }
    #[test]
    fn test_invalidaction_error_code() {
        let err = SfnError::InvalidAction("test".to_string());
        assert_eq!(err.error_code(), "InvalidAction");
    }
    #[test]
    fn test_message() {
        let err = SfnError::StateMachineAlreadyExists("hello world".to_string());
        assert_eq!(err.message(), "hello world");
    }
    #[test]
    fn test_statemachinealreadyexists_status() {
        let err = SfnError::StateMachineAlreadyExists("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_statemachinedoesnotexist_status() {
        let err = SfnError::StateMachineDoesNotExist("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_executiondoesnotexist_status() {
        let err = SfnError::ExecutionDoesNotExist("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_executionalreadyexists_status() {
        let err = SfnError::ExecutionAlreadyExists("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_invalidarn_status() {
        let err = SfnError::InvalidArn("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_invalidaction_status() {
        let err = SfnError::InvalidAction("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_into_response() {
        let err = SfnError::StateMachineAlreadyExists("test error".to_string());
        let resp = err.into_response();
        assert!(resp.status().is_client_error());
    }
}
