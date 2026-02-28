#!/usr/bin/env bash
#
# Run all integration tests in parallel.
#
# Usage:
#   ./tests/run_all.sh              # run all tests with default parallelism (8)
#   ./tests/run_all.sh -j 16        # run with 16 parallel jobs
#   ./tests/run_all.sh tests/s3_integration.sh tests/dynamodb_integration.sh  # run specific tests
#
set -uo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$ROOT_DIR"

PARALLELISM=8
TEST_SCRIPTS=()

# Parse arguments
while [[ $# -gt 0 ]]; do
  case "$1" in
    -j|--jobs)
      PARALLELISM="$2"
      shift 2
      ;;
    *)
      TEST_SCRIPTS+=("$1")
      shift
      ;;
  esac
done

# If no specific tests given, find all integration test scripts
if [ ${#TEST_SCRIPTS[@]} -eq 0 ]; then
  while IFS= read -r f; do
    TEST_SCRIPTS+=("$f")
  done < <(find tests -name '*_integration.sh' ! -name 'run_all.sh' ! -name 'test_helpers.sh' | sort)
fi

TOTAL=${#TEST_SCRIPTS[@]}
if [ "$TOTAL" -eq 0 ]; then
  echo "No test scripts found."
  exit 0
fi

# ── Build once ───────────────────────────────────────────────────────────

echo "Building..."
cargo build --quiet 2>&1
if [ $? -ne 0 ]; then
  echo "ERROR: cargo build failed"
  exit 1
fi

# ── Start server once ───────────────────────────────────────────────────

source "$SCRIPT_DIR/test_helpers.sh"
ensure_server

echo ""
echo "Running $TOTAL integration tests with $PARALLELISM parallel jobs..."
echo ""

# ── Run tests in parallel ───────────────────────────────────────────────

RESULTS_DIR=$(mktemp -d)
START_TIME=$(date +%s)

# Launch tests in parallel batches, tracking PIDs to avoid waiting on server
PIDS=()
batch=0
for test_script in "${TEST_SCRIPTS[@]}"; do
  test_name=$(basename "$test_script" .sh)
  log_file="${RESULTS_DIR}/${test_name}.log"

  (
    if bash "$test_script" > "$log_file" 2>&1; then
      echo "PASS" > "${RESULTS_DIR}/${test_name}.status"
      echo "  PASS  $test_name"
    else
      echo "FAIL" > "${RESULTS_DIR}/${test_name}.status"
      echo "  FAIL  $test_name"
    fi
  ) &
  PIDS+=($!)

  batch=$((batch + 1))
  if [ "$batch" -ge "$PARALLELISM" ]; then
    for pid in "${PIDS[@]}"; do wait "$pid" 2>/dev/null || true; done
    PIDS=()
    batch=0
  fi
done
for pid in "${PIDS[@]}"; do wait "$pid" 2>/dev/null || true; done

END_TIME=$(date +%s)
ELAPSED=$((END_TIME - START_TIME))

# ── Collect results ─────────────────────────────────────────────────────

PASSED=0
FAILED=0
FAILED_TESTS=()

for test_script in "${TEST_SCRIPTS[@]}"; do
  test_name=$(basename "$test_script" .sh)
  status_file="${RESULTS_DIR}/${test_name}.status"
  if [ -f "$status_file" ] && [ "$(cat "$status_file")" = "PASS" ]; then
    PASSED=$((PASSED + 1))
  else
    FAILED=$((FAILED + 1))
    FAILED_TESTS+=("$test_name")
  fi
done

# ── Stop the server ─────────────────────────────────────────────────────

if [ -f "$PIDFILE" ]; then
  SERVER_PID=$(cat "$PIDFILE")
  kill "$SERVER_PID" 2>/dev/null || true
  wait "$SERVER_PID" 2>/dev/null || true
  rm -f "$PIDFILE"
  rmdir "$LOCKFILE" 2>/dev/null || true
fi

# ── Report ──────────────────────────────────────────────────────────────

echo ""
echo "═══════════════════════════════════════════════════════════════"
echo "  Integration Test Suite Results"
echo "═══════════════════════════════════════════════════════════════"
echo "  Total:  $TOTAL"
echo "  Passed: $PASSED"
echo "  Failed: $FAILED"
echo "  Time:   ${ELAPSED}s"
echo "═══════════════════════════════════════════════════════════════"

if [ ${#FAILED_TESTS[@]} -gt 0 ]; then
  echo ""
  echo "  Failed tests:"
  for t in "${FAILED_TESTS[@]}"; do
    echo "    - $t"
    echo "      Log: ${RESULTS_DIR}/${t}.log"
  done
  echo ""
fi

# Clean up results dir on success
if [ "$FAILED" -eq 0 ]; then
  rm -rf "$RESULTS_DIR"
fi

exit "$FAILED"
