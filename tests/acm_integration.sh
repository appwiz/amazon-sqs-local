#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port acm)
ENDPOINT="http://localhost:${PORT}"

aws_acm() {
  aws acm "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for ACM"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  ACM server started")

report_results "ACM"
exit $?
