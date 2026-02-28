#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port kendra)
ENDPOINT="http://localhost:${PORT}"

aws_kendra() {
  aws kendra "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for Kendra"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  Kendra server started")

report_results "KENDRA"
exit $?
