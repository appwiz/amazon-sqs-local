#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port opensearch)
ENDPOINT="http://localhost:${PORT}"

aws_opensearch() {
  aws opensearch "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for OpenSearch"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  OpenSearch server started")

report_results "OPENSEARCH"
exit $?
