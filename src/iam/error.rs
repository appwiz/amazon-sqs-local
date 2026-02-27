use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum IAMError {
    ResourceNotFoundException(String),
    ResourceAlreadyExistsException(String),
    ValidationException(String),
    InvalidAction(String),
}

impl IAMError {
    #[allow(dead_code)]
    fn error_code(&self) -> &str {
        match self {
            IAMError::ResourceNotFoundException(_) => "ResourceNotFoundException",
            IAMError::ResourceAlreadyExistsException(_) => "ResourceAlreadyExistsException",
            IAMError::ValidationException(_) => "ValidationException",
            IAMError::InvalidAction(_) => "InvalidAction",
        }
    }

    #[allow(dead_code)]
    fn status_code(&self) -> StatusCode {
        match self {
            IAMError::ResourceNotFoundException(_) => StatusCode::NOT_FOUND,
            _ => StatusCode::BAD_REQUEST,
        }
    }

    #[allow(dead_code)]
    fn message(&self) -> &str {
        match self {
            IAMError::ResourceNotFoundException(m)
            | IAMError::ResourceAlreadyExistsException(m)
            | IAMError::ValidationException(m)
            | IAMError::InvalidAction(m) => m,
        }
    }
}

impl IntoResponse for IAMError {
    #[allow(dead_code)]
    fn into_response(self) -> Response {
        let body = format!(
            r#"<ErrorResponse xmlns="https://iam.amazonaws.com/doc/2012-10-01/">
  <Error>
    <Code>{}</Code>
    <Message>{}</Message>
  </Error>
  <RequestId>{}</RequestId>
</ErrorResponse>"#,
            self.error_code(),
            self.message(),
            uuid::Uuid::new_v4(),
        );
        (self.status_code(), [("content-type", "text/xml")], body).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resourcenotfoundexception_error_code() {
        let err = IAMError::ResourceNotFoundException("test".to_string());
        assert_eq!(err.error_code(), "ResourceNotFoundException");
    }
    #[test]
    fn test_resourcealreadyexistsexception_error_code() {
        let err = IAMError::ResourceAlreadyExistsException("test".to_string());
        assert_eq!(err.error_code(), "ResourceAlreadyExistsException");
    }
    #[test]
    fn test_validationexception_error_code() {
        let err = IAMError::ValidationException("test".to_string());
        assert_eq!(err.error_code(), "ValidationException");
    }
    #[test]
    fn test_invalidaction_error_code() {
        let err = IAMError::InvalidAction("test".to_string());
        assert_eq!(err.error_code(), "InvalidAction");
    }
    #[test]
    fn test_message() {
        let err = IAMError::ResourceNotFoundException("hello world".to_string());
        assert_eq!(err.message(), "hello world");
    }
    #[test]
    fn test_resourcenotfoundexception_status() {
        let err = IAMError::ResourceNotFoundException("test".to_string());
        assert_eq!(err.status_code(), StatusCode::NOT_FOUND);
    }
    #[test]
    fn test_resourcealreadyexistsexception_status() {
        let err = IAMError::ResourceAlreadyExistsException("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_validationexception_status() {
        let err = IAMError::ValidationException("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_invalidaction_status() {
        let err = IAMError::InvalidAction("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_into_response() {
        let err = IAMError::ResourceNotFoundException("test error".to_string());
        let resp = err.into_response();
        assert!(resp.status().is_client_error());
    }
}
