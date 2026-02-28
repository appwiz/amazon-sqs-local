#!/usr/bin/env bash
#
# Shared test helpers and server management for integration tests.
# Source this file at the top of each test script.
#
# Usage:
#   source "$(dirname "$0")/test_helpers.sh"
#   PORT=$(service_port dynamodb)
#   ENDPOINT="http://localhost:${PORT}"
#   ensure_server
#   ... run tests ...
#   report_results "DynamoDB"
#

set -uo pipefail

ACCOUNT="000000000000"
REGION="us-east-1"
BINARY="./target/debug/aws-inmemory-services"
LOCKFILE="/tmp/aws-inmemory-services-test.lock"
PIDFILE="/tmp/aws-inmemory-services-test.pid"
HEALTH_PORT=9000  # S3 port, always started

PASS=0
FAIL=0
TESTS=()

# ── Default port map (matches main.rs defaults) ─────────────────────────

service_port() {
  case "$1" in
    acm) echo 10034;; amplify) echo 10154;; apigateway) echo 4567;;
    appfabric) echo 10128;; appflow) echo 10112;; appmesh) echo 10158;;
    apprunner) echo 10006;; appsync) echo 9700;; athena) echo 10050;;
    autoscaling) echo 10011;; b2bi) echo 10116;; backup) echo 10146;;
    batch) echo 10007;; bedrock) echo 10093;; billingconductor) echo 10129;;
    braket) echo 10150;; budgets) echo 10130;; chime) echo 10123;;
    cleanrooms) echo 10061;; cloudformation) echo 10070;; cloudfront) echo 10021;;
    cloudhsm) echo 10044;; cloudmap) echo 10024;; cloudsearch) echo 10051;;
    cloudtrail) echo 10071;; cloudwatch) echo 10067;; cloudwatchlogs) echo 9201;;
    codeartifact) echo 10083;; codebuild) echo 10084;; codecatalyst) echo 10082;;
    codecommit) echo 10085;; codedeploy) echo 10086;; codepipeline) echo 10087;;
    cognito) echo 9229;; comprehend) echo 10094;; computeoptimizer) echo 10072;;
    config) echo 9500;; connect) echo 10124;; controltower) echo 10073;;
    costexplorer) echo 10131;; dataexchange) echo 10062;; datapipeline) echo 10063;;
    datasync) echo 10138;; datazone) echo 10052;; detective) echo 10040;;
    devicefarm) echo 10155;; devopsguru) echo 10106;; directconnect) echo 10025;;
    directoryservice) echo 10043;; dms) echo 10018;; documentdb) echo 10013;;
    drs) echo 10149;; dynamodb) echo 8000;; ec2) echo 10001;;
    ecr) echo 10002;; ecs) echo 10003;; efs) echo 9600;;
    eks) echo 10004;; elasticache) echo 10014;; elasticbeanstalk) echo 10008;;
    elastictranscoder) echo 10132;; elb) echo 10027;; emr) echo 10053;;
    entityresolution) echo 10064;; eventbridge) echo 9195;; finspace) echo 10054;;
    firehose) echo 4573;; firewallmanager) echo 10047;; fis) echo 10088;;
    forecast) echo 10095;; frauddetector) echo 10096;; fsx) echo 10147;;
    gamelift) echo 10156;; globalaccelerator) echo 10026;; glue) echo 10065;;
    groundstation) echo 10151;; guardduty) echo 10037;; health) echo 10074;;
    healthlake) echo 10107;; iam) echo 10033;; iamidentitycenter) echo 10049;;
    imagebuilder) echo 10010;; inspector) echo 10038;; iotcore) echo 10117;;
    iotevents) echo 10118;; iotfleetwise) echo 10119;; iotgreengrass) echo 10120;;
    iotsitewise) echo 10121;; iottwinmaker) echo 10122;; ivs) echo 10133;;
    kendra) echo 10097;; keyspaces) echo 10015;; kinesis) echo 4568;;
    kinesisvideostreams) echo 10055;; kms) echo 7600;; lakeformation) echo 10066;;
    lambda) echo 9001;; lex) echo 10098;; licensemanager) echo 10075;;
    lightsail) echo 10005;; location) echo 10153;; macie) echo 10039;;
    mainframemod) echo 10139;; managedblockchain) echo 10157;; managedflink) echo 10056;;
    managedgrafana) echo 10068;; managedprometheus) echo 10069;; mediaconvert) echo 10134;;
    medialive) echo 10135;; mediapackage) echo 10136;; mediastore) echo 10137;;
    memorydb) echo 6379;; migrationhub) echo 10140;; mq) echo 10113;;
    msk) echo 10057;; mwaa) echo 10114;; neptune) echo 10016;;
    networkfirewall) echo 10048;; opensearch) echo 10058;; organizations) echo 10076;;
    outposts) echo 10009;; personalize) echo 10099;; pinpoint) echo 10125;;
    polly) echo 10100;; proton) echo 10077;; qbusiness) echo 10108;;
    quicksight) echo 10059;; ram) echo 10045;; rds) echo 10012;;
    redshift) echo 10060;; rekognition) echo 10101;; route53) echo 10022;;
    s3) echo 9000;; sagemaker) echo 10102;; secretsmanager) echo 7700;;
    securityhub) echo 10046;; securitylake) echo 10041;; servicecatalog) echo 9400;;
    ses) echo 9300;; shield) echo 10036;; sns) echo 9911;;
    sqs) echo 9324;; ssm) echo 9100;; stepfunctions) echo 8083;;
    storagegateway) echo 10148;; swf) echo 10115;; textract) echo 10103;;
    timestream) echo 10017;; transcribe) echo 10104;; transferfamily) echo 10141;;
    translate) echo 10105;; trustedadvisor) echo 10078;; verifiedpermissions) echo 10042;;
    vpclattice) echo 10023;; waf) echo 10035;; workdocs) echo 10126;;
    workmail) echo 10127;; workspaces) echo 10152;; xray) echo 10089;;
    *) echo "ERROR: unknown service '$1'" >&2; return 1;;
  esac
}

