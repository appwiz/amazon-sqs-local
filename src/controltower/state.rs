use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::ControltowerError;
use super::types::*;

#[allow(dead_code)]
struct ControltowerStateInner {
    landing_zones: HashMap<String, StoredLandingZone>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredLandingZone {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct ControltowerState {
    inner: Arc<Mutex<ControltowerStateInner>>,
}

impl ControltowerState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        ControltowerState {
            inner: Arc::new(Mutex::new(ControltowerStateInner {
                landing_zones: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_landing_zone(&self, req: CreateLandingZoneRequest) -> Result<LandingZoneDetail, ControltowerError> {
        let mut state = self.inner.lock().await;
        let name = req.name.or(req.name_pascal).unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        if state.landing_zones.contains_key(&name) {
            return Err(ControltowerError::ResourceAlreadyExistsException(format!("LandingZone {} already exists", name)));
        }
        let arn = format!("arn:aws:controltower:{}:{}:landing-zones/{}", state.region, state.account_id, name);
        let detail = LandingZoneDetail {
            name: name.clone(),
            arn: arn.clone(),
            status: "ACTIVE".to_string(),
            tags: req.tags.clone(),
        };
        state.landing_zones.insert(name, StoredLandingZone {
            name: detail.name.clone(),
            arn,
            tags: req.tags.unwrap_or_default(),
        });
        Ok(detail)
    }

    #[allow(dead_code)]
    pub async fn get_landing_zone(&self, name: &str) -> Result<LandingZoneDetail, ControltowerError> {
        let state = self.inner.lock().await;
        let stored = state.landing_zones.get(name)
            .ok_or_else(|| ControltowerError::ResourceNotFoundException(format!("LandingZone {} not found", name)))?;
        Ok(LandingZoneDetail {
            name: stored.name.clone(),
            arn: stored.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(stored.tags.clone()),
        })
    }

    #[allow(dead_code)]
    pub async fn list_landing_zones(&self) -> Result<ListLandingZonesResponse, ControltowerError> {
        let state = self.inner.lock().await;
        let items: Vec<LandingZoneDetail> = state.landing_zones.values().map(|s| LandingZoneDetail {
            name: s.name.clone(),
            arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListLandingZonesResponse {
            items: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_landing_zone(&self, name: &str) -> Result<(), ControltowerError> {
        let mut state = self.inner.lock().await;
        state.landing_zones.remove(name)
            .ok_or_else(|| ControltowerError::ResourceNotFoundException(format!("LandingZone {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = ControltowerState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_landing_zone() {
        let state = ControltowerState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateLandingZoneRequest::default();
        let result = state.create_landing_zone(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_get_landing_zone_not_found() {
        let state = ControltowerState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.get_landing_zone("nonexistent").await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_landing_zones() {
        let state = ControltowerState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.list_landing_zones().await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_landing_zone_not_found() {
        let state = ControltowerState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.delete_landing_zone("nonexistent").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_landing_zone_full_crud() {
        let state = ControltowerState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateLandingZoneRequest::default();
        create_req.name = Some("test-crud-resource".to_string());
        let create_result = state.create_landing_zone(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let get_result = state.get_landing_zone("test-crud-resource").await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let del_result = state.delete_landing_zone("test-crud-resource").await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
