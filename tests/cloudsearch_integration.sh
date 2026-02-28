#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port cloudsearch)
ENDPOINT="http://localhost:${PORT}"

aws_cloudsearch() {
  aws cloudsearch "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for CloudSearch"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  CloudSearch server started")

report_results "CloudSearch"
exit $?
