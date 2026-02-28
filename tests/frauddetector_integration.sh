#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port frauddetector)
ENDPOINT="http://localhost:${PORT}"

aws_frauddetector() {
  aws frauddetector "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for FraudDetector"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  FraudDetector server started")

report_results "FRAUDDETECTOR"
exit $?
