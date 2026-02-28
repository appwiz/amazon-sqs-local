# Management & Governance

## CloudFormation

| Property | Value |
|----------|-------|
| Port | `10070` |
| Protocol | Query/XML |
| Endpoint | `http://localhost:10070` |
| Action parameter | `Action` query parameter |

### Operations (3)

| Operation | Description |
|-----------|-------------|
| `CreateStack` | Create a new stack |
| `DescribeStacks` | Describe one or more stacks |
| `DeleteStack` | Delete a stack |

### CLI Example

```bash
aws cloudformation create-stack \
  --stack-name my-stack \
  --template-body '{"Resources":{}}' \
  --endpoint-url http://localhost:10070 \
  --no-sign-request
```

---

## CloudWatch

| Property | Value |
|----------|-------|
| Port | `10067` |
| Protocol | Query/XML |
| Endpoint | `http://localhost:10067` |
| Action parameter | `Action` query parameter |

### Operations (3)

| Operation | Description |
|-----------|-------------|
| `CreateAlarm` | Create or update an alarm |
| `DescribeAlarms` | Describe one or more alarms |
| `DeleteAlarm` | Delete an alarm |

### CLI Example

```bash
aws cloudwatch put-metric-alarm \
  --alarm-name my-alarm \
  --metric-name CPUUtilization \
  --namespace AWS/EC2 \
  --statistic Average \
  --period 300 \
  --threshold 80 \
  --comparison-operator GreaterThanThreshold \
  --evaluation-periods 1 \
  --endpoint-url http://localhost:10067 \
  --no-sign-request
```

---

## CloudTrail

| Property | Value |
|----------|-------|
| Port | `10071` |
| Protocol | JSON RPC |
| Endpoint | `http://localhost:10071` |
| Target prefix | `CloudTrail_20131101` |

### Operations (4)

| Operation | Description |
|-----------|-------------|
| `CreateTrail` | Create a new trail |
| `DescribeTrail` | Describe a trail |
| `ListTrails` | List all trails |
| `DeleteTrail` | Delete a trail |

### CLI Example

```bash
aws cloudtrail create-trail \
  --name my-trail \
  --s3-bucket-name my-bucket \
  --endpoint-url http://localhost:10071 \
  --no-sign-request
```

---

## CloudWatch Logs

| Property | Value |
|----------|-------|
| Port | `9201` |
| Protocol | JSON RPC |
| Endpoint | `http://localhost:9201` |
| Target prefix | `Logs_20140328` |

### Operations (17)

| Operation | Description |
|-----------|-------------|
| `CreateLogGroup` | Create a new log group |
| `DeleteLogGroup` | Delete a log group |
| `DescribeLogGroups` | List and describe log groups |
| `CreateLogStream` | Create a new log stream within a log group |
| `DeleteLogStream` | Delete a log stream |
| `DescribeLogStreams` | List and describe log streams within a log group |
| `PutLogEvents` | Upload log events to a log stream |
| `GetLogEvents` | Retrieve log events from a log stream |
| `FilterLogEvents` | Search log events across streams using substring matching |
| `PutRetentionPolicy` | Set the retention policy for a log group |
| `DeleteRetentionPolicy` | Remove the retention policy from a log group |
| `TagLogGroup` | Add tags to a log group (legacy) |
| `UntagLogGroup` | Remove tags from a log group (legacy) |
| `ListTagsLogGroup` | List tags for a log group (legacy) |
| `TagResource` | Add tags to a log group (new ARN-based API) |
| `UntagResource` | Remove tags from a log group (new ARN-based API) |
| `ListTagsForResource` | List tags for a log group (new ARN-based API) |

### Wire Protocol

All requests are HTTP POST to `/` with headers:

```
Content-Type: application/x-amz-json-1.1
X-Amz-Target: Logs_20140328.<Action>
```

The request body is a JSON object specific to each action.

### CLI Examples

**Create a log group and stream, then write and read events:**

