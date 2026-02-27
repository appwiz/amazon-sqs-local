mod _types {
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct CreateFleetRequest {
    #[serde(rename = "FleetName")]
    pub fleet_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct CreateFleetResponse {
    #[serde(rename = "FleetArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet_arn: Option<String>,
    #[serde(rename = "FleetName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet_name: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DescribeFleetRequest {
    #[serde(rename = "FleetName")]
    pub fleet_name: Option<String>,
    #[serde(rename = "FleetArn")]
    pub fleet_arn: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
#[allow(dead_code)]
pub struct FleetDetail {
    #[serde(rename = "FleetName")]
    pub fleet_name: String,
    #[serde(rename = "FleetArn")]
    pub fleet_arn: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct DescribeFleetResponse {
    #[serde(rename = "Fleet")]
    pub fleet: FleetDetail,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct ListFleetsRequest {
    #[serde(rename = "MaxResults")]
    pub max_results: Option<usize>,
    #[serde(rename = "NextToken")]
    pub next_token: Option<String>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct ListFleetsResponse {
    #[serde(rename = "Fleets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleets: Option<Vec<FleetDetail>>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DeleteFleetRequest {
    #[serde(rename = "FleetName")]
    pub fleet_name: Option<String>,
    #[serde(rename = "FleetArn")]
    pub fleet_arn: Option<String>,
}

}
pub use _types::*;
