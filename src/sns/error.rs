use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum SnsError {
    NotFound(String),
    InvalidParameter(String),
    TagLimitExceeded(String),
    InvalidAction(String),
}

impl SnsError {
    fn error_code(&self) -> &str {
        match self {
            SnsError::NotFound(_) => "NotFound",
            SnsError::InvalidParameter(_) => "InvalidParameter",
            SnsError::TagLimitExceeded(_) => "TagLimitExceeded",
            SnsError::InvalidAction(_) => "InvalidAction",
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            SnsError::NotFound(_) => StatusCode::NOT_FOUND,
            _ => StatusCode::BAD_REQUEST,
        }
    }

    fn message(&self) -> &str {
        match self {
            SnsError::NotFound(m)
            | SnsError::InvalidParameter(m)
            | SnsError::TagLimitExceeded(m)
            | SnsError::InvalidAction(m) => m,
        }
    }

    fn error_type(&self) -> &str {
        match self {
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