```bash
# Create a log group
aws logs create-log-group \
  --log-group-name /app/my-service \
  --endpoint-url http://localhost:9201 \
  --no-sign-request

# Create a log stream
aws logs create-log-stream \
  --log-group-name /app/my-service \
  --log-stream-name instance-001 \
  --endpoint-url http://localhost:9201 \
  --no-sign-request

# Write log events
aws logs put-log-events \
  --log-group-name /app/my-service \
  --log-stream-name instance-001 \
  --log-events '[{"timestamp":1700000000000,"message":"Hello from my service"}]' \
  --endpoint-url http://localhost:9201 \
  --no-sign-request

# Read log events
aws logs get-log-events \
  --log-group-name /app/my-service \
  --log-stream-name instance-001 \
  --endpoint-url http://localhost:9201 \
  --no-sign-request

# Filter log events (uses substring matching, not CloudWatch filter syntax)
aws logs filter-log-events \
  --log-group-name /app/my-service \
  --filter-pattern "Hello" \
  --endpoint-url http://localhost:9201 \
  --no-sign-request

# Set retention policy
aws logs put-retention-policy \
  --log-group-name /app/my-service \
  --retention-in-days 30 \
  --endpoint-url http://localhost:9201 \
  --no-sign-request

# List log groups
aws logs describe-log-groups \
  --endpoint-url http://localhost:9201 \
  --no-sign-request

# List log streams
aws logs describe-log-streams \
  --log-group-name /app/my-service \
  --endpoint-url http://localhost:9201 \
  --no-sign-request
```

### Limitations

- `FilterLogEvents` uses simple substring matching, not the full CloudWatch Logs filter pattern syntax.
- All state is in-memory only. Restarting the server clears all log groups, streams, and events.
- No CloudWatch metrics integration.

---

## Organizations

| Property | Value |
|----------|-------|
| Port | `10076` |
| Protocol | JSON RPC |
| Endpoint | `http://localhost:10076` |
| Target prefix | `AWSOrganizationsV20161128` |

### Operations (8)

| Operation | Description |
|-----------|-------------|
| `CreateOrganization` | Create a new organization |
| `DescribeOrganization` | Describe the organization |
| `ListOrganizations` | List organizations |
| `DeleteOrganization` | Delete the organization |
| `CreateAccount` | Create a new account |
| `DescribeAccount` | Describe an account |
| `ListAccounts` | List all accounts |
| `DeleteAccount` | Delete an account |

### CLI Example

```bash
aws organizations create-organization \
  --endpoint-url http://localhost:10076 \
  --no-sign-request
```

---

## Config

| Property | Value |
|----------|-------|
| Port | `9500` |
| Protocol | JSON RPC |
| Endpoint | `http://localhost:9500` |
| Target prefix | `StarlingDoveService` |

### Operations (19)

| Operation | Description |
|-----------|-------------|
| `PutConfigurationRecorder` | Create or update a configuration recorder |
| `DescribeConfigurationRecorders` | List and describe configuration recorders |
| `DeleteConfigurationRecorder` | Delete a configuration recorder |
| `DescribeConfigurationRecorderStatus` | Get the status of configuration recorders |
| `StartConfigurationRecorder` | Start recording resource configurations |
| `StopConfigurationRecorder` | Stop recording resource configurations |
| `PutDeliveryChannel` | Create or update a delivery channel |
| `DescribeDeliveryChannels` | List and describe delivery channels |
| `DeleteDeliveryChannel` | Delete a delivery channel |
| `PutConfigRule` | Create or update a Config rule |
| `DescribeConfigRules` | List and describe Config rules |
| `DeleteConfigRule` | Delete a Config rule |
| `PutEvaluations` | Submit compliance evaluation results |
| `GetComplianceDetailsByConfigRule` | Get compliance details for a rule |
| `DescribeComplianceByConfigRule` | Get compliance summary by rule |
| `DescribeComplianceByResource` | Get compliance summary by resource |
| `TagResource` | Add tags to a Config resource |
| `UntagResource` | Remove tags from a Config resource |
| `ListTagsForResource` | List tags for a Config resource |

### Wire Protocol

All requests are HTTP POST to `/` with headers:

```
Content-Type: application/x-amz-json-1.1
X-Amz-Target: StarlingDoveService.<Action>
```

The request body is a JSON object specific to each action.

### CLI Examples

**Set up a configuration recorder and delivery channel:**

```bash
# Create a configuration recorder
aws configservice put-configuration-recorder \
  --configuration-recorder name=default,roleARN=arn:aws:iam::000000000000:role/config-role \
  --endpoint-url http://localhost:9500 \
  --no-sign-request

# Create a delivery channel
aws configservice put-delivery-channel \
  --delivery-channel name=default,s3BucketName=my-config-bucket \
  --endpoint-url http://localhost:9500 \
  --no-sign-request

# Start recording
aws configservice start-configuration-recorder \
  --configuration-recorder-name default \
  --endpoint-url http://localhost:9500 \
  --no-sign-request

# Check recorder status
aws configservice describe-configuration-recorder-status \
  --endpoint-url http://localhost:9500 \
  --no-sign-request

# Create a Config rule
aws configservice put-config-rule \
  --config-rule '{"ConfigRuleName":"my-rule","Source":{"Owner":"AWS","SourceIdentifier":"S3_BUCKET_VERSIONING_ENABLED"}}' \
  --endpoint-url http://localhost:9500 \
  --no-sign-request

# Describe Config rules
aws configservice describe-config-rules \
  --endpoint-url http://localhost:9500 \
  --no-sign-request

# Check compliance
aws configservice describe-compliance-by-config-rule \
  --endpoint-url http://localhost:9500 \
  --no-sign-request

# Stop recording
aws configservice stop-configuration-recorder \
  --configuration-recorder-name default \
  --endpoint-url http://localhost:9500 \
  --no-sign-request
```

