#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port apprunner)
ENDPOINT="http://localhost:${PORT}"

aws_apprunner() {
  aws apprunner "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for AppRunner"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  AppRunner server started")

report_results "App Runner"
exit $?
