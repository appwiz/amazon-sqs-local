use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::DetectiveError;
use super::types::*;

#[allow(dead_code)]
struct DetectiveStateInner {
    graphs: HashMap<String, StoredGraph>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredGraph {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct DetectiveState {
    inner: Arc<Mutex<DetectiveStateInner>>,
}

impl DetectiveState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        DetectiveState {
            inner: Arc::new(Mutex::new(DetectiveStateInner {
                graphs: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_graph(&self, req: CreateGraphRequest) -> Result<GraphDetail, DetectiveError> {
        let mut state = self.inner.lock().await;
        let name = req.name.or(req.name_pascal).unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        if state.graphs.contains_key(&name) {
            return Err(DetectiveError::ResourceAlreadyExistsException(format!("Graph {} already exists", name)));
        }
        let arn = format!("arn:aws:detective:{}:{}:graphs/{}", state.region, state.account_id, name);
        let detail = GraphDetail {
            name: name.clone(),
            arn: arn.clone(),
            status: "ACTIVE".to_string(),
            tags: req.tags.clone(),
        };
        state.graphs.insert(name, StoredGraph {
            name: detail.name.clone(),
            arn,
            tags: req.tags.unwrap_or_default(),
        });
        Ok(detail)
    }

    #[allow(dead_code)]
    pub async fn get_graph(&self, name: &str) -> Result<GraphDetail, DetectiveError> {
        let state = self.inner.lock().await;
        let stored = state.graphs.get(name)
            .ok_or_else(|| DetectiveError::ResourceNotFoundException(format!("Graph {} not found", name)))?;
        Ok(GraphDetail {
            name: stored.name.clone(),
            arn: stored.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(stored.tags.clone()),
        })
    }

    #[allow(dead_code)]
    pub async fn list_graphs(&self) -> Result<ListGraphsResponse, DetectiveError> {
        let state = self.inner.lock().await;
        let items: Vec<GraphDetail> = state.graphs.values().map(|s| GraphDetail {
            name: s.name.clone(),
            arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListGraphsResponse {
            items: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_graph(&self, name: &str) -> Result<(), DetectiveError> {
        let mut state = self.inner.lock().await;
        state.graphs.remove(name)
            .ok_or_else(|| DetectiveError::ResourceNotFoundException(format!("Graph {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = DetectiveState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_graph() {
        let state = DetectiveState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateGraphRequest::default();
        let result = state.create_graph(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_get_graph_not_found() {
        let state = DetectiveState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.get_graph("nonexistent").await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_graphs() {
        let state = DetectiveState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.list_graphs().await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_graph_not_found() {
        let state = DetectiveState::new("123456789012".to_string(), "us-east-1".to_string());
        let result = state.delete_graph("nonexistent").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_graph_full_crud() {
        let state = DetectiveState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateGraphRequest::default();
        create_req.name = Some("test-crud-resource".to_string());
        let create_result = state.create_graph(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let get_result = state.get_graph("test-crud-resource").await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let del_result = state.delete_graph("test-crud-resource").await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
