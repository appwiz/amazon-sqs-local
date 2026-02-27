use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum EC2Error {
    ResourceNotFoundException(String),
    ResourceAlreadyExistsException(String),
    ValidationException(String),
    InvalidAction(String),
}

impl EC2Error {
    #[allow(dead_code)]
    fn error_code(&self) -> &str {
        match self {
            EC2Error::ResourceNotFoundException(_) => "ResourceNotFoundException",
            EC2Error::ResourceAlreadyExistsException(_) => "ResourceAlreadyExistsException",
            EC2Error::ValidationException(_) => "ValidationException",
            EC2Error::InvalidAction(_) => "InvalidAction",
        }
    }

    #[allow(dead_code)]
    fn status_code(&self) -> StatusCode {
        match self {
            EC2Error::ResourceNotFoundException(_) => StatusCode::NOT_FOUND,
            _ => StatusCode::BAD_REQUEST,
        }
    }

    #[allow(dead_code)]
    fn message(&self) -> &str {
        match self {
            EC2Error::ResourceNotFoundException(m)
            | EC2Error::ResourceAlreadyExistsException(m)
            | EC2Error::ValidationException(m)
            | EC2Error::InvalidAction(m) => m,
        }
    }
}

impl IntoResponse for EC2Error {
    #[allow(dead_code)]
    fn into_response(self) -> Response {
        let body = format!(
            r#"<ErrorResponse xmlns="https://ec2.amazonaws.com/doc/2012-10-01/">
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
        let err = EC2Error::ResourceNotFoundException("test".to_string());
        assert_eq!(err.error_code(), "ResourceNotFoundException");
    }
    #[test]
    fn test_resourcealreadyexistsexception_error_code() {
        let err = EC2Error::ResourceAlreadyExistsException("test".to_string());
        assert_eq!(err.error_code(), "ResourceAlreadyExistsException");
    }
    #[test]
    fn test_validationexception_error_code() {
        let err = EC2Error::ValidationException("test".to_string());
        assert_eq!(err.error_code(), "ValidationException");
    }
    #[test]
    fn test_invalidaction_error_code() {
        let err = EC2Error::InvalidAction("test".to_string());
        assert_eq!(err.error_code(), "InvalidAction");
    }
    #[test]
    fn test_message() {
        let err = EC2Error::ResourceNotFoundException("hello world".to_string());
        assert_eq!(err.message(), "hello world");
    }
    #[test]
    fn test_resourcenotfoundexception_status() {
        let err = EC2Error::ResourceNotFoundException("test".to_string());
        assert_eq!(err.status_code(), StatusCode::NOT_FOUND);
    }
    #[test]
    fn test_resourcealreadyexistsexception_status() {
        let err = EC2Error::ResourceAlreadyExistsException("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_validationexception_status() {
        let err = EC2Error::ValidationException("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_invalidaction_status() {
        let err = EC2Error::InvalidAction("test".to_string());
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
    #[test]
    fn test_into_response() {
        let err = EC2Error::ResourceNotFoundException("test error".to_string());
        let resp = err.into_response();
        assert!(resp.status().is_client_error());
    }
}
