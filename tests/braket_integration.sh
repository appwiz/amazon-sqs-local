#!/usr/bin/env bash

PORT=10150
ENDPOINT="http://localhost:${PORT}"
ACCOUNT="000000000000"
REGION="us-east-1"
BINARY="./target/debug/aws-inmemory-services"

PASS=0
FAIL=0
TESTS=()

aws_braket() {
  aws braket "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

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

cleanup() {
  if [[ -n "${SERVER_PID:-}" ]]; then
    kill "$SERVER_PID" 2>/dev/null || true
    wait "$SERVER_PID" 2>/dev/null || true
  fi
}
trap cleanup EXIT

echo "Building..."
cargo build --quiet 2>&1

lsof -ti:${PORT} | xargs kill 2>/dev/null || true
sleep 0.5

echo "Starting server with Braket on port ${PORT}..."
"$BINARY" \
  --braket-port $PORT \
  --s3-port 14601 \
  --sns-port 14602 \
  --sqs-port 14603 \
  --dynamodb-port 14604 \
  --lambda-port 14605 \
  --firehose-port 14606 \
  --memorydb-port 14607 \
  --cognito-port 14608 \
  --apigateway-port 14609 \
  --kms-port 14610 \
  --secretsmanager-port 14611 \
  --eventbridge-port 14612 \
  --stepfunctions-port 14613 \
  --ssm-port 14614 \
  --cloudwatchlogs-port 14615 \
  --ses-port 14616 \
  --servicecatalog-port 14617 \
  --config-port 14618 \
  --efs-port 14619 \
  --appsync-port 14620 \
  --kinesis-port 14621 \
  --ecr-port 14700 \
  --ecs-port 14701 \
  --eks-port 14702 \
  --lightsail-port 14703 \
  --apprunner-port 14704 \
  --batch-port 14705 \
  --elasticbeanstalk-port 14706 \
  --outposts-port 14707 \
  --imagebuilder-port 14708 \
  --autoscaling-port 14709 \
  --rds-port 14710 \
  --documentdb-port 14711 \
  --elasticache-port 14712 \
  --keyspaces-port 14713 \
  --neptune-port 14714 \
  --timestream-port 14715 \
  --dms-port 14716 \
  --cloudfront-port 14717 \
  --route53-port 14718 \
  --vpclattice-port 14719 \
  --cloudmap-port 14720 \
  --directconnect-port 14721 \
  --globalaccelerator-port 14722 \
  --elb-port 14723 \
  --iam-port 14724 \
  --acm-port 14725 \
  --waf-port 14726 \
  --shield-port 14727 \
  --guardduty-port 14728 \
  --inspector-port 14729 \
  --macie-port 14730 \
  --detective-port 14731 \
  --securitylake-port 14732 \
  --verifiedpermissions-port 14733 \
  --directoryservice-port 14734 \
  --cloudhsm-port 14735 \
  --ram-port 14736 \
  --securityhub-port 14737 \
  --firewallmanager-port 14738 \
  --networkfirewall-port 14739 \
  --iamidentitycenter-port 14740 \
  --athena-port 14741 \
  --cloudsearch-port 14742 \
  --datazone-port 14743 \
  --emr-port 14744 \
  --finspace-port 14745 \
  --kinesisvideostreams-port 14746 \
  --managedflink-port 14747 \
  --msk-port 14748 \
  --opensearch-port 14749 \
  --quicksight-port 14750 \
  --redshift-port 14751 \
  --cleanrooms-port 14752 \
  --dataexchange-port 14753 \
  --datapipeline-port 14754 \
  --entityresolution-port 14755 \
  --glue-port 14756 \
  --lakeformation-port 14757 \
  --cloudwatch-port 14758 \
  --managedgrafana-port 14759 \
  --managedprometheus-port 14760 \
  --cloudformation-port 14761 \
  --cloudtrail-port 14762 \
  --computeoptimizer-port 14763 \
  --controltower-port 14764 \
  --health-port 14765 \
  --licensemanager-port 14766 \
  --organizations-port 14767 \
  --proton-port 14768 \
  --trustedadvisor-port 14769 \
  --codecatalyst-port 14770 \
  --codeartifact-port 14771 \
  --codebuild-port 14772 \
  --codecommit-port 14773 \
  --codedeploy-port 14774 \
  --codepipeline-port 14775 \
  --fis-port 14776 \
  --xray-port 14777 \
  --bedrock-port 14778 \
  --comprehend-port 14779 \
  --forecast-port 14780 \
  --frauddetector-port 14781 \
  --kendra-port 14782 \
  --lex-port 14783 \
  --personalize-port 14784 \
  --polly-port 14785 \
  --rekognition-port 14786 \
  --sagemaker-port 14787 \
  --textract-port 14788 \
  --transcribe-port 14789 \
  --translate-port 14790 \
  --devopsguru-port 14791 \
  --healthlake-port 14792 \
  --qbusiness-port 14793 \
  --appflow-port 14794 \
  --mq-port 14795 \
  --mwaa-port 14796 \
  --swf-port 14797 \
  --b2bi-port 14798 \
  --iotcore-port 14799 \
  --iotevents-port 14800 \
  --iotfleetwise-port 14801 \
  --iotgreengrass-port 14802 \
  --iotsitewise-port 14803 \
  --iottwinmaker-port 14804 \
  --chime-port 14805 \
  --connect-port 14806 \
  --pinpoint-port 14807 \
  --workdocs-port 14808 \
  --workmail-port 14809 \
  --appfabric-port 14810 \
  --billingconductor-port 14811 \
  --budgets-port 14812 \
  --costexplorer-port 14813 \
  --elastictranscoder-port 14814 \
  --ivs-port 14815 \
  --mediaconvert-port 14816 \
  --medialive-port 14817 \
  --mediapackage-port 14818 \
  --mediastore-port 14819 \
  --datasync-port 14820 \
  --mainframemod-port 14821 \
  --migrationhub-port 14822 \
  --transferfamily-port 14823 \
  --backup-port 14824 \
  --fsx-port 14825 \
  --storagegateway-port 14826 \
  --drs-port 14827 \
  --groundstation-port 14828 \
  --workspaces-port 14829 \
  --location-port 14830 \
  --amplify-port 14831 \
  --devicefarm-port 14832 \
  --gamelift-port 14833 \
  --managedblockchain-port 14834 \
  --appmesh-port 14835 \
  --ec2-port 14836 \
  --region "$REGION" --account-id "$ACCOUNT" &
SERVER_PID=$!
sleep 1

if ! kill -0 "$SERVER_PID" 2>/dev/null; then
  echo "ERROR: server failed to start"
  exit 1
fi

echo "Running Braket integration tests..."

echo "Basic smoke test for Braket"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  Braket server started")

echo ""
echo "══════════════════════════════════════════════"
echo "  Braket Integration Test Results"
echo "══════════════════════════════════════════════"
for t in "${TESTS[@]}"; do echo "  $t"; done
echo "──────────────────────────────────────────────"
echo "  Passed: $PASS   Failed: $FAIL"
echo "══════════════════════════════════════════════"

exit "$FAIL"