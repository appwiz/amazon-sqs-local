mod _types {
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct CreateKeyspaceRequest {
    #[serde(rename = "KeyspaceName")]
    pub keyspace_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct CreateKeyspaceResponse {
    #[serde(rename = "KeyspaceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyspace_arn: Option<String>,
    #[serde(rename = "KeyspaceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyspace_name: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DescribeKeyspaceRequest {
    #[serde(rename = "KeyspaceName")]
    pub keyspace_name: Option<String>,
    #[serde(rename = "KeyspaceArn")]
    pub keyspace_arn: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
#[allow(dead_code)]
pub struct KeyspaceDetail {
    #[serde(rename = "KeyspaceName")]
    pub keyspace_name: String,
    #[serde(rename = "KeyspaceArn")]
    pub keyspace_arn: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct DescribeKeyspaceResponse {
    #[serde(rename = "Keyspace")]
    pub keyspace: KeyspaceDetail,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct ListKeyspacesRequest {
    #[serde(rename = "MaxResults")]
    pub max_results: Option<usize>,
    #[serde(rename = "NextToken")]
    pub next_token: Option<String>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct ListKeyspacesResponse {
    #[serde(rename = "Keyspaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyspaces: Option<Vec<KeyspaceDetail>>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DeleteKeyspaceRequest {
    #[serde(rename = "KeyspaceName")]
    pub keyspace_name: Option<String>,
    #[serde(rename = "KeyspaceArn")]
    pub keyspace_arn: Option<String>,
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

}
pub use _types::*;
