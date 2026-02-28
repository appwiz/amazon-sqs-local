#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port textract)
ENDPOINT="http://localhost:${PORT}"

aws_textract() {
  aws textract "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for Textract"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  Textract server started")

report_results "TEXTRACT"
exit $?
