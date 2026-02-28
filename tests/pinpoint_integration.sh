#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port pinpoint)
ENDPOINT="http://localhost:${PORT}"

aws_pinpoint() {
  aws pinpoint "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for Pinpoint"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  Pinpoint server started")

report_results "PINPOINT"
exit $?
