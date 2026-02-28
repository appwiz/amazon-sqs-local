#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port appmesh)
ENDPOINT="http://localhost:${PORT}"

aws_appmesh() {
  aws appmesh "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for AppMesh"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  AppMesh server started")

report_results "App Mesh"
exit $?