# ── Assertion helpers ────────────────────────────────────────────────────

assert_contains() {
  local label="$1" output="$2" expected="$3"
  if echo "$output" | grep -qF "$expected"; then
    PASS=$((PASS + 1))
    TESTS+=("PASS  $label")
  else
    FAIL=$((FAIL + 1))
    TESTS+=("FAIL  $label  (expected '$expected' in output)")
    echo "FAIL: $label" >&2
    echo "  expected: $expected" >&2
    echo "  output:   $output" >&2
  fi
}

assert_not_contains() {
  local label="$1" output="$2" unexpected="$3"
  if echo "$output" | grep -qF "$unexpected"; then
    FAIL=$((FAIL + 1))
    TESTS+=("FAIL  $label  (did not expect '$unexpected' in output)")
    echo "FAIL: $label" >&2
    echo "  unexpected: $unexpected" >&2
    echo "  output:     $output" >&2
  else
    PASS=$((PASS + 1))
    TESTS+=("PASS  $label")
  fi
}

assert_exit_zero() {
  local label="$1"
  shift
  if "$@" > /dev/null 2>&1; then
    PASS=$((PASS + 1))
    TESTS+=("PASS  $label")
  else
    FAIL=$((FAIL + 1))
    TESTS+=("FAIL  $label  (non-zero exit)")
    echo "FAIL: $label" >&2
  fi
}

json_field() {
  python3 -c "import sys,json; print(json.load(sys.stdin)$1)" 2>/dev/null
}

# ── Server management ───────────────────────────────────────────────────

_server_is_running() {
  curl -sf -o /dev/null --max-time 1 "http://localhost:${HEALTH_PORT}/" 2>/dev/null
  return $?
}

_wait_for_server() {
  local max_wait=30
  local waited=0
  while [ $waited -lt $max_wait ]; do
    if curl -sf -o /dev/null --max-time 1 "http://localhost:${HEALTH_PORT}/" 2>/dev/null; then
      return 0
    fi
    sleep 0.2
    waited=$((waited + 1))
  done
  echo "ERROR: server failed to start within ${max_wait} attempts" >&2
  return 1
}

ensure_server() {
  # If server is already running, nothing to do
  if _server_is_running; then
    return 0
  fi

  # Use mkdir as an atomic lock (works on all platforms, unlike flock)
  while ! mkdir "$LOCKFILE" 2>/dev/null; do
    sleep 0.1
  done

  # Double-check after acquiring lock (another script may have started it)
  if _server_is_running; then
    rmdir "$LOCKFILE" 2>/dev/null
    return 0
  fi

  echo "Starting server..." >&2
  "$BINARY" --region "$REGION" --account-id "$ACCOUNT" &
  local pid=$!
  echo "$pid" > "$PIDFILE"

  # Release lock after server PID is written
  rmdir "$LOCKFILE" 2>/dev/null

  # Wait for server to be ready
  _wait_for_server
}

# ── Results reporting ────────────────────────────────────────────────────

report_results() {
  local service_name="${1:-Tests}"
  echo ""
  echo "══════════════════════════════════════════════"
  echo "  ${service_name} Integration Test Results"
  echo "══════════════════════════════════════════════"
  if [ ${#TESTS[@]} -gt 0 ]; then
    for t in "${TESTS[@]}"; do echo "  $t"; done
  fi
  echo "──────────────────────────────────────────────"
  echo "  Passed: $PASS   Failed: $FAIL"
  echo "══════════════════════════════════════════════"

  if [ "$FAIL" -gt 0 ]; then
    return 1
  fi
  return 0
}
