#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port kinesisvideostreams)
ENDPOINT="http://localhost:${PORT}"

aws_kinesisvideostreams() {
  aws kinesisvideostreams "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for KinesisVideoStreams"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  KinesisVideoStreams server started")

report_results "KINESISVIDEOSTREAMS"
exit $?
