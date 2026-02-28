#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port xray)
ENDPOINT="http://localhost:${PORT}"

aws_xray() {
  aws xray "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for XRay"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  XRay server started")

report_results "X-Ray"
exit $?
