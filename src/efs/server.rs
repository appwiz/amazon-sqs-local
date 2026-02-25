use std::sync::Arc;

use axum::body::Bytes;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{delete, get, post, put};
use axum::{Json, Router};
use serde::Deserialize;

use super::error::EfsError;
use super::state::EfsState;
use super::types::*;

// --- File System handlers ---

// POST /2015-02-01/file-systems
async fn create_file_system(
    State(state): State<Arc<EfsState>>,
    body: Bytes,
) -> Result<axum::response::Response, EfsError> {
    let req: CreateFileSystemRequest =
        serde_json::from_slice(&body).map_err(|e| EfsError::BadRequest(e.to_string()))?;
    let resp = state.create_file_system(req).await?;
    Ok((StatusCode::CREATED, Json(serde_json::to_value(resp).unwrap())).into_response())
}

#[derive(Deserialize)]
struct DescribeFileSystemsQuery {
    #[serde(rename = "FileSystemId")]
    file_system_id: Option<String>,
    #[serde(rename = "CreationToken")]
    creation_token: Option<String>,
}

// GET /2015-02-01/file-systems
async fn describe_file_systems(
    State(state): State<Arc<EfsState>>,
    Query(query): Query<DescribeFileSystemsQuery>,
) -> Result<axum::response::Response, EfsError> {
    let resp = state
        .describe_file_systems(query.file_system_id, query.creation_token)
        .await?;
    Ok((StatusCode::OK, Json(serde_json::to_value(resp).unwrap())).into_response())
}

// PUT /2015-02-01/file-systems/{FileSystemId}
async fn update_file_system(
    State(state): State<Arc<EfsState>>,
    Path(fs_id): Path<String>,
    body: Bytes,
) -> Result<axum::response::Response, EfsError> {
    let req: UpdateFileSystemRequest =
        serde_json::from_slice(&body).map_err(|e| EfsError::BadRequest(e.to_string()))?;
    let resp = state.update_file_system(fs_id, req).await?;
    Ok((StatusCode::OK, Json(serde_json::to_value(resp).unwrap())).into_response())
}

// DELETE /2015-02-01/file-systems/{FileSystemId}
async fn delete_file_system(
    State(state): State<Arc<EfsState>>,
    Path(fs_id): Path<String>,
) -> Result<axum::response::Response, EfsError> {
    state.delete_file_system(fs_id).await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}

// --- Mount Target handlers ---

// POST /2015-02-01/mount-targets
async fn create_mount_target(
    State(state): State<Arc<EfsState>>,
    body: Bytes,
) -> Result<axum::response::Response, EfsError> {
    let req: CreateMountTargetRequest =
        serde_json::from_slice(&body).map_err(|e| EfsError::BadRequest(e.to_string()))?;
    let resp = state.create_mount_target(req).await?;
    Ok((StatusCode::OK, Json(serde_json::to_value(resp).unwrap())).into_response())
}

#[derive(Deserialize)]
struct DescribeMountTargetsQuery {
    #[serde(rename = "MountTargetId")]
    mount_target_id: Option<String>,
    #[serde(rename = "FileSystemId")]
    file_system_id: Option<String>,
}

// GET /2015-02-01/mount-targets
async fn describe_mount_targets(
    State(state): State<Arc<EfsState>>,
    Query(query): Query<DescribeMountTargetsQuery>,
) -> Result<axum::response::Response, EfsError> {
    let resp = state
        .describe_mount_targets(query.mount_target_id, query.file_system_id)
        .await?;
    Ok((StatusCode::OK, Json(serde_json::to_value(resp).unwrap())).into_response())
}

// DELETE /2015-02-01/mount-targets/{MountTargetId}
async fn delete_mount_target(
    State(state): State<Arc<EfsState>>,
    Path(mt_id): Path<String>,
) -> Result<axum::response::Response, EfsError> {
    state.delete_mount_target(mt_id).await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}

// --- Access Point handlers ---

// POST /2015-02-01/access-points
async fn create_access_point(
    State(state): State<Arc<EfsState>>,
    body: Bytes,
) -> Result<axum::response::Response, EfsError> {
    let req: CreateAccessPointRequest =
        serde_json::from_slice(&body).map_err(|e| EfsError::BadRequest(e.to_string()))?;
    let resp = state.create_access_point(req).await?;
    Ok((StatusCode::OK, Json(serde_json::to_value(resp).unwrap())).into_response())
}

#[derive(Deserialize)]
struct DescribeAccessPointsQuery {
    #[serde(rename = "AccessPointId")]
    access_point_id: Option<String>,
    #[serde(rename = "FileSystemId")]
    file_system_id: Option<String>,
}

// GET /2015-02-01/access-points
async fn describe_access_points(
    State(state): State<Arc<EfsState>>,
    Query(query): Query<DescribeAccessPointsQuery>,
) -> Result<axum::response::Response, EfsError> {
    let resp = state
        .describe_access_points(query.access_point_id, query.file_system_id)
        .await?;
    Ok((StatusCode::OK, Json(serde_json::to_value(resp).unwrap())).into_response())
}

