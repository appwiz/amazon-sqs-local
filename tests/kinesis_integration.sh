#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port kinesis)
ENDPOINT="http://localhost:${PORT}"

aws_kinesis() {
  aws kinesis "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

# 1. CreateStream
OUT=$(aws_kinesis create-stream --stream-name mystream --shard-count 1)
assert_contains "CreateStream" "$OUT" ""

# 2. DescribeStream
OUT=$(aws_kinesis describe-stream --stream-name mystream)
assert_contains "DescribeStream name" "$OUT" "mystream"
assert_contains "DescribeStream status" "$OUT" "ACTIVE"
assert_contains "DescribeStream shards" "$OUT" "Shards"

# 3. DescribeStreamSummary
OUT=$(aws_kinesis describe-stream-summary --stream-name mystream)
assert_contains "DescribeStreamSummary" "$OUT" "mystream"
assert_contains "DescribeStreamSummary status" "$OUT" "ACTIVE"

# 4. ListStreams
OUT=$(aws_kinesis list-streams)
assert_contains "ListStreams" "$OUT" "mystream"

# 5. ListShards
OUT=$(aws_kinesis list-shards --stream-name mystream)
assert_contains "ListShards" "$OUT" "Shards"
assert_contains "ListShards shard id" "$OUT" "shardId"

# 6. PutRecord
DATA_B64=$(echo -n "Hello Kinesis!" | base64)
OUT=$(aws_kinesis put-record \
  --stream-name mystream \
  --data "$DATA_B64" \
  --partition-key "pk1")
assert_contains "PutRecord ShardId" "$OUT" "ShardId"
assert_contains "PutRecord SequenceNumber" "$OUT" "SequenceNumber"

# 7. PutRecords
OUT=$(aws_kinesis put-records \
  --stream-name mystream \
  --records Data=$(echo -n "Record1" | base64),PartitionKey=pk1 Data=$(echo -n "Record2" | base64),PartitionKey=pk2)
assert_contains "PutRecords" "$OUT" "Records"
assert_contains "PutRecords failed count" "$OUT" "FailedRecordCount"

# 8. GetShardIterator - TRIM_HORIZON
OUT=$(aws_kinesis get-shard-iterator \
  --stream-name mystream \
  --shard-id shardId-000000000000 \
  --shard-iterator-type TRIM_HORIZON)
assert_contains "GetShardIterator" "$OUT" "ShardIterator"
ITERATOR=$(echo "$OUT" | python3 -c "import sys,json; print(json.load(sys.stdin)['ShardIterator'])" 2>/dev/null || echo "")

# 9. GetRecords
OUT=$(aws_kinesis get-records --shard-iterator "$ITERATOR")
assert_contains "GetRecords" "$OUT" "Records"
assert_contains "GetRecords millis" "$OUT" "MillisBehindLatest"
assert_contains "GetRecords next iterator" "$OUT" "NextShardIterator"

# 10. AddTagsToStream
OUT=$(aws_kinesis add-tags-to-stream --stream-name mystream --tags env=test,team=data)
assert_contains "AddTagsToStream" "$OUT" ""

# 11. ListTagsForStream
OUT=$(aws_kinesis list-tags-for-stream --stream-name mystream)
assert_contains "ListTagsForStream env" "$OUT" "env"
assert_contains "ListTagsForStream data" "$OUT" "data"

# 12. RemoveTagsFromStream
OUT=$(aws_kinesis remove-tags-from-stream --stream-name mystream --tag-keys env)
assert_contains "RemoveTagsFromStream" "$OUT" ""

# 13. Verify tag removed
OUT=$(aws_kinesis list-tags-for-stream --stream-name mystream)
assert_not_contains "RemoveTagsFromStream verify" "$OUT" '"env"'

# 14. IncreaseStreamRetentionPeriod
OUT=$(aws_kinesis increase-stream-retention-period \
  --stream-name mystream \
  --retention-period-hours 48)
assert_contains "IncreaseRetentionPeriod" "$OUT" ""

# 15. DecreaseStreamRetentionPeriod
OUT=$(aws_kinesis decrease-stream-retention-period \
  --stream-name mystream \
  --retention-period-hours 24)
assert_contains "DecreaseRetentionPeriod" "$OUT" ""

# 16. CreateStream duplicate
OUT=$(aws_kinesis create-stream --stream-name mystream --shard-count 1)
assert_contains "CreateStream duplicate" "$OUT" "ResourceInUseException"

# 17. DeleteStream
OUT=$(aws_kinesis delete-stream --stream-name mystream)
assert_contains "DeleteStream" "$OUT" ""

# 18. DescribeStream not found
OUT=$(aws_kinesis describe-stream --stream-name mystream)
assert_contains "DescribeStream not found" "$OUT" "ResourceNotFoundException"

# ── report ───────────────────────────────────────────────────────────────

report_results "Kinesis"
exit $?
