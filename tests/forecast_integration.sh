#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port forecast)
ENDPOINT="http://localhost:${PORT}"

aws_forecast() {
  aws forecast "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for Forecast"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  Forecast server started")

report_results "FORECAST"
exit $?
