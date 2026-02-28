#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port iotevents)
ENDPOINT="http://localhost:${PORT}"

aws_iotevents() {
  aws iotevents "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for IoTEvents"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  IoTEvents server started")

report_results "IOTEVENTS"
exit $?
