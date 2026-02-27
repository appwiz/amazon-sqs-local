use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::IotcoreError;
use super::types::*;

#[allow(dead_code)]
struct IotcoreStateInner {
    things: HashMap<String, StoredThing>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredThing {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct IotcoreState {
    inner: Arc<Mutex<IotcoreStateInner>>,
}

impl IotcoreState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        IotcoreState {
            inner: Arc::new(Mutex::new(IotcoreStateInner {
                things: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_thing(&self, req: CreateThingRequest) -> Result<ThingDetail, IotcoreError> {
        let mut state = self.inner.lock().await;
        let name = req.name.or(req.name_pascal).unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        if state.things.contains_key(&name) {
            return Err(IotcoreError::ResourceAlreadyExistsException(format!("Thing {} already exists", name)));
        }
        let arn = format!("arn:aws:iot:{}:{}:things/{}", state.region, state.account_id, name);
        let detail = ThingDetail {
            name: name.clone(),
            arn: arn.clone(),
            status: "ACTIVE".to_string(),
            tags: req.tags.clone(),
        };
        state.things.insert(name, StoredThing {
            name: detail.name.clone(),
            arn,
            tags: req.tags.unwrap_or_default(),
        });
        Ok(detail)
    }

    #[allow(dead_code)]
    pub async fn get_thing(&self, name: &str) -> Result<ThingDetail, IotcoreError> {
        let state = self.inner.lock().await;
        let stored = state.things.get(name)
            .ok_or_else(|| IotcoreError::ResourceNotFoundException(format!("Thing {} not found", name)))?;
        Ok(ThingDetail {
            name: stored.name.clone(),
            arn: stored.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(stored.tags.clone()),
        })
    }

    #[allow(dead_code)]
    pub async fn list_things(&self) -> Result<ListThingsResponse, IotcoreError> {
        let state = self.inner.lock().await;
        let items: Vec<ThingDetail> = state.things.values().map(|s| ThingDetail {
            name: s.name.clone(),
            arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListThingsResponse {
            items: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_thing(&self, name: &str) -> Result<(), IotcoreError> {
        let mut state = self.inner.lock().await;
        state.things.remove(name)
            .ok_or_else(|| IotcoreError::ResourceNotFoundException(format!("Thing {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = IotcoreState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_thing() {
        let state = IotcoreState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateThingRequest::default();
        let result = state.create_thing(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_get_thing_not_found() {
        let state = IotcoreState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.get_thing("nonexistent").await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_things() {
        let state = IotcoreState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.list_things().await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_thing_not_found() {
        let state = IotcoreState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.delete_thing("nonexistent").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_thing_full_crud() {
        let state = IotcoreState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateThingRequest::default();
        create_req.name = Some("test-crud-resource".to_string());
        let create_result = state.create_thing(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let get_result = state.get_thing("test-crud-resource").await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let del_result = state.delete_thing("test-crud-resource").await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
