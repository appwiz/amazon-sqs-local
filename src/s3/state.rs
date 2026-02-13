use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;
use uuid::Uuid;

use crate::s3::bucket::*;
use crate::s3::error::S3Error;
use crate::s3::types::*;

struct S3StateInner {
    buckets: HashMap<String, Bucket>,
    account_id: String,
    region: String,
}

pub struct S3State {
    inner: Arc<Mutex<S3StateInner>>,
}

impl S3State {
    pub fn new(account_id: String, region: String) -> Self {
        S3State {
            inner: Arc::new(Mutex::new(S3StateInner {
                buckets: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    // --- Bucket operations ---

    pub async fn create_bucket(
        &self,
        name: String,
        _location: Option<String>,
    ) -> Result<(), S3Error> {
        let mut inner = self.inner.lock().await;

        if name.len() < 3 || name.len() > 63 {
            return Err(S3Error::InvalidBucketName(
                "Bucket name must be between 3 and 63 characters".into(),
            ));
        }

        if inner.buckets.contains_key(&name) {
            return Err(S3Error::BucketAlreadyOwnedByYou(
                "Your previous request to create the named bucket succeeded and you already own it.".into(),
            ));
        }

        let bucket = Bucket::new(name.clone(), inner.region.clone());
        inner.buckets.insert(name, bucket);
        Ok(())
    }

    pub async fn delete_bucket(&self, name: &str) -> Result<(), S3Error> {
        let mut inner = self.inner.lock().await;
        let bucket = inner.buckets.get(name).ok_or_else(|| {
            S3Error::NoSuchBucket(format!("The specified bucket does not exist: {name}"))
        })?;

        if !bucket.objects.is_empty() {
            return Err(S3Error::BucketNotEmpty(
                "The bucket you tried to delete is not empty".into(),
            ));
        }

        inner.buckets.remove(name);
        Ok(())
    }

    pub async fn head_bucket(&self, name: &str) -> Result<String, S3Error> {
        let inner = self.inner.lock().await;
        let bucket = inner.buckets.get(name).ok_or_else(|| {
            S3Error::NoSuchBucket(format!("The specified bucket does not exist: {name}"))
        })?;
        Ok(bucket.region.clone())
    }

    pub async fn list_buckets(&self) -> Result<ListAllMyBucketsResult, S3Error> {
        let inner = self.inner.lock().await;
        let mut entries: Vec<BucketEntry> = inner
            .buckets
            .values()
            .map(|b| BucketEntry {
                name: b.name.clone(),
                creation_date: b.created_at.clone(),
            })
            .collect();
        entries.sort_by(|a, b| a.name.cmp(&b.name));

        Ok(ListAllMyBucketsResult {
            owner: Owner {
                id: inner.account_id.clone(),
                display_name: inner.account_id.clone(),
            },
            buckets: BucketList { bucket: entries },
        })
    }

    pub async fn get_bucket_location(&self, name: &str) -> Result<String, S3Error> {
        let inner = self.inner.lock().await;
        let bucket = inner.buckets.get(name).ok_or_else(|| {
            S3Error::NoSuchBucket(format!("The specified bucket does not exist: {name}"))
        })?;
        Ok(bucket.region.clone())
    }

    pub async fn get_bucket_versioning(&self, name: &str) -> Result<Option<String>, S3Error> {
        let inner = self.inner.lock().await;
        let bucket = inner.buckets.get(name).ok_or_else(|| {
            S3Error::NoSuchBucket(format!("The specified bucket does not exist: {name}"))
        })?;
        Ok(match bucket.versioning {
            VersioningStatus::Disabled => None,
            VersioningStatus::Enabled => Some("Enabled".into()),
            VersioningStatus::Suspended => Some("Suspended".into()),
        })
    }

    pub async fn put_bucket_versioning(
        &self,
        name: &str,
        status: Option<String>,
    ) -> Result<(), S3Error> {
        let mut inner = self.inner.lock().await;
        let bucket = inner.buckets.get_mut(name).ok_or_else(|| {
            S3Error::NoSuchBucket(format!("The specified bucket does not exist: {name}"))
        })?;
        bucket.versioning = match status.as_deref() {
            Some("Enabled") => VersioningStatus::Enabled,
            Some("Suspended") => VersioningStatus::Suspended,
            _ => VersioningStatus::Disabled,
        };
        Ok(())
    }

    pub async fn get_bucket_tagging(
        &self,
        name: &str,
    ) -> Result<HashMap<String, String>, S3Error> {
        let inner = self.inner.lock().await;
        let bucket = inner.buckets.get(name).ok_or_else(|| {
            S3Error::NoSuchBucket(format!("The specified bucket does not exist: {name}"))
        })?;
        if bucket.tags.is_empty() {
            return Err(S3Error::NoSuchTagSet("The TagSet does not exist".into()));
        }
        Ok(bucket.tags.clone())
    }

    pub async fn put_bucket_tagging(
        &self,
        name: &str,
        tags: HashMap<String, String>,
    ) -> Result<(), S3Error> {
        let mut inner = self.inner.lock().await;
        let bucket = inner.buckets.get_mut(name).ok_or_else(|| {
            S3Error::NoSuchBucket(format!("The specified bucket does not exist: {name}"))
        })?;
        bucket.tags = tags;
        Ok(())
    }

    pub async fn delete_bucket_tagging(&self, name: &str) -> Result<(), S3Error> {
        let mut inner = self.inner.lock().await;
        let bucket = inner.buckets.get_mut(name).ok_or_else(|| {
            S3Error::NoSuchBucket(format!("The specified bucket does not exist: {name}"))
        })?;
        bucket.tags.clear();
        Ok(())
    }

    // --- Object operations ---

    pub async fn put_object(
        &self,
        bucket_name: &str,
        key: String,
        data: Vec<u8>,
        content_type: Option<String>,
        metadata: HashMap<String, String>,
    ) -> Result<String, S3Error> {
        let mut inner = self.inner.lock().await;
        let bucket = inner.buckets.get_mut(bucket_name).ok_or_else(|| {
            S3Error::NoSuchBucket(format!(
                "The specified bucket does not exist: {bucket_name}"
            ))
        })?;
        let obj = Object::new(key.clone(), data, content_type, metadata);
        let etag = obj.etag.clone();
        bucket.objects.insert(key, obj);
        Ok(etag)
    }

    pub async fn get_object(
        &self,
        bucket_name: &str,
        key: &str,
        range: Option<(u64, Option<u64>)>,
    ) -> Result<(Object, Option<(u64, u64, u64)>), S3Error> {
        let inner = self.inner.lock().await;
        let bucket = inner.buckets.get(bucket_name).ok_or_else(|| {
            S3Error::NoSuchBucket(format!(
                "The specified bucket does not exist: {bucket_name}"
            ))
        })?;
        let obj = bucket.objects.get(key).ok_or_else(|| {
            S3Error::NoSuchKey(format!("The specified key does not exist: {key}"))
        })?;

        if let Some((start, end)) = range {
            let total = obj.data.len() as u64;
            let end = end.unwrap_or(total - 1).min(total - 1);
            if start >= total {
                return Err(S3Error::InvalidRange("Range not satisfiable".into()));
            }
            Ok((obj.clone(), Some((start, end, total))))
        } else {
            Ok((obj.clone(), None))
        }
    }

    pub async fn head_object(
        &self,
        bucket_name: &str,
        key: &str,
    ) -> Result<Object, S3Error> {
        let inner = self.inner.lock().await;
        let bucket = inner.buckets.get(bucket_name).ok_or_else(|| {
            S3Error::NoSuchBucket(format!(
                "The specified bucket does not exist: {bucket_name}"
            ))
        })?;
        let obj = bucket.objects.get(key).ok_or_else(|| {
            S3Error::NoSuchKey(format!("The specified key does not exist: {key}"))
        })?;
        Ok(obj.clone())
    }

    pub async fn delete_object(&self, bucket_name: &str, key: &str) -> Result<(), S3Error> {
        let mut inner = self.inner.lock().await;
        let bucket = inner.buckets.get_mut(bucket_name).ok_or_else(|| {
            S3Error::NoSuchBucket(format!(
                "The specified bucket does not exist: {bucket_name}"
            ))
        })?;
        bucket.objects.remove(key);
        Ok(())
    }

    pub async fn copy_object(
        &self,
        dest_bucket: &str,
        dest_key: String,
        source_bucket: &str,
        source_key: &str,
        metadata_directive: Option<&str>,
        content_type: Option<String>,
        metadata: HashMap<String, String>,
    ) -> Result<CopyObjectResult, S3Error> {
        let mut inner = self.inner.lock().await;

        let src_obj = {
            let src_bucket = inner.buckets.get(source_bucket).ok_or_else(|| {
                S3Error::NoSuchBucket(format!(
                    "The specified bucket does not exist: {source_bucket}"
                ))
            })?;
            src_bucket.objects.get(source_key).ok_or_else(|| {
                S3Error::NoSuchKey(format!(
                    "The specified key does not exist: {source_key}"
                ))
            })?.clone()
        };

        let (new_ct, new_meta) = if metadata_directive == Some("REPLACE") {
            (content_type.unwrap_or(src_obj.content_type.clone()), metadata)
        } else {
            (src_obj.content_type.clone(), src_obj.metadata.clone())
        };

        let new_obj = Object::new(dest_key.clone(), src_obj.data.clone(), Some(new_ct), new_meta);
        let result = CopyObjectResult {
            etag: new_obj.etag.clone(),
            last_modified: new_obj.last_modified.clone(),
        };

        let dest = inner.buckets.get_mut(dest_bucket).ok_or_else(|| {
            S3Error::NoSuchBucket(format!(
                "The specified bucket does not exist: {dest_bucket}"
            ))
        })?;
        dest.objects.insert(dest_key, new_obj);
        Ok(result)
    }

    pub async fn list_objects_v2(
        &self,
        bucket_name: &str,
        prefix: &str,
        delimiter: Option<&str>,
        max_keys: i32,
        continuation_token: Option<&str>,
        start_after: Option<&str>,
    ) -> Result<ListBucketResult, S3Error> {
        let inner = self.inner.lock().await;
        let bucket = inner.buckets.get(bucket_name).ok_or_else(|| {
            S3Error::NoSuchBucket(format!(
                "The specified bucket does not exist: {bucket_name}"
            ))
        })?;

        let mut keys: Vec<&String> = bucket
            .objects
            .keys()
            .filter(|k| k.starts_with(prefix))
            .collect();
        keys.sort();

        // Apply start_after or continuation_token
        let start_key = continuation_token.or(start_after);
        if let Some(start) = start_key {
            keys.retain(|k| k.as_str() > start);
        }

        let mut contents = Vec::new();
        let mut common_prefixes = Vec::new();
        let mut seen_prefixes = std::collections::HashSet::new();
        let max = max_keys.min(1000) as usize;
        let mut count = 0;

        for key in &keys {
            if count >= max {
                break;
            }

            if let Some(delim) = delimiter {
                let after_prefix = &key[prefix.len()..];
                if let Some(pos) = after_prefix.find(delim) {
                    let cp = format!("{}{}", prefix, &after_prefix[..=pos + delim.len() - 1]);
                    if seen_prefixes.insert(cp.clone()) {
                        common_prefixes.push(CommonPrefix { prefix: cp });
                        count += 1;
                    }
                    continue;
                }
            }

            let obj = &bucket.objects[*key];
            contents.push(ObjectEntry {
                key: obj.key.clone(),
                last_modified: obj.last_modified.clone(),
                etag: obj.etag.clone(),
                size: obj.size(),
                storage_class: obj.storage_class.clone(),
            });
            count += 1;
        }

        let is_truncated = count >= max && keys.len() > max;
        let next_token = if is_truncated {
            contents.last().map(|e| e.key.clone())
        } else {
            None
        };

        Ok(ListBucketResult {
            name: bucket_name.into(),
            prefix: prefix.into(),
            key_count: (contents.len() + common_prefixes.len()) as i32,
            max_keys,
            is_truncated,
            contents,
            common_prefixes,
            delimiter: delimiter.map(String::from),
            continuation_token: continuation_token.map(String::from),
            next_continuation_token: next_token,
            encoding_type: None,
        })
    }

    pub async fn delete_objects(
        &self,
        bucket_name: &str,
        keys: Vec<String>,
        quiet: bool,
    ) -> Result<DeleteResult, S3Error> {
        let mut inner = self.inner.lock().await;
        let bucket = inner.buckets.get_mut(bucket_name).ok_or_else(|| {
            S3Error::NoSuchBucket(format!(
                "The specified bucket does not exist: {bucket_name}"
            ))
        })?;

        let mut deleted = Vec::new();
        let errors = Vec::new();

        for key in keys {
            bucket.objects.remove(&key);
            if !quiet {
                deleted.push(DeletedEntry { key });
            }
        }

        Ok(DeleteResult { deleted, errors })
    }

    // --- Object tagging ---

    pub async fn get_object_tagging(
        &self,
        bucket_name: &str,
        key: &str,
    ) -> Result<HashMap<String, String>, S3Error> {
        let inner = self.inner.lock().await;
        let bucket = inner.buckets.get(bucket_name).ok_or_else(|| {
            S3Error::NoSuchBucket(format!(
                "The specified bucket does not exist: {bucket_name}"
            ))
        })?;
        let obj = bucket.objects.get(key).ok_or_else(|| {
            S3Error::NoSuchKey(format!("The specified key does not exist: {key}"))
        })?;
        Ok(obj.tags.clone())
    }

    pub async fn put_object_tagging(
        &self,
        bucket_name: &str,
        key: &str,
        tags: HashMap<String, String>,
    ) -> Result<(), S3Error> {
        let mut inner = self.inner.lock().await;
        let bucket = inner.buckets.get_mut(bucket_name).ok_or_else(|| {
            S3Error::NoSuchBucket(format!(
                "The specified bucket does not exist: {bucket_name}"
            ))
        })?;
        let obj = bucket.objects.get_mut(key).ok_or_else(|| {
            S3Error::NoSuchKey(format!("The specified key does not exist: {key}"))
        })?;
        obj.tags = tags;
        Ok(())
    }

    pub async fn delete_object_tagging(
        &self,
        bucket_name: &str,
        key: &str,
    ) -> Result<(), S3Error> {
        let mut inner = self.inner.lock().await;
        let bucket = inner.buckets.get_mut(bucket_name).ok_or_else(|| {
            S3Error::NoSuchBucket(format!(
                "The specified bucket does not exist: {bucket_name}"
            ))
        })?;
        let obj = bucket.objects.get_mut(key).ok_or_else(|| {
            S3Error::NoSuchKey(format!("The specified key does not exist: {key}"))
        })?;
        obj.tags.clear();
        Ok(())
    }

    // --- Multipart upload ---

    pub async fn create_multipart_upload(
        &self,
        bucket_name: &str,
        key: String,
        content_type: Option<String>,
        metadata: HashMap<String, String>,
    ) -> Result<InitiateMultipartUploadResult, S3Error> {
        let mut inner = self.inner.lock().await;
        let bucket = inner.buckets.get_mut(bucket_name).ok_or_else(|| {
            S3Error::NoSuchBucket(format!(
                "The specified bucket does not exist: {bucket_name}"
            ))
        })?;

        let upload_id = Uuid::new_v4().to_string();
        let upload = MultipartUpload {
            upload_id: upload_id.clone(),
            bucket: bucket_name.into(),
            key: key.clone(),
            parts: HashMap::new(),
            initiated: chrono::Utc::now()
                .format("%Y-%m-%dT%H:%M:%S%.3fZ")
                .to_string(),
            content_type,
            metadata,
        };
        bucket.multipart_uploads.insert(upload_id.clone(), upload);

        Ok(InitiateMultipartUploadResult {
            bucket: bucket_name.into(),
            key,
            upload_id,
        })
    }

    pub async fn upload_part(
        &self,
        bucket_name: &str,
        key: &str,
        upload_id: &str,
        part_number: i32,
        data: Vec<u8>,
    ) -> Result<String, S3Error> {
        let mut inner = self.inner.lock().await;
        let bucket = inner.buckets.get_mut(bucket_name).ok_or_else(|| {
            S3Error::NoSuchBucket(format!(
                "The specified bucket does not exist: {bucket_name}"
            ))
        })?;

        let upload = bucket
            .multipart_uploads
            .get_mut(upload_id)
            .ok_or_else(|| {
                S3Error::NoSuchUpload(format!(
                    "The specified upload does not exist: {upload_id}"
                ))
            })?;

        if upload.key != key {
            return Err(S3Error::InvalidArgument("Key does not match upload".into()));
        }

        let etag = etag_quoted(&data);
        let size = data.len();
        upload.parts.insert(
            part_number,
            Part {
                part_number,
                data,
                etag: etag.clone(),
                size,
                last_modified: chrono::Utc::now()
                    .format("%Y-%m-%dT%H:%M:%S%.3fZ")
                    .to_string(),
            },
        );
        Ok(etag)
    }

    pub async fn complete_multipart_upload(
        &self,
        bucket_name: &str,
        key: &str,
        upload_id: &str,
        parts: Vec<CompletePart>,
    ) -> Result<CompleteMultipartUploadResult, S3Error> {
        let mut inner = self.inner.lock().await;
        let bucket = inner.buckets.get_mut(bucket_name).ok_or_else(|| {
            S3Error::NoSuchBucket(format!(
                "The specified bucket does not exist: {bucket_name}"
            ))
        })?;

        let upload = bucket
            .multipart_uploads
            .remove(upload_id)
            .ok_or_else(|| {
                S3Error::NoSuchUpload(format!(
                    "The specified upload does not exist: {upload_id}"
                ))
            })?;

        // Validate parts are in order and exist
        let mut prev = 0;
        let mut combined_data = Vec::new();
        let mut etag_parts: Vec<Vec<u8>> = Vec::new();

        for cp in &parts {
            if cp.part_number <= prev {
                return Err(S3Error::InvalidPartOrder(
                    "Parts must be in ascending order".into(),
                ));
            }
            prev = cp.part_number;

            let part = upload.parts.get(&cp.part_number).ok_or_else(|| {
                S3Error::InvalidPart(format!("Part {} not found", cp.part_number))
            })?;

            combined_data.extend_from_slice(&part.data);
            // Parse the hex MD5 from the quoted etag for the composite etag
            let hex_md5 = part.etag.trim_matches('"');
            if let Ok(bytes) = hex::decode(hex_md5) {
                etag_parts.push(bytes);
            } else {
                etag_parts.push(md5_raw(&part.data));
            }
        }

        // Compute multipart ETag: MD5 of concatenated part MD5s, with -N suffix
        let mut combined_md5 = Vec::new();
        for part_md5 in &etag_parts {
            combined_md5.extend_from_slice(part_md5);
        }
        let final_md5 = md5_hex(&combined_md5);
        let etag = format!("\"{}-{}\"", final_md5, parts.len());

        let ct = upload
            .content_type
            .unwrap_or_else(|| "application/octet-stream".into());
        let obj = Object::new(key.to_string(), combined_data, Some(ct), upload.metadata);
        // Override the etag with the multipart etag
        let mut obj = obj;
        obj.etag = etag.clone();

        bucket.objects.insert(key.to_string(), obj);

        Ok(CompleteMultipartUploadResult {
            location: format!("/{}/{}", bucket_name, key),
            bucket: bucket_name.into(),
            key: key.into(),
            etag,
        })
    }

    pub async fn abort_multipart_upload(
        &self,
        bucket_name: &str,
        _key: &str,
        upload_id: &str,
    ) -> Result<(), S3Error> {
        let mut inner = self.inner.lock().await;
        let bucket = inner.buckets.get_mut(bucket_name).ok_or_else(|| {
            S3Error::NoSuchBucket(format!(
                "The specified bucket does not exist: {bucket_name}"
            ))
        })?;

        bucket
            .multipart_uploads
            .remove(upload_id)
            .ok_or_else(|| {
                S3Error::NoSuchUpload(format!(
                    "The specified upload does not exist: {upload_id}"
                ))
            })?;
        Ok(())
    }

    pub async fn list_multipart_uploads(
        &self,
        bucket_name: &str,
    ) -> Result<ListMultipartUploadsResult, S3Error> {
        let inner = self.inner.lock().await;
        let bucket = inner.buckets.get(bucket_name).ok_or_else(|| {
            S3Error::NoSuchBucket(format!(
                "The specified bucket does not exist: {bucket_name}"
            ))
        })?;

        let mut uploads: Vec<UploadEntry> = bucket
            .multipart_uploads
            .values()
            .map(|u| UploadEntry {
                key: u.key.clone(),
                upload_id: u.upload_id.clone(),
                initiated: u.initiated.clone(),
                storage_class: "STANDARD".into(),
            })
            .collect();
        uploads.sort_by(|a, b| a.key.cmp(&b.key));

        Ok(ListMultipartUploadsResult {
            bucket: bucket_name.into(),
            key_marker: String::new(),
            upload_id_marker: String::new(),
            max_uploads: 1000,
            is_truncated: false,
            uploads,
        })
    }

    pub async fn list_parts(
        &self,
        bucket_name: &str,
        key: &str,
        upload_id: &str,
    ) -> Result<ListPartsResult, S3Error> {
        let inner = self.inner.lock().await;
        let bucket = inner.buckets.get(bucket_name).ok_or_else(|| {
            S3Error::NoSuchBucket(format!(
                "The specified bucket does not exist: {bucket_name}"
            ))
        })?;

        let upload = bucket
            .multipart_uploads
            .get(upload_id)
            .ok_or_else(|| {
                S3Error::NoSuchUpload(format!(
                    "The specified upload does not exist: {upload_id}"
                ))
            })?;

        let mut parts: Vec<PartEntry> = upload
            .parts
            .values()
            .map(|p| PartEntry {
                part_number: p.part_number,
                last_modified: p.last_modified.clone(),
                etag: p.etag.clone(),
                size: p.size,
            })
            .collect();
        parts.sort_by_key(|p| p.part_number);

        Ok(ListPartsResult {
            bucket: bucket_name.into(),
            key: key.into(),
            upload_id: upload_id.into(),
            max_parts: 1000,
            is_truncated: false,
            parts,
        })
    }
}

fn md5_raw(data: &[u8]) -> Vec<u8> {
    use md5::{Digest, Md5};
    let mut hasher = Md5::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

mod hex {
    pub fn decode(s: &str) -> Result<Vec<u8>, ()> {
        if s.len() % 2 != 0 {
            return Err(());
        }
        (0..s.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|_| ()))
            .collect()
    }
}
