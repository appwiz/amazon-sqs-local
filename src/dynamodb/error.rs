use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;

#[derive(Debug, Clone)]
pub enum DynamoDbError {
    ResourceNotFoundException(String),
    ResourceInUseException(String),
    ValidationException(String),
    SerializationException(String),
    ConditionalCheckFailedException(String),
}

impl DynamoDbError {
    fn error_code(&self) -> &str {
        match self {
            DynamoDbError::ResourceNotFoundException(_) => "ResourceNotFoundException",
            DynamoDbError::ResourceInUseException(_) => "ResourceInUseException",
            DynamoDbError::ValidationException(_) => "ValidationException",
            DynamoDbError::SerializationException(_) => "SerializationException",
            DynamoDbError::ConditionalCheckFailedException(_) => "ConditionalCheckFailedException",
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            DynamoDbError::ResourceNotFoundException(_) => StatusCode::BAD_REQUEST,
            DynamoDbError::ResourceInUseException(_) => StatusCode::BAD_REQUEST,
            DynamoDbError::ValidationException(_) => StatusCode::BAD_REQUEST,
            DynamoDbError::SerializationException(_) => StatusCode::BAD_REQUEST,
            DynamoDbError::ConditionalCheckFailedException(_) => StatusCode::BAD_REQUEST,
        }
    }

    fn message(&self) -> &str {
        match self {
            DynamoDbError::ResourceNotFoundException(m)
            | DynamoDbError::ResourceInUseException(m)
            | DynamoDbError::ValidationException(m)
            | DynamoDbError::SerializationException(m)
            | DynamoDbError::ConditionalCheckFailedException(m) => m,
        }
    }
}

impl IntoResponse for DynamoDbError {
    fn into_response(self) -> Response {
        let body = json!({
            "__type": format!("com.amazonaws.dynamodb.v20120810#{}", self.error_code()),
            "message": self.message(),
        });
        (self.status_code(), axum::Json(body)).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resourcenotfoundexception_error_code() {
        let err = DynamoDbError::ResourceNotFoundException("test".to_string());
        assert_eq!(err.error_code(), "ResourceNotFoundException");
    }
    #[test]
    fn test_resourceinuseexception_error_code() {
        let err = DynamoDbError::ResourceInUseException("test".to_string());
        assert_eq!(err.error_code(), "ResourceInUseException");
    }
    #[test]
    fn test_validationexception_error_code() {
        let err = DynamoDbError::ValidationException("test".to_string());
        assert_eq!(err.error_code(), "ValidationException");
    }
    #[test]
    fn test_serializationexception_error_code() {
        let err = DynamoDbError::SerializationException("test".to_string());
        assert_eq!(err.error_code(), "SerializationException");
    }
    #[test]
    fn test_message() {
        let err = DynamoDbError::ResourceNotFoundException("hello world".to_string());
        assert_eq!(err.message(), "hello world");
    }
    #[test]
    fn test_resourcenotfoundexception_status() {
        let err = DynamoDbError::ResourceNotFoundException("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_resourceinuseexception_status() {
        let err = DynamoDbError::ResourceInUseException("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_validationexception_status() {
        let err = DynamoDbError::ValidationException("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_serializationexception_status() {
        let err = DynamoDbError::SerializationException("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_into_response() {
        let err = DynamoDbError::ResourceNotFoundException("test error".to_string());
        let resp = err.into_response();
        assert!(resp.status().is_client_error());
    }
}
