use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct PutParameterRequest {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Type")]
    pub param_type: Option<String>,
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "Overwrite")]
    pub overwrite: Option<bool>,
    #[serde(rename = "KeyId")]
    pub key_id: Option<String>,
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "Tier")]
    pub tier: Option<String>,
    #[serde(rename = "DataType")]
    pub data_type: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PutParameterResponse {
    #[serde(rename = "Version")]
    pub version: i64,
    #[serde(rename = "Tier")]
    pub tier: String,
}

#[derive(Debug, Deserialize)]
pub struct GetParameterRequest {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "WithDecryption")]
    pub with_decryption: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct GetParameterResponse {
    #[serde(rename = "Parameter")]
    pub parameter: Parameter,
}

#[derive(Debug, Serialize, Clone)]
pub struct Parameter {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Type")]
    pub param_type: String,
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Version")]
    pub version: i64,
    #[serde(rename = "ARN")]
    pub arn: String,
    #[serde(rename = "LastModifiedDate")]
    pub last_modified_date: f64,
    #[serde(rename = "DataType")]
    pub data_type: String,
}

#[derive(Debug, Deserialize)]
pub struct GetParametersRequest {
    #[serde(rename = "Names")]
    pub names: Vec<String>,
    #[serde(rename = "WithDecryption")]
    pub with_decryption: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct GetParametersResponse {
    #[serde(rename = "Parameters")]
    pub parameters: Vec<Parameter>,
    #[serde(rename = "InvalidParameters")]
    pub invalid_parameters: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct GetParametersByPathRequest {
    #[serde(rename = "Path")]
    pub path: String,
    #[serde(rename = "Recursive")]
    pub recursive: Option<bool>,
    #[serde(rename = "WithDecryption")]
    pub with_decryption: Option<bool>,
    #[serde(rename = "MaxResults")]
    pub max_results: Option<usize>,
    #[serde(rename = "NextToken")]
    pub next_token: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct GetParametersByPathResponse {
    #[serde(rename = "Parameters")]
    pub parameters: Vec<Parameter>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DeleteParameterRequest {
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct DeleteParametersRequest {
    #[serde(rename = "Names")]
    pub names: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct DeleteParametersResponse {
    #[serde(rename = "DeletedParameters")]
    pub deleted_parameters: Vec<String>,
    #[serde(rename = "InvalidParameters")]
    pub invalid_parameters: Vec<String>,
}

#[derive(Debug, Deserialize, Default)]
pub struct DescribeParametersRequest {
    #[serde(rename = "MaxResults")]
    pub max_results: Option<usize>,
    #[serde(rename = "NextToken")]
    pub next_token: Option<String>,
    #[serde(rename = "ParameterFilters")]
    pub parameter_filters: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Serialize)]
pub struct DescribeParametersResponse {
    #[serde(rename = "Parameters")]
    pub parameters: Vec<ParameterMetadata>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct ParameterMetadata {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Type")]
    pub param_type: String,
    #[serde(rename = "Version")]
    pub version: i64,
    #[serde(rename = "LastModifiedDate")]
    pub last_modified_date: f64,
    #[serde(rename = "ARN")]
    pub arn: String,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Tier")]
    pub tier: String,
    #[serde(rename = "DataType")]
    pub data_type: String,
}

#[derive(Debug, Deserialize)]
pub struct AddTagsToResourceRequest {
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

#[derive(Debug, Deserialize)]
pub struct RemoveTagsFromResourceRequest {
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct ListTagsForResourceRequest {
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
}

#[derive(Debug, Serialize)]
pub struct ListTagsForResourceResponse {
    #[serde(rename = "TagList")]
    pub tag_list: Vec<Tag>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,
}
