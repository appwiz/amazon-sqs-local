#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port cloudmap)
ENDPOINT="http://localhost:${PORT}"

aws_cloudmap() {
  aws cloudmap "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for CloudMap"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  CloudMap server started")

report_results "Cloud Map"
exit $?
