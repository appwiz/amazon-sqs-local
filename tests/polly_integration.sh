#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port polly)
ENDPOINT="http://localhost:${PORT}"

aws_polly() {
  aws polly "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for Polly"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  Polly server started")

report_results "POLLY"
exit $?
