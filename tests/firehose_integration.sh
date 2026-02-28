#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port firehose)
ENDPOINT="http://localhost:${PORT}"

aws_firehose() {
  aws firehose "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

# ── Tests ────────────────────────────────────────────────────────────────

echo "Running Firehose integration tests..."

# 1. CreateDeliveryStream
OUT=$(aws_firehose create-delivery-stream \
  --delivery-stream-name mystream)
assert_contains "CreateDeliveryStream" "$OUT" "DeliveryStreamARN"
assert_contains "CreateDeliveryStream arn" "$OUT" "mystream"

# 2. DescribeDeliveryStream
OUT=$(aws_firehose describe-delivery-stream \
  --delivery-stream-name mystream)
assert_contains "DescribeDeliveryStream name" "$OUT" "mystream"
assert_contains "DescribeDeliveryStream status" "$OUT" "ACTIVE"
assert_contains "DescribeDeliveryStream type" "$OUT" "DirectPut"

# 3. ListDeliveryStreams
OUT=$(aws_firehose list-delivery-streams)
assert_contains "ListDeliveryStreams" "$OUT" "mystream"

# 4. PutRecord
OUT=$(aws_firehose put-record \
  --delivery-stream-name mystream \
  --record '{"Data":"SGVsbG8gV29ybGQ="}')
assert_contains "PutRecord" "$OUT" "RecordId"

# 5. PutRecordBatch
OUT=$(aws_firehose put-record-batch \
  --delivery-stream-name mystream \
  --records '{"Data":"UmVjb3JkMQ=="}' '{"Data":"UmVjb3JkMg=="}')
assert_contains "PutRecordBatch count" "$OUT" "FailedPutCount"
assert_contains "PutRecordBatch responses" "$OUT" "RequestResponses"

# 6. TagDeliveryStream
OUT=$(aws_firehose tag-delivery-stream \
  --delivery-stream-name mystream \
  --tags Key=env,Value=test Key=team,Value=data)
assert_contains "TagDeliveryStream" "$OUT" ""

# 7. ListTagsForDeliveryStream
OUT=$(aws_firehose list-tags-for-delivery-stream \
  --delivery-stream-name mystream)
assert_contains "ListTagsForDeliveryStream env" "$OUT" "env"
assert_contains "ListTagsForDeliveryStream team" "$OUT" "data"

# 8. UntagDeliveryStream
OUT=$(aws_firehose untag-delivery-stream \
  --delivery-stream-name mystream \
  --tag-keys env)
assert_contains "UntagDeliveryStream" "$OUT" ""

# Verify tag removed
OUT=$(aws_firehose list-tags-for-delivery-stream \
  --delivery-stream-name mystream)
assert_not_contains "UntagDeliveryStream verify" "$OUT" "env"

# 9. CreateDeliveryStream duplicate error
OUT=$(aws_firehose create-delivery-stream \
  --delivery-stream-name mystream)
assert_contains "CreateDeliveryStream duplicate" "$OUT" "ResourceInUseException"

# 10. DescribeDeliveryStream not found
OUT=$(aws_firehose describe-delivery-stream \
  --delivery-stream-name nonexistent)
assert_contains "DescribeDeliveryStream not found" "$OUT" "ResourceNotFoundException"

# 11. DeleteDeliveryStream
OUT=$(aws_firehose delete-delivery-stream \
  --delivery-stream-name mystream)
assert_contains "DeleteDeliveryStream" "$OUT" ""

# 12. DeleteDeliveryStream not found
OUT=$(aws_firehose delete-delivery-stream \
  --delivery-stream-name mystream)
assert_contains "DeleteDeliveryStream not found" "$OUT" "ResourceNotFoundException"

# ── report ───────────────────────────────────────────────────────────────

report_results "Firehose"
exit $?
