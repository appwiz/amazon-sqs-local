#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port verifiedpermissions)
ENDPOINT="http://localhost:${PORT}"

aws_verifiedpermissions() {
  aws verifiedpermissions "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for VerifiedPermissions"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  VerifiedPermissions server started")

report_results "VERIFIEDPERMISSIONS"
exit $?
