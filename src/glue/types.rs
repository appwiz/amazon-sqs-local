mod _types {
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct CreateDatabaseRequest {
    #[serde(rename = "DatabaseName")]
    pub database_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct CreateDatabaseResponse {
    #[serde(rename = "DatabaseArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_arn: Option<String>,
    #[serde(rename = "DatabaseName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DescribeDatabaseRequest {
    #[serde(rename = "DatabaseName")]
    pub database_name: Option<String>,
    #[serde(rename = "DatabaseArn")]
    pub database_arn: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
#[allow(dead_code)]
pub struct DatabaseDetail {
    #[serde(rename = "DatabaseName")]
    pub database_name: String,
    #[serde(rename = "DatabaseArn")]
    pub database_arn: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct DescribeDatabaseResponse {
    #[serde(rename = "Database")]
    pub database: DatabaseDetail,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct ListDatabasesRequest {
    #[serde(rename = "MaxResults")]
    pub max_results: Option<usize>,
    #[serde(rename = "NextToken")]
    pub next_token: Option<String>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct ListDatabasesResponse {
    #[serde(rename = "Databases")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub databases: Option<Vec<DatabaseDetail>>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DeleteDatabaseRequest {
    #[serde(rename = "DatabaseName")]
    pub database_name: Option<String>,
    #[serde(rename = "DatabaseArn")]
    pub database_arn: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct CreateTableRequest {
    #[serde(rename = "TableName")]
    pub table_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct CreateTableResponse {
    #[serde(rename = "TableArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_arn: Option<String>,
    #[serde(rename = "TableName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DescribeTableRequest {
    #[serde(rename = "TableName")]
    pub table_name: Option<String>,
    #[serde(rename = "TableArn")]
    pub table_arn: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
#[allow(dead_code)]
pub struct TableDetail {
    #[serde(rename = "TableName")]
    pub table_name: String,
    #[serde(rename = "TableArn")]
    pub table_arn: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct DescribeTableResponse {
    #[serde(rename = "Table")]
    pub table: TableDetail,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct ListTablesRequest {
    #[serde(rename = "MaxResults")]
    pub max_results: Option<usize>,
    #[serde(rename = "NextToken")]
    pub next_token: Option<String>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct ListTablesResponse {
    #[serde(rename = "Tables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tables: Option<Vec<TableDetail>>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DeleteTableRequest {
    #[serde(rename = "TableName")]
    pub table_name: Option<String>,
    #[serde(rename = "TableArn")]
    pub table_arn: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct CreateJobRequest {
    #[serde(rename = "JobName")]
    pub job_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct CreateJobResponse {
    #[serde(rename = "JobArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_arn: Option<String>,
    #[serde(rename = "JobName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DescribeJobRequest {
    #[serde(rename = "JobName")]
    pub job_name: Option<String>,
    #[serde(rename = "JobArn")]
    pub job_arn: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
#[allow(dead_code)]
pub struct JobDetail {
    #[serde(rename = "JobName")]
    pub job_name: String,
    #[serde(rename = "JobArn")]
    pub job_arn: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct DescribeJobResponse {
    #[serde(rename = "Job")]
    pub job: JobDetail,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct ListJobsRequest {
    #[serde(rename = "MaxResults")]
    pub max_results: Option<usize>,
    #[serde(rename = "NextToken")]
    pub next_token: Option<String>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct ListJobsResponse {
    #[serde(rename = "Jobs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jobs: Option<Vec<JobDetail>>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DeleteJobRequest {
    #[serde(rename = "JobName")]
    pub job_name: Option<String>,
    #[serde(rename = "JobArn")]
    pub job_arn: Option<String>,
}

}
pub use _types::*;
