#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port imagebuilder)
ENDPOINT="http://localhost:${PORT}"

aws_imagebuilder() {
  aws imagebuilder "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for ImageBuilder"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  ImageBuilder server started")

report_results "IMAGEBUILDER"
exit $?
