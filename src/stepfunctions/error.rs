use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum SfnError {
    StateMachineAlreadyExists(String),
    StateMachineDoesNotExist(String),
    ExecutionDoesNotExist(String),
    ExecutionAlreadyExists(String),
    InvalidArn(String),
    InvalidDefinition(String),
    InvalidExecutionInput(String),
    InvalidName(String),
    TaskDoesNotExist(String),
    TaskTimedOut(String),
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
            SfnError::InvalidDefinition(_) => "InvalidDefinition",
            SfnError::InvalidExecutionInput(_) => "InvalidExecutionInput",
            SfnError::InvalidName(_) => "InvalidName",
            SfnError::TaskDoesNotExist(_) => "TaskDoesNotExist",
            SfnError::TaskTimedOut(_) => "TaskTimedOut",
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
            | SfnError::InvalidDefinition(m)
            | SfnError::InvalidExecutionInput(m)
            | SfnError::InvalidName(m)
            | SfnError::TaskDoesNotExist(m)
            | SfnError::TaskTimedOut(m)
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
