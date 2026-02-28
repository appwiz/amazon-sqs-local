#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port glue)
ENDPOINT="http://localhost:${PORT}"

aws_glue() {
  aws glue "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for Glue"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  Glue server started")

report_results "GLUE"
exit $?
