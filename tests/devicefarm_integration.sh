#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port devicefarm)
ENDPOINT="http://localhost:${PORT}"

aws_devicefarm() {
  aws devicefarm "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for DeviceFarm"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  DeviceFarm server started")

report_results "DEVICEFARM"
exit $?
