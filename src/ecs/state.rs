use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::ECSError;
use super::types::*;

#[allow(dead_code)]
struct ECSStateInner {
    clusters: HashMap<String, StoredCluster>,
    services: HashMap<String, StoredService>,
    task_definitions: HashMap<String, StoredTaskDefinition>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredCluster {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
struct StoredService {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
struct StoredTaskDefinition {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct ECSState {
    inner: Arc<Mutex<ECSStateInner>>,
}

impl ECSState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        ECSState {
            inner: Arc::new(Mutex::new(ECSStateInner {
                clusters: HashMap::new(),
                services: HashMap::new(),
                task_definitions: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_cluster(&self, req: CreateClusterRequest) -> Result<CreateClusterResponse, ECSError> {
        let mut state = self.inner.lock().await;
        let name = req.cluster_name.clone();
        if state.clusters.contains_key(&name) {
            return Err(ECSError::ResourceAlreadyExistsException(format!("Cluster {} already exists", name)));
        }
        let arn = format!("arn:aws:ecs:{}:{}:clusters/{}", state.region, state.account_id, name);
        state.clusters.insert(name.clone(), StoredCluster {
            name: name.clone(),
            arn: arn.clone(),
            tags: req.tags.unwrap_or_default(),
        });
        Ok(CreateClusterResponse {
            cluster_arn: Some(arn),
            cluster_name: Some(name),
        })
    }

    #[allow(dead_code)]
    pub async fn describe_cluster(&self, req: DescribeClusterRequest) -> Result<DescribeClusterResponse, ECSError> {
        let state = self.inner.lock().await;
        let name = req.cluster_name.or_else(|| req.cluster_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| ECSError::ValidationException("Name or ARN required".to_string()))?;
        let stored = state.clusters.get(&name)
            .ok_or_else(|| ECSError::ResourceNotFoundException(format!("Cluster {} not found", name)))?;
        Ok(DescribeClusterResponse {
            cluster: ClusterDetail {
                cluster_name: stored.name.clone(),
                cluster_arn: stored.arn.clone(),
                status: "ACTIVE".to_string(),
                tags: Some(stored.tags.clone()),
            },
        })
    }

    #[allow(dead_code)]
    pub async fn list_clusters(&self, _req: ListClustersRequest) -> Result<ListClustersResponse, ECSError> {
        let state = self.inner.lock().await;
        let items: Vec<ClusterDetail> = state.clusters.values().map(|s| ClusterDetail {
            cluster_name: s.name.clone(),
            cluster_arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListClustersResponse {
            clusters: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_cluster(&self, req: DeleteClusterRequest) -> Result<(), ECSError> {
        let mut state = self.inner.lock().await;
        let name = req.cluster_name.or_else(|| req.cluster_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| ECSError::ValidationException("Name or ARN required".to_string()))?;
        state.clusters.remove(&name)
            .ok_or_else(|| ECSError::ResourceNotFoundException(format!("Cluster {} not found", name)))?;
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn create_service(&self, req: CreateServiceRequest) -> Result<CreateServiceResponse, ECSError> {
        let mut state = self.inner.lock().await;
        let name = req.service_name.clone();
        if state.services.contains_key(&name) {
            return Err(ECSError::ResourceAlreadyExistsException(format!("Service {} already exists", name)));
        }
        let arn = format!("arn:aws:ecs:{}:{}:services/{}", state.region, state.account_id, name);
        state.services.insert(name.clone(), StoredService {
            name: name.clone(),
            arn: arn.clone(),
            tags: req.tags.unwrap_or_default(),
        });
        Ok(CreateServiceResponse {
            service_arn: Some(arn),
            service_name: Some(name),
        })
    }

    #[allow(dead_code)]
    pub async fn describe_service(&self, req: DescribeServiceRequest) -> Result<DescribeServiceResponse, ECSError> {
        let state = self.inner.lock().await;
        let name = req.service_name.or_else(|| req.service_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| ECSError::ValidationException("Name or ARN required".to_string()))?;
        let stored = state.services.get(&name)
            .ok_or_else(|| ECSError::ResourceNotFoundException(format!("Service {} not found", name)))?;
        Ok(DescribeServiceResponse {
            service: ServiceDetail {
                service_name: stored.name.clone(),
                service_arn: stored.arn.clone(),
                status: "ACTIVE".to_string(),
                tags: Some(stored.tags.clone()),
            },
        })
    }

    #[allow(dead_code)]
    pub async fn list_services(&self, _req: ListServicesRequest) -> Result<ListServicesResponse, ECSError> {
        let state = self.inner.lock().await;
        let items: Vec<ServiceDetail> = state.services.values().map(|s| ServiceDetail {
            service_name: s.name.clone(),
            service_arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListServicesResponse {
            services: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_service(&self, req: DeleteServiceRequest) -> Result<(), ECSError> {
        let mut state = self.inner.lock().await;
        let name = req.service_name.or_else(|| req.service_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| ECSError::ValidationException("Name or ARN required".to_string()))?;
        state.services.remove(&name)
            .ok_or_else(|| ECSError::ResourceNotFoundException(format!("Service {} not found", name)))?;
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn create_task_definition(&self, req: CreateTaskDefinitionRequest) -> Result<CreateTaskDefinitionResponse, ECSError> {
        let mut state = self.inner.lock().await;
        let name = req.task_definition_name.clone();
        if state.task_definitions.contains_key(&name) {
            return Err(ECSError::ResourceAlreadyExistsException(format!("TaskDefinition {} already exists", name)));
        }
        let arn = format!("arn:aws:ecs:{}:{}:task-definitions/{}", state.region, state.account_id, name);
        state.task_definitions.insert(name.clone(), StoredTaskDefinition {
            name: name.clone(),
            arn: arn.clone(),
            tags: req.tags.unwrap_or_default(),
        });
        Ok(CreateTaskDefinitionResponse {
            task_definition_arn: Some(arn),
            task_definition_name: Some(name),
        })
    }

    #[allow(dead_code)]
    pub async fn describe_task_definition(&self, req: DescribeTaskDefinitionRequest) -> Result<DescribeTaskDefinitionResponse, ECSError> {
        let state = self.inner.lock().await;
        let name = req.task_definition_name.or_else(|| req.task_definition_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| ECSError::ValidationException("Name or ARN required".to_string()))?;
        let stored = state.task_definitions.get(&name)
            .ok_or_else(|| ECSError::ResourceNotFoundException(format!("TaskDefinition {} not found", name)))?;
        Ok(DescribeTaskDefinitionResponse {
            task_definition: TaskDefinitionDetail {
                task_definition_name: stored.name.clone(),
                task_definition_arn: stored.arn.clone(),
                status: "ACTIVE".to_string(),
                tags: Some(stored.tags.clone()),
            },
        })
    }

    #[allow(dead_code)]
    pub async fn list_task_definitions(&self, _req: ListTaskDefinitionsRequest) -> Result<ListTaskDefinitionsResponse, ECSError> {
        let state = self.inner.lock().await;
        let items: Vec<TaskDefinitionDetail> = state.task_definitions.values().map(|s| TaskDefinitionDetail {
            task_definition_name: s.name.clone(),
            task_definition_arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListTaskDefinitionsResponse {
            task_definitions: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_task_definition(&self, req: DeleteTaskDefinitionRequest) -> Result<(), ECSError> {
        let mut state = self.inner.lock().await;
        let name = req.task_definition_name.or_else(|| req.task_definition_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| ECSError::ValidationException("Name or ARN required".to_string()))?;
        state.task_definitions.remove(&name)
            .ok_or_else(|| ECSError::ResourceNotFoundException(format!("TaskDefinition {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = ECSState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_cluster() {
        let state = ECSState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateClusterRequest::default();
        let result = state.create_cluster(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_describe_cluster_not_found() {
        let state = ECSState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DescribeClusterRequest::default();
        let result = state.describe_cluster(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_clusters_empty() {
        let state = ECSState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = ListClustersRequest::default();
        let result = state.list_clusters(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_cluster_not_found() {
        let state = ECSState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DeleteClusterRequest::default();
        let result = state.delete_cluster(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_create_service() {
        let state = ECSState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateServiceRequest::default();
        let result = state.create_service(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_describe_service_not_found() {
        let state = ECSState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DescribeServiceRequest::default();
        let result = state.describe_service(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_services_empty() {
        let state = ECSState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = ListServicesRequest::default();
        let result = state.list_services(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_service_not_found() {
        let state = ECSState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DeleteServiceRequest::default();
        let result = state.delete_service(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_create_task_definition() {
        let state = ECSState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateTaskDefinitionRequest::default();
        let result = state.create_task_definition(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_describe_task_definition_not_found() {
        let state = ECSState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DescribeTaskDefinitionRequest::default();
        let result = state.describe_task_definition(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_task_definitions_empty() {
        let state = ECSState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = ListTaskDefinitionsRequest::default();
        let result = state.list_task_definitions(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_task_definition_not_found() {
        let state = ECSState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DeleteTaskDefinitionRequest::default();
        let result = state.delete_task_definition(req).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_cluster_create_and_list() {
        let state = ECSState::new("123456789012".to_string(), "us-east-1".to_string());
        let create_req = CreateClusterRequest::default();
        let _created = state.create_cluster(create_req).await.unwrap();
        let list_req = ListClustersRequest::default();
        let listed = state.list_clusters(list_req).await.unwrap();
        let _ = listed;
    }
    #[tokio::test]
    async fn test_service_create_and_list() {
        let state = ECSState::new("123456789012".to_string(), "us-east-1".to_string());
        let create_req = CreateServiceRequest::default();
        let _created = state.create_service(create_req).await.unwrap();
        let list_req = ListServicesRequest::default();
        let listed = state.list_services(list_req).await.unwrap();
        let _ = listed;
    }
    #[tokio::test]
    async fn test_task_definition_create_and_list() {
        let state = ECSState::new("123456789012".to_string(), "us-east-1".to_string());
        let create_req = CreateTaskDefinitionRequest::default();
        let _created = state.create_task_definition(create_req).await.unwrap();
        let list_req = ListTaskDefinitionsRequest::default();
        let listed = state.list_task_definitions(list_req).await.unwrap();
        let _ = listed;
    }

    #[tokio::test]
    async fn test_cluster_full_crud() {
        let state = ECSState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateClusterRequest::default();
        create_req.cluster_name = "test-crud-resource".to_string();
        let create_result = state.create_cluster(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let mut get_req = DescribeClusterRequest::default();
        get_req.cluster_name = Some("test-crud-resource".to_string());
        let get_result = state.describe_cluster(get_req).await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let mut del_req = DeleteClusterRequest::default();
        del_req.cluster_name = Some("test-crud-resource".to_string());
        let del_result = state.delete_cluster(del_req).await;
        assert!(del_result.is_ok(), "delete should succeed");
    }

    #[tokio::test]
    async fn test_service_full_crud() {
        let state = ECSState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateServiceRequest::default();
        create_req.service_name = "test-crud-resource".to_string();
        let create_result = state.create_service(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let mut get_req = DescribeServiceRequest::default();
        get_req.service_name = Some("test-crud-resource".to_string());
        let get_result = state.describe_service(get_req).await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let mut del_req = DeleteServiceRequest::default();
        del_req.service_name = Some("test-crud-resource".to_string());
        let del_result = state.delete_service(del_req).await;
        assert!(del_result.is_ok(), "delete should succeed");
    }

    #[tokio::test]
    async fn test_task_definition_full_crud() {
        let state = ECSState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateTaskDefinitionRequest::default();
        create_req.task_definition_name = "test-crud-resource".to_string();
        let create_result = state.create_task_definition(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let mut get_req = DescribeTaskDefinitionRequest::default();
        get_req.task_definition_name = Some("test-crud-resource".to_string());
        let get_result = state.describe_task_definition(get_req).await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let mut del_req = DeleteTaskDefinitionRequest::default();
        del_req.task_definition_name = Some("test-crud-resource".to_string());
        let del_result = state.delete_task_definition(del_req).await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
