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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queuealreadyexists_error_code() {
        let err = SqsError::QueueAlreadyExists("test".to_string());
        assert_eq!(err.error_code(), "QueueAlreadyExists");
    }
    #[test]
    fn test_queuedoesnotexist_error_code() {
        let err = SqsError::QueueDoesNotExist("test".to_string());
        assert_eq!(err.error_code(), "QueueDoesNotExist");
    }
    #[test]
    fn test_invalidattributename_error_code() {
        let err = SqsError::InvalidAttributeName("test".to_string());
        assert_eq!(err.error_code(), "InvalidAttributeName");
    }
    #[test]
    fn test_invalidattributevalue_error_code() {
        let err = SqsError::InvalidAttributeValue("test".to_string());
        assert_eq!(err.error_code(), "InvalidAttributeValue");
    }
    #[test]
    fn test_invalidparametervalue_error_code() {
        let err = SqsError::InvalidParameterValue("test".to_string());
        assert_eq!(err.error_code(), "InvalidParameterValue");
    }
    #[test]
    fn test_purgequeueinprogress_error_code() {
        let err = SqsError::PurgeQueueInProgress("test".to_string());
        assert_eq!(err.error_code(), "PurgeQueueInProgress");
    }
    #[test]
    fn test_messagenotinflight_error_code() {
        let err = SqsError::MessageNotInflight("test".to_string());
        assert_eq!(err.error_code(), "MessageNotInflight");
    }
    #[test]
    fn test_overlimit_error_code() {
        let err = SqsError::OverLimit("test".to_string());
        assert_eq!(err.error_code(), "OverLimit");
    }
    #[test]
    fn test_emptybatchrequest_error_code() {
        let err = SqsError::EmptyBatchRequest("test".to_string());
        assert_eq!(err.error_code(), "EmptyBatchRequest");
    }
    #[test]
    fn test_toomanyentriesinbatchrequest_error_code() {
        let err = SqsError::TooManyEntriesInBatchRequest("test".to_string());
        assert_eq!(err.error_code(), "TooManyEntriesInBatchRequest");
    }
    #[test]
    fn test_batchentryidsnotdistinct_error_code() {
        let err = SqsError::BatchEntryIdsNotDistinct("test".to_string());
        assert_eq!(err.error_code(), "BatchEntryIdsNotDistinct");
    }
    #[test]
    fn test_invalidbatchentryid_error_code() {
        let err = SqsError::InvalidBatchEntryId("test".to_string());
        assert_eq!(err.error_code(), "InvalidBatchEntryId");
    }
    #[test]
    fn test_resourcenotfoundexception_error_code() {
        let err = SqsError::ResourceNotFoundException("test".to_string());
        assert_eq!(err.error_code(), "ResourceNotFoundException");
    }
    #[test]
    fn test_missingparameter_error_code() {
        let err = SqsError::MissingParameter("test".to_string());
        assert_eq!(err.error_code(), "MissingParameter");
    }
    #[test]
    fn test_invalidaction_error_code() {
        let err = SqsError::InvalidAction("test".to_string());
        assert_eq!(err.error_code(), "InvalidAction");
    }
    #[test]
    fn test_message() {
        let err = SqsError::QueueAlreadyExists("hello world".to_string());
        assert_eq!(err.message(), "hello world");
    }
    #[test]
    fn test_queuealreadyexists_status() {
        let err = SqsError::QueueAlreadyExists("test".to_string());
        assert_eq!(err.status_code(), StatusCode::CONFLICT);
    }
    #[test]
    fn test_queuedoesnotexist_status() {
        let err = SqsError::QueueDoesNotExist("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_invalidattributename_status() {
        let err = SqsError::InvalidAttributeName("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_invalidattributevalue_status() {
        let err = SqsError::InvalidAttributeValue("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_invalidparametervalue_status() {
        let err = SqsError::InvalidParameterValue("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_purgequeueinprogress_status() {
        let err = SqsError::PurgeQueueInProgress("test".to_string());
        assert_eq!(err.status_code(), StatusCode::CONFLICT);
    }
    #[test]
    fn test_messagenotinflight_status() {
        let err = SqsError::MessageNotInflight("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_overlimit_status() {
        let err = SqsError::OverLimit("test".to_string());
        assert_eq!(err.status_code(), StatusCode::FORBIDDEN);
    }
    #[test]
    fn test_emptybatchrequest_status() {
        let err = SqsError::EmptyBatchRequest("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_toomanyentriesinbatchrequest_status() {
        let err = SqsError::TooManyEntriesInBatchRequest("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_batchentryidsnotdistinct_status() {
        let err = SqsError::BatchEntryIdsNotDistinct("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_invalidbatchentryid_status() {
        let err = SqsError::InvalidBatchEntryId("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_resourcenotfoundexception_status() {
        let err = SqsError::ResourceNotFoundException("test".to_string());
        assert_eq!(err.status_code(), StatusCode::NOT_FOUND);
    }
    #[test]
    fn test_missingparameter_status() {
        let err = SqsError::MissingParameter("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_invalidaction_status() {
        let err = SqsError::InvalidAction("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_into_response() {
        let err = SqsError::QueueAlreadyExists("test error".to_string());
        let resp = err.into_response();
        assert!(resp.status().is_client_error());
    }
}
