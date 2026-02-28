#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port datapipeline)
ENDPOINT="http://localhost:${PORT}"

aws_datapipeline() {
  aws datapipeline "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for DataPipeline"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  DataPipeline server started")

report_results "DATAPIPELINE"
exit $?
