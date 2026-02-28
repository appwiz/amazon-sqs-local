#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port securityhub)
ENDPOINT="http://localhost:${PORT}"

aws_securityhub() {
  aws securityhub "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for SecurityHub"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  SecurityHub server started")

report_results "SECURITYHUB"
exit $?
