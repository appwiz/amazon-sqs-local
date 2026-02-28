#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port mediastore)
ENDPOINT="http://localhost:${PORT}"

aws_mediastore() {
  aws mediastore "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for MediaStore"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  MediaStore server started")

report_results "MEDIASTORE"
exit $?
