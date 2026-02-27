use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

#[derive(Debug, Clone)]
pub enum S3Error {
    NoSuchBucket(String),
    NoSuchKey(String),
    BucketAlreadyOwnedByYou(String),
    BucketNotEmpty(String),
    InvalidBucketName(String),
    NoSuchUpload(String),
    InvalidPart(String),
    InvalidPartOrder(String),
    InvalidArgument(String),
    InvalidRequest(String),
    NoSuchTagSet(String),
    MalformedXML(String),
    InvalidRange(String),
}

impl S3Error {
    fn error_code(&self) -> &str {
        match self {
            S3Error::NoSuchBucket(_) => "NoSuchBucket",
            S3Error::NoSuchKey(_) => "NoSuchKey",
            S3Error::BucketAlreadyOwnedByYou(_) => "BucketAlreadyOwnedByYou",
            S3Error::BucketNotEmpty(_) => "BucketNotEmpty",
            S3Error::InvalidBucketName(_) => "InvalidBucketName",
            S3Error::NoSuchUpload(_) => "NoSuchUpload",
            S3Error::InvalidPart(_) => "InvalidPart",
            S3Error::InvalidPartOrder(_) => "InvalidPartOrder",
            S3Error::InvalidArgument(_) => "InvalidArgument",
            S3Error::InvalidRequest(_) => "InvalidRequest",
            S3Error::NoSuchTagSet(_) => "NoSuchTagSet",
            S3Error::MalformedXML(_) => "MalformedXML",
            S3Error::InvalidRange(_) => "InvalidRange",
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            S3Error::NoSuchBucket(_) | S3Error::NoSuchKey(_) | S3Error::NoSuchUpload(_) | S3Error::NoSuchTagSet(_) => {
                StatusCode::NOT_FOUND
            }
            S3Error::BucketAlreadyOwnedByYou(_) => {
                StatusCode::CONFLICT
            }
            S3Error::BucketNotEmpty(_) => StatusCode::CONFLICT,
            S3Error::InvalidBucketName(_)
            | S3Error::InvalidPart(_)
            | S3Error::InvalidPartOrder(_)
            | S3Error::InvalidArgument(_)
            | S3Error::InvalidRequest(_)
            | S3Error::MalformedXML(_) => StatusCode::BAD_REQUEST,
            S3Error::InvalidRange(_) => StatusCode::RANGE_NOT_SATISFIABLE,
        }
    }

    fn message(&self) -> &str {
        match self {
            S3Error::NoSuchBucket(m)
            | S3Error::NoSuchKey(m)
            | S3Error::BucketAlreadyOwnedByYou(m)
            | S3Error::BucketNotEmpty(m)
            | S3Error::InvalidBucketName(m)
            | S3Error::NoSuchUpload(m)
            | S3Error::InvalidPart(m)
            | S3Error::InvalidPartOrder(m)
            | S3Error::InvalidArgument(m)
            | S3Error::InvalidRequest(m)
            | S3Error::NoSuchTagSet(m)
            | S3Error::MalformedXML(m)
            | S3Error::InvalidRange(m) => m,
        }
    }
}

impl IntoResponse for S3Error {
    fn into_response(self) -> Response {
        let body = format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<Error>
  <Code>{}</Code>
  <Message>{}</Message>
</Error>"#,
            self.error_code(),
            self.message()
        );
        (
            self.status_code(),
            [("content-type", "application/xml")],
            body,
        )
            .into_response()
    }
}