### Limitations

- Configuration recording is simulated. The recorder toggling is tracked but resources are not actually monitored.
- Compliance evaluations are stored but no automatic evaluation occurs.
- All state is in-memory only.

---

## SSM Parameter Store

| Property | Value |
|----------|-------|
| Port | `9100` |
| Protocol | JSON RPC |
| Endpoint | `http://localhost:9100` |
| Target prefix | `AmazonSSM` |

### Operations (10)

| Operation | Description |
|-----------|-------------|
| `PutParameter` | Create or update a parameter |
| `GetParameter` | Get a single parameter by name |
| `GetParameters` | Get multiple parameters by name |
| `GetParametersByPath` | Get parameters under a hierarchy path |
| `DeleteParameter` | Delete a single parameter |
| `DeleteParameters` | Delete multiple parameters |
| `DescribeParameters` | List and describe parameters with filtering |
| `AddTagsToResource` | Add tags to a parameter |
| `RemoveTagsFromResource` | Remove tags from a parameter |
| `ListTagsForResource` | List tags for a parameter |

### Wire Protocol

All requests are HTTP POST to `/` with headers:

```
Content-Type: application/x-amz-json-1.1
X-Amz-Target: AmazonSSM.<Action>
```

The request body is a JSON object specific to each action.

### CLI Examples

**Store, retrieve, and manage parameters:**

```bash
# Create a String parameter
aws ssm put-parameter \
  --name "/app/config/db-host" \
  --value "localhost" \
  --type String \
  --endpoint-url http://localhost:9100 \
  --no-sign-request

# Create a SecureString parameter (stored in plaintext, no KMS encryption)
aws ssm put-parameter \
  --name "/app/config/db-password" \
  --value "secret123" \
  --type SecureString \
  --endpoint-url http://localhost:9100 \
  --no-sign-request

# Get a single parameter
aws ssm get-parameter \
  --name "/app/config/db-host" \
  --endpoint-url http://localhost:9100 \
  --no-sign-request

# Get multiple parameters
aws ssm get-parameters \
  --names "/app/config/db-host" "/app/config/db-password" \
  --endpoint-url http://localhost:9100 \
  --no-sign-request

# Get parameters by path
aws ssm get-parameters-by-path \
  --path "/app/config" \
  --endpoint-url http://localhost:9100 \
  --no-sign-request

# Describe parameters (list with filtering)
aws ssm describe-parameters \
  --endpoint-url http://localhost:9100 \
  --no-sign-request

# Delete a parameter
aws ssm delete-parameter \
  --name "/app/config/db-host" \
  --endpoint-url http://localhost:9100 \
  --no-sign-request

# Tag a parameter
aws ssm add-tags-to-resource \
  --resource-type Parameter \
  --resource-id "/app/config/db-password" \
  --tags Key=env,Value=dev \
  --endpoint-url http://localhost:9100 \
  --no-sign-request
```

### Limitations

- SecureString values are stored in plaintext. No KMS encryption is performed.
- Parameter versioning tracks the version number but does not maintain version history.
- All state is in-memory only.

---

## Trusted Advisor

| Property | Value |
|----------|-------|
| Port | `10078` |
| Protocol | JSON RPC |
| Endpoint | `http://localhost:10078` |
| Target prefix | `AWSSupport_20130415` |

### Operations (4)

| Operation | Description |
|-----------|-------------|
| `CreateCheck` | Create a check |
| `DescribeCheck` | Describe a check |
| `ListChecks` | List all checks |
| `DeleteCheck` | Delete a check |

### CLI Example

```bash
aws support describe-trusted-advisor-checks \
  --language en \
  --endpoint-url http://localhost:10078 \
  --no-sign-request
```

---

## Health

| Property | Value |
|----------|-------|
| Port | `10074` |
| Protocol | JSON RPC |
| Endpoint | `http://localhost:10074` |
| Target prefix | `AWSHealth_20160804` |

### Operations (4)

| Operation | Description |
|-----------|-------------|
| `CreateEvent` | Create a health event |
| `DescribeEvent` | Describe a health event |
| `ListEvents` | List health events |
| `DeleteEvent` | Delete a health event |

### CLI Example

