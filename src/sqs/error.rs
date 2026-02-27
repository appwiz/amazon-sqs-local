use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;

#[derive(Debug, Clone)]
pub enum SqsError {
    QueueAlreadyExists(String),
    QueueDoesNotExist(String),
    InvalidAttributeName(String),
    InvalidAttributeValue(String),
    InvalidParameterValue(String),
    PurgeQueueInProgress(String),
    MessageNotInflight(String),
    OverLimit(String),
    EmptyBatchRequest(String),
    TooManyEntriesInBatchRequest(String),
    BatchEntryIdsNotDistinct(String),
    InvalidBatchEntryId(String),
    ResourceNotFoundException(String),
    MissingParameter(String),
    InvalidAction(String),
}

impl SqsError {
    fn error_code(&self) -> &str {
        match self {
            SqsError::QueueAlreadyExists(_) => "QueueAlreadyExists",
            SqsError::QueueDoesNotExist(_) => "QueueDoesNotExist",
            SqsError::InvalidAttributeName(_) => "InvalidAttributeName",
            SqsError::InvalidAttributeValue(_) => "InvalidAttributeValue",
            SqsError::InvalidParameterValue(_) => "InvalidParameterValue",
            SqsError::PurgeQueueInProgress(_) => "PurgeQueueInProgress",
            SqsError::MessageNotInflight(_) => "MessageNotInflight",
            SqsError::OverLimit(_) => "OverLimit",
            SqsError::EmptyBatchRequest(_) => "EmptyBatchRequest",
            SqsError::TooManyEntriesInBatchRequest(_) => "TooManyEntriesInBatchRequest",
            SqsError::BatchEntryIdsNotDistinct(_) => "BatchEntryIdsNotDistinct",
            SqsError::InvalidBatchEntryId(_) => "InvalidBatchEntryId",
            SqsError::ResourceNotFoundException(_) => "ResourceNotFoundException",
            SqsError::MissingParameter(_) => "MissingParameter",
            SqsError::InvalidAction(_) => "InvalidAction",
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            SqsError::QueueAlreadyExists(_) => StatusCode::CONFLICT,
            SqsError::PurgeQueueInProgress(_) => StatusCode::CONFLICT,
            SqsError::ResourceNotFoundException(_) => StatusCode::NOT_FOUND,
            SqsError::OverLimit(_) => StatusCode::FORBIDDEN,
            _ => StatusCode::BAD_REQUEST,
        }
    }

    fn message(&self) -> &str {
        match self {
            SqsError::QueueAlreadyExists(m)
            | SqsError::QueueDoesNotExist(m)
            | SqsError::InvalidAttributeName(m)
            | SqsError::InvalidAttributeValue(m)
            | SqsError::InvalidParameterValue(m)
            | SqsError::PurgeQueueInProgress(m)
            | SqsError::MessageNotInflight(m)
            | SqsError::OverLimit(m)
            | SqsError::EmptyBatchRequest(m)
            | SqsError::TooManyEntriesInBatchRequest(m)
            | SqsError::BatchEntryIdsNotDistinct(m)
            | SqsError::InvalidBatchEntryId(m)
            | SqsError::ResourceNotFoundException(m)
            | SqsError::MissingParameter(m)
            | SqsError::InvalidAction(m) => m,
        }
    }
}

impl IntoResponse for SqsError {
    fn into_response(self) -> Response {
        let body = json!({
            "__type": format!("com.amazonaws.sqs#{}", self.error_code()),
            "message": self.message(),
        });
        (self.status_code(), axum::Json(body)).into_response()
    }
}
