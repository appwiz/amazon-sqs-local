#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port iotfleetwise)
ENDPOINT="http://localhost:${PORT}"

aws_iotfleetwise() {
  aws iotfleetwise "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for IoTFleetWise"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  IoTFleetWise server started")

report_results "IOTFLEETWISE"
exit $?
