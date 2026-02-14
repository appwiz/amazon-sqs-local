use std::sync::Arc;

use axum::body::Bytes;
use axum::extract::State;
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};

use super::error::MemoryDbError;
use super::state::MemoryDbState;
use super::types::*;

macro_rules! dispatch {
    ($state:expr, $body:expr, $req_type:ty, $method:ident) => {{
        let req: $req_type = serde_json::from_slice(&$body)
            .map_err(|e| MemoryDbError::InvalidParameterValue(e.to_string()))?;
        let resp = $state.$method(req).await?;
        Ok(Json(serde_json::to_value(resp).unwrap()).into_response())
    }};
}

async fn handle_request(
    State(state): State<Arc<MemoryDbState>>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<axum::response::Response, MemoryDbError> {
    let target = headers
        .get("x-amz-target")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| MemoryDbError::InvalidAction("Missing X-Amz-Target header".into()))?;

    let action = target.strip_prefix("AmazonMemoryDB.").ok_or_else(|| {
        MemoryDbError::InvalidAction(format!("Invalid target: {target}"))
    })?;

    match action {
        "CreateCluster" => dispatch!(state, body, CreateClusterRequest, create_cluster),
        "DeleteCluster" => dispatch!(state, body, DeleteClusterRequest, delete_cluster),
        "DescribeClusters" => dispatch!(state, body, DescribeClustersRequest, describe_clusters),
        "UpdateCluster" => dispatch!(state, body, UpdateClusterRequest, update_cluster),
        "CreateSubnetGroup" => {
            dispatch!(state, body, CreateSubnetGroupRequest, create_subnet_group)
        }
        "DeleteSubnetGroup" => {
            dispatch!(state, body, DeleteSubnetGroupRequest, delete_subnet_group)
        }
        "DescribeSubnetGroups" => {
            dispatch!(
                state,
                body,
                DescribeSubnetGroupsRequest,
                describe_subnet_groups
            )
        }
        "CreateUser" => dispatch!(state, body, CreateUserRequest, create_user),
        "DeleteUser" => dispatch!(state, body, DeleteUserRequest, delete_user),
        "DescribeUsers" => dispatch!(state, body, DescribeUsersRequest, describe_users),
        "UpdateUser" => dispatch!(state, body, UpdateUserRequest, update_user),
        "CreateACL" => dispatch!(state, body, CreateAclRequest, create_acl),
        "DeleteACL" => dispatch!(state, body, DeleteAclRequest, delete_acl),
        "DescribeACLs" => dispatch!(state, body, DescribeAclsRequest, describe_acls),
        "UpdateACL" => dispatch!(state, body, UpdateAclRequest, update_acl),
        "CreateSnapshot" => dispatch!(state, body, CreateSnapshotRequest, create_snapshot),
        "DeleteSnapshot" => dispatch!(state, body, DeleteSnapshotRequest, delete_snapshot),
        "DescribeSnapshots" => {
            dispatch!(state, body, DescribeSnapshotsRequest, describe_snapshots)
        }
        "TagResource" => dispatch!(state, body, TagResourceRequest, tag_resource),
        "UntagResource" => dispatch!(state, body, UntagResourceRequest, untag_resource),
        "ListTags" => dispatch!(state, body, ListTagsRequest, list_tags),
        _ => Err(MemoryDbError::InvalidAction(format!(
            "Unknown action: {action}"
        ))),
    }
}

pub fn create_router(state: Arc<MemoryDbState>) -> Router {
    Router::new()
        .route("/", post(handle_request))
        .with_state(state)
}
