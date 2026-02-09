use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use uuid::Uuid;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum SnsError {
    NotFound(String),
    InvalidParameter(String),
    AuthorizationError(String),
    TopicLimitExceeded(String),
    SubscriptionLimitExceeded(String),
    InvalidAction(String),
    InternalError(String),
    TagLimitExceeded(String),
    InvalidSecurity(String),
    ConcurrentAccess(String),
    StaleTag(String),
    TagPolicy(String),
}

impl SnsError {
    fn error_code(&self) -> &str {
        match self {
            SnsError::NotFound(_) => "NotFound",
            SnsError::InvalidParameter(_) => "InvalidParameter",
            SnsError::AuthorizationError(_) => "AuthorizationError",
            SnsError::TopicLimitExceeded(_) => "TopicLimitExceeded",
            SnsError::SubscriptionLimitExceeded(_) => "SubscriptionLimitExceeded",
            SnsError::InvalidAction(_) => "InvalidAction",
            SnsError::InternalError(_) => "InternalError",
            SnsError::TagLimitExceeded(_) => "TagLimitExceeded",
            SnsError::InvalidSecurity(_) => "InvalidSecurity",
            SnsError::ConcurrentAccess(_) => "ConcurrentAccess",
            SnsError::StaleTag(_) => "StaleTag",
            SnsError::TagPolicy(_) => "TagPolicy",
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            SnsError::NotFound(_) => StatusCode::NOT_FOUND,
            SnsError::AuthorizationError(_) | SnsError::InvalidSecurity(_) => {
                StatusCode::FORBIDDEN
            }
            SnsError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::BAD_REQUEST,
        }
    }

    fn message(&self) -> &str {
        match self {
            SnsError::NotFound(m)
            | SnsError::InvalidParameter(m)
            | SnsError::AuthorizationError(m)
            | SnsError::TopicLimitExceeded(m)
            | SnsError::SubscriptionLimitExceeded(m)
            | SnsError::InvalidAction(m)
            | SnsError::InternalError(m)
            | SnsError::TagLimitExceeded(m)
            | SnsError::InvalidSecurity(m)
            | SnsError::ConcurrentAccess(m)
            | SnsError::StaleTag(m)
            | SnsError::TagPolicy(m) => m,
        }
    }

    fn error_type(&self) -> &str {
        match self {
            SnsError::InternalError(_) => "Receiver",
            _ => "Sender",
        }
    }
}

impl IntoResponse for SnsError {
    fn into_response(self) -> Response {
        let request_id = Uuid::new_v4().to_string();
        let body = format!(
            r#"<ErrorResponse xmlns="http://sns.amazonaws.com/doc/2010-03-31/">
  <Error>
    <Type>{}</Type>
    <Code>{}</Code>
    <Message>{}</Message>
  </Error>
  <RequestId>{}</RequestId>
</ErrorResponse>"#,
            self.error_type(),
            self.error_code(),
            xml_escape(self.message()),
            request_id,
        );
        (
            self.status_code(),
            [("content-type", "text/xml")],
            body,
        )
            .into_response()
    }
}

fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}
