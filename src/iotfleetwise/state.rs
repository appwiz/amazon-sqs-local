use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::IotfleetwiseError;
use super::types::*;

#[allow(dead_code)]
struct IotfleetwiseStateInner {
    vehicles: HashMap<String, StoredVehicle>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredVehicle {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct IotfleetwiseState {
    inner: Arc<Mutex<IotfleetwiseStateInner>>,
}

impl IotfleetwiseState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        IotfleetwiseState {
            inner: Arc::new(Mutex::new(IotfleetwiseStateInner {
                vehicles: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_vehicle(&self, req: CreateVehicleRequest) -> Result<CreateVehicleResponse, IotfleetwiseError> {
        let mut state = self.inner.lock().await;
        let name = req.vehicle_name.clone();
        if state.vehicles.contains_key(&name) {
            return Err(IotfleetwiseError::ResourceAlreadyExistsException(format!("Vehicle {} already exists", name)));
        }
        let arn = format!("arn:aws:iotfleetwise:{}:{}:vehicles/{}", state.region, state.account_id, name);
        state.vehicles.insert(name.clone(), StoredVehicle {
            name: name.clone(),
            arn: arn.clone(),
            tags: req.tags.unwrap_or_default(),
        });
        Ok(CreateVehicleResponse {
            vehicle_arn: Some(arn),
            vehicle_name: Some(name),
        })
    }

    #[allow(dead_code)]
    pub async fn describe_vehicle(&self, req: DescribeVehicleRequest) -> Result<DescribeVehicleResponse, IotfleetwiseError> {
        let state = self.inner.lock().await;
        let name = req.vehicle_name.or_else(|| req.vehicle_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| IotfleetwiseError::ValidationException("Name or ARN required".to_string()))?;
        let stored = state.vehicles.get(&name)
            .ok_or_else(|| IotfleetwiseError::ResourceNotFoundException(format!("Vehicle {} not found", name)))?;
        Ok(DescribeVehicleResponse {
            vehicle: VehicleDetail {
                vehicle_name: stored.name.clone(),
                vehicle_arn: stored.arn.clone(),
                status: "ACTIVE".to_string(),
                tags: Some(stored.tags.clone()),
            },
        })
    }

    #[allow(dead_code)]
    pub async fn list_vehicles(&self, _req: ListVehiclesRequest) -> Result<ListVehiclesResponse, IotfleetwiseError> {
        let state = self.inner.lock().await;
        let items: Vec<VehicleDetail> = state.vehicles.values().map(|s| VehicleDetail {
            vehicle_name: s.name.clone(),
            vehicle_arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListVehiclesResponse {
            vehicles: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_vehicle(&self, req: DeleteVehicleRequest) -> Result<(), IotfleetwiseError> {
        let mut state = self.inner.lock().await;
        let name = req.vehicle_name.or_else(|| req.vehicle_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| IotfleetwiseError::ValidationException("Name or ARN required".to_string()))?;
        state.vehicles.remove(&name)
            .ok_or_else(|| IotfleetwiseError::ResourceNotFoundException(format!("Vehicle {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = IotfleetwiseState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_vehicle() {
        let state = IotfleetwiseState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateVehicleRequest::default();
        let result = state.create_vehicle(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_describe_vehicle_not_found() {
        let state = IotfleetwiseState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DescribeVehicleRequest::default();
        let result = state.describe_vehicle(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_vehicles_empty() {
        let state = IotfleetwiseState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = ListVehiclesRequest::default();
        let result = state.list_vehicles(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_vehicle_not_found() {
        let state = IotfleetwiseState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DeleteVehicleRequest::default();
        let result = state.delete_vehicle(req).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_vehicle_create_and_list() {
        let state = IotfleetwiseState::new("123456789012".to_string(), "us-east-1".to_string());
        let create_req = CreateVehicleRequest::default();
        let _created = state.create_vehicle(create_req).await.unwrap();
        let list_req = ListVehiclesRequest::default();
        let listed = state.list_vehicles(list_req).await.unwrap();
        let _ = listed;
    }

    #[tokio::test]
    async fn test_vehicle_full_crud() {
        let state = IotfleetwiseState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateVehicleRequest::default();
        create_req.vehicle_name = "test-crud-resource".to_string();
        let create_result = state.create_vehicle(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let mut get_req = DescribeVehicleRequest::default();
        get_req.vehicle_name = Some("test-crud-resource".to_string());
        let get_result = state.describe_vehicle(get_req).await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let mut del_req = DeleteVehicleRequest::default();
        del_req.vehicle_name = Some("test-crud-resource".to_string());
        let del_result = state.delete_vehicle(del_req).await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
