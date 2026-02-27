use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;

#[derive(Debug, Clone)]
pub enum MemoryDbError {
    ClusterAlreadyExistsFault(String),
    ClusterNotFoundFault(String),
    SubnetGroupAlreadyExistsFault(String),
    SubnetGroupNotFoundFault(String),
    SubnetGroupInUseFault(String),
    UserAlreadyExistsFault(String),
    UserNotFoundFault(String),
    ACLAlreadyExistsFault(String),
    ACLNotFoundFault(String),
    SnapshotAlreadyExistsFault(String),
    SnapshotNotFoundFault(String),
    InvalidParameterValue(String),
    InvalidARNFault(String),
    InvalidAction(String),
}

impl MemoryDbError {
    fn error_code(&self) -> &str {
        match self {
            MemoryDbError::ClusterAlreadyExistsFault(_) => "ClusterAlreadyExistsFault",
            MemoryDbError::ClusterNotFoundFault(_) => "ClusterNotFoundFault",
            MemoryDbError::SubnetGroupAlreadyExistsFault(_) => "SubnetGroupAlreadyExistsFault",
            MemoryDbError::SubnetGroupNotFoundFault(_) => "SubnetGroupNotFoundFault",
            MemoryDbError::SubnetGroupInUseFault(_) => "SubnetGroupInUseFault",
            MemoryDbError::UserAlreadyExistsFault(_) => "UserAlreadyExistsFault",
            MemoryDbError::UserNotFoundFault(_) => "UserNotFoundFault",
            MemoryDbError::ACLAlreadyExistsFault(_) => "ACLAlreadyExistsFault",
            MemoryDbError::ACLNotFoundFault(_) => "ACLNotFoundFault",
            MemoryDbError::SnapshotAlreadyExistsFault(_) => "SnapshotAlreadyExistsFault",
            MemoryDbError::SnapshotNotFoundFault(_) => "SnapshotNotFoundFault",
            MemoryDbError::InvalidParameterValue(_) => "InvalidParameterValueException",
            MemoryDbError::InvalidARNFault(_) => "InvalidARNFault",
            MemoryDbError::InvalidAction(_) => "InvalidAction",
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            MemoryDbError::ClusterAlreadyExistsFault(_)
            | MemoryDbError::SubnetGroupAlreadyExistsFault(_)
            | MemoryDbError::UserAlreadyExistsFault(_)
            | MemoryDbError::ACLAlreadyExistsFault(_)
            | MemoryDbError::SnapshotAlreadyExistsFault(_) => StatusCode::CONFLICT,
            MemoryDbError::ClusterNotFoundFault(_)
            | MemoryDbError::SubnetGroupNotFoundFault(_)
            | MemoryDbError::UserNotFoundFault(_)
            | MemoryDbError::ACLNotFoundFault(_)
            | MemoryDbError::SnapshotNotFoundFault(_) => StatusCode::NOT_FOUND,
            _ => StatusCode::BAD_REQUEST,
        }
    }

    fn message(&self) -> &str {
        match self {
            MemoryDbError::ClusterAlreadyExistsFault(m)
            | MemoryDbError::ClusterNotFoundFault(m)
            | MemoryDbError::SubnetGroupAlreadyExistsFault(m)
            | MemoryDbError::SubnetGroupNotFoundFault(m)
            | MemoryDbError::SubnetGroupInUseFault(m)
            | MemoryDbError::UserAlreadyExistsFault(m)
            | MemoryDbError::UserNotFoundFault(m)
            | MemoryDbError::ACLAlreadyExistsFault(m)
            | MemoryDbError::ACLNotFoundFault(m)
            | MemoryDbError::SnapshotAlreadyExistsFault(m)
            | MemoryDbError::SnapshotNotFoundFault(m)
            | MemoryDbError::InvalidParameterValue(m)
            | MemoryDbError::InvalidARNFault(m)
            | MemoryDbError::InvalidAction(m) => m,
        }
    }
}

impl IntoResponse for MemoryDbError {
    fn into_response(self) -> Response {
        let body = json!({
            "__type": self.error_code(),
            "message": self.message(),
        });
        (self.status_code(), axum::Json(body)).into_response()
    }
}
