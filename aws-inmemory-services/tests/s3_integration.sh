#!/usr/bin/env bash
#
# Integration tests for in-memory S3 service using the system awscli.
# Exercises all S3 API operations.
#
set -uo pipefail

PORT=19000
ENDPOINT="http://localhost:${PORT}"
REGION="us-east-1"
BINARY="./target/debug/aws-inmemory-services"

PASS=0
FAIL=0
TESTS=()

# ── helpers ──────────────────────────────────────────────────────────────

aws_s3api() {
  aws s3api "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

assert_contains() {
  local label="$1" output="$2" expected="$3"
  if echo "$output" | grep -qF "$expected"; then
    PASS=$((PASS + 1))
    TESTS+=("PASS  $label")
  else
    FAIL=$((FAIL + 1))
    TESTS+=("FAIL  $label  (expected '$expected' in output)")
    echo "FAIL: $label" >&2
    echo "  expected: $expected" >&2
    echo "  output:   $output" >&2
  fi
}

assert_not_contains() {
  local label="$1" output="$2" unexpected="$3"
  if echo "$output" | grep -qF "$unexpected"; then
    FAIL=$((FAIL + 1))
    TESTS+=("FAIL  $label  (did not expect '$unexpected' in output)")
    echo "FAIL: $label" >&2
    echo "  unexpected: $unexpected" >&2
    echo "  output:     $output" >&2
  else
    PASS=$((PASS + 1))
    TESTS+=("PASS  $label")
  fi
}

assert_exit_zero() {
  local label="$1"
  shift
  if "$@" > /dev/null 2>&1; then
    PASS=$((PASS + 1))
    TESTS+=("PASS  $label")
  else
    FAIL=$((FAIL + 1))
    TESTS+=("FAIL  $label  (non-zero exit)")
    echo "FAIL: $label" >&2
  fi
}

json_field() {
  python3 -c "import sys,json; print(json.load(sys.stdin)$1)" 2>/dev/null
}

cleanup() {
  if [[ -n "${SERVER_PID:-}" ]]; then
    kill "$SERVER_PID" 2>/dev/null || true
    wait "$SERVER_PID" 2>/dev/null || true
  fi
  rm -f /tmp/s3-test-upload.txt /tmp/s3-test-download.txt /tmp/s3-test-copy.txt
  rm -f /tmp/s3-test-part1.bin /tmp/s3-test-part2.bin /tmp/s3-test-multipart.txt
}
trap cleanup EXIT

# ── build & start server ─────────────────────────────────────────────────

echo "Building..."
cargo build --quiet 2>&1

echo "Starting server on port ${PORT}..."
"$BINARY" --s3-port "$PORT" --sns-port 19911 --region "$REGION" &
SERVER_PID=$!
sleep 1

if ! kill -0 "$SERVER_PID" 2>/dev/null; then
  echo "FATAL: server failed to start" >&2
  exit 1
fi

echo "Server running (pid $SERVER_PID). Running tests..."
echo

# ═════════════════════════════════════════════════════════════════════════
# 1. CreateBucket
# ═════════════════════════════════════════════════════════════════════════

assert_exit_zero "CreateBucket: test-bucket" \
  aws s3api create-bucket \
    --endpoint-url "$ENDPOINT" --region "$REGION" --no-sign-request --no-cli-pager \
    --bucket test-bucket

assert_exit_zero "CreateBucket: another-bucket" \
  aws s3api create-bucket \
    --endpoint-url "$ENDPOINT" --region "$REGION" --no-sign-request --no-cli-pager \
    --bucket another-bucket

# ═════════════════════════════════════════════════════════════════════════
# 2. ListBuckets
# ═════════════════════════════════════════════════════════════════════════

OUT=$(aws_s3api list-buckets)
assert_contains "ListBuckets: test-bucket" "$OUT" "test-bucket"
assert_contains "ListBuckets: another-bucket" "$OUT" "another-bucket"

# ═════════════════════════════════════════════════════════════════════════
# 3. HeadBucket
# ═════════════════════════════════════════════════════════════════════════

assert_exit_zero "HeadBucket: exists" \
  aws s3api head-bucket \
    --endpoint-url "$ENDPOINT" --region "$REGION" --no-sign-request --no-cli-pager \
    --bucket test-bucket

# ═════════════════════════════════════════════════════════════════════════
# 4. PutObject
# ═════════════════════════════════════════════════════════════════════════

echo "Hello, S3!" > /tmp/s3-test-upload.txt

OUT=$(aws_s3api put-object --bucket test-bucket --key hello.txt --body /tmp/s3-test-upload.txt)
assert_contains "PutObject: returns ETag" "$OUT" "ETag"

OUT=$(aws_s3api put-object --bucket test-bucket --key folder/nested.txt --body /tmp/s3-test-upload.txt \
  --content-type "text/plain" --metadata '{"custom-key":"custom-value"}')
assert_contains "PutObject nested: returns ETag" "$OUT" "ETag"

# ═════════════════════════════════════════════════════════════════════════
# 5. GetObject
# ═════════════════════════════════════════════════════════════════════════

aws_s3api get-object --bucket test-bucket --key hello.txt /tmp/s3-test-download.txt > /dev/null 2>&1
CONTENT=$(cat /tmp/s3-test-download.txt)
assert_contains "GetObject: correct content" "$CONTENT" "Hello, S3!"

# ═════════════════════════════════════════════════════════════════════════
# 6. HeadObject
# ═════════════════════════════════════════════════════════════════════════

OUT=$(aws_s3api head-object --bucket test-bucket --key hello.txt)
assert_contains "HeadObject: has ContentLength" "$OUT" "ContentLength"
assert_contains "HeadObject: has ETag" "$OUT" "ETag"
assert_contains "HeadObject: has ContentType" "$OUT" "ContentType"

# ═════════════════════════════════════════════════════════════════════════
# 7. CopyObject
# ═════════════════════════════════════════════════════════════════════════

OUT=$(aws_s3api copy-object --bucket test-bucket --key hello-copy.txt \
  --copy-source test-bucket/hello.txt)
assert_contains "CopyObject: returns ETag" "$OUT" "ETag"

# Verify copy content
aws_s3api get-object --bucket test-bucket --key hello-copy.txt /tmp/s3-test-copy.txt > /dev/null 2>&1
COPY_CONTENT=$(cat /tmp/s3-test-copy.txt)
assert_contains "CopyObject: content matches" "$COPY_CONTENT" "Hello, S3!"

# ═════════════════════════════════════════════════════════════════════════
# 8. ListObjectsV2
# ═════════════════════════════════════════════════════════════════════════

OUT=$(aws_s3api list-objects-v2 --bucket test-bucket)
assert_contains "ListObjectsV2: has hello.txt" "$OUT" "hello.txt"
assert_contains "ListObjectsV2: has hello-copy.txt" "$OUT" "hello-copy.txt"
assert_contains "ListObjectsV2: has nested.txt" "$OUT" "folder/nested.txt"

# With prefix
OUT=$(aws_s3api list-objects-v2 --bucket test-bucket --prefix "folder/")
assert_contains "ListObjectsV2 prefix: has nested.txt" "$OUT" "folder/nested.txt"
assert_not_contains "ListObjectsV2 prefix: no hello.txt" "$OUT" '"Key": "hello.txt"'

# With delimiter
OUT=$(aws_s3api list-objects-v2 --bucket test-bucket --delimiter "/")
assert_contains "ListObjectsV2 delimiter: has CommonPrefixes" "$OUT" "folder/"

# ═════════════════════════════════════════════════════════════════════════
# 9. DeleteObject
# ═════════════════════════════════════════════════════════════════════════

assert_exit_zero "DeleteObject: hello-copy.txt" \
  aws s3api delete-object \
    --endpoint-url "$ENDPOINT" --region "$REGION" --no-sign-request --no-cli-pager \
    --bucket test-bucket --key hello-copy.txt

OUT=$(aws_s3api list-objects-v2 --bucket test-bucket)
assert_not_contains "DeleteObject: removed from listing" "$OUT" "hello-copy.txt"

# ═════════════════════════════════════════════════════════════════════════
# 10. DeleteObjects (batch)
# ═════════════════════════════════════════════════════════════════════════

# Add a few objects first
echo "batch1" > /tmp/s3-test-upload.txt
aws_s3api put-object --bucket test-bucket --key batch1.txt --body /tmp/s3-test-upload.txt > /dev/null
echo "batch2" > /tmp/s3-test-upload.txt
aws_s3api put-object --bucket test-bucket --key batch2.txt --body /tmp/s3-test-upload.txt > /dev/null

OUT=$(aws_s3api delete-objects --bucket test-bucket \
  --delete '{"Objects":[{"Key":"batch1.txt"},{"Key":"batch2.txt"}]}')
assert_contains "DeleteObjects: has Deleted" "$OUT" "Deleted"
assert_contains "DeleteObjects: batch1 deleted" "$OUT" "batch1.txt"
assert_contains "DeleteObjects: batch2 deleted" "$OUT" "batch2.txt"

# ═════════════════════════════════════════════════════════════════════════
# 11. Bucket Tagging
# ═════════════════════════════════════════════════════════════════════════

assert_exit_zero "PutBucketTagging: succeeds" \
  aws s3api put-bucket-tagging \
    --endpoint-url "$ENDPOINT" --region "$REGION" --no-sign-request --no-cli-pager \
    --bucket test-bucket \
    --tagging '{"TagSet":[{"Key":"Env","Value":"test"},{"Key":"Team","Value":"backend"}]}'

OUT=$(aws_s3api get-bucket-tagging --bucket test-bucket)
assert_contains "GetBucketTagging: Env tag" "$OUT" '"Env"'
assert_contains "GetBucketTagging: test value" "$OUT" '"test"'
assert_contains "GetBucketTagging: Team tag" "$OUT" '"Team"'

assert_exit_zero "DeleteBucketTagging: succeeds" \
  aws s3api delete-bucket-tagging \
    --endpoint-url "$ENDPOINT" --region "$REGION" --no-sign-request --no-cli-pager \
    --bucket test-bucket

# ═════════════════════════════════════════════════════════════════════════
# 12. Object Tagging
# ═════════════════════════════════════════════════════════════════════════

assert_exit_zero "PutObjectTagging: succeeds" \
  aws s3api put-object-tagging \
    --endpoint-url "$ENDPOINT" --region "$REGION" --no-sign-request --no-cli-pager \
    --bucket test-bucket --key hello.txt \
    --tagging '{"TagSet":[{"Key":"Status","Value":"active"}]}'

OUT=$(aws_s3api get-object-tagging --bucket test-bucket --key hello.txt)
assert_contains "GetObjectTagging: Status tag" "$OUT" '"Status"'
assert_contains "GetObjectTagging: active value" "$OUT" '"active"'

assert_exit_zero "DeleteObjectTagging: succeeds" \
  aws s3api delete-object-tagging \
    --endpoint-url "$ENDPOINT" --region "$REGION" --no-sign-request --no-cli-pager \
    --bucket test-bucket --key hello.txt

# ═════════════════════════════════════════════════════════════════════════
# 13. Bucket Versioning
# ═════════════════════════════════════════════════════════════════════════

OUT=$(aws_s3api get-bucket-versioning --bucket test-bucket)
# Initially empty/disabled - no Status field
assert_not_contains "GetBucketVersioning: initially disabled" "$OUT" '"Status"'

assert_exit_zero "PutBucketVersioning: enable" \
  aws s3api put-bucket-versioning \
    --endpoint-url "$ENDPOINT" --region "$REGION" --no-sign-request --no-cli-pager \
    --bucket test-bucket \
    --versioning-configuration Status=Enabled

OUT=$(aws_s3api get-bucket-versioning --bucket test-bucket)
assert_contains "GetBucketVersioning: Enabled" "$OUT" '"Enabled"'

# ═════════════════════════════════════════════════════════════════════════
# 14. GetBucketLocation
# ═════════════════════════════════════════════════════════════════════════

OUT=$(aws_s3api get-bucket-location --bucket test-bucket)
assert_contains "GetBucketLocation: has region" "$OUT" "LocationConstraint"

# ═════════════════════════════════════════════════════════════════════════
# 15. Multipart Upload
# ═════════════════════════════════════════════════════════════════════════

# Create multipart upload
OUT=$(aws_s3api create-multipart-upload --bucket test-bucket --key multipart.txt)
assert_contains "CreateMultipartUpload: has UploadId" "$OUT" "UploadId"
UPLOAD_ID=$(echo "$OUT" | json_field '["UploadId"]')

# Upload parts
dd if=/dev/urandom of=/tmp/s3-test-part1.bin bs=1024 count=5120 2>/dev/null
dd if=/dev/urandom of=/tmp/s3-test-part2.bin bs=1024 count=5120 2>/dev/null

OUT1=$(aws_s3api upload-part --bucket test-bucket --key multipart.txt \
  --upload-id "$UPLOAD_ID" --part-number 1 --body /tmp/s3-test-part1.bin)
assert_contains "UploadPart 1: has ETag" "$OUT1" "ETag"
ETAG1=$(echo "$OUT1" | json_field '["ETag"]')

OUT2=$(aws_s3api upload-part --bucket test-bucket --key multipart.txt \
  --upload-id "$UPLOAD_ID" --part-number 2 --body /tmp/s3-test-part2.bin)
assert_contains "UploadPart 2: has ETag" "$OUT2" "ETag"
ETAG2=$(echo "$OUT2" | json_field '["ETag"]')

# List parts
OUT=$(aws_s3api list-parts --bucket test-bucket --key multipart.txt --upload-id "$UPLOAD_ID")
assert_contains "ListParts: has Part" "$OUT" "PartNumber"

# Complete multipart upload
OUT=$(aws_s3api complete-multipart-upload --bucket test-bucket --key multipart.txt \
  --upload-id "$UPLOAD_ID" \
  --multipart-upload "{\"Parts\":[{\"PartNumber\":1,\"ETag\":$ETAG1},{\"PartNumber\":2,\"ETag\":$ETAG2}]}")
assert_contains "CompleteMultipartUpload: has ETag" "$OUT" "ETag"
assert_contains "CompleteMultipartUpload: has Key" "$OUT" "multipart.txt"

# Verify the multipart object exists
OUT=$(aws_s3api head-object --bucket test-bucket --key multipart.txt)
assert_contains "Multipart object: has ContentLength" "$OUT" "ContentLength"

# ═════════════════════════════════════════════════════════════════════════
# 16. ListMultipartUploads
# ═════════════════════════════════════════════════════════════════════════

# Create and abort a multipart upload
OUT=$(aws_s3api create-multipart-upload --bucket test-bucket --key abort-me.txt)
ABORT_UPLOAD_ID=$(echo "$OUT" | json_field '["UploadId"]')

OUT=$(aws_s3api list-multipart-uploads --bucket test-bucket)
assert_contains "ListMultipartUploads: has abort-me" "$OUT" "abort-me.txt"

assert_exit_zero "AbortMultipartUpload: succeeds" \
  aws s3api abort-multipart-upload \
    --endpoint-url "$ENDPOINT" --region "$REGION" --no-sign-request --no-cli-pager \
    --bucket test-bucket --key abort-me.txt --upload-id "$ABORT_UPLOAD_ID"

# ═════════════════════════════════════════════════════════════════════════
# 17. DeleteBucket
# ═════════════════════════════════════════════════════════════════════════

# Delete all objects first
aws_s3api delete-object --bucket test-bucket --key hello.txt > /dev/null 2>&1 || true
aws_s3api delete-object --bucket test-bucket --key folder/nested.txt > /dev/null 2>&1 || true
aws_s3api delete-object --bucket test-bucket --key multipart.txt > /dev/null 2>&1 || true

assert_exit_zero "DeleteBucket: test-bucket" \
  aws s3api delete-bucket \
    --endpoint-url "$ENDPOINT" --region "$REGION" --no-sign-request --no-cli-pager \
    --bucket test-bucket

assert_exit_zero "DeleteBucket: another-bucket" \
  aws s3api delete-bucket \
    --endpoint-url "$ENDPOINT" --region "$REGION" --no-sign-request --no-cli-pager \
    --bucket another-bucket

OUT=$(aws_s3api list-buckets)
assert_not_contains "DeleteBucket: all removed" "$OUT" "test-bucket"

# ═════════════════════════════════════════════════════════════════════════
# Summary
# ═════════════════════════════════════════════════════════════════════════

echo
echo "═══════════════════════════════════════════════════"
echo "  S3 Results: ${PASS} passed, ${FAIL} failed"
echo "═══════════════════════════════════════════════════"
echo
for t in "${TESTS[@]}"; do
  echo "  $t"
done
echo

if [ "$FAIL" -gt 0 ]; then
  exit 1
fi
