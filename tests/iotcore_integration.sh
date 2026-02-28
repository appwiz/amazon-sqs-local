#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port iotcore)
ENDPOINT="http://localhost:${PORT}"

aws_iotcore() {
  aws iotcore "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for IoTCore"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  IoTCore server started")

report_results "IOTCORE"
exit $?
