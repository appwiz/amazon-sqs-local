use std::collections::HashMap;

use chrono::{DateTime, Utc};
use md5::{Digest, Md5};

fn now_rfc3339() -> String {
    let dt: DateTime<Utc> = Utc::now();
    dt.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string()
}

pub fn md5_hex(data: &[u8]) -> String {
    let mut hasher = Md5::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

pub fn etag_quoted(data: &[u8]) -> String {
    format!("\"{}\"", md5_hex(data))
}

#[derive(Debug, Clone)]
pub struct Object {
    pub key: String,
    pub data: Vec<u8>,
    pub content_type: String,
    pub etag: String,
    pub last_modified: String,
    pub metadata: HashMap<String, String>,
    pub tags: HashMap<String, String>,
    pub storage_class: String,
}

impl Object {
    pub fn new(key: String, data: Vec<u8>, content_type: Option<String>, metadata: HashMap<String, String>) -> Self {
        let etag = etag_quoted(&data);
        Object {
            key,
            content_type: content_type.unwrap_or_else(|| "application/octet-stream".into()),
            etag,
            last_modified: now_rfc3339(),
            data,
            metadata,
            tags: HashMap::new(),
            storage_class: "STANDARD".into(),
        }
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }
}

#[derive(Debug, Clone)]
pub struct Part {
    pub part_number: i32,
    pub data: Vec<u8>,
    pub etag: String,
    pub size: usize,
    pub last_modified: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct MultipartUpload {
    pub upload_id: String,
    pub bucket: String,
    pub key: String,
    pub parts: HashMap<i32, Part>,
    pub initiated: String,
    pub content_type: Option<String>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum VersioningStatus {
    Disabled,
    Enabled,
    Suspended,
}

#[derive(Debug, Clone)]
pub struct Bucket {
    pub name: String,
    pub region: String,
    pub created_at: String,
    pub objects: HashMap<String, Object>,
    pub tags: HashMap<String, String>,
    pub versioning: VersioningStatus,
    pub multipart_uploads: HashMap<String, MultipartUpload>,
}

impl Bucket {
    pub fn new(name: String, region: String) -> Self {
        Bucket {
            name,
            region,
            created_at: now_rfc3339(),
            objects: HashMap::new(),
            tags: HashMap::new(),
            versioning: VersioningStatus::Disabled,
            multipart_uploads: HashMap::new(),
        }
    }
}
