#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port fis)
ENDPOINT="http://localhost:${PORT}"

aws_fis() {
  aws fis "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for FIS"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  FIS server started")

report_results "FIS"
exit $?
