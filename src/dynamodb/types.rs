use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// DynamoDB AttributeValue is represented as serde_json::Value
// e.g. {"S": "hello"}, {"N": "123"}, {"BOOL": true}, {"NULL": true},
//      {"L": [...]}, {"M": {...}}, {"SS": [...]}, {"NS": [...]}, {"BS": [...]}
pub type AttributeValue = Value;
pub type Item = HashMap<String, AttributeValue>;

// --- Shared types ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeySchemaElement {
    #[serde(rename = "AttributeName")]
    pub attribute_name: String,
    #[serde(rename = "KeyType")]
    pub key_type: String, // HASH or RANGE
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttributeDefinition {
    #[serde(rename = "AttributeName")]
    pub attribute_name: String,
    #[serde(rename = "AttributeType")]
    pub attribute_type: String, // S, N, B
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvisionedThroughput {
    #[serde(rename = "ReadCapacityUnits")]
    pub read_capacity_units: i64,
    #[serde(rename = "WriteCapacityUnits")]
    pub write_capacity_units: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvisionedThroughputDescription {
    #[serde(rename = "ReadCapacityUnits")]
    pub read_capacity_units: i64,
    #[serde(rename = "WriteCapacityUnits")]
    pub write_capacity_units: i64,
    #[serde(rename = "LastIncreaseDateTime", skip_serializing_if = "Option::is_none")]
    pub last_increase_date_time: Option<f64>,
    #[serde(rename = "LastDecreaseDateTime", skip_serializing_if = "Option::is_none")]
    pub last_decrease_date_time: Option<f64>,
    #[serde(rename = "NumberOfDecreasesToday")]
    pub number_of_decreases_today: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct TableDescription {
    #[serde(rename = "TableName")]
    pub table_name: String,
    #[serde(rename = "TableStatus")]
    pub table_status: String,
    #[serde(rename = "TableArn")]
    pub table_arn: String,
    #[serde(rename = "TableId")]
    pub table_id: String,
    #[serde(rename = "CreationDateTime")]
    pub creation_date_time: f64,
    #[serde(rename = "KeySchema")]
    pub key_schema: Vec<KeySchemaElement>,
    #[serde(rename = "AttributeDefinitions")]
    pub attribute_definitions: Vec<AttributeDefinition>,
    #[serde(rename = "ProvisionedThroughput")]
    pub provisioned_throughput: ProvisionedThroughputDescription,
    #[serde(rename = "BillingModeSummary", skip_serializing_if = "Option::is_none")]
    pub billing_mode_summary: Option<BillingModeSummary>,
    #[serde(rename = "ItemCount")]
    pub item_count: i64,
    #[serde(rename = "TableSizeBytes")]
    pub table_size_bytes: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct BillingModeSummary {
    #[serde(rename = "BillingMode")]
    pub billing_mode: String,
    #[serde(rename = "LastUpdateToPayPerRequestDateTime", skip_serializing_if = "Option::is_none")]
    pub last_update_to_pay_per_request_date_time: Option<f64>,
}

// --- CreateTable ---

#[derive(Debug, Clone, Deserialize)]
pub struct CreateTableRequest {
    #[serde(rename = "TableName")]
    pub table_name: String,
    #[serde(rename = "KeySchema")]
    pub key_schema: Vec<KeySchemaElement>,
    #[serde(rename = "AttributeDefinitions")]
    pub attribute_definitions: Vec<AttributeDefinition>,
    #[serde(rename = "BillingMode", default)]
    pub billing_mode: Option<String>,
    #[serde(rename = "ProvisionedThroughput", default)]
    pub provisioned_throughput: Option<ProvisionedThroughput>,
    #[serde(rename = "Tags", default)]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateTableResponse {
    #[serde(rename = "TableDescription")]
    pub table_description: TableDescription,
}

// --- DeleteTable ---

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteTableRequest {
    #[serde(rename = "TableName")]
    pub table_name: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct DeleteTableResponse {
    #[serde(rename = "TableDescription")]
    pub table_description: TableDescription,
}

// --- DescribeTable ---

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeTableRequest {
    #[serde(rename = "TableName")]
    pub table_name: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct DescribeTableResponse {
    #[serde(rename = "Table")]
    pub table: TableDescription,
}

// --- ListTables ---

#[derive(Debug, Clone, Deserialize, Default)]
pub struct ListTablesRequest {
    #[serde(rename = "ExclusiveStartTableName", default)]
    pub exclusive_start_table_name: Option<String>,
    #[serde(rename = "Limit", default)]
    pub limit: Option<i32>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ListTablesResponse {
    #[serde(rename = "TableNames")]
    pub table_names: Vec<String>,
    #[serde(rename = "LastEvaluatedTableName", skip_serializing_if = "Option::is_none")]
    pub last_evaluated_table_name: Option<String>,
}

// --- UpdateTable ---

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateTableRequest {
    #[serde(rename = "TableName")]
    pub table_name: String,
    #[serde(rename = "BillingMode", default)]
    pub billing_mode: Option<String>,
    #[serde(rename = "ProvisionedThroughput", default)]
    pub provisioned_throughput: Option<ProvisionedThroughput>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateTableResponse {
    #[serde(rename = "TableDescription")]
    pub table_description: TableDescription,
}

// --- PutItem ---

#[derive(Debug, Clone, Deserialize)]
pub struct PutItemRequest {
    #[serde(rename = "TableName")]
    pub table_name: String,
    #[serde(rename = "Item")]
    pub item: Item,
    #[serde(rename = "ConditionExpression", default)]
    pub condition_expression: Option<String>,
    #[serde(rename = "ExpressionAttributeNames", default)]
    pub expression_attribute_names: Option<HashMap<String, String>>,
    #[serde(rename = "ExpressionAttributeValues", default)]
    pub expression_attribute_values: Option<Item>,
    #[serde(rename = "ReturnValues", default)]
    pub return_values: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PutItemResponse {
    #[serde(rename = "Attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Item>,
}

// --- GetItem ---

#[derive(Debug, Clone, Deserialize)]
pub struct GetItemRequest {
    #[serde(rename = "TableName")]
    pub table_name: String,
    #[serde(rename = "Key")]
    pub key: Item,
    #[serde(rename = "ConsistentRead", default)]
    pub consistent_read: Option<bool>,
    #[serde(rename = "ProjectionExpression", default)]
    pub projection_expression: Option<String>,
    #[serde(rename = "ExpressionAttributeNames", default)]
    pub expression_attribute_names: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct GetItemResponse {
    #[serde(rename = "Item", skip_serializing_if = "Option::is_none")]
    pub item: Option<Item>,
}

// --- DeleteItem ---

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteItemRequest {
    #[serde(rename = "TableName")]
    pub table_name: String,
    #[serde(rename = "Key")]
    pub key: Item,
    #[serde(rename = "ConditionExpression", default)]
    pub condition_expression: Option<String>,
    #[serde(rename = "ExpressionAttributeNames", default)]
    pub expression_attribute_names: Option<HashMap<String, String>>,
    #[serde(rename = "ExpressionAttributeValues", default)]
    pub expression_attribute_values: Option<Item>,
    #[serde(rename = "ReturnValues", default)]
    pub return_values: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DeleteItemResponse {
    #[serde(rename = "Attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Item>,
}

// --- UpdateItem ---

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateItemRequest {
    #[serde(rename = "TableName")]
    pub table_name: String,
    #[serde(rename = "Key")]
    pub key: Item,
    #[serde(rename = "UpdateExpression", default)]
    pub update_expression: Option<String>,
    #[serde(rename = "ConditionExpression", default)]
    pub condition_expression: Option<String>,
    #[serde(rename = "ExpressionAttributeNames", default)]
    pub expression_attribute_names: Option<HashMap<String, String>>,
    #[serde(rename = "ExpressionAttributeValues", default)]
    pub expression_attribute_values: Option<Item>,
    #[serde(rename = "ReturnValues", default)]
    pub return_values: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateItemResponse {
    #[serde(rename = "Attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Item>,
}

// --- Query ---

#[derive(Debug, Clone, Deserialize)]
pub struct QueryRequest {
    #[serde(rename = "TableName")]
    pub table_name: String,
    #[serde(rename = "KeyConditionExpression", default)]
    pub key_condition_expression: Option<String>,
    #[serde(rename = "FilterExpression", default)]
    pub filter_expression: Option<String>,
    #[serde(rename = "ProjectionExpression", default)]
    pub projection_expression: Option<String>,
    #[serde(rename = "ExpressionAttributeNames", default)]
    pub expression_attribute_names: Option<HashMap<String, String>>,
    #[serde(rename = "ExpressionAttributeValues", default)]
    pub expression_attribute_values: Option<Item>,
    #[serde(rename = "ScanIndexForward", default)]
    pub scan_index_forward: Option<bool>,
    #[serde(rename = "Limit", default)]
    pub limit: Option<i32>,
    #[serde(rename = "ExclusiveStartKey", default)]
    pub exclusive_start_key: Option<Item>,
    #[serde(rename = "Select", default)]
    pub select: Option<String>,
    #[serde(rename = "ConsistentRead", default)]
    pub consistent_read: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
pub struct QueryResponse {
    #[serde(rename = "Items")]
    pub items: Vec<Item>,
    #[serde(rename = "Count")]
    pub count: i64,
    #[serde(rename = "ScannedCount")]
    pub scanned_count: i64,
    #[serde(rename = "LastEvaluatedKey", skip_serializing_if = "Option::is_none")]
    pub last_evaluated_key: Option<Item>,
}

// --- Scan ---

#[derive(Debug, Clone, Deserialize)]
pub struct ScanRequest {
    #[serde(rename = "TableName")]
    pub table_name: String,
    #[serde(rename = "FilterExpression", default)]
    pub filter_expression: Option<String>,
    #[serde(rename = "ProjectionExpression", default)]
    pub projection_expression: Option<String>,
    #[serde(rename = "ExpressionAttributeNames", default)]
    pub expression_attribute_names: Option<HashMap<String, String>>,
    #[serde(rename = "ExpressionAttributeValues", default)]
    pub expression_attribute_values: Option<Item>,
    #[serde(rename = "Limit", default)]
    pub limit: Option<i32>,
    #[serde(rename = "ExclusiveStartKey", default)]
    pub exclusive_start_key: Option<Item>,
    #[serde(rename = "Select", default)]
    pub select: Option<String>,
    #[serde(rename = "ConsistentRead", default)]
    pub consistent_read: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ScanResponse {
    #[serde(rename = "Items")]
    pub items: Vec<Item>,
    #[serde(rename = "Count")]
    pub count: i64,
    #[serde(rename = "ScannedCount")]
    pub scanned_count: i64,
    #[serde(rename = "LastEvaluatedKey", skip_serializing_if = "Option::is_none")]
    pub last_evaluated_key: Option<Item>,
}

// --- BatchGetItem ---

#[derive(Debug, Clone, Deserialize)]
pub struct BatchGetItemRequest {
    #[serde(rename = "RequestItems")]
    pub request_items: HashMap<String, KeysAndAttributes>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KeysAndAttributes {
    #[serde(rename = "Keys")]
    pub keys: Vec<Item>,
    #[serde(rename = "ConsistentRead", default)]
    pub consistent_read: Option<bool>,
    #[serde(rename = "ProjectionExpression", default)]
    pub projection_expression: Option<String>,
    #[serde(rename = "ExpressionAttributeNames", default)]
    pub expression_attribute_names: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct BatchGetItemResponse {
    #[serde(rename = "Responses")]
    pub responses: HashMap<String, Vec<Item>>,
    #[serde(rename = "UnprocessedKeys")]
    pub unprocessed_keys: HashMap<String, Value>,
}

// --- BatchWriteItem ---

#[derive(Debug, Clone, Deserialize)]
pub struct BatchWriteItemRequest {
    #[serde(rename = "RequestItems")]
    pub request_items: HashMap<String, Vec<WriteRequest>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WriteRequest {
    #[serde(rename = "PutRequest", default)]
    pub put_request: Option<PutRequest>,
    #[serde(rename = "DeleteRequest", default)]
    pub delete_request: Option<DeleteRequest>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PutRequest {
    #[serde(rename = "Item")]
    pub item: Item,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteRequest {
    #[serde(rename = "Key")]
    pub key: Item,
}

#[derive(Debug, Clone, Serialize)]
pub struct BatchWriteItemResponse {
    #[serde(rename = "UnprocessedItems")]
    pub unprocessed_items: HashMap<String, Value>,
}

// --- TagResource ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TagResourceRequest {
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

// --- UntagResource ---

#[derive(Debug, Clone, Deserialize)]
pub struct UntagResourceRequest {
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

// --- ListTagsOfResource ---

#[derive(Debug, Clone, Deserialize)]
pub struct ListTagsOfResourceRequest {
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    #[serde(rename = "NextToken", default)]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ListTagsOfResourceResponse {
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
    #[serde(rename = "NextToken", skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}
