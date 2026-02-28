# aws-inmemory-services

In-memory implementations of 159 AWS services written in Rust. All services run as a single binary on separate ports, are compatible with the AWS CLI and SDKs, and require no external dependencies. All state is held in memory — there is no disk persistence. Restarting the server clears all data.

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) (1.70+)
- [AWS CLI v2](https://docs.aws.amazon.com/cli/latest/userguide/getting-started-install.html) (for integration tests)

### Build

```bash
cargo build --release
```

### Run

```bash
./target/release/aws-inmemory-services
```

All 159 services start on their default ports. Override any port with `--<service>-port <PORT>`, and set region/account with `--region` and `--account-id`:

```bash
./target/release/aws-inmemory-services --region eu-west-1 --account-id 123456789012
```

### Connecting with the AWS CLI

Point the AWS CLI at any service using `--endpoint-url` and skip authentication:

```bash
aws s3api create-bucket \
  --bucket my-bucket \
  --endpoint-url http://localhost:9000 \
  --no-sign-request

aws dynamodb create-table \
  --table-name MyTable \
  --attribute-definitions AttributeName=pk,AttributeType=S \
  --key-schema AttributeName=pk,KeyType=HASH \
  --billing-mode PAY_PER_REQUEST \
  --endpoint-url http://localhost:8000 \
  --no-sign-request
```

### Connecting with AWS SDKs

```javascript
import { S3Client, PutObjectCommand } from "@aws-sdk/client-s3";

const client = new S3Client({
  endpoint: "http://localhost:9000",
  region: "us-east-1",
  credentials: { accessKeyId: "test", secretAccessKey: "test" },
  forcePathStyle: true,
});

await client.send(new PutObjectCommand({
  Bucket: "my-bucket",
  Key: "hello.txt",
  Body: "Hello, world!",
}));
```

## Services (159)

All services are organized by category. Click the category link for detailed documentation including supported operations, CLI examples, SDK examples, and wire protocol details.

### Compute (13 services) — [Full Documentation](docs/compute.md)

| Service | Port | Operations |
|---------|------|------------|
| EC2 | `10001` | 9 |
| ECS | `10003` | 12 |
| EKS | `10004` | 4 |
| ECR | `10002` | 4 |
| Lambda | `9001` | 22 |
| Batch | `10007` | 8 |
| Lightsail | `10005` | 4 |
| Outposts | `10009` | 4 |
| Auto Scaling | `10011` | 3 |
| Elastic Beanstalk | `10008` | 3 |
| App Runner | `10006` | 4 |
| EC2 Image Builder | `10010` | 4 |
| GameLift | `10156` | 4 |

### Storage (5 services) — [Full Documentation](docs/storage.md)

| Service | Port | Operations |
|---------|------|------------|
| S3 | `9000` | 26 |
| EFS | `9600` | 15 |
| FSx | `10147` | 4 |
| Backup | `10146` | 8 |
| Storage Gateway | `10148` | 4 |

### Databases (9 services) — [Full Documentation](docs/databases.md)

| Service | Port | Operations |
|---------|------|------------|
| DynamoDB | `8000` | 16 |
| RDS | `10012` | 6 |
| ElastiCache | `10014` | 3 |
| Neptune | `10016` | 3 |
| DocumentDB | `10013` | 3 |
| Timestream | `10017` | 8 |
| Keyspaces | `10015` | 8 |
| MemoryDB | `6379` | 21 |
| Redshift | `10060` | 3 |

### Networking & Content Delivery (10 services) — [Full Documentation](docs/networking.md)

| Service | Port | Operations |
|---------|------|------------|
| API Gateway | `4567` | 30 |
| CloudFront | `10021` | 4 |
| Route 53 | `10022` | 4 |
| ELB | `10027` | 6 |
| VPC Lattice | `10023` | 4 |
| Direct Connect | `10025` | 4 |
| Global Accelerator | `10026` | 4 |
| Cloud Map | `10024` | 4 |
| Network Firewall | `10048` | 8 |
| App Mesh | `10158` | 4 |

### Security, Identity & Compliance (19 services) — [Full Documentation](docs/security.md)

| Service | Port | Operations |
|---------|------|------------|
| IAM | `10033` | 9 |
| Cognito | `9229` | 33 |
| KMS | `7600` | 22 |
| Secrets Manager | `7700` | 11 |
| WAF | `10035` | 8 |
| Shield | `10036` | 4 |
| GuardDuty | `10037` | 4 |
| ACM | `10034` | 4 |
| Macie | `10039` | 4 |
| Inspector | `10038` | 4 |
| Security Hub | `10046` | 4 |
| Security Lake | `10041` | 4 |
| Verified Permissions | `10042` | 4 |
| CloudHSM | `10044` | 4 |
| RAM | `10045` | 4 |
| Directory Service | `10043` | 4 |
| IAM Identity Center | `10049` | 4 |
| Firewall Manager | `10047` | 4 |
| Detective | `10040` | 4 |

### Management & Governance (16 services) — [Full Documentation](docs/management.md)

| Service | Port | Operations |
|---------|------|------------|
| CloudFormation | `10070` | 3 |
| CloudWatch | `10067` | 3 |
| CloudTrail | `10071` | 4 |
| CloudWatch Logs | `9201` | 17 |
| Organizations | `10076` | 8 |
| Config | `9500` | 19 |
| SSM Parameter Store | `9100` | 10 |
| Trusted Advisor | `10078` | 4 |
| Health | `10074` | 4 |
| Control Tower | `10073` | 4 |
| Compute Optimizer | `10072` | 4 |
| License Manager | `10075` | 4 |
| Proton | `10077` | 4 |
| Managed Grafana | `10068` | 4 |
| Managed Prometheus | `10069` | 4 |
| X-Ray | `10089` | 4 |

### Developer Tools (7 services) — [Full Documentation](docs/developer-tools.md)

| Service | Port | Operations |
|---------|------|------------|
| CodeBuild | `10084` | 4 |
| CodePipeline | `10087` | 4 |
| CodeCommit | `10085` | 4 |
| CodeDeploy | `10086` | 4 |
| CodeArtifact | `10083` | 8 |
| CodeCatalyst | `10082` | 4 |
| FIS | `10088` | 4 |

### Analytics (18 services) — [Full Documentation](docs/analytics.md)

| Service | Port | Operations |
|---------|------|------------|
| Athena | `10050` | 4 |
| Glue | `10065` | 12 |
| EMR | `10053` | 4 |
| OpenSearch | `10058` | 4 |
| Kinesis | `4568` | 15 |
| Firehose | `4573` | 10 |
| QuickSight | `10059` | 4 |
| CloudSearch | `10051` | 3 |
| MSK | `10057` | 4 |
| Kinesis Video Streams | `10055` | 4 |
| Managed Flink | `10056` | 4 |
| DataZone | `10052` | 4 |
| FinSpace | `10054` | 4 |
| Lake Formation | `10066` | 4 |
| Data Exchange | `10062` | 4 |
| Data Pipeline | `10063` | 4 |
| Entity Resolution | `10064` | 4 |
| Clean Rooms | `10061` | 4 |

### Machine Learning & AI (14 services) — [Full Documentation](docs/ml-ai.md)

| Service | Port | Operations |
|---------|------|------------|
| SageMaker | `10102` | 4 |
| Bedrock | `10093` | 4 |
| Comprehend | `10094` | 4 |
| Rekognition | `10101` | 4 |
| Textract | `10103` | 4 |
| Transcribe | `10104` | 4 |
| Translate | `10105` | 4 |
| Polly | `10100` | 4 |
| Lex | `10098` | 4 |
| Kendra | `10097` | 4 |
| Personalize | `10099` | 4 |
| Forecast | `10095` | 4 |
| Fraud Detector | `10096` | 4 |
| HealthLake | `10107` | 4 |

### IoT (6 services) — [Full Documentation](docs/iot.md)

| Service | Port | Operations |
|---------|------|------------|
| IoT Core | `10117` | 4 |
| IoT Events | `10118` | 4 |
| IoT SiteWise | `10121` | 4 |
| IoT TwinMaker | `10122` | 4 |
| IoT Greengrass | `10120` | 4 |
| IoT FleetWise | `10119` | 4 |

### Application Integration (9 services) — [Full Documentation](docs/application-integration.md)

| Service | Port | Operations |
|---------|------|------------|
| SNS | `9911` | 17 |
| SQS | `9324` | 23 |
| EventBridge | `9195` | 15 |
| AppSync | `9700` | 19 |
| Step Functions | `8083` | 15 |
| SWF | `10115` | 4 |
| MQ | `10113` | 4 |
| MWAA | `10114` | 4 |
| Service Catalog | `9400` | 16 |

### Business Applications (7 services) — [Full Documentation](docs/business.md)

| Service | Port | Operations |
|---------|------|------------|
| Connect | `10124` | 4 |
| Chime | `10123` | 4 |
| WorkDocs | `10126` | 4 |
| WorkMail | `10127` | 4 |
| WorkSpaces | `10152` | 4 |
| Pinpoint | `10125` | 4 |
| SES | `9300` | 5 |

### Media Services (6 services) — [Full Documentation](docs/media.md)

| Service | Port | Operations |
|---------|------|------------|
| MediaConvert | `10134` | 4 |
| MediaLive | `10135` | 4 |
| MediaPackage | `10136` | 4 |
| MediaStore | `10137` | 4 |
| IVS | `10133` | 4 |
| Elastic Transcoder | `10132` | 4 |

### Migration & Transfer (6 services) — [Full Documentation](docs/migration.md)

| Service | Port | Operations |
|---------|------|------------|
| DMS | `10018` | 4 |
| DataSync | `10138` | 4 |
| Transfer Family | `10141` | 4 |
| Migration Hub | `10140` | 4 |
| Mainframe Modernization | `10139` | 4 |
| DRS | `10149` | 4 |

### Cost Management (3 services) — [Full Documentation](docs/cost-management.md)

| Service | Port | Operations |
|---------|------|------------|
| Cost Explorer | `10131` | 4 |
| Budgets | `10130` | 4 |
| Billing Conductor | `10129` | 4 |

### Other Services (11 services) — [Full Documentation](docs/other.md)

| Service | Port | Operations |
|---------|------|------------|
| Amplify | `10154` | 4 |
| AppFlow | `10112` | 4 |
| AppFabric | `10128` | 4 |
| B2BI | `10116` | 4 |
| Braket | `10150` | 4 |
| Q Business | `10108` | 4 |
| DevOps Guru | `10106` | 4 |
| Device Farm | `10155` | 4 |
| Ground Station | `10151` | 4 |
| Location Service | `10153` | 4 |
| Managed Blockchain | `10157` | 4 |

## Running Tests

### Unit Tests

```bash
cargo test
```

### Integration Tests

Integration tests use the AWS CLI to exercise all API operations. Each script builds the binary, starts the server on isolated ports, runs all test cases, and reports pass/fail counts.

```bash
# Run individual service tests
bash tests/s3_integration.sh
bash tests/dynamodb_integration.sh
bash tests/lambda_integration.sh
bash tests/sns_integration.sh
bash tests/sqs_integration.sh

# Run all integration tests
for f in tests/*_integration.sh; do bash "$f"; done
```

See the [tests/](tests/) directory for the full list of 159 integration test scripts.

## Differences from AWS

This is a local development tool, not a production replacement. Key differences:

- **In-memory only** — all state is lost when the server stops. No disk persistence or replication.
- **No authentication** — all requests are accepted without signature verification. Use `--no-sign-request`.
- **No TLS** — the server speaks plain HTTP only.
- **Single-process** — no distributed behavior.
- **S3 versioning** — versioning status can be toggled but version history is not maintained. Only the latest version of each object is stored.
- **SNS subscriptions auto-confirm** — all subscriptions are immediately confirmed without requiring endpoint verification.
- **SNS message delivery** — messages are accepted and assigned IDs but not actually delivered to endpoints.
- **SQS permissions stored but not enforced** — `AddPermission` / `RemovePermission` update the queue's policy, but no access checks are performed.
- **DynamoDB expressions** — basic `KeyConditionExpression`, `UpdateExpression` (SET, REMOVE), `FilterExpression`, and `ProjectionExpression` are supported. Transactions, GSIs/LSIs, and streams are not implemented.
- **Lambda invocation** — `Invoke` returns a stub 200 response. Functions are not actually executed.
- **Firehose delivery** — records are accepted and stored in memory but not delivered to any destination.
- **MemoryDB clusters** — clusters are created with simulated metadata but no actual Redis instances are started.
- **Cognito authentication** — auth flows return stub token responses. No actual JWT signing or token validation is performed.
- **API Gateway invocations** — the service manages REST API configuration but does not route or proxy actual HTTP requests.
- **KMS cryptography is simulated** — Encrypt/Decrypt, Sign/Verify produce deterministic fake outputs. No actual cryptographic operations are performed.
- **Secrets Manager deletion is immediate** — `DeleteSecret` with `--force-delete-without-recovery` removes the secret immediately.
- **EventBridge rules do not evaluate events** — `PutEvents` accepts events but does not match them against rules or invoke targets.
- **Step Functions executions do not run** — `StartExecution` creates an execution in RUNNING state but does not evaluate the state machine definition.
- **SSM SecureString values are stored in plaintext** — no KMS encryption is performed.
- **CloudWatch Logs FilterLogEvents uses substring matching** — not CloudWatch Logs filter syntax.
- **SES emails are not delivered** — `SendEmail` accepts the request but does not deliver email. All identities are auto-verified.
- **Service Catalog provisioning is simulated** — `ProvisionProduct` creates a record but does not deploy CloudFormation stacks.
- **Config recording is simulated** — recorder toggling is tracked but resources are not actually monitored.
- **EFS file systems are immediately available** — no provisioning delay. Mount targets are simulated.
- **AppSync GraphQL APIs are simulated** — APIs are created with synthetic URIs but no actual GraphQL endpoint is running.
- **No CloudWatch metrics** — no metrics integration.
- **Encryption attributes are accepted but not applied** — KMS-related attributes are stored but data is not encrypted.

## CLI Options

| Flag | Default | Description |
|------|---------|-------------|
| `--region` | `us-east-1` | AWS region used in ARNs |
| `--account-id` | `000000000000` | AWS account ID used in ARNs |
| `--<service>-port` | *(see tables above)* | Port for the specified service |

Every service has a `--<service>-port` flag. See the service tables above for default port assignments.

## License

This project is for local development and testing purposes.
