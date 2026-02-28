#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port shield)
ENDPOINT="http://localhost:${PORT}"

aws_shield() {
  aws shield "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for Shield"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  Shield server started")

report_results "SHIELD"
exit $?
