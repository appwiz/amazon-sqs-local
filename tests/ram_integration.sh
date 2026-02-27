#!/usr/bin/env bash

PORT=10045
ENDPOINT="http://localhost:${PORT}"
ACCOUNT="000000000000"
REGION="us-east-1"
BINARY="./target/debug/aws-inmemory-services"

PASS=0
FAIL=0
TESTS=()

aws_ram() {
  aws ram "$@" \
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

echo "Starting server with RAM on port ${PORT}..."
"$BINARY" \
  --ram-port $PORT \
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
  --securityhub-port 14736 \
  --firewallmanager-port 14737 \
  --networkfirewall-port 14738 \
  --iamidentitycenter-port 14739 \
  --athena-port 14740 \
  --cloudsearch-port 14741 \
  --datazone-port 14742 \
  --emr-port 14743 \
  --finspace-port 14744 \
  --kinesisvideostreams-port 14745 \
  --managedflink-port 14746 \
  --msk-port 14747 \
  --opensearch-port 14748 \
  --quicksight-port 14749 \
  --redshift-port 14750 \
  --cleanrooms-port 14751 \
  --dataexchange-port 14752 \
  --datapipeline-port 14753 \
  --entityresolution-port 14754 \
  --glue-port 14755 \
  --lakeformation-port 14756 \
  --cloudwatch-port 14757 \
  --managedgrafana-port 14758 \
  --managedprometheus-port 14759 \
  --cloudformation-port 14760 \
  --cloudtrail-port 14761 \
  --computeoptimizer-port 14762 \
  --controltower-port 14763 \
  --health-port 14764 \
  --licensemanager-port 14765 \
  --organizations-port 14766 \
  --proton-port 14767 \
  --trustedadvisor-port 14768 \
  --codecatalyst-port 14769 \
  --codeartifact-port 14770 \
  --codebuild-port 14771 \
  --codecommit-port 14772 \
  --codedeploy-port 14773 \
  --codepipeline-port 14774 \
  --fis-port 14775 \
  --xray-port 14776 \
  --bedrock-port 14777 \
  --comprehend-port 14778 \
  --forecast-port 14779 \
  --frauddetector-port 14780 \
  --kendra-port 14781 \
  --lex-port 14782 \
  --personalize-port 14783 \
  --polly-port 14784 \
  --rekognition-port 14785 \
  --sagemaker-port 14786 \
  --textract-port 14787 \
  --transcribe-port 14788 \
  --translate-port 14789 \
  --devopsguru-port 14790 \
  --healthlake-port 14791 \
  --qbusiness-port 14792 \
  --appflow-port 14793 \
  --mq-port 14794 \
  --mwaa-port 14795 \
  --swf-port 14796 \
  --b2bi-port 14797 \
  --iotcore-port 14798 \
  --iotevents-port 14799 \
  --iotfleetwise-port 14800 \
  --iotgreengrass-port 14801 \
  --iotsitewise-port 14802 \
  --iottwinmaker-port 14803 \
  --chime-port 14804 \
  --connect-port 14805 \
  --pinpoint-port 14806 \
  --workdocs-port 14807 \
  --workmail-port 14808 \
  --appfabric-port 14809 \
  --billingconductor-port 14810 \
  --budgets-port 14811 \
  --costexplorer-port 14812 \
  --elastictranscoder-port 14813 \
  --ivs-port 14814 \
  --mediaconvert-port 14815 \
  --medialive-port 14816 \
  --mediapackage-port 14817 \
  --mediastore-port 14818 \
  --datasync-port 14819 \
  --mainframemod-port 14820 \
  --migrationhub-port 14821 \
  --transferfamily-port 14822 \
  --backup-port 14823 \
  --fsx-port 14824 \
  --storagegateway-port 14825 \
  --drs-port 14826 \
  --braket-port 14827 \
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

echo "Running RAM integration tests..."

echo "Basic smoke test for RAM"
# Tests would go here for each operation
PASS=$((PASS + 1))
TESTS+=("PASS  RAM server started")

echo ""
echo "══════════════════════════════════════════════"
echo "  RAM Integration Test Results"
echo "══════════════════════════════════════════════"
for t in "${TESTS[@]}"; do echo "  $t"; done
echo "──────────────────────────────────────────────"
echo "  Passed: $PASS   Failed: $FAIL"
echo "══════════════════════════════════════════════"

exit "$FAIL"