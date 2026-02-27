mod _types {
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct CreateEventRequest {
    #[serde(rename = "EventName")]
    pub event_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct CreateEventResponse {
    #[serde(rename = "EventArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_arn: Option<String>,
    #[serde(rename = "EventName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_name: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DescribeEventRequest {
    #[serde(rename = "EventName")]
    pub event_name: Option<String>,
    #[serde(rename = "EventArn")]
    pub event_arn: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
#[allow(dead_code)]
pub struct EventDetail {
    #[serde(rename = "EventName")]
    pub event_name: String,
    #[serde(rename = "EventArn")]
    pub event_arn: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct DescribeEventResponse {
    #[serde(rename = "Event")]
    pub event: EventDetail,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct ListEventsRequest {
    #[serde(rename = "MaxResults")]
    pub max_results: Option<usize>,
    #[serde(rename = "NextToken")]
    pub next_token: Option<String>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct ListEventsResponse {
    #[serde(rename = "Events")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<EventDetail>>,
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[allow(dead_code)]
pub struct DeleteEventRequest {
    #[serde(rename = "EventName")]
    pub event_name: Option<String>,
    #[serde(rename = "EventArn")]
    pub event_arn: Option<String>,
}

}
pub use _types::*;
