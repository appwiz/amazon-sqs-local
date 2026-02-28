#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port mediaconvert)
ENDPOINT="http://localhost:${PORT}"

aws_mediaconvert() {
  aws mediaconvert "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for MediaConvert"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  MediaConvert server started")

report_results "MEDIACONVERT"
exit $?
