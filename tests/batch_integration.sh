#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port batch)
ENDPOINT="http://localhost:${PORT}"

aws_batch() {
  aws batch "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for Batch"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  Batch server started")

report_results "BATCH"
exit $?
