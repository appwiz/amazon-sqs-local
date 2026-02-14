use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct DeliveryStream {
    pub name: String,
    pub arn: String,
    pub status: String,
    pub stream_type: String,
    pub create_timestamp: f64,
    pub last_update_timestamp: f64,
    pub version_id: String,
    pub destinations: Vec<Destination>,
    pub tags: HashMap<String, String>,
    pub records: Vec<StoredRecord>,
}

#[derive(Debug, Clone)]
pub struct Destination {
    pub destination_id: String,
    pub config: serde_json::Value,
}

#[derive(Debug, Clone)]
pub struct StoredRecord {
    pub record_id: String,
    pub data: String,
}

impl DeliveryStream {
    pub fn new(name: String, arn: String, stream_type: String) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();

        DeliveryStream {
            name,
            arn,
            status: "ACTIVE".to_string(),
            stream_type,
            create_timestamp: now,
            last_update_timestamp: now,
            version_id: "1".to_string(),
            destinations: vec![Destination {
                destination_id: "destinationId-000000000001".to_string(),
                config: serde_json::json!({}),
            }],
            tags: HashMap::new(),
            records: Vec::new(),
        }
    }
}
