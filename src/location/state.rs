use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::LocationError;
use super::types::*;

#[allow(dead_code)]
struct LocationStateInner {
    maps: HashMap<String, StoredMap>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredMap {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct LocationState {
    inner: Arc<Mutex<LocationStateInner>>,
}

impl LocationState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        LocationState {
            inner: Arc::new(Mutex::new(LocationStateInner {
                maps: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_map(&self, req: CreateMapRequest) -> Result<MapDetail, LocationError> {
        let mut state = self.inner.lock().await;
        let name = req.name.or(req.name_pascal).unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        if state.maps.contains_key(&name) {
            return Err(LocationError::ResourceAlreadyExistsException(format!("Map {} already exists", name)));
        }
        let arn = format!("arn:aws:geo:{}:{}:maps/{}", state.region, state.account_id, name);
        let detail = MapDetail {
            name: name.clone(),
            arn: arn.clone(),
            status: "ACTIVE".to_string(),
            tags: req.tags.clone(),
        };
        state.maps.insert(name, StoredMap {
            name: detail.name.clone(),
            arn,
            tags: req.tags.unwrap_or_default(),
        });
        Ok(detail)
    }

    #[allow(dead_code)]
    pub async fn get_map(&self, name: &str) -> Result<MapDetail, LocationError> {
        let state = self.inner.lock().await;
        let stored = state.maps.get(name)
            .ok_or_else(|| LocationError::ResourceNotFoundException(format!("Map {} not found", name)))?;
        Ok(MapDetail {
            name: stored.name.clone(),
            arn: stored.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(stored.tags.clone()),
        })
    }

    #[allow(dead_code)]
    pub async fn list_maps(&self) -> Result<ListMapsResponse, LocationError> {
        let state = self.inner.lock().await;
        let items: Vec<MapDetail> = state.maps.values().map(|s| MapDetail {
            name: s.name.clone(),
            arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListMapsResponse {
            items: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_map(&self, name: &str) -> Result<(), LocationError> {
        let mut state = self.inner.lock().await;
        state.maps.remove(name)
            .ok_or_else(|| LocationError::ResourceNotFoundException(format!("Map {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = LocationState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_map() {
        let state = LocationState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateMapRequest::default();
        let result = state.create_map(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_get_map_not_found() {
        let state = LocationState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.get_map("nonexistent").await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_maps() {
        let state = LocationState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.list_maps().await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_map_not_found() {
        let state = LocationState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.delete_map("nonexistent").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_map_full_crud() {
        let state = LocationState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateMapRequest::default();
        create_req.name = Some("test-crud-resource".to_string());
        let create_result = state.create_map(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let get_result = state.get_map("test-crud-resource").await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let del_result = state.delete_map("test-crud-resource").await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
