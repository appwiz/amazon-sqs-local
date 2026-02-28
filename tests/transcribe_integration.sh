#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port transcribe)
ENDPOINT="http://localhost:${PORT}"

aws_transcribe() {
  aws transcribe "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

echo "Basic smoke test for Transcribe"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  Transcribe server started")

report_results "TRANSCRIBE"
exit $?
