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

    if let Some(upload_id) = params.get("uploadId") {
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
        let start_idx = start as usize;
        let end_idx = (end as usize).min(obj.data.len().saturating_sub(1));
        if start_idx > end_idx || start_idx >= obj.data.len() {
            return Err(S3Error::InvalidRange(format!(
                "Range {}-{} not satisfiable for object of size {}",
                start, end, obj.data.len()
            )));
        }
        let slice = &obj.data[start_idx..=end_idx];
        builder = builder.status(StatusCode::PARTIAL_CONTENT);
        builder = builder.header("content-range", format!("bytes {}-{}/{}", start, end, total));
        builder = builder.header("content-length", slice.len().to_string());
        Ok(builder
            .body(axum::body::Body::from(slice.to_vec()))
            .map_err(|_| S3Error::InternalError("Failed to build response".into()))?)
    } else {
        builder = builder.status(StatusCode::OK);
        builder = builder.header("content-length", obj.data.len().to_string());
        Ok(builder
            .body(axum::body::Body::from(obj.data))
            .map_err(|_| S3Error::InternalError("Failed to build response".into()))?)
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

    Ok(builder
        .body(axum::body::Body::empty())
        .map_err(|_| S3Error::InternalError("Failed to build response".into()))?)
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

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;

    fn new_state() -> Arc<S3State> {
        Arc::new(S3State::new("123456789012".to_string(), "us-east-1".to_string()))
    }

    #[tokio::test]
    async fn test_list_buckets_empty() {
        let app = create_router(new_state());
        let req = Request::builder()
            .method("GET")
            .uri("/")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_create_bucket() {
        let app = create_router(new_state());
        let req = Request::builder()
            .method("PUT")
            .uri("/my-bucket")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_create_and_list_buckets() {
        let state = new_state();
        let app = create_router(state.clone());
        let req = Request::builder()
            .method("PUT")
            .uri("/my-bucket")
            .body(Body::empty())
            .unwrap();
        app.oneshot(req).await.unwrap();

        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX).await.unwrap();
        let body_str = String::from_utf8(body.to_vec()).unwrap();
        assert!(body_str.contains("my-bucket"));
    }

    #[tokio::test]
    async fn test_head_bucket() {
        let state = new_state();
        let app = create_router(state.clone());
        let req = Request::builder()
            .method("PUT")
            .uri("/my-bucket")
            .body(Body::empty())
            .unwrap();
        app.oneshot(req).await.unwrap();

        let app = create_router(state);
        let req = Request::builder()
            .method("HEAD")
            .uri("/my-bucket")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        assert!(resp.headers().get("x-amz-bucket-region").is_some());
    }

    #[tokio::test]
    async fn test_head_bucket_not_found() {
        let app = create_router(new_state());
        let req = Request::builder()
            .method("HEAD")
            .uri("/nonexistent")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_delete_bucket() {
        let state = new_state();
        let app = create_router(state.clone());
        let req = Request::builder()
            .method("PUT")
            .uri("/my-bucket")
            .body(Body::empty())
            .unwrap();
        app.oneshot(req).await.unwrap();

        let app = create_router(state);
        let req = Request::builder()
            .method("DELETE")
            .uri("/my-bucket")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::NO_CONTENT);
    }

    #[tokio::test]
    async fn test_put_and_get_object() {
        let state = new_state();
        // Create bucket
        let app = create_router(state.clone());
        let req = Request::builder()
            .method("PUT")
            .uri("/my-bucket")
            .body(Body::empty())
            .unwrap();
        app.oneshot(req).await.unwrap();

        // Put object
        let app = create_router(state.clone());
        let req = Request::builder()
            .method("PUT")
            .uri("/my-bucket/test-key.txt")
            .header("content-type", "text/plain")
            .body(Body::from("hello world"))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        assert!(resp.headers().get("etag").is_some());

        // Get object
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/my-bucket/test-key.txt")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX).await.unwrap();
        assert_eq!(&body[..], b"hello world");
    }

    #[tokio::test]
    async fn test_head_object() {
        let state = new_state();
        let app = create_router(state.clone());
        let req = Request::builder().method("PUT").uri("/bkt").body(Body::empty()).unwrap();
        app.oneshot(req).await.unwrap();

        let app = create_router(state.clone());
        let req = Request::builder()
            .method("PUT")
            .uri("/bkt/key")
            .body(Body::from("data"))
            .unwrap();
        app.oneshot(req).await.unwrap();

        let app = create_router(state);
        let req = Request::builder()
            .method("HEAD")
            .uri("/bkt/key")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        assert!(resp.headers().get("etag").is_some());
        assert!(resp.headers().get("content-length").is_some());
    }

    #[tokio::test]
    async fn test_delete_object() {
        let state = new_state();
        let app = create_router(state.clone());
        let req = Request::builder().method("PUT").uri("/bkt").body(Body::empty()).unwrap();
        app.oneshot(req).await.unwrap();

        let app = create_router(state.clone());
        let req = Request::builder()
            .method("PUT")
            .uri("/bkt/key")
            .body(Body::from("data"))
            .unwrap();
        app.oneshot(req).await.unwrap();

        let app = create_router(state.clone());
        let req = Request::builder()
            .method("DELETE")
            .uri("/bkt/key")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::NO_CONTENT);

        // Verify object is gone
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/bkt/key")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_list_objects_v2() {
        let state = new_state();
        let app = create_router(state.clone());
        let req = Request::builder().method("PUT").uri("/bkt").body(Body::empty()).unwrap();
        app.oneshot(req).await.unwrap();

        let app = create_router(state.clone());
        let req = Request::builder()
            .method("PUT")
            .uri("/bkt/file1.txt")
            .body(Body::from("aaa"))
            .unwrap();
        app.oneshot(req).await.unwrap();

        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/bkt?prefix=file")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX).await.unwrap();
        let body_str = String::from_utf8(body.to_vec()).unwrap();
        assert!(body_str.contains("file1.txt"));
    }

    #[tokio::test]
    async fn test_get_bucket_location() {
        let state = new_state();
        let app = create_router(state.clone());
        let req = Request::builder().method("PUT").uri("/bkt").body(Body::empty()).unwrap();
        app.oneshot(req).await.unwrap();

        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/bkt?location")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_put_and_get_bucket_versioning() {
        let state = new_state();
        let app = create_router(state.clone());
        let req = Request::builder().method("PUT").uri("/bkt").body(Body::empty()).unwrap();
        app.oneshot(req).await.unwrap();

        let versioning_xml = r#"<VersioningConfiguration xmlns="http://s3.amazonaws.com/doc/2006-03-01/"><Status>Enabled</Status></VersioningConfiguration>"#;
        let app = create_router(state.clone());
        let req = Request::builder()
            .method("PUT")
            .uri("/bkt?versioning")
            .header("content-type", "application/xml")
            .body(Body::from(versioning_xml))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/bkt?versioning")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_put_and_get_bucket_tagging() {
        let state = new_state();
        let app = create_router(state.clone());
        let req = Request::builder().method("PUT").uri("/bkt").body(Body::empty()).unwrap();
        app.oneshot(req).await.unwrap();

        let tagging_xml = r#"<Tagging><TagSet><Tag><Key>env</Key><Value>test</Value></Tag></TagSet></Tagging>"#;
        let app = create_router(state.clone());
        let req = Request::builder()
            .method("PUT")
            .uri("/bkt?tagging")
            .body(Body::from(tagging_xml))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::NO_CONTENT);

        let app = create_router(state.clone());
        let req = Request::builder()
            .method("GET")
            .uri("/bkt?tagging")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        // Delete tagging
        let app = create_router(state);
        let req = Request::builder()
            .method("DELETE")
            .uri("/bkt?tagging")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::NO_CONTENT);
    }

    #[tokio::test]
    async fn test_object_metadata() {
        let state = new_state();
        let app = create_router(state.clone());
        let req = Request::builder().method("PUT").uri("/bkt").body(Body::empty()).unwrap();
        app.oneshot(req).await.unwrap();

        let app = create_router(state.clone());
        let req = Request::builder()
            .method("PUT")
            .uri("/bkt/key")
            .header("x-amz-meta-custom", "value123")
            .body(Body::from("data"))
            .unwrap();
        app.oneshot(req).await.unwrap();

        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/bkt/key")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(
            resp.headers().get("x-amz-meta-custom").unwrap().to_str().unwrap(),
            "value123"
        );
    }

    #[tokio::test]
    async fn test_copy_object() {
        let state = new_state();
        let app = create_router(state.clone());
        let req = Request::builder().method("PUT").uri("/src-bkt").body(Body::empty()).unwrap();
        app.oneshot(req).await.unwrap();

        let app = create_router(state.clone());
        let req = Request::builder().method("PUT").uri("/dst-bkt").body(Body::empty()).unwrap();
        app.oneshot(req).await.unwrap();

        let app = create_router(state.clone());
        let req = Request::builder()
            .method("PUT")
            .uri("/src-bkt/orig")
            .body(Body::from("original"))
            .unwrap();
        app.oneshot(req).await.unwrap();

        let app = create_router(state.clone());
        let req = Request::builder()
            .method("PUT")
            .uri("/dst-bkt/copy")
            .header("x-amz-copy-source", "/src-bkt/orig")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        // Verify copy
        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/dst-bkt/copy")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX).await.unwrap();
        assert_eq!(&body[..], b"original");
    }

    #[tokio::test]
    async fn test_get_object_range() {
        let state = new_state();
        let app = create_router(state.clone());
        let req = Request::builder().method("PUT").uri("/bkt").body(Body::empty()).unwrap();
        app.oneshot(req).await.unwrap();

        let app = create_router(state.clone());
        let req = Request::builder()
            .method("PUT")
            .uri("/bkt/key")
            .body(Body::from("0123456789"))
            .unwrap();
        app.oneshot(req).await.unwrap();

        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/bkt/key")
            .header("range", "bytes=0-4")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::PARTIAL_CONTENT);
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX).await.unwrap();
        assert_eq!(&body[..], b"01234");
    }

    #[tokio::test]
    async fn test_put_and_get_object_tagging() {
        let state = new_state();
        let app = create_router(state.clone());
        let req = Request::builder().method("PUT").uri("/bkt").body(Body::empty()).unwrap();
        app.oneshot(req).await.unwrap();

        let app = create_router(state.clone());
        let req = Request::builder()
            .method("PUT")
            .uri("/bkt/key")
            .body(Body::from("data"))
            .unwrap();
        app.oneshot(req).await.unwrap();

        let tagging_xml = r#"<Tagging><TagSet><Tag><Key>env</Key><Value>prod</Value></Tag></TagSet></Tagging>"#;
        let app = create_router(state.clone());
        let req = Request::builder()
            .method("PUT")
            .uri("/bkt/key?tagging")
            .body(Body::from(tagging_xml))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/bkt/key?tagging")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_delete_objects_batch() {
        let state = new_state();
        let app = create_router(state.clone());
        let req = Request::builder().method("PUT").uri("/bkt").body(Body::empty()).unwrap();
        app.oneshot(req).await.unwrap();

        for name in &["a", "b", "c"] {
            let app = create_router(state.clone());
            let req = Request::builder()
                .method("PUT")
                .uri(format!("/bkt/{}", name))
                .body(Body::from("data"))
                .unwrap();
            app.oneshot(req).await.unwrap();
        }

        let delete_xml = r#"<Delete><Object><Key>a</Key></Object><Object><Key>b</Key></Object></Delete>"#;
        let app = create_router(state);
        let req = Request::builder()
            .method("POST")
            .uri("/bkt?delete")
            .body(Body::from(delete_xml))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_multipart_upload() {
        let state = new_state();
        let app = create_router(state.clone());
        let req = Request::builder().method("PUT").uri("/bkt").body(Body::empty()).unwrap();
        app.oneshot(req).await.unwrap();

        // Initiate multipart upload
        let app = create_router(state.clone());
        let req = Request::builder()
            .method("POST")
            .uri("/bkt/bigfile?uploads")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX).await.unwrap();
        let body_str = String::from_utf8(body.to_vec()).unwrap();
        assert!(body_str.contains("UploadId"));
    }

    #[tokio::test]
    async fn test_list_multipart_uploads() {
        let state = new_state();
        let app = create_router(state.clone());
        let req = Request::builder().method("PUT").uri("/bkt").body(Body::empty()).unwrap();
        app.oneshot(req).await.unwrap();

        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/bkt?uploads")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_get_object_not_found() {
        let state = new_state();
        let app = create_router(state.clone());
        let req = Request::builder().method("PUT").uri("/bkt").body(Body::empty()).unwrap();
        app.oneshot(req).await.unwrap();

        let app = create_router(state);
        let req = Request::builder()
            .method("GET")
            .uri("/bkt/nokey")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }
}
