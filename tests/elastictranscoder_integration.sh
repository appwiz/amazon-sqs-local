#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port elastictranscoder)
ENDPOINT="http://localhost:${PORT}"

aws_elastictranscoder() {
  aws elastictranscoder "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for ElasticTranscoder"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  ElasticTranscoder server started")

report_results "ELASTICTRANSCODER"
exit $?
