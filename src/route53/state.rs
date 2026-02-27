use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::Route53Error;
use super::types::*;

#[allow(dead_code)]
struct Route53StateInner {
    hosted_zones: HashMap<String, StoredHostedZone>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredHostedZone {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct Route53State {
    inner: Arc<Mutex<Route53StateInner>>,
}

impl Route53State {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        Route53State {
            inner: Arc::new(Mutex::new(Route53StateInner {
                hosted_zones: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_hosted_zone(&self, req: CreateHostedZoneRequest) -> Result<HostedZoneDetail, Route53Error> {
        let mut state = self.inner.lock().await;
        let name = req.name.or(req.name_pascal).unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        if state.hosted_zones.contains_key(&name) {
            return Err(Route53Error::ResourceAlreadyExistsException(format!("HostedZone {} already exists", name)));
        }
        let arn = format!("arn:aws:route53:{}:{}:hosted-zones/{}", state.region, state.account_id, name);
        let detail = HostedZoneDetail {
            name: name.clone(),
            arn: arn.clone(),
            status: "ACTIVE".to_string(),
            tags: req.tags.clone(),
        };
        state.hosted_zones.insert(name, StoredHostedZone {
            name: detail.name.clone(),
            arn,
            tags: req.tags.unwrap_or_default(),
        });
        Ok(detail)
    }

    #[allow(dead_code)]
    pub async fn get_hosted_zone(&self, name: &str) -> Result<HostedZoneDetail, Route53Error> {
        let state = self.inner.lock().await;
        let stored = state.hosted_zones.get(name)
            .ok_or_else(|| Route53Error::ResourceNotFoundException(format!("HostedZone {} not found", name)))?;
        Ok(HostedZoneDetail {
            name: stored.name.clone(),
            arn: stored.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(stored.tags.clone()),
        })
    }

    #[allow(dead_code)]
    pub async fn list_hosted_zones(&self) -> Result<ListHostedZonesResponse, Route53Error> {
        let state = self.inner.lock().await;
        let items: Vec<HostedZoneDetail> = state.hosted_zones.values().map(|s| HostedZoneDetail {
            name: s.name.clone(),
            arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListHostedZonesResponse {
            items: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_hosted_zone(&self, name: &str) -> Result<(), Route53Error> {
        let mut state = self.inner.lock().await;
        state.hosted_zones.remove(name)
            .ok_or_else(|| Route53Error::ResourceNotFoundException(format!("HostedZone {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = Route53State::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_hosted_zone() {
        let state = Route53State::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateHostedZoneRequest::default();
        let result = state.create_hosted_zone(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_get_hosted_zone_not_found() {
        let state = Route53State::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.get_hosted_zone("nonexistent").await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_hosted_zones() {
        let state = Route53State::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.list_hosted_zones().await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_hosted_zone_not_found() {
        let state = Route53State::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.delete_hosted_zone("nonexistent").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_hosted_zone_full_crud() {
        let state = Route53State::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateHostedZoneRequest::default();
        create_req.name = Some("test-crud-resource".to_string());
        let create_result = state.create_hosted_zone(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let get_result = state.get_hosted_zone("test-crud-resource").await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let del_result = state.delete_hosted_zone("test-crud-resource").await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
