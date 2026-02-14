use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum DynamoDbError {
    ResourceNotFoundException(String),
    ResourceInUseException(String),
    ValidationException(String),
    ConditionalCheckFailedException(String),
    ItemCollectionSizeLimitExceededException(String),
    ProvisionedThroughputExceededException(String),
    InternalServerError(String),
    SerializationException(String),
}

impl DynamoDbError {
    fn error_code(&self) -> &str {
        match self {
            DynamoDbError::ResourceNotFoundException(_) => "ResourceNotFoundException",
            DynamoDbError::ResourceInUseException(_) => "ResourceInUseException",
            DynamoDbError::ValidationException(_) => "ValidationException",
            DynamoDbError::ConditionalCheckFailedException(_) => "ConditionalCheckFailedException",
            DynamoDbError::ItemCollectionSizeLimitExceededException(_) => {
                "ItemCollectionSizeLimitExceededException"
            }
            DynamoDbError::ProvisionedThroughputExceededException(_) => {
                "ProvisionedThroughputExceededException"
            }
            DynamoDbError::InternalServerError(_) => "InternalServerError",
            DynamoDbError::SerializationException(_) => "SerializationException",
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            DynamoDbError::ResourceNotFoundException(_) => StatusCode::BAD_REQUEST,
            DynamoDbError::ResourceInUseException(_) => StatusCode::BAD_REQUEST,
            DynamoDbError::ValidationException(_) => StatusCode::BAD_REQUEST,
            DynamoDbError::ConditionalCheckFailedException(_) => StatusCode::BAD_REQUEST,
            DynamoDbError::ItemCollectionSizeLimitExceededException(_) => StatusCode::BAD_REQUEST,
            DynamoDbError::ProvisionedThroughputExceededException(_) => StatusCode::BAD_REQUEST,
            DynamoDbError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            DynamoDbError::SerializationException(_) => StatusCode::BAD_REQUEST,
        }
    }

    fn message(&self) -> &str {
        match self {
            DynamoDbError::ResourceNotFoundException(m)
            | DynamoDbError::ResourceInUseException(m)
            | DynamoDbError::ValidationException(m)
            | DynamoDbError::ConditionalCheckFailedException(m)
            | DynamoDbError::ItemCollectionSizeLimitExceededException(m)
            | DynamoDbError::ProvisionedThroughputExceededException(m)
            | DynamoDbError::InternalServerError(m)
            | DynamoDbError::SerializationException(m) => m,
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
