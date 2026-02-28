#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port computeoptimizer)
ENDPOINT="http://localhost:${PORT}"

aws_computeoptimizer() {
  aws computeoptimizer "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for ComputeOptimizer"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  ComputeOptimizer server started")

report_results "COMPUTEOPTIMIZER"
exit $?
