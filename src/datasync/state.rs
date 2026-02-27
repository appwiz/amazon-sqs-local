use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::DatasyncError;
use super::types::*;

#[allow(dead_code)]
struct DatasyncStateInner {
    tasks: HashMap<String, StoredTask>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredTask {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct DatasyncState {
    inner: Arc<Mutex<DatasyncStateInner>>,
}

impl DatasyncState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        DatasyncState {
            inner: Arc::new(Mutex::new(DatasyncStateInner {
                tasks: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_task(&self, req: CreateTaskRequest) -> Result<CreateTaskResponse, DatasyncError> {
        let mut state = self.inner.lock().await;
        let name = req.task_name.clone();
        if state.tasks.contains_key(&name) {
            return Err(DatasyncError::ResourceAlreadyExistsException(format!("Task {} already exists", name)));
        }
        let arn = format!("arn:aws:datasync:{}:{}:tasks/{}", state.region, state.account_id, name);
        state.tasks.insert(name.clone(), StoredTask {
            name: name.clone(),
            arn: arn.clone(),
            tags: req.tags.unwrap_or_default(),
        });
        Ok(CreateTaskResponse {
            task_arn: Some(arn),
            task_name: Some(name),
        })
    }

    #[allow(dead_code)]
    pub async fn describe_task(&self, req: DescribeTaskRequest) -> Result<DescribeTaskResponse, DatasyncError> {
        let state = self.inner.lock().await;
        let name = req.task_name.or_else(|| req.task_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| DatasyncError::ValidationException("Name or ARN required".to_string()))?;
        let stored = state.tasks.get(&name)
            .ok_or_else(|| DatasyncError::ResourceNotFoundException(format!("Task {} not found", name)))?;
        Ok(DescribeTaskResponse {
            task: TaskDetail {
                task_name: stored.name.clone(),
                task_arn: stored.arn.clone(),
                status: "ACTIVE".to_string(),
                tags: Some(stored.tags.clone()),
            },
        })
    }

    #[allow(dead_code)]
    pub async fn list_tasks(&self, _req: ListTasksRequest) -> Result<ListTasksResponse, DatasyncError> {
        let state = self.inner.lock().await;
        let items: Vec<TaskDetail> = state.tasks.values().map(|s| TaskDetail {
            task_name: s.name.clone(),
            task_arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListTasksResponse {
            tasks: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_task(&self, req: DeleteTaskRequest) -> Result<(), DatasyncError> {
        let mut state = self.inner.lock().await;
        let name = req.task_name.or_else(|| req.task_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| DatasyncError::ValidationException("Name or ARN required".to_string()))?;
        state.tasks.remove(&name)
            .ok_or_else(|| DatasyncError::ResourceNotFoundException(format!("Task {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = DatasyncState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_task() {
        let state = DatasyncState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateTaskRequest::default();
        let result = state.create_task(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_describe_task_not_found() {
        let state = DatasyncState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DescribeTaskRequest::default();
        let result = state.describe_task(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_tasks_empty() {
        let state = DatasyncState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = ListTasksRequest::default();
        let result = state.list_tasks(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_task_not_found() {
        let state = DatasyncState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DeleteTaskRequest::default();
        let result = state.delete_task(req).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_task_create_and_list() {
        let state = DatasyncState::new("123456789012".to_string(), "us-east-1".to_string());
        let create_req = CreateTaskRequest::default();
        let _created = state.create_task(create_req).await.unwrap();
        let list_req = ListTasksRequest::default();
        let listed = state.list_tasks(list_req).await.unwrap();
        let _ = listed;
    }

    #[tokio::test]
    async fn test_task_full_crud() {
        let state = DatasyncState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateTaskRequest::default();
        create_req.task_name = "test-crud-resource".to_string();
        let create_result = state.create_task(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let mut get_req = DescribeTaskRequest::default();
        get_req.task_name = Some("test-crud-resource".to_string());
        let get_result = state.describe_task(get_req).await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let mut del_req = DeleteTaskRequest::default();
        del_req.task_name = Some("test-crud-resource".to_string());
        let del_result = state.delete_task(del_req).await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
