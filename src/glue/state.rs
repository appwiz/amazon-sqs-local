use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use super::error::GlueError;
use super::types::*;

#[allow(dead_code)]
struct GlueStateInner {
    databases: HashMap<String, StoredDatabase>,
    tables: HashMap<String, StoredTable>,
    jobs: HashMap<String, StoredJob>,
    account_id: String,
    region: String,
}

#[allow(dead_code)]
struct StoredDatabase {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
struct StoredTable {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
struct StoredJob {
    name: String,
    arn: String,
    tags: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct GlueState {
    inner: Arc<Mutex<GlueStateInner>>,
}

impl GlueState {
    #[allow(dead_code)]
    pub fn new(account_id: String, region: String) -> Self {
        GlueState {
            inner: Arc::new(Mutex::new(GlueStateInner {
                databases: HashMap::new(),
                tables: HashMap::new(),
                jobs: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    #[allow(dead_code)]
    pub async fn create_database(&self, req: CreateDatabaseRequest) -> Result<CreateDatabaseResponse, GlueError> {
        let mut state = self.inner.lock().await;
        let name = req.database_name.clone();
        if state.databases.contains_key(&name) {
            return Err(GlueError::ResourceAlreadyExistsException(format!("Database {} already exists", name)));
        }
        let arn = format!("arn:aws:glue:{}:{}:databases/{}", state.region, state.account_id, name);
        state.databases.insert(name.clone(), StoredDatabase {
            name: name.clone(),
            arn: arn.clone(),
            tags: req.tags.unwrap_or_default(),
        });
        Ok(CreateDatabaseResponse {
            database_arn: Some(arn),
            database_name: Some(name),
        })
    }

    #[allow(dead_code)]
    pub async fn describe_database(&self, req: DescribeDatabaseRequest) -> Result<DescribeDatabaseResponse, GlueError> {
        let state = self.inner.lock().await;
        let name = req.database_name.or_else(|| req.database_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| GlueError::ValidationException("Name or ARN required".to_string()))?;
        let stored = state.databases.get(&name)
            .ok_or_else(|| GlueError::ResourceNotFoundException(format!("Database {} not found", name)))?;
        Ok(DescribeDatabaseResponse {
            database: DatabaseDetail {
                database_name: stored.name.clone(),
                database_arn: stored.arn.clone(),
                status: "ACTIVE".to_string(),
                tags: Some(stored.tags.clone()),
            },
        })
    }

    #[allow(dead_code)]
    pub async fn list_databases(&self, _req: ListDatabasesRequest) -> Result<ListDatabasesResponse, GlueError> {
        let state = self.inner.lock().await;
        let items: Vec<DatabaseDetail> = state.databases.values().map(|s| DatabaseDetail {
            database_name: s.name.clone(),
            database_arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListDatabasesResponse {
            databases: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_database(&self, req: DeleteDatabaseRequest) -> Result<(), GlueError> {
        let mut state = self.inner.lock().await;
        let name = req.database_name.or_else(|| req.database_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| GlueError::ValidationException("Name or ARN required".to_string()))?;
        state.databases.remove(&name)
            .ok_or_else(|| GlueError::ResourceNotFoundException(format!("Database {} not found", name)))?;
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn create_table(&self, req: CreateTableRequest) -> Result<CreateTableResponse, GlueError> {
        let mut state = self.inner.lock().await;
        let name = req.table_name.clone();
        if state.tables.contains_key(&name) {
            return Err(GlueError::ResourceAlreadyExistsException(format!("Table {} already exists", name)));
        }
        let arn = format!("arn:aws:glue:{}:{}:tables/{}", state.region, state.account_id, name);
        state.tables.insert(name.clone(), StoredTable {
            name: name.clone(),
            arn: arn.clone(),
            tags: req.tags.unwrap_or_default(),
        });
        Ok(CreateTableResponse {
            table_arn: Some(arn),
            table_name: Some(name),
        })
    }

    #[allow(dead_code)]
    pub async fn describe_table(&self, req: DescribeTableRequest) -> Result<DescribeTableResponse, GlueError> {
        let state = self.inner.lock().await;
        let name = req.table_name.or_else(|| req.table_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| GlueError::ValidationException("Name or ARN required".to_string()))?;
        let stored = state.tables.get(&name)
            .ok_or_else(|| GlueError::ResourceNotFoundException(format!("Table {} not found", name)))?;
        Ok(DescribeTableResponse {
            table: TableDetail {
                table_name: stored.name.clone(),
                table_arn: stored.arn.clone(),
                status: "ACTIVE".to_string(),
                tags: Some(stored.tags.clone()),
            },
        })
    }

    #[allow(dead_code)]
    pub async fn list_tables(&self, _req: ListTablesRequest) -> Result<ListTablesResponse, GlueError> {
        let state = self.inner.lock().await;
        let items: Vec<TableDetail> = state.tables.values().map(|s| TableDetail {
            table_name: s.name.clone(),
            table_arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListTablesResponse {
            tables: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_table(&self, req: DeleteTableRequest) -> Result<(), GlueError> {
        let mut state = self.inner.lock().await;
        let name = req.table_name.or_else(|| req.table_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| GlueError::ValidationException("Name or ARN required".to_string()))?;
        state.tables.remove(&name)
            .ok_or_else(|| GlueError::ResourceNotFoundException(format!("Table {} not found", name)))?;
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn create_job(&self, req: CreateJobRequest) -> Result<CreateJobResponse, GlueError> {
        let mut state = self.inner.lock().await;
        let name = req.job_name.clone();
        if state.jobs.contains_key(&name) {
            return Err(GlueError::ResourceAlreadyExistsException(format!("Job {} already exists", name)));
        }
        let arn = format!("arn:aws:glue:{}:{}:jobs/{}", state.region, state.account_id, name);
        state.jobs.insert(name.clone(), StoredJob {
            name: name.clone(),
            arn: arn.clone(),
            tags: req.tags.unwrap_or_default(),
        });
        Ok(CreateJobResponse {
            job_arn: Some(arn),
            job_name: Some(name),
        })
    }

    #[allow(dead_code)]
    pub async fn describe_job(&self, req: DescribeJobRequest) -> Result<DescribeJobResponse, GlueError> {
        let state = self.inner.lock().await;
        let name = req.job_name.or_else(|| req.job_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| GlueError::ValidationException("Name or ARN required".to_string()))?;
        let stored = state.jobs.get(&name)
            .ok_or_else(|| GlueError::ResourceNotFoundException(format!("Job {} not found", name)))?;
        Ok(DescribeJobResponse {
            job: JobDetail {
                job_name: stored.name.clone(),
                job_arn: stored.arn.clone(),
                status: "ACTIVE".to_string(),
                tags: Some(stored.tags.clone()),
            },
        })
    }

    #[allow(dead_code)]
    pub async fn list_jobs(&self, _req: ListJobsRequest) -> Result<ListJobsResponse, GlueError> {
        let state = self.inner.lock().await;
        let items: Vec<JobDetail> = state.jobs.values().map(|s| JobDetail {
            job_name: s.name.clone(),
            job_arn: s.arn.clone(),
            status: "ACTIVE".to_string(),
            tags: Some(s.tags.clone()),
        }).collect();
        Ok(ListJobsResponse {
            jobs: Some(items),
            next_token: None,
        })
    }

    #[allow(dead_code)]
    pub async fn delete_job(&self, req: DeleteJobRequest) -> Result<(), GlueError> {
        let mut state = self.inner.lock().await;
        let name = req.job_name.or_else(|| req.job_arn.as_ref().and_then(|a| a.rsplit('/').next().map(|s| s.to_string())))
            .ok_or_else(|| GlueError::ValidationException("Name or ARN required".to_string()))?;
        state.jobs.remove(&name)
            .ok_or_else(|| GlueError::ResourceNotFoundException(format!("Job {} not found", name)))?;
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_state() {
        let _state = GlueState::new("123456789012".to_string(), "us-east-1".to_string());
    }
    #[tokio::test]
    async fn test_create_database() {
        let state = GlueState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateDatabaseRequest::default();
        let result = state.create_database(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_describe_database_not_found() {
        let state = GlueState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DescribeDatabaseRequest::default();
        let result = state.describe_database(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_databases_empty() {
        let state = GlueState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = ListDatabasesRequest::default();
        let result = state.list_databases(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_database_not_found() {
        let state = GlueState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DeleteDatabaseRequest::default();
        let result = state.delete_database(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_create_table() {
        let state = GlueState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateTableRequest::default();
        let result = state.create_table(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_describe_table_not_found() {
        let state = GlueState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DescribeTableRequest::default();
        let result = state.describe_table(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_tables_empty() {
        let state = GlueState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = ListTablesRequest::default();
        let result = state.list_tables(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_table_not_found() {
        let state = GlueState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DeleteTableRequest::default();
        let result = state.delete_table(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_create_job() {
        let state = GlueState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = CreateJobRequest::default();
        let result = state.create_job(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_describe_job_not_found() {
        let state = GlueState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DescribeJobRequest::default();
        let result = state.describe_job(req).await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_list_jobs_empty() {
        let state = GlueState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = ListJobsRequest::default();
        let result = state.list_jobs(req).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_job_not_found() {
        let state = GlueState::new("123456789012".to_string(), "us-east-1".to_string());
        let req = DeleteJobRequest::default();
        let result = state.delete_job(req).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_database_create_and_list() {
        let state = GlueState::new("123456789012".to_string(), "us-east-1".to_string());
        let create_req = CreateDatabaseRequest::default();
        let _created = state.create_database(create_req).await.unwrap();
        let list_req = ListDatabasesRequest::default();
        let listed = state.list_databases(list_req).await.unwrap();
        let _ = listed;
    }
    #[tokio::test]
    async fn test_table_create_and_list() {
        let state = GlueState::new("123456789012".to_string(), "us-east-1".to_string());
        let create_req = CreateTableRequest::default();
        let _created = state.create_table(create_req).await.unwrap();
        let list_req = ListTablesRequest::default();
        let listed = state.list_tables(list_req).await.unwrap();
        let _ = listed;
    }
    #[tokio::test]
    async fn test_job_create_and_list() {
        let state = GlueState::new("123456789012".to_string(), "us-east-1".to_string());
        let create_req = CreateJobRequest::default();
        let _created = state.create_job(create_req).await.unwrap();
        let list_req = ListJobsRequest::default();
        let listed = state.list_jobs(list_req).await.unwrap();
        let _ = listed;
    }

    #[tokio::test]
    async fn test_database_full_crud() {
        let state = GlueState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateDatabaseRequest::default();
        create_req.database_name = "test-crud-resource".to_string();
        let create_result = state.create_database(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let mut get_req = DescribeDatabaseRequest::default();
        get_req.database_name = Some("test-crud-resource".to_string());
        let get_result = state.describe_database(get_req).await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let mut del_req = DeleteDatabaseRequest::default();
        del_req.database_name = Some("test-crud-resource".to_string());
        let del_result = state.delete_database(del_req).await;
        assert!(del_result.is_ok(), "delete should succeed");
    }

    #[tokio::test]
    async fn test_table_full_crud() {
        let state = GlueState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateTableRequest::default();
        create_req.table_name = "test-crud-resource".to_string();
        let create_result = state.create_table(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let mut get_req = DescribeTableRequest::default();
        get_req.table_name = Some("test-crud-resource".to_string());
        let get_result = state.describe_table(get_req).await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let mut del_req = DeleteTableRequest::default();
        del_req.table_name = Some("test-crud-resource".to_string());
        let del_result = state.delete_table(del_req).await;
        assert!(del_result.is_ok(), "delete should succeed");
    }

    #[tokio::test]
    async fn test_job_full_crud() {
        let state = GlueState::new("123456789012".to_string(), "us-east-1".to_string());
        
        // Create
        let mut create_req = CreateJobRequest::default();
        create_req.job_name = "test-crud-resource".to_string();
        let create_result = state.create_job(create_req).await;
        assert!(create_result.is_ok(), "create should succeed");

        // Get/Describe
        let mut get_req = DescribeJobRequest::default();
        get_req.job_name = Some("test-crud-resource".to_string());
        let get_result = state.describe_job(get_req).await;
        assert!(get_result.is_ok(), "get should succeed after create");

        // Delete
        let mut del_req = DeleteJobRequest::default();
        del_req.job_name = Some("test-crud-resource".to_string());
        let del_result = state.delete_job(del_req).await;
        assert!(del_result.is_ok(), "delete should succeed");
    }
}
