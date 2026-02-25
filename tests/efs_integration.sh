#!/usr/bin/env bash
#
# Integration tests for Amazon EFS service within aws-inmemory-services.
#
set -uo pipefail

PORT=19500
ENDPOINT="http://localhost:${PORT}"
ACCOUNT="000000000000"
REGION="us-east-1"
BINARY="./target/debug/aws-inmemory-services"

PASS=0
FAIL=0
TESTS=()

aws_efs() {
  aws efs "$@" \
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

cleanup() {
  if [[ -n "${SERVER_PID:-}" ]]; then
    kill "$SERVER_PID" 2>/dev/null || true
    wait "$SERVER_PID" 2>/dev/null || true
  fi
}
trap cleanup EXIT

echo "Building..."
cargo build --quiet 2>&1

lsof -ti:${PORT} | xargs kill 2>/dev/null || true
sleep 0.5

echo "Starting server with EFS on port ${PORT}..."
"$BINARY" \
  --efs-port "$PORT" \
  --s3-port 19501 --sns-port 19502 --sqs-port 19503 --dynamodb-port 19504 \
  --lambda-port 19505 --firehose-port 19506 --memorydb-port 19507 \
  --cognito-port 19508 --apigateway-port 19509 --kms-port 19510 \
  --secretsmanager-port 19511 --kinesis-port 19512 --eventbridge-port 19513 \
  --stepfunctions-port 19514 --ssm-port 19515 --cloudwatchlogs-port 19516 \
  --ses-port 19517 --servicecatalog-port 19518 --config-port 19519 \
  --region "$REGION" --account-id "$ACCOUNT" &
SERVER_PID=$!
sleep 1

if ! kill -0 "$SERVER_PID" 2>/dev/null; then
  echo "ERROR: server failed to start"
  exit 1
fi

echo "Running EFS integration tests..."

# 1. CreateFileSystem
OUT=$(aws_efs create-file-system --creation-token my-fs-1 --tags Key=Name,Value=TestFS)
assert_contains "CreateFileSystem" "$OUT" "FileSystemId"
assert_contains "CreateFileSystem has ARN" "$OUT" "arn:aws:elasticfilesystem"
assert_contains "CreateFileSystem has token" "$OUT" "my-fs-1"
assert_contains "CreateFileSystem available" "$OUT" "available"
FS_ID=$(echo "$OUT" | python3 -c "import sys,json; print(json.load(sys.stdin)['FileSystemId'])")

# 2. CreateFileSystem idempotency (same creation token returns same FS)
OUT=$(aws_efs create-file-system --creation-token my-fs-1)
assert_contains "CreateFileSystem idempotent" "$OUT" "$FS_ID"

# 3. DescribeFileSystems (list all)
OUT=$(aws_efs describe-file-systems)
assert_contains "DescribeFileSystems" "$OUT" "FileSystems"
assert_contains "DescribeFileSystems has FS" "$OUT" "$FS_ID"

# 4. DescribeFileSystems by ID
OUT=$(aws_efs describe-file-systems --file-system-id "$FS_ID")
assert_contains "DescribeFileSystems by ID" "$OUT" "$FS_ID"

# 5. UpdateFileSystem
OUT=$(aws_efs update-file-system --file-system-id "$FS_ID" --throughput-mode provisioned --provisioned-throughput-in-mibps 128.0)
assert_contains "UpdateFileSystem" "$OUT" "provisioned"
assert_contains "UpdateFileSystem throughput" "$OUT" "128"

# 6. CreateMountTarget
OUT=$(aws_efs create-mount-target --file-system-id "$FS_ID" --subnet-id subnet-12345678)
assert_contains "CreateMountTarget" "$OUT" "MountTargetId"
assert_contains "CreateMountTarget FS" "$OUT" "$FS_ID"
assert_contains "CreateMountTarget available" "$OUT" "available"
MT_ID=$(echo "$OUT" | python3 -c "import sys,json; print(json.load(sys.stdin)['MountTargetId'])")

# 7. DescribeMountTargets by file system
OUT=$(aws_efs describe-mount-targets --file-system-id "$FS_ID")
assert_contains "DescribeMountTargets" "$OUT" "MountTargets"
assert_contains "DescribeMountTargets has MT" "$OUT" "$MT_ID"

# 8. Cannot delete file system with mount targets
OUT=$(aws_efs delete-file-system --file-system-id "$FS_ID")
assert_contains "DeleteFileSystem in use" "$OUT" "FileSystemInUse"

# 9. CreateAccessPoint
OUT=$(aws_efs create-access-point \
  --client-token ap-token-1 \
  --file-system-id "$FS_ID" \
  --posix-user Uid=1000,Gid=1000 \
  --root-directory "Path=/export/data,CreationInfo={OwnerUid=1000,OwnerGid=1000,Permissions=755}" \
  --tags Key=Name,Value=TestAP)
assert_contains "CreateAccessPoint" "$OUT" "AccessPointId"
assert_contains "CreateAccessPoint FS" "$OUT" "$FS_ID"
assert_contains "CreateAccessPoint available" "$OUT" "available"
AP_ID=$(echo "$OUT" | python3 -c "import sys,json; print(json.load(sys.stdin)['AccessPointId'])")

# 10. CreateAccessPoint idempotency
OUT=$(aws_efs create-access-point --client-token ap-token-1 --file-system-id "$FS_ID")
assert_contains "CreateAccessPoint idempotent" "$OUT" "$AP_ID"

# 11. DescribeAccessPoints
OUT=$(aws_efs describe-access-points --file-system-id "$FS_ID")
assert_contains "DescribeAccessPoints" "$OUT" "AccessPoints"
assert_contains "DescribeAccessPoints has AP" "$OUT" "$AP_ID"

# 12. TagResource (add tags to file system)
OUT=$(aws_efs tag-resource --resource-id "$FS_ID" --tags Key=Env,Value=Test)
assert_not_contains "TagResource" "$OUT" "Error"

# 13. ListTagsForResource
OUT=$(aws_efs list-tags-for-resource --resource-id "$FS_ID")
assert_contains "ListTagsForResource" "$OUT" "Env"
assert_contains "ListTagsForResource value" "$OUT" "Test"

# 14. UntagResource
OUT=$(aws_efs untag-resource --resource-id "$FS_ID" --tag-keys Env)
assert_not_contains "UntagResource" "$OUT" "Error"

# 15. ListTagsForResource after untag
OUT=$(aws_efs list-tags-for-resource --resource-id "$FS_ID")
assert_not_contains "ListTagsForResource after untag" "$OUT" "Env"

# 16. PutLifecycleConfiguration
OUT=$(aws_efs put-lifecycle-configuration \
  --file-system-id "$FS_ID" \
  --lifecycle-policies TransitionToIA=AFTER_30_DAYS)
assert_contains "PutLifecycleConfiguration" "$OUT" "AFTER_30_DAYS"

# 17. DescribeLifecycleConfiguration
OUT=$(aws_efs describe-lifecycle-configuration --file-system-id "$FS_ID")
assert_contains "DescribeLifecycleConfiguration" "$OUT" "AFTER_30_DAYS"

# 18. DeleteAccessPoint
OUT=$(aws_efs delete-access-point --access-point-id "$AP_ID")
assert_not_contains "DeleteAccessPoint" "$OUT" "Error"

# 19. DescribeAccessPoints after delete
OUT=$(aws_efs describe-access-points --file-system-id "$FS_ID")
assert_not_contains "DescribeAccessPoints after delete" "$OUT" "$AP_ID"

# 20. DeleteMountTarget
OUT=$(aws_efs delete-mount-target --mount-target-id "$MT_ID")
assert_not_contains "DeleteMountTarget" "$OUT" "Error"

# 21. DeleteFileSystem (now possible without mount targets)
OUT=$(aws_efs delete-file-system --file-system-id "$FS_ID")
assert_not_contains "DeleteFileSystem" "$OUT" "Error"

# 22. DescribeFileSystems after delete
OUT=$(aws_efs describe-file-systems)
assert_not_contains "DescribeFileSystems after delete" "$OUT" "$FS_ID"

# 23. DescribeFileSystems not found
OUT=$(aws_efs describe-file-systems --file-system-id fs-nonexistent12345)
assert_contains "DescribeFileSystems not found" "$OUT" "FileSystemNotFound"

# 24. DeleteAccessPoint not found
OUT=$(aws_efs delete-access-point --access-point-id fsap-nonexistent1234)
assert_contains "DeleteAccessPoint not found" "$OUT" "AccessPointNotFound"

# ── report ───────────────────────────────────────────────────────────────

echo ""
echo "══════════════════════════════════════════════"
echo "  EFS Integration Test Results"
echo "══════════════════════════════════════════════"
for t in "${TESTS[@]}"; do echo "  $t"; done
echo "──────────────────────────────────────────────"
echo "  Passed: $PASS   Failed: $FAIL"
echo "══════════════════════════════════════════════"

exit "$FAIL"
