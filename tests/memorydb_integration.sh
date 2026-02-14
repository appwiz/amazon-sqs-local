#!/usr/bin/env bash
#
# Integration tests for MemoryDB service within aws-inmemory-services.
#
set -uo pipefail

PORT=16379
ENDPOINT="http://localhost:${PORT}"
ACCOUNT="000000000000"
REGION="us-east-1"
BINARY="./target/debug/aws-inmemory-services"

PASS=0
FAIL=0
TESTS=()

# ── helpers ──────────────────────────────────────────────────────────────

aws_mdb() {
  aws memorydb "$@" \
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

# ── build & start server ─────────────────────────────────────────────────

echo "Building..."
cargo build --quiet 2>&1

lsof -ti:${PORT} | xargs kill 2>/dev/null || true
sleep 0.5

echo "Starting server with MemoryDB on port ${PORT}..."
"$BINARY" --memorydb-port "$PORT" --s3-port 16001 --sns-port 16002 --sqs-port 16003 --dynamodb-port 16004 --lambda-port 16005 --firehose-port 16006 --region "$REGION" --account-id "$ACCOUNT" &
SERVER_PID=$!
sleep 1

if ! kill -0 "$SERVER_PID" 2>/dev/null; then
  echo "ERROR: server failed to start"
  exit 1
fi

# ── Tests ────────────────────────────────────────────────────────────────

echo "Running MemoryDB integration tests..."

# 1. CreateUser
OUT=$(aws_mdb create-user \
  --user-name myuser \
  --access-string "on ~* +@all" \
  --authentication-mode Type=no-password)
assert_contains "CreateUser" "$OUT" "myuser"
assert_contains "CreateUser status" "$OUT" "active"

# 2. DescribeUsers
OUT=$(aws_mdb describe-users)
assert_contains "DescribeUsers" "$OUT" "myuser"

# 3. UpdateUser
OUT=$(aws_mdb update-user \
  --user-name myuser \
  --access-string "on ~app:* +@read")
assert_contains "UpdateUser" "$OUT" "~app:"

# 4. CreateACL
OUT=$(aws_mdb create-acl \
  --acl-name myacl \
  --user-names myuser)
assert_contains "CreateACL" "$OUT" "myacl"
assert_contains "CreateACL user" "$OUT" "myuser"

# 5. DescribeACLs
OUT=$(aws_mdb describe-acls)
assert_contains "DescribeACLs" "$OUT" "myacl"

# 6. UpdateACL - add user
OUT=$(aws_mdb create-user \
  --user-name seconduser \
  --access-string "on ~* +@all" \
  --authentication-mode Type=no-password)
OUT=$(aws_mdb update-acl \
  --acl-name myacl \
  --user-names-to-add seconduser)
assert_contains "UpdateACL add user" "$OUT" "seconduser"

# 7. CreateSubnetGroup
OUT=$(aws_mdb create-subnet-group \
  --subnet-group-name mysubnet \
  --subnet-ids subnet-12345 subnet-67890)
assert_contains "CreateSubnetGroup" "$OUT" "mysubnet"

# 8. DescribeSubnetGroups
OUT=$(aws_mdb describe-subnet-groups)
assert_contains "DescribeSubnetGroups" "$OUT" "mysubnet"

# 9. CreateCluster
OUT=$(aws_mdb create-cluster \
  --cluster-name mycluster \
  --node-type db.t4g.small \
  --acl-name myacl)
assert_contains "CreateCluster" "$OUT" "mycluster"
assert_contains "CreateCluster status" "$OUT" "available"
assert_contains "CreateCluster endpoint" "$OUT" "6379"

# 10. DescribeClusters
OUT=$(aws_mdb describe-clusters)
assert_contains "DescribeClusters" "$OUT" "mycluster"

# 11. UpdateCluster
OUT=$(aws_mdb update-cluster \
  --cluster-name mycluster \
  --description "Updated cluster")
assert_contains "UpdateCluster" "$OUT" "Updated cluster"

# 12. CreateSnapshot
OUT=$(aws_mdb create-snapshot \
  --cluster-name mycluster \
  --snapshot-name mysnap)
assert_contains "CreateSnapshot" "$OUT" "mysnap"
assert_contains "CreateSnapshot status" "$OUT" "available"

# 13. DescribeSnapshots
OUT=$(aws_mdb describe-snapshots)
assert_contains "DescribeSnapshots" "$OUT" "mysnap"

# 14. TagResource
CLUSTER_ARN="arn:aws:memorydb:${REGION}:${ACCOUNT}:cluster/mycluster"
OUT=$(aws_mdb tag-resource \
  --resource-arn "$CLUSTER_ARN" \
  --tags Key=env,Value=test)
assert_contains "TagResource" "$OUT" "env"

# 15. ListTags
OUT=$(aws_mdb list-tags \
  --resource-arn "$CLUSTER_ARN")
assert_contains "ListTags" "$OUT" "env"
assert_contains "ListTags value" "$OUT" "test"

# 16. UntagResource
OUT=$(aws_mdb untag-resource \
  --resource-arn "$CLUSTER_ARN" \
  --tag-keys env)
assert_contains "UntagResource" "$OUT" "TagList"

# 17. DeleteSnapshot
OUT=$(aws_mdb delete-snapshot \
  --snapshot-name mysnap)
assert_contains "DeleteSnapshot" "$OUT" "deleting"

# 18. DeleteCluster
OUT=$(aws_mdb delete-cluster \
  --cluster-name mycluster)
assert_contains "DeleteCluster" "$OUT" "deleting"

# 19. DeleteSubnetGroup
OUT=$(aws_mdb delete-subnet-group \
  --subnet-group-name mysubnet)
assert_contains "DeleteSubnetGroup" "$OUT" "mysubnet"

# 20. DeleteACL
OUT=$(aws_mdb delete-acl \
  --acl-name myacl)
assert_contains "DeleteACL" "$OUT" "deleting"

# 21. DeleteUser
OUT=$(aws_mdb delete-user \
  --user-name myuser)
assert_contains "DeleteUser" "$OUT" "deleting"

aws_mdb delete-user --user-name seconduser > /dev/null 2>&1

# 22. Error: cluster not found
OUT=$(aws_mdb describe-clusters \
  --cluster-name nonexistent)
assert_contains "DescribeClusters not found" "$OUT" "ClusterNotFoundFault"

# 23. Error: duplicate user
aws_mdb create-user \
  --user-name dupuser \
  --access-string "on ~* +@all" \
  --authentication-mode Type=no-password > /dev/null 2>&1
OUT=$(aws_mdb create-user \
  --user-name dupuser \
  --access-string "on ~* +@all" \
  --authentication-mode Type=no-password)
assert_contains "CreateUser duplicate" "$OUT" "UserAlreadyExistsFault"
aws_mdb delete-user --user-name dupuser > /dev/null 2>&1

# ── report ───────────────────────────────────────────────────────────────

echo ""
echo "══════════════════════════════════════════════"
echo "  MemoryDB Integration Test Results"
echo "══════════════════════════════════════════════"
for t in "${TESTS[@]}"; do echo "  $t"; done
echo "──────────────────────────────────────────────"
echo "  Passed: $PASS   Failed: $FAIL"
echo "══════════════════════════════════════════════"

exit "$FAIL"
