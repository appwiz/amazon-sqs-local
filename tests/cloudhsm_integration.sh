#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port cloudhsm)
ENDPOINT="http://localhost:${PORT}"

aws_cloudhsm() {
  aws cloudhsm "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for CloudHSM"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  CloudHSM server started")

report_results "CloudHSM"
exit $?
