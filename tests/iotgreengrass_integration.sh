#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port iotgreengrass)
ENDPOINT="http://localhost:${PORT}"

aws_iotgreengrass() {
  aws iotgreengrass "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for IoTGreengrass"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  IoTGreengrass server started")

report_results "IOTGREENGRASS"
exit $?
