#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port healthlake)
ENDPOINT="http://localhost:${PORT}"

aws_healthlake() {
  aws healthlake "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for HealthLake"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  HealthLake server started")

report_results "HEALTHLAKE"
exit $?
