mod _types {
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct CreateVehicleRequest {
    #[serde(rename = "VehicleName")]
    pub vehicle_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct CreateVehicleResponse {
    #[serde(rename = "VehicleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vehicle_arn: Option<String>,
    #[serde(rename = "VehicleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vehicle_name: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DescribeVehicleRequest {
    #[serde(rename = "VehicleName")]
    pub vehicle_name: Option<String>,
    #[serde(rename = "VehicleArn")]
    pub vehicle_arn: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
#[allow(dead_code)]
pub struct VehicleDetail {
    #[serde(rename = "VehicleName")]
    pub vehicle_name: String,
    #[serde(rename = "VehicleArn")]
    pub vehicle_arn: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct DescribeVehicleResponse {
    #[serde(rename = "Vehicle")]
    pub vehicle: VehicleDetail,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct ListVehiclesRequest {
    #[serde(rename = "MaxResults")]
    pub max_results: Option<usize>,
    #[serde(rename = "NextToken")]
    pub next_token: Option<String>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct ListVehiclesResponse {
    #[serde(rename = "Vehicles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vehicles: Option<Vec<VehicleDetail>>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DeleteVehicleRequest {
    #[serde(rename = "VehicleName")]
    pub vehicle_name: Option<String>,
    #[serde(rename = "VehicleArn")]
    pub vehicle_arn: Option<String>,
}

}
pub use _types::*;
