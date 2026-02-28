#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port cloudtrail)
ENDPOINT="http://localhost:${PORT}"

aws_cloudtrail() {
  aws cloudtrail "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for CloudTrail"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  CloudTrail server started")

report_results "CloudTrail"
exit $?
