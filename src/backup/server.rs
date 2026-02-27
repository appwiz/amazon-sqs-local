use std::sync::Arc;

use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use axum::http::StatusCode;

use super::error::BackupError;
use super::state::BackupState;
use super::types::*;

async fn create_backup_vault_handler(
    State(state): State<Arc<BackupState>>,
    Json(req): Json<CreateBackupVaultRequest>,
) -> Result<axum::response::Response, BackupError> {
    let detail = state.create_backup_vault(req).await?;
    Ok((StatusCode::CREATED, Json(detail)).into_response())
}

async fn get_backup_vault_handler(
    State(state): State<Arc<BackupState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, BackupError> {
    let detail = state.get_backup_vault(&name).await?;
    Ok(Json(detail).into_response())
}

async fn list_backup_vaults_handler(
    State(state): State<Arc<BackupState>>,
) -> Result<axum::response::Response, BackupError> {
    let resp = state.list_backup_vaults().await?;
    Ok(Json(resp).into_response())
}

async fn delete_backup_vault_handler(
    State(state): State<Arc<BackupState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, BackupError> {
    state.delete_backup_vault(&name).await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}

async fn create_backup_plan_handler(
    State(state): State<Arc<BackupState>>,
    Json(req): Json<CreateBackupPlanRequest>,
) -> Result<axum::response::Response, BackupError> {
    let detail = state.create_backup_plan(req).await?;
    Ok((StatusCode::CREATED, Json(detail)).into_response())
}

async fn get_backup_plan_handler(
    State(state): State<Arc<BackupState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, BackupError> {
    let detail = state.get_backup_plan(&name).await?;
    Ok(Json(detail).into_response())
}

async fn list_backup_plans_handler(
    State(state): State<Arc<BackupState>>,
) -> Result<axum::response::Response, BackupError> {
    let resp = state.list_backup_plans().await?;
    Ok(Json(resp).into_response())
}

async fn delete_backup_plan_handler(
    State(state): State<Arc<BackupState>>,
    Path(name): Path<String>,
) -> Result<axum::response::Response, BackupError> {
    state.delete_backup_plan(&name).await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}

pub fn create_router(state: Arc<BackupState>) -> Router {
    Router::new()
        .route("/backup-vaults", post(create_backup_vault_handler).get(list_backup_vaults_handler))
        .route("/backup-vaults/{name}", get(get_backup_vault_handler).delete(delete_backup_vault_handler))
        .route("/backup-plans", post(create_backup_plan_handler).get(list_backup_plans_handler))
        .route("/backup-plans/{name}", get(get_backup_plan_handler).delete(delete_backup_plan_handler))
        .with_state(state)
}


#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_list_endpoint() {
        let state = Arc::new(BackupState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/backup-vaults")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[tokio::test]
    async fn test_get_not_found() {
        let state = Arc::new(BackupState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/backup-vaults/nonexistent")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_create_backup_vaults() {
        let state = Arc::new(BackupState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/backup-vaults")
            .header("content-type", "application/json")
            .body(Body::from("{}"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert!(resp.status().is_success() || resp.status().is_client_error());
    }
    #[tokio::test]
    async fn test_create_backup_plans() {
        let state = Arc::new(BackupState::new("123456789012".to_string(), "us-east-1".to_string()));
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/backup-plans")
            .header("content-type", "application/json")
            .body(Body::from("{}"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert!(resp.status().is_success() || resp.status().is_client_error());
    }
}
