#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port appflow)
ENDPOINT="http://localhost:${PORT}"

aws_appflow() {
  aws appflow "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for AppFlow"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  AppFlow server started")

report_results "AppFlow"
exit $?
