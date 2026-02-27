use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;

#[derive(Debug, Clone)]
pub enum ConfigError {
    NoSuchConfigurationRecorderException(String),
    NoSuchDeliveryChannelException(String),
    NoSuchConfigRuleException(String),
    MaxNumberOfConfigurationRecordersExceededException(String),
    MaxNumberOfDeliveryChannelsExceededException(String),
    InvalidParameterValueException(String),
    ResourceNotFoundException(String),
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
            ConfigError::InvalidParameterValueException(_) => "InvalidParameterValueException",
            ConfigError::ResourceNotFoundException(_) => "ResourceNotFoundException",
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
            ConfigError::InvalidParameterValueException(_) => StatusCode::BAD_REQUEST,
            ConfigError::ResourceNotFoundException(_) => StatusCode::BAD_REQUEST,
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
            | ConfigError::InvalidParameterValueException(m)
            | ConfigError::ResourceNotFoundException(m)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nosuchconfigurationrecorderexception_error_code() {
        let err = ConfigError::NoSuchConfigurationRecorderException("test".to_string());
        assert_eq!(err.error_code(), "NoSuchConfigurationRecorderException");
    }
    #[test]
    fn test_nosuchdeliverychannelexception_error_code() {
        let err = ConfigError::NoSuchDeliveryChannelException("test".to_string());
        assert_eq!(err.error_code(), "NoSuchDeliveryChannelException");
    }
    #[test]
    fn test_nosuchconfigruleexception_error_code() {
        let err = ConfigError::NoSuchConfigRuleException("test".to_string());
        assert_eq!(err.error_code(), "NoSuchConfigRuleException");
    }
    #[test]
    fn test_maxnumberofconfigurationrecordersexceededexception_error_code() {
        let err = ConfigError::MaxNumberOfConfigurationRecordersExceededException("test".to_string());
        assert_eq!(err.error_code(), "MaxNumberOfConfigurationRecordersExceededException");
    }
    #[test]
    fn test_maxnumberofdeliverychannelsexceededexception_error_code() {
        let err = ConfigError::MaxNumberOfDeliveryChannelsExceededException("test".to_string());
        assert_eq!(err.error_code(), "MaxNumberOfDeliveryChannelsExceededException");
    }
    #[test]
    fn test_invalidparametervalueexception_error_code() {
        let err = ConfigError::InvalidParameterValueException("test".to_string());
        assert_eq!(err.error_code(), "InvalidParameterValueException");
    }
    #[test]
    fn test_resourcenotfoundexception_error_code() {
        let err = ConfigError::ResourceNotFoundException("test".to_string());
        assert_eq!(err.error_code(), "ResourceNotFoundException");
    }
    #[test]
    fn test_invalidaction_error_code() {
        let err = ConfigError::InvalidAction("test".to_string());
        assert_eq!(err.error_code(), "InvalidAction");
    }
    #[test]
    fn test_message() {
        let err = ConfigError::NoSuchConfigurationRecorderException("hello world".to_string());
        assert_eq!(err.message(), "hello world");
    }
    #[test]
    fn test_nosuchconfigurationrecorderexception_status() {
        let err = ConfigError::NoSuchConfigurationRecorderException("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_nosuchdeliverychannelexception_status() {
        let err = ConfigError::NoSuchDeliveryChannelException("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_nosuchconfigruleexception_status() {
        let err = ConfigError::NoSuchConfigRuleException("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_maxnumberofconfigurationrecordersexceededexception_status() {
        let err = ConfigError::MaxNumberOfConfigurationRecordersExceededException("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_maxnumberofdeliverychannelsexceededexception_status() {
        let err = ConfigError::MaxNumberOfDeliveryChannelsExceededException("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_invalidparametervalueexception_status() {
        let err = ConfigError::InvalidParameterValueException("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_resourcenotfoundexception_status() {
        let err = ConfigError::ResourceNotFoundException("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_invalidaction_status() {
        let err = ConfigError::InvalidAction("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_into_response() {
        let err = ConfigError::NoSuchConfigurationRecorderException("test error".to_string());
        let resp = err.into_response();
        assert!(resp.status().is_client_error());
    }
}
