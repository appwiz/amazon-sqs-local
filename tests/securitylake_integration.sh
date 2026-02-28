#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port securitylake)
ENDPOINT="http://localhost:${PORT}"

aws_securitylake() {
  aws securitylake "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for SecurityLake"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  SecurityLake server started")

report_results "SECURITYLAKE"
exit $?
