use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::HealthError;
use super::types::*;

#[allow(dead_code)]
struct HealthStateInner {
    events: HashMap<String, StoredEvent>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredEvent {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct HealthState {
    inner: Arc<Mutex<HealthStateInner>>,
}

impl HealthState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        HealthState {
            inner: Arc::new(Mutex::new(HealthStateInner {
                events: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_event(&self, req: CreateEventRequest) -> Result<CreateEventResponse, HealthError> {
        let mut state = self.inner.lock().await;
        let name = req.event_name.clone();
        if state.events.contains_key(&name) {
            return Err(HealthError::ResourceAlreadyExistsException(format!("Event {} already exists", name)));
        }
        let arn = format!("arn:aws:health:{}:{}:events/{}", state.region, state.account_id, name);
        state.events.insert(name.clone(), StoredEvent {
            name: name.clone(),
            arn: arn.clone(),
            tags: req.tags.unwrap_or_default(),
        });
        Ok(CreateEventResponse {
            event_arn: Some(arn),
            event_name: Some(name),
        })
    }

    #[allow(dead_code)]
    pub async fn describe_event(&self, req: DescribeEventRequest) -> Result<DescribeEventResponse, HealthError> {
        let state = self.inner.lock().await;
        let name = req.event_name.or_else(|| req.event_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| HealthError::ValidationException("Name or ARN required".to_string()))?;
        let stored = state.events.get(&name)
            .ok_or_else(|| HealthError::ResourceNotFoundException(format!("Event {} not found", name)))?;
        Ok(DescribeEventResponse {
            event: EventDetail {
                event_name: stored.name.clone(),
                event_arn: stored.arn.clone(),
                status: "ACTIVE".to_string(),
                tags: Some(stored.tags.clone()),
            },
        })
    }

    #[allow(dead_code)]
    pub async fn list_events(&self, _req: ListEventsRequest) -> Result<ListEventsResponse, HealthError> {
        let state = self.inner.lock().await;
        let items: Vec<EventDetail> = state.events.values().map(|s| EventDetail {
            event_name: s.name.clone(),
            event_arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListEventsResponse {
            events: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_event(&self, req: DeleteEventRequest) -> Result<(), HealthError> {
        let mut state = self.inner.lock().await;
        let name = req.event_name.or_else(|| req.event_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| HealthError::ValidationException("Name or ARN required".to_string()))?;
        state.events.remove(&name)
            .ok_or_else(|| HealthError::ResourceNotFoundException(format!("Event {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = HealthState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_event() {
        let state = HealthState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateEventRequest::default();
        let result = state.create_event(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_describe_event_not_found() {
        let state = HealthState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DescribeEventRequest::default();
        let result = state.describe_event(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_events_empty() {
        let state = HealthState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = ListEventsRequest::default();
        let result = state.list_events(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_event_not_found() {
        let state = HealthState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DeleteEventRequest::default();
        let result = state.delete_event(req).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_event_create_and_list() {
        let state = HealthState::new("123456789012".to_string(), "us-east-1".to_string());
        let create_req = CreateEventRequest::default();
        let _created = state.create_event(create_req).await.unwrap();
        let list_req = ListEventsRequest::default();
        let listed = state.list_events(list_req).await.unwrap();
        let _ = listed;
    }

    #[tokio::test]
    async fn test_event_full_crud() {
        let state = HealthState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateEventRequest::default();
        create_req.event_name = "test-crud-resource".to_string();
        let create_result = state.create_event(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let mut get_req = DescribeEventRequest::default();
        get_req.event_name = Some("test-crud-resource".to_string());
        let get_result = state.describe_event(get_req).await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let mut del_req = DeleteEventRequest::default();
        del_req.event_name = Some("test-crud-resource".to_string());
        let del_result = state.delete_event(del_req).await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
