use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum ConfigError {
    NoSuchConfigurationRecorderException(String),
    NoSuchDeliveryChannelException(String),
    NoSuchConfigRuleException(String),
    MaxNumberOfConfigurationRecordersExceededException(String),
    MaxNumberOfDeliveryChannelsExceededException(String),
    MaxNumberOfConfigRulesExceededException(String),
    InvalidParameterValueException(String),
    ResourceNotFoundException(String),
    ResourceInUseException(String),
    ValidationException(String),
    InvalidAction(String),
}

impl ConfigError {
    fn error_code(&self) -> &str {
        match self {
            ConfigError::NoSuchConfigurationRecorderException(_) => {
                "NoSuchConfigurationRecorderException"
            }
            ConfigError::NoSuchDeliveryChannelException(_) => "NoSuchDeliveryChannelException",
            ConfigError::NoSuchConfigRuleException(_) => "NoSuchConfigRuleException",
            ConfigError::MaxNumberOfConfigurationRecordersExceededException(_) => {
                "MaxNumberOfConfigurationRecordersExceededException"
            }
            ConfigError::MaxNumberOfDeliveryChannelsExceededException(_) => {
                "MaxNumberOfDeliveryChannelsExceededException"
            }
            ConfigError::MaxNumberOfConfigRulesExceededException(_) => {
                "MaxNumberOfConfigRulesExceededException"
            }
            ConfigError::InvalidParameterValueException(_) => "InvalidParameterValueException",
            ConfigError::ResourceNotFoundException(_) => "ResourceNotFoundException",
            ConfigError::ResourceInUseException(_) => "ResourceInUseException",
            ConfigError::ValidationException(_) => "ValidationException",
            ConfigError::InvalidAction(_) => "InvalidAction",
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            ConfigError::NoSuchConfigurationRecorderException(_) => StatusCode::BAD_REQUEST,
            ConfigError::NoSuchDeliveryChannelException(_) => StatusCode::BAD_REQUEST,
            ConfigError::NoSuchConfigRuleException(_) => StatusCode::BAD_REQUEST,
            ConfigError::MaxNumberOfConfigurationRecordersExceededException(_) => {
                StatusCode::BAD_REQUEST
            }
            ConfigError::MaxNumberOfDeliveryChannelsExceededException(_) => StatusCode::BAD_REQUEST,
            ConfigError::MaxNumberOfConfigRulesExceededException(_) => StatusCode::BAD_REQUEST,
            ConfigError::InvalidParameterValueException(_) => StatusCode::BAD_REQUEST,
            ConfigError::ResourceNotFoundException(_) => StatusCode::BAD_REQUEST,
            ConfigError::ResourceInUseException(_) => StatusCode::BAD_REQUEST,
            ConfigError::ValidationException(_) => StatusCode::BAD_REQUEST,
            ConfigError::InvalidAction(_) => StatusCode::BAD_REQUEST,
        }
    }

    fn message(&self) -> &str {
        match self {
            ConfigError::NoSuchConfigurationRecorderException(m)
            | ConfigError::NoSuchDeliveryChannelException(m)
            | ConfigError::NoSuchConfigRuleException(m)
            | ConfigError::MaxNumberOfConfigurationRecordersExceededException(m)
            | ConfigError::MaxNumberOfDeliveryChannelsExceededException(m)
            | ConfigError::MaxNumberOfConfigRulesExceededException(m)
            | ConfigError::InvalidParameterValueException(m)
            | ConfigError::ResourceNotFoundException(m)
            | ConfigError::ResourceInUseException(m)
            | ConfigError::ValidationException(m)
            | ConfigError::InvalidAction(m) => m,
        }
    }
}

impl IntoResponse for ConfigError {
    fn into_response(self) -> Response {
        let body = json!({
            "__type": self.error_code(),
            "Message": self.message(),
        });
        (self.status_code(), axum::Json(body)).into_response()
    }
}
