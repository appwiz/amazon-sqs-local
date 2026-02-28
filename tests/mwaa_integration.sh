#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port mwaa)
ENDPOINT="http://localhost:${PORT}"

aws_mwaa() {
  aws mwaa "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for MWAA"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  MWAA server started")

report_results "MWAA"
exit $?
