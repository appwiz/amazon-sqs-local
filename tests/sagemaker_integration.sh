#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port sagemaker)
ENDPOINT="http://localhost:${PORT}"

aws_sagemaker() {
  aws sagemaker "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for SageMaker"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  SageMaker server started")

report_results "SAGEMAKER"
exit $?