// DELETE /2015-02-01/access-points/{AccessPointId}
async fn delete_access_point(
    State(state): State<Arc<EfsState>>,
    Path(ap_id): Path<String>,
) -> Result<axum::response::Response, EfsError> {
    state.delete_access_point(ap_id).await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}

// --- Tag handlers ---

// POST /2015-02-01/resource-tags/{ResourceId}
async fn tag_resource(
    State(state): State<Arc<EfsState>>,
    Path(resource_id): Path<String>,
    body: Bytes,
) -> Result<axum::response::Response, EfsError> {
    let req: TagResourceRequest =
        serde_json::from_slice(&body).map_err(|e| EfsError::BadRequest(e.to_string()))?;
    state.tag_resource(resource_id, req).await?;
    Ok(StatusCode::OK.into_response())
}

#[derive(Deserialize)]
struct UntagQuery {
    #[serde(rename = "tagKeys")]
    tag_keys: Option<String>,
}

// DELETE /2015-02-01/resource-tags/{ResourceId}
async fn untag_resource(
    State(state): State<Arc<EfsState>>,
    Path(resource_id): Path<String>,
    Query(query): Query<UntagQuery>,
) -> Result<axum::response::Response, EfsError> {
    let tag_keys: Vec<String> = query
        .tag_keys
        .map(|s| s.split(',').map(|k| k.to_string()).collect())
        .unwrap_or_default();
    let req = UntagResourceRequest { tag_keys };
    state.untag_resource(resource_id, req).await?;
    Ok(StatusCode::OK.into_response())
}

// GET /2015-02-01/resource-tags/{ResourceId}
async fn list_tags_for_resource(
    State(state): State<Arc<EfsState>>,
    Path(resource_id): Path<String>,
) -> Result<axum::response::Response, EfsError> {
    let resp = state.list_tags_for_resource(resource_id).await?;
    Ok((StatusCode::OK, Json(serde_json::to_value(resp).unwrap())).into_response())
}

// --- Lifecycle Configuration handlers ---

// PUT /2015-02-01/file-systems/{FileSystemId}/lifecycle-configuration
async fn put_lifecycle_configuration(
    State(state): State<Arc<EfsState>>,
    Path(fs_id): Path<String>,
    body: Bytes,
) -> Result<axum::response::Response, EfsError> {
    let req: PutLifecycleConfigurationRequest =
        serde_json::from_slice(&body).map_err(|e| EfsError::BadRequest(e.to_string()))?;
    let resp = state.put_lifecycle_configuration(fs_id, req).await?;
    Ok((StatusCode::OK, Json(serde_json::to_value(resp).unwrap())).into_response())
}

// GET /2015-02-01/file-systems/{FileSystemId}/lifecycle-configuration
async fn describe_lifecycle_configuration(
    State(state): State<Arc<EfsState>>,
    Path(fs_id): Path<String>,
) -> Result<axum::response::Response, EfsError> {
    let resp = state.describe_lifecycle_configuration(fs_id).await?;
    Ok((StatusCode::OK, Json(serde_json::to_value(resp).unwrap())).into_response())
}

pub fn create_router(state: Arc<EfsState>) -> Router {
    Router::new()
        // File System routes
        .route("/2015-02-01/file-systems", post(create_file_system))
        .route("/2015-02-01/file-systems", get(describe_file_systems))
        .route("/2015-02-01/file-systems/{FileSystemId}", put(update_file_system))
        .route("/2015-02-01/file-systems/{FileSystemId}", delete(delete_file_system))
        // Mount Target routes
        .route("/2015-02-01/mount-targets", post(create_mount_target))
        .route("/2015-02-01/mount-targets", get(describe_mount_targets))
        .route("/2015-02-01/mount-targets/{MountTargetId}", delete(delete_mount_target))
        // Access Point routes
        .route("/2015-02-01/access-points", post(create_access_point))
        .route("/2015-02-01/access-points", get(describe_access_points))
        .route("/2015-02-01/access-points/{AccessPointId}", delete(delete_access_point))
        // Tag routes
        .route("/2015-02-01/resource-tags/{ResourceId}", post(tag_resource))
        .route("/2015-02-01/resource-tags/{ResourceId}", delete(untag_resource))
        .route("/2015-02-01/resource-tags/{ResourceId}", get(list_tags_for_resource))
        // Lifecycle Configuration routes
        .route(
            "/2015-02-01/file-systems/{FileSystemId}/lifecycle-configuration",
            put(put_lifecycle_configuration),
        )
        .route(
            "/2015-02-01/file-systems/{FileSystemId}/lifecycle-configuration",
            get(describe_lifecycle_configuration),
        )
        .with_state(state)
}
