use std::collections::HashMap;
use std::sync::Arc;

use axum::body::Bytes;
use axum::extract::{Path, Query, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::extract::DefaultBodyLimit;
use axum::routing::get;
use axum::Router;

use crate::s3::error::S3Error;
use crate::s3::state::S3State;
use crate::s3::types::*;

fn xml_response<T: serde::Serialize>(value: &T) -> Response {
    let mut xml = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
    if let Ok(s) = quick_xml::se::to_string(value) {
        xml.push_str(&s);
    }
    (
        StatusCode::OK,
        [("content-type", "application/xml")],
        xml,
    )
        .into_response()
}

fn extract_metadata(headers: &HeaderMap) -> HashMap<String, String> {
    let mut metadata = HashMap::new();
    for (name, value) in headers.iter() {
        let name_str = name.as_str();
        if let Some(meta_key) = name_str.strip_prefix("x-amz-meta-") {
            if let Ok(v) = value.to_str() {
                metadata.insert(meta_key.to_string(), v.to_string());
            }
        }
    }
    metadata
}

fn parse_range(header: Option<&str>, total_size: u64) -> Option<(u64, Option<u64>)> {
    let header = header?;
    let range = header.strip_prefix("bytes=")?;
    let parts: Vec<&str> = range.split('-').collect();
    if parts.len() != 2 {
        return None;
    }
    if parts[0].is_empty() {
        // suffix range: -N means last N bytes
        let n: u64 = parts[1].parse().ok()?;
        Some((total_size.saturating_sub(n), None))
    } else {
        let start: u64 = parts[0].parse().ok()?;
        let end = if parts[1].is_empty() {
            None
        } else {
            Some(parts[1].parse::<u64>().ok()?)
        };
        Some((start, end))
    }
}

fn parse_copy_source(header: &str) -> (String, String) {
    let path = if let Some(stripped) = header.strip_prefix('/') {
        stripped
    } else {
        header
    };
    // URL-decode the path
    let decoded = percent_encoding::percent_decode_str(path)
        .decode_utf8_lossy()
        .to_string();
    if let Some(pos) = decoded.find('/') {
        (decoded[..pos].to_string(), decoded[pos + 1..].to_string())
    } else {
        (decoded, String::new())
    }
}

fn tags_to_xml(tags: &HashMap<String, String>) -> String {
    let tag_entries: Vec<Tag> = tags
        .iter()
        .map(|(k, v)| Tag {
            key: k.clone(),
            value: v.clone(),
        })
        .collect();
    let tagging = Tagging {
        tag_set: TagSet { tags: tag_entries },
    };
    let mut xml = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
    if let Ok(s) = quick_xml::se::to_string(&tagging) {
        xml.push_str(&s);
    }
    xml
}

fn parse_tags_xml(body: &[u8]) -> Result<HashMap<String, String>, S3Error> {
    let tagging: Tagging = quick_xml::de::from_reader(body)
        .map_err(|e| S3Error::MalformedXML(format!("Invalid tagging XML: {e}")))?;
    let mut map = HashMap::new();
    for tag in tagging.tag_set.tags {
        map.insert(tag.key, tag.value);
    }
    Ok(map)
}

// --- Route handlers ---

async fn list_buckets_handler(
    State(state): State<Arc<S3State>>,
) -> Result<Response, S3Error> {
    let result = state.list_buckets().await?;
    Ok(xml_response(&result))
}

async fn bucket_get_handler(
    State(state): State<Arc<S3State>>,
    Path(bucket): Path<String>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Response, S3Error> {
    if params.contains_key("location") {
        let region = state.get_bucket_location(&bucket).await?;
        let resp = LocationConstraint { location: region };
        return Ok(xml_response(&resp));
    }

    if params.contains_key("versioning") {
        let status = state.get_bucket_versioning(&bucket).await?;
        let resp = VersioningConfigurationResponse { status };
        return Ok(xml_response(&resp));
    }

    if params.contains_key("tagging") {
        let tags = state.get_bucket_tagging(&bucket).await?;
        let xml = tags_to_xml(&tags);
        return Ok((StatusCode::OK, [("content-type", "application/xml")], xml).into_response());
    }

    if params.contains_key("uploads") {
        let result = state.list_multipart_uploads(&bucket).await?;
        return Ok(xml_response(&result));
    }

    // Default: ListObjectsV2
    let prefix = params.get("prefix").map(|s| s.as_str()).unwrap_or("");
    let delimiter = params.get("delimiter").map(|s| s.as_str());
    let max_keys = params
        .get("max-keys")
        .and_then(|s| s.parse().ok())
        .unwrap_or(1000);
    let continuation_token = params.get("continuation-token").map(|s| s.as_str());
    let start_after = params.get("start-after").map(|s| s.as_str());

    let result = state
        .list_objects_v2(&bucket, prefix, delimiter, max_keys, continuation_token, start_after)
        .await?;
    Ok(xml_response(&result))
}

async fn bucket_put_handler(
    State(state): State<Arc<S3State>>,
    Path(bucket): Path<String>,
    Query(params): Query<HashMap<String, String>>,
    _headers: HeaderMap,
    body: Bytes,
) -> Result<Response, S3Error> {
    if params.contains_key("versioning") {
        let config: VersioningConfiguration = quick_xml::de::from_reader(body.as_ref())
            .map_err(|e| S3Error::MalformedXML(format!("Invalid versioning XML: {e}")))?;
        state
            .put_bucket_versioning(&bucket, config.status)
            .await?;
        return Ok(StatusCode::OK.into_response());
    }

    if params.contains_key("tagging") {
        let tags = parse_tags_xml(&body)?;
        state.put_bucket_tagging(&bucket, tags).await?;
        return Ok(StatusCode::NO_CONTENT.into_response());
    }

    // CreateBucket
    let location = if body.is_empty() {
        None
    } else {
        let config: Result<CreateBucketConfiguration, _> =
            quick_xml::de::from_reader(body.as_ref());
        config
            .ok()
            .and_then(|c| c.location_constraint)
    };
    state.create_bucket(bucket, location).await?;
    Ok(StatusCode::OK.into_response())
}

async fn bucket_delete_handler(
    State(state): State<Arc<S3State>>,
    Path(bucket): Path<String>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Response, S3Error> {
    if params.contains_key("tagging") {
        state.delete_bucket_tagging(&bucket).await?;
        return Ok(StatusCode::NO_CONTENT.into_response());
    }

    state.delete_bucket(&bucket).await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}

async fn head_bucket_handler(
    State(state): State<Arc<S3State>>,
    Path(bucket): Path<String>,
) -> Result<Response, S3Error> {
    let region = state.head_bucket(&bucket).await?;
    Ok((
        StatusCode::OK,
        [("x-amz-bucket-region", region.as_str())],
        "",
    )
        .into_response())
}

async fn object_get_handler(
    State(state): State<Arc<S3State>>,
    Path((bucket, key)): Path<(String, String)>,
    Query(params): Query<HashMap<String, String>>,
    headers: HeaderMap,
) -> Result<Response, S3Error> {
    if params.contains_key("tagging") {
        let tags = state.get_object_tagging(&bucket, &key).await?;
        let xml = tags_to_xml(&tags);
        return Ok((StatusCode::OK, [("content-type", "application/xml")], xml).into_response());
    }

    if params.contains_key("uploadId") {
        let upload_id = params.get("uploadId").unwrap();
        let result = state.list_parts(&bucket, &key, upload_id).await?;
        return Ok(xml_response(&result));
    }

    // GetObject
    let range = headers
        .get("range")
        .and_then(|v| v.to_str().ok());

    // We need the object size for suffix ranges, so first get object to determine size
    let (obj, range_info) = if let Some(range_str) = range {
        // First get object to know total size for suffix ranges
        let (obj, _) = state.get_object(&bucket, &key, None).await?;
        let total = obj.data.len() as u64;
        let parsed_range = parse_range(Some(range_str), total);
        if let Some(r) = parsed_range {
            let (obj, ri) = state.get_object(&bucket, &key, Some(r)).await?;
            (obj, ri)
        } else {
            (obj, None)
        }
    } else {
        state.get_object(&bucket, &key, None).await?
    };

    let mut builder = Response::builder();
    builder = builder.header("content-type", &obj.content_type);
    builder = builder.header("etag", &obj.etag);
    builder = builder.header("last-modified", &obj.last_modified);

    for (k, v) in &obj.metadata {
        builder = builder.header(format!("x-amz-meta-{}", k), v);
    }

    if let Some((start, end, total)) = range_info {
        let slice = &obj.data[start as usize..=end as usize];
        builder = builder.status(StatusCode::PARTIAL_CONTENT);
        builder = builder.header("content-range", format!("bytes {}-{}/{}", start, end, total));
        builder = builder.header("content-length", slice.len().to_string());
        Ok(builder.body(axum::body::Body::from(slice.to_vec())).unwrap())
    } else {
        builder = builder.status(StatusCode::OK);
        builder = builder.header("content-length", obj.data.len().to_string());
        Ok(builder.body(axum::body::Body::from(obj.data)).unwrap())
    }
}

async fn object_put_handler(
    State(state): State<Arc<S3State>>,
    Path((bucket, key)): Path<(String, String)>,
    Query(params): Query<HashMap<String, String>>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, S3Error> {
    if params.contains_key("tagging") {
        let tags = parse_tags_xml(&body)?;
        state.put_object_tagging(&bucket, &key, tags).await?;
        return Ok(StatusCode::OK.into_response());
    }

    if let Some(part_str) = params.get("partNumber") {
        let upload_id = params
            .get("uploadId")
            .ok_or_else(|| S3Error::InvalidArgument("Missing uploadId".into()))?;
        let part_number: i32 = part_str
            .parse()
            .map_err(|_| S3Error::InvalidArgument("Invalid partNumber".into()))?;
        let etag = state
            .upload_part(&bucket, &key, upload_id, part_number, body.to_vec())
            .await?;
        return Ok((StatusCode::OK, [("etag", etag.as_str())], "").into_response());
    }

    // Check for CopyObject
    if let Some(copy_source) = headers.get("x-amz-copy-source").and_then(|v| v.to_str().ok()) {
        let (src_bucket, src_key) = parse_copy_source(copy_source);
        let metadata_directive = headers
            .get("x-amz-metadata-directive")
            .and_then(|v| v.to_str().ok());
        let content_type = headers
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .map(String::from);
        let metadata = extract_metadata(&headers);

        let result = state
            .copy_object(
                &bucket,
                key,
                &src_bucket,
                &src_key,
                metadata_directive,
                content_type,
                metadata,
            )
            .await?;
        return Ok(xml_response(&result));
    }

    // PutObject
    let content_type = headers
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .map(String::from);
    let metadata = extract_metadata(&headers);

    let etag = state
        .put_object(&bucket, key, body.to_vec(), content_type, metadata)
        .await?;
    Ok((StatusCode::OK, [("etag", etag.as_str())], "").into_response())
}

async fn object_delete_handler(
    State(state): State<Arc<S3State>>,
    Path((bucket, key)): Path<(String, String)>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Response, S3Error> {
    if params.contains_key("tagging") {
        state.delete_object_tagging(&bucket, &key).await?;
        return Ok(StatusCode::NO_CONTENT.into_response());
    }

    if let Some(upload_id) = params.get("uploadId") {
        state
            .abort_multipart_upload(&bucket, &key, upload_id)
            .await?;
        return Ok(StatusCode::NO_CONTENT.into_response());
    }

    state.delete_object(&bucket, &key).await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}

async fn head_object_handler(
    State(state): State<Arc<S3State>>,
    Path((bucket, key)): Path<(String, String)>,
) -> Result<Response, S3Error> {
    let obj = state.head_object(&bucket, &key).await?;
    let mut builder = Response::builder()
        .status(StatusCode::OK)
        .header("content-type", &obj.content_type)
        .header("content-length", obj.size().to_string())
        .header("etag", &obj.etag)
        .header("last-modified", &obj.last_modified);

    for (k, v) in &obj.metadata {
        builder = builder.header(format!("x-amz-meta-{}", k), v);
    }

    Ok(builder.body(axum::body::Body::empty()).unwrap())
}

async fn object_post_handler(
    State(state): State<Arc<S3State>>,
    Path((bucket, key)): Path<(String, String)>,
    Query(params): Query<HashMap<String, String>>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, S3Error> {
    if params.contains_key("uploads") {
        // CreateMultipartUpload
        let content_type = headers
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .map(String::from);
        let metadata = extract_metadata(&headers);
        let result = state
            .create_multipart_upload(&bucket, key, content_type, metadata)
            .await?;
        return Ok(xml_response(&result));
    }

    if let Some(upload_id) = params.get("uploadId") {
        // CompleteMultipartUpload
        let req: CompleteMultipartUploadRequest = quick_xml::de::from_reader(body.as_ref())
            .map_err(|e| S3Error::MalformedXML(format!("Invalid complete upload XML: {e}")))?;
        let result = state
            .complete_multipart_upload(&bucket, &key, upload_id, req.parts)
            .await?;
        return Ok(xml_response(&result));
    }

    Err(S3Error::InvalidRequest("Unknown POST operation".into()))
}

async fn bucket_post_handler(
    State(state): State<Arc<S3State>>,
    Path(bucket): Path<String>,
    Query(params): Query<HashMap<String, String>>,
    body: Bytes,
) -> Result<Response, S3Error> {
    if params.contains_key("delete") {
        let req: DeleteRequest = quick_xml::de::from_reader(body.as_ref())
            .map_err(|e| S3Error::MalformedXML(format!("Invalid delete XML: {e}")))?;
        let quiet = req.quiet.unwrap_or(false);
        let keys: Vec<String> = req.objects.into_iter().map(|o| o.key).collect();
        let result = state.delete_objects(&bucket, keys, quiet).await?;
        return Ok(xml_response(&result));
    }

    Err(S3Error::InvalidRequest("Unknown POST operation".into()))
}

pub fn create_router(state: Arc<S3State>) -> Router {
    Router::new()
        .route(
            "/",
            get(list_buckets_handler),
        )
        .route(
            "/{bucket}",
            get(bucket_get_handler)
                .put(bucket_put_handler)
                .delete(bucket_delete_handler)
                .head(head_bucket_handler)
                .post(bucket_post_handler),
        )
        .route(
            "/{bucket}/{*key}",
            get(object_get_handler)
                .put(object_put_handler)
                .delete(object_delete_handler)
                .head(head_object_handler)
                .post(object_post_handler),
        )
        .layer(DefaultBodyLimit::max(5 * 1024 * 1024 * 1024)) // 5GB max
        .with_state(state)
}
