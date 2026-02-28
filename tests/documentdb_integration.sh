#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port documentdb)
ENDPOINT="http://localhost:${PORT}"

aws_documentdb() {
  aws documentdb "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for DocumentDB"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  DocumentDB server started")

report_results "DOCUMENTDB"
exit $?
