use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::AppmeshError;
use super::types::*;

#[allow(dead_code)]
struct AppmeshStateInner {
    meshs: HashMap<String, StoredMesh>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredMesh {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct AppmeshState {
    inner: Arc<Mutex<AppmeshStateInner>>,
}

impl AppmeshState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        AppmeshState {
            inner: Arc::new(Mutex::new(AppmeshStateInner {
                meshs: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_mesh(&self, req: CreateMeshRequest) -> Result<MeshDetail, AppmeshError> {
        let mut state = self.inner.lock().await;
        let name = req.name.or(req.name_pascal).unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        if state.meshs.contains_key(&name) {
            return Err(AppmeshError::ResourceAlreadyExistsException(format!("Mesh {} already exists", name)));
        }
        let arn = format!("arn:aws:appmesh:{}:{}:meshes/{}", state.region, state.account_id, name);
        let detail = MeshDetail {
            name: name.clone(),
            arn: arn.clone(),
            status: "ACTIVE".to_string(),
            tags: req.tags.clone(),
        };
        state.meshs.insert(name, StoredMesh {
            name: detail.name.clone(),
            arn,
            tags: req.tags.unwrap_or_default(),
        });
        Ok(detail)
    }

    #[allow(dead_code)]
    pub async fn get_mesh(&self, name: &str) -> Result<MeshDetail, AppmeshError> {
        let state = self.inner.lock().await;
        let stored = state.meshs.get(name)
            .ok_or_else(|| AppmeshError::ResourceNotFoundException(format!("Mesh {} not found", name)))?;
        Ok(MeshDetail {
            name: stored.name.clone(),
            arn: stored.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(stored.tags.clone()),
        })
    }

    #[allow(dead_code)]
    pub async fn list_meshs(&self) -> Result<ListMeshsResponse, AppmeshError> {
        let state = self.inner.lock().await;
        let items: Vec<MeshDetail> = state.meshs.values().map(|s| MeshDetail {
            name: s.name.clone(),
            arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListMeshsResponse {
            items: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_mesh(&self, name: &str) -> Result<(), AppmeshError> {
        let mut state = self.inner.lock().await;
        state.meshs.remove(name)
            .ok_or_else(|| AppmeshError::ResourceNotFoundException(format!("Mesh {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = AppmeshState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_mesh() {
        let state = AppmeshState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateMeshRequest::default();
        let result = state.create_mesh(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_get_mesh_not_found() {
        let state = AppmeshState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.get_mesh("nonexistent").await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_meshs() {
        let state = AppmeshState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.list_meshs().await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_mesh_not_found() {
        let state = AppmeshState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.delete_mesh("nonexistent").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_mesh_full_crud() {
        let state = AppmeshState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateMeshRequest::default();
        create_req.name = Some("test-crud-resource".to_string());
        let create_result = state.create_mesh(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let get_result = state.get_mesh("test-crud-resource").await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let del_result = state.delete_mesh("test-crud-resource").await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
