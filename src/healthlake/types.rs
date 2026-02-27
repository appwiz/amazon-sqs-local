mod _types {
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct CreateFHIRDatastoreRequest {
    #[serde(rename = "FHIRDatastoreName")]
    pub f_h_i_r_datastore_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct CreateFHIRDatastoreResponse {
    #[serde(rename = "FHIRDatastoreArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub f_h_i_r_datastore_arn: Option<String>,
    #[serde(rename = "FHIRDatastoreName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub f_h_i_r_datastore_name: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DescribeFHIRDatastoreRequest {
    #[serde(rename = "FHIRDatastoreName")]
    pub f_h_i_r_datastore_name: Option<String>,
    #[serde(rename = "FHIRDatastoreArn")]
    pub f_h_i_r_datastore_arn: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
#[allow(dead_code)]
pub struct FHIRDatastoreDetail {
    #[serde(rename = "FHIRDatastoreName")]
    pub f_h_i_r_datastore_name: String,
    #[serde(rename = "FHIRDatastoreArn")]
    pub f_h_i_r_datastore_arn: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct DescribeFHIRDatastoreResponse {
    #[serde(rename = "FHIRDatastore")]
    pub f_h_i_r_datastore: FHIRDatastoreDetail,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct ListFHIRDatastoresRequest {
    #[serde(rename = "MaxResults")]
    pub max_results: Option<usize>,
    #[serde(rename = "NextToken")]
    pub next_token: Option<String>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct ListFHIRDatastoresResponse {
    #[serde(rename = "FHIRDatastores")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub f_h_i_r_datastores: Option<Vec<FHIRDatastoreDetail>>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DeleteFHIRDatastoreRequest {
    #[serde(rename = "FHIRDatastoreName")]
    pub f_h_i_r_datastore_name: Option<String>,
    #[serde(rename = "FHIRDatastoreArn")]
    pub f_h_i_r_datastore_arn: Option<String>,
}

}
pub use _types::*;