```bash
aws health describe-events \
  --endpoint-url http://localhost:10074 \
  --no-sign-request
```

---

## Control Tower

| Property | Value |
|----------|-------|
| Port | `10073` |
| Protocol | REST JSON |
| Endpoint | `http://localhost:10073` |

### Operations (4)

| Operation | Method | Path |
|-----------|--------|------|
| `CreateLandingZone` | POST | `/create-landing-zone` |
| `GetLandingZone` | GET | `/get-landing-zone` |
| `ListLandingZones` | GET | `/list-landing-zones` |
| `DeleteLandingZone` | DELETE | `/delete-landing-zone` |

### CLI Example

```bash
aws controltower list-landing-zones \
  --endpoint-url http://localhost:10073 \
  --no-sign-request
```

---

## Compute Optimizer

| Property | Value |
|----------|-------|
| Port | `10072` |
| Protocol | JSON RPC |
| Endpoint | `http://localhost:10072` |
| Target prefix | `ComputeOptimizerService` |

### Operations (4)

| Operation | Description |
|-----------|-------------|
| `CreateRecommendation` | Create a recommendation |
| `DescribeRecommendation` | Describe a recommendation |
| `ListRecommendations` | List recommendations |
| `DeleteRecommendation` | Delete a recommendation |

### CLI Example

```bash
aws compute-optimizer get-enrollment-status \
  --endpoint-url http://localhost:10072 \
  --no-sign-request
```

---

## License Manager

| Property | Value |
|----------|-------|
| Port | `10075` |
| Protocol | JSON RPC |
| Endpoint | `http://localhost:10075` |
| Target prefix | `AWSLicenseManager` |

### Operations (4)

| Operation | Description |
|-----------|-------------|
| `CreateLicense` | Create a license |
| `DescribeLicense` | Describe a license |
| `ListLicenses` | List licenses |
| `DeleteLicense` | Delete a license |

### CLI Example

```bash
aws license-manager list-licenses \
  --endpoint-url http://localhost:10075 \
  --no-sign-request
```

---

## Proton

| Property | Value |
|----------|-------|
| Port | `10077` |
| Protocol | JSON RPC |
| Endpoint | `http://localhost:10077` |
| Target prefix | `AwsProton20200720` |

### Operations (4)

| Operation | Description |
|-----------|-------------|
| `CreateEnvironmentTemplate` | Create an environment template |
| `DescribeEnvironmentTemplate` | Describe an environment template |
| `ListEnvironmentTemplates` | List environment templates |
| `DeleteEnvironmentTemplate` | Delete an environment template |

### CLI Example

```bash
aws proton list-environment-templates \
  --endpoint-url http://localhost:10077 \
  --no-sign-request
```

---

## Managed Grafana

| Property | Value |
|----------|-------|
| Port | `10068` |
| Protocol | REST JSON |
| Endpoint | `http://localhost:10068` |

### Operations (4)

| Operation | Method | Path |
|-----------|--------|------|
| `CreateWorkspace` | POST | `/workspaces` |
| `ListWorkspaces` | GET | `/workspaces` |
| `GetWorkspace` | GET | `/workspaces/{workspaceId}` |
| `DeleteWorkspace` | DELETE | `/workspaces/{workspaceId}` |

### CLI Example

```bash
aws grafana list-workspaces \
  --endpoint-url http://localhost:10068 \
  --no-sign-request
```

---

## Managed Prometheus

| Property | Value |
|----------|-------|
| Port | `10069` |
| Protocol | REST JSON |
| Endpoint | `http://localhost:10069` |

### Operations (4)

| Operation | Method | Path |
|-----------|--------|------|
| `CreateWorkspace` | POST | `/workspaces` |
| `ListWorkspaces` | GET | `/workspaces` |
| `GetWorkspace` | GET | `/workspaces/{workspaceId}` |
| `DeleteWorkspace` | DELETE | `/workspaces/{workspaceId}` |

### CLI Example

```bash
aws amp list-workspaces \
  --endpoint-url http://localhost:10069 \
  --no-sign-request
```

---

## X-Ray

| Property | Value |
|----------|-------|
| Port | `10089` |
| Protocol | REST JSON |
| Endpoint | `http://localhost:10089` |

### Operations (4)

| Operation | Method | Path |
|-----------|--------|------|
| `CreateGroup` | POST | `/CreateGroup` |
| `ListGroups` | POST | `/Groups` |
| `GetGroup` | POST | `/GetGroup` |
| `DeleteGroup` | POST | `/DeleteGroup` |

### CLI Example

```bash
aws xray get-groups \
  --endpoint-url http://localhost:10089 \
  --no-sign-request
```
