#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port elasticache)
ENDPOINT="http://localhost:${PORT}"

aws_elasticache() {
  aws elasticache "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for ElastiCache"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  ElastiCache server started")

report_results "ELASTICACHE"
exit $?
