#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port globalaccelerator)
ENDPOINT="http://localhost:${PORT}"

aws_globalaccelerator() {
  aws globalaccelerator "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for GlobalAccelerator"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  GlobalAccelerator server started")

report_results "GLOBALACCELERATOR"
exit $?
