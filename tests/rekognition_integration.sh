#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port rekognition)
ENDPOINT="http://localhost:${PORT}"

aws_rekognition() {
  aws rekognition "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for Rekognition"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  Rekognition server started")

report_results "REKOGNITION"
exit $?
