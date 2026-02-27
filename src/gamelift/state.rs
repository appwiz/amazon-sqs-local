use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::GameliftError;
use super::types::*;

#[allow(dead_code)]
struct GameliftStateInner {
    fleets: HashMap<String, StoredFleet>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredFleet {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct GameliftState {
    inner: Arc<Mutex<GameliftStateInner>>,
}

impl GameliftState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        GameliftState {
            inner: Arc::new(Mutex::new(GameliftStateInner {
                fleets: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_fleet(&self, req: CreateFleetRequest) -> Result<CreateFleetResponse, GameliftError> {
        let mut state = self.inner.lock().await;
        let name = req.fleet_name.clone();
        if state.fleets.contains_key(&name) {
            return Err(GameliftError::ResourceAlreadyExistsException(format!("Fleet {} already exists", name)));
        }
        let arn = format!("arn:aws:gamelift:{}:{}:fleets/{}", state.region, state.account_id, name);
        state.fleets.insert(name.clone(), StoredFleet {
            name: name.clone(),
            arn: arn.clone(),
            tags: req.tags.unwrap_or_default(),
        });
        Ok(CreateFleetResponse {
            fleet_arn: Some(arn),
            fleet_name: Some(name),
        })
    }

    #[allow(dead_code)]
    pub async fn describe_fleet(&self, req: DescribeFleetRequest) -> Result<DescribeFleetResponse, GameliftError> {
        let state = self.inner.lock().await;
        let name = req.fleet_name.or_else(|| req.fleet_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| GameliftError::ValidationException("Name or ARN required".to_string()))?;
        let stored = state.fleets.get(&name)
            .ok_or_else(|| GameliftError::ResourceNotFoundException(format!("Fleet {} not found", name)))?;
        Ok(DescribeFleetResponse {
            fleet: FleetDetail {
                fleet_name: stored.name.clone(),
                fleet_arn: stored.arn.clone(),
                status: "ACTIVE".to_string(),
                tags: Some(stored.tags.clone()),
            },
        })
    }

    #[allow(dead_code)]
    pub async fn list_fleets(&self, _req: ListFleetsRequest) -> Result<ListFleetsResponse, GameliftError> {
        let state = self.inner.lock().await;
        let items: Vec<FleetDetail> = state.fleets.values().map(|s| FleetDetail {
            fleet_name: s.name.clone(),
            fleet_arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListFleetsResponse {
            fleets: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_fleet(&self, req: DeleteFleetRequest) -> Result<(), GameliftError> {
        let mut state = self.inner.lock().await;
        let name = req.fleet_name.or_else(|| req.fleet_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| GameliftError::ValidationException("Name or ARN required".to_string()))?;
        state.fleets.remove(&name)
            .ok_or_else(|| GameliftError::ResourceNotFoundException(format!("Fleet {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = GameliftState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_fleet() {
        let state = GameliftState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateFleetRequest::default();
        let result = state.create_fleet(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_describe_fleet_not_found() {
        let state = GameliftState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DescribeFleetRequest::default();
        let result = state.describe_fleet(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_fleets_empty() {
        let state = GameliftState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = ListFleetsRequest::default();
        let result = state.list_fleets(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_fleet_not_found() {
        let state = GameliftState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DeleteFleetRequest::default();
        let result = state.delete_fleet(req).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_fleet_create_and_list() {
        let state = GameliftState::new("123456789012".to_string(), "us-east-1".to_string());
        let create_req = CreateFleetRequest::default();
        let _created = state.create_fleet(create_req).await.unwrap();
        let list_req = ListFleetsRequest::default();
        let listed = state.list_fleets(list_req).await.unwrap();
        let _ = listed;
    }

    #[tokio::test]
    async fn test_fleet_full_crud() {
        let state = GameliftState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateFleetRequest::default();
        create_req.fleet_name = "test-crud-resource".to_string();
        let create_result = state.create_fleet(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let mut get_req = DescribeFleetRequest::default();
        get_req.fleet_name = Some("test-crud-resource".to_string());
        let get_result = state.describe_fleet(get_req).await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let mut del_req = DeleteFleetRequest::default();
        del_req.fleet_name = Some("test-crud-resource".to_string());
        let del_result = state.delete_fleet(del_req).await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
