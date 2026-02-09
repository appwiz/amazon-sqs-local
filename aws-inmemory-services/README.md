# aws-inmemory-services

In-memory implementations of Amazon S3 and Amazon SNS, written in Rust. Both services run as a single binary on separate ports, are compatible with the AWS CLI and SDKs, and require no external dependencies.

All state is held in memory — there is no disk persistence. Restarting the server clears all buckets, objects, topics, and subscriptions.

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

S3 listens on `http://0.0.0.0:9000` and SNS on `http://0.0.0.0:9911` by default.

### CLI Options

| Flag | Default | Description |
|------|---------|-------------|
| `--s3-port` | `9000` | Port for the S3 service |
| `--sns-port` | `9911` | Port for the SNS service |
| `--region` | `us-east-1` | AWS region used in ARNs |
| `--account-id` | `000000000000` | AWS account ID used in ARNs |

```bash
./target/release/aws-inmemory-services --s3-port 9000 --sns-port 9911 --region eu-west-1 --account-id 123456789012
```

---

## S3 Service

### Usage with the AWS CLI

Point the AWS CLI at the local endpoint with `--endpoint-url` and skip signature authentication:

```bash
# Create a bucket
aws s3api create-bucket \
  --bucket my-bucket \
  --endpoint-url http://localhost:9000 \
  --no-sign-request

# Upload a file
aws s3api put-object \
  --bucket my-bucket \
  --key hello.txt \
  --body hello.txt \
  --endpoint-url http://localhost:9000 \
  --no-sign-request

# Download a file
aws s3api get-object \
  --bucket my-bucket \
  --key hello.txt \
  output.txt \
  --endpoint-url http://localhost:9000 \
  --no-sign-request

# List objects
aws s3api list-objects-v2 \
  --bucket my-bucket \
  --endpoint-url http://localhost:9000 \
  --no-sign-request

# High-level s3 commands also work
aws s3 cp localfile.txt s3://my-bucket/key \
  --endpoint-url http://localhost:9000 \
  --no-sign-request
```

### Usage with AWS SDKs

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

### Wire Protocol

S3 uses a REST API — the HTTP method and path determine the operation:

- **Bucket operations**: `GET/PUT/DELETE/HEAD /{bucket}`
- **Object operations**: `GET/PUT/DELETE/HEAD /{bucket}/{key}`
- **Query parameters** distinguish sub-resources: `?versioning`, `?tagging`, `?uploads`, `?location`, `?list-type=2`, `?delete`, `?uploadId=X`, `?partNumber=N`
- **Responses**: XML (bucket/list operations) or raw bytes (object data)
- **ETags**: MD5 hex digest in double quotes, e.g. `"d41d8cd98f00b204e9800998ecf8427e"`

### Supported Operations (26)

#### Bucket Operations

| Operation | Method | Path | Query |
|-----------|--------|------|-------|
| CreateBucket | PUT | `/{bucket}` | |
| DeleteBucket | DELETE | `/{bucket}` | |
| HeadBucket | HEAD | `/{bucket}` | |
| ListBuckets | GET | `/` | |
| GetBucketLocation | GET | `/{bucket}` | `?location` |
| GetBucketVersioning | GET | `/{bucket}` | `?versioning` |
| PutBucketVersioning | PUT | `/{bucket}` | `?versioning` |
| GetBucketTagging | GET | `/{bucket}` | `?tagging` |
| PutBucketTagging | PUT | `/{bucket}` | `?tagging` |
| DeleteBucketTagging | DELETE | `/{bucket}` | `?tagging` |

#### Object Operations

| Operation | Method | Path | Query / Header |
|-----------|--------|------|----------------|
| PutObject | PUT | `/{bucket}/{key}` | |
| GetObject | GET | `/{bucket}/{key}` | `Range` header for partial reads |
| DeleteObject | DELETE | `/{bucket}/{key}` | |
| HeadObject | HEAD | `/{bucket}/{key}` | |
| CopyObject | PUT | `/{bucket}/{key}` | `x-amz-copy-source` header |
| ListObjectsV2 | GET | `/{bucket}` | `?list-type=2&prefix=&delimiter=&max-keys=&continuation-token=` |
| DeleteObjects | POST | `/{bucket}` | `?delete` (XML body) |
| GetObjectTagging | GET | `/{bucket}/{key}` | `?tagging` |
| PutObjectTagging | PUT | `/{bucket}/{key}` | `?tagging` |
| DeleteObjectTagging | DELETE | `/{bucket}/{key}` | `?tagging` |

#### Multipart Upload Operations

| Operation | Method | Path | Query |
|-----------|--------|------|-------|
| CreateMultipartUpload | POST | `/{bucket}/{key}` | `?uploads` |
| UploadPart | PUT | `/{bucket}/{key}` | `?partNumber=N&uploadId=X` |
| CompleteMultipartUpload | POST | `/{bucket}/{key}` | `?uploadId=X` |
| AbortMultipartUpload | DELETE | `/{bucket}/{key}` | `?uploadId=X` |
| ListMultipartUploads | GET | `/{bucket}` | `?uploads` |
| ListParts | GET | `/{bucket}/{key}` | `?uploadId=X` |

### S3 Error Response Format

```xml
<Error>
  <Code>NoSuchBucket</Code>
  <Message>The specified bucket does not exist</Message>
</Error>
```

| HTTP Status | Code | Description |
|-------------|------|-------------|
| 200 | | Success |
| 206 | | Partial Content (range request) |
| 400 | MalformedXML | Request body is not valid XML |
| 400 | InvalidArgument | Invalid parameter |
| 404 | NoSuchBucket | Bucket does not exist |
| 404 | NoSuchKey | Object does not exist |
| 404 | NoSuchUpload | Multipart upload does not exist |
| 409 | BucketAlreadyOwnedByYou | Bucket already exists |
| 409 | BucketNotEmpty | Bucket has objects |

---

## SNS Service

### Usage with the AWS CLI

```bash
# Create a topic
aws sns create-topic \
  --name my-topic \
  --endpoint-url http://localhost:9911 \
  --no-sign-request

# Create a FIFO topic
aws sns create-topic \
  --name my-fifo.fifo \
  --attributes FifoTopic=true,ContentBasedDeduplication=true \
  --endpoint-url http://localhost:9911 \
  --no-sign-request

# Subscribe
aws sns subscribe \
  --topic-arn arn:aws:sns:us-east-1:000000000000:my-topic \
  --protocol email \
  --notification-endpoint test@example.com \
  --endpoint-url http://localhost:9911 \
  --no-sign-request

# Publish a message
aws sns publish \
  --topic-arn arn:aws:sns:us-east-1:000000000000:my-topic \
  --message "Hello from SNS!" \
  --endpoint-url http://localhost:9911 \
  --no-sign-request

# List topics
aws sns list-topics \
  --endpoint-url http://localhost:9911 \
  --no-sign-request

# Tag a topic
aws sns tag-resource \
  --resource-arn arn:aws:sns:us-east-1:000000000000:my-topic \
  --tags Key=Environment,Value=test \
  --endpoint-url http://localhost:9911 \
  --no-sign-request

# Delete a topic
aws sns delete-topic \
  --topic-arn arn:aws:sns:us-east-1:000000000000:my-topic \
  --endpoint-url http://localhost:9911 \
  --no-sign-request
```

### Usage with AWS SDKs

```javascript
import { SNSClient, CreateTopicCommand, PublishCommand } from "@aws-sdk/client-sns";

const client = new SNSClient({
  endpoint: "http://localhost:9911",
  region: "us-east-1",
  credentials: { accessKeyId: "test", secretAccessKey: "test" },
});

const { TopicArn } = await client.send(new CreateTopicCommand({
  Name: "my-topic",
}));

await client.send(new PublishCommand({
  TopicArn,
  Message: "Hello, world!",
}));
```

### Wire Protocol

SNS uses the **AWS Query** protocol over HTTP POST:

- **Content-Type**: `application/x-www-form-urlencoded`
- **Action routing**: `Action=<ActionName>` form parameter
- **Request body**: URL-encoded form parameters
- **Response body**: XML with `xmlns="http://sns.amazonaws.com/doc/2010-03-31/"`
- **Endpoint**: `http://localhost:<port>/`

### Supported Operations (17)

#### Topic Management

| Operation | Form Parameters |
|-----------|----------------|
| CreateTopic | `Name`, `Attributes.entry.N.key/value`, `Tags.member.N.Key/Value` |
| DeleteTopic | `TopicArn` |
| ListTopics | `NextToken` (optional) |
| GetTopicAttributes | `TopicArn` |
| SetTopicAttributes | `TopicArn`, `AttributeName`, `AttributeValue` |

#### Subscriptions

| Operation | Form Parameters |
|-----------|----------------|
| Subscribe | `TopicArn`, `Protocol`, `Endpoint`, `ReturnSubscriptionArn` |
| Unsubscribe | `SubscriptionArn` |
| ConfirmSubscription | `TopicArn`, `Token` |
| ListSubscriptions | `NextToken` (optional) |
| ListSubscriptionsByTopic | `TopicArn`, `NextToken` (optional) |
| GetSubscriptionAttributes | `SubscriptionArn` |
| SetSubscriptionAttributes | `SubscriptionArn`, `AttributeName`, `AttributeValue` |

#### Publishing

| Operation | Form Parameters |
|-----------|----------------|
| Publish | `TopicArn`, `Message`, `Subject`, `MessageGroupId`, `MessageDeduplicationId` |
| PublishBatch | `TopicArn`, `PublishBatchRequestEntries.member.N.Id/Message/...` |

#### Tagging

| Operation | Form Parameters |
|-----------|----------------|
| TagResource | `ResourceArn`, `Tags.member.N.Key/Value` |
| UntagResource | `ResourceArn`, `TagKeys.member.N` |
| ListTagsForResource | `ResourceArn` |

### Topic Attributes

| Attribute | Description |
|-----------|-------------|
| `TopicArn` | The ARN of the topic (read-only) |
| `Owner` | Account ID of the topic owner (read-only) |
| `DisplayName` | Display name for the topic |
| `Policy` | Access policy |
| `DeliveryPolicy` | Delivery retry policy |
| `KmsMasterKeyId` | KMS key for encryption |
| `FifoTopic` | Whether the topic is FIFO (set at creation) |
| `ContentBasedDeduplication` | Deduplication using message body hash (FIFO only) |
| `SubscriptionsConfirmed` | Count of confirmed subscriptions (read-only) |
| `SubscriptionsPending` | Count of pending subscriptions (read-only) |

### Subscription Attributes

| Attribute | Description |
|-----------|-------------|
| `SubscriptionArn` | ARN of the subscription (read-only) |
| `TopicArn` | ARN of the topic (read-only) |
| `Protocol` | Subscription protocol (read-only) |
| `Endpoint` | Subscription endpoint (read-only) |
| `Owner` | Account ID (read-only) |
| `RawMessageDelivery` | Deliver raw message without JSON wrapping |
| `FilterPolicy` | JSON filter policy for message filtering |
| `FilterPolicyScope` | Scope of filter policy (`MessageAttributes` or `MessageBody`) |
| `RedrivePolicy` | Dead-letter queue configuration |
| `PendingConfirmation` | Whether the subscription is pending confirmation (read-only) |
| `ConfirmationWasAuthenticated` | Whether the confirmation was authenticated (read-only) |

### SNS Error Response Format

```xml
<ErrorResponse xmlns="http://sns.amazonaws.com/doc/2010-03-31/">
  <Error>
    <Type>Sender</Type>
    <Code>NotFound</Code>
    <Message>Topic does not exist</Message>
  </Error>
  <RequestId>uuid</RequestId>
</ErrorResponse>
```

| HTTP Status | Code | Description |
|-------------|------|-------------|
| 200 | | Success |
| 400 | InvalidParameter | Invalid or missing parameter |
| 400 | InvalidAction | Unknown action |
| 404 | NotFound | Resource does not exist |

---

## Running Tests

The integration test suites use the AWS CLI to exercise all API operations:

```bash
# Run S3 tests (49 assertions)
bash tests/s3_integration.sh

# Run SNS tests (42 assertions)
bash tests/sns_integration.sh
```

Each script builds the binary, starts the server, runs all test cases, and reports pass/fail counts.

---

## Differences from AWS

This is a local development tool, not a production replacement. Key differences:

- **In-memory only** — all state is lost when the server stops. No disk persistence or replication.
- **No authentication** — all requests are accepted without signature verification. Use `--no-sign-request`.
- **No TLS** — the server speaks plain HTTP only.
- **Single-process** — no distributed behavior.
- **S3 versioning** — versioning status can be toggled but version history is not maintained. Only the latest version of each object is stored.
- **SNS subscriptions auto-confirm** — all subscriptions are immediately confirmed without requiring endpoint verification.
- **SNS message delivery** — messages are accepted and assigned IDs but not actually delivered to endpoints. Use this service for API compatibility testing, not delivery testing.
- **No CloudWatch metrics** — no metrics integration.
- **Encryption attributes are accepted but not applied** — KMS-related attributes are stored but data is not encrypted.
- **Upload size limit** — S3 supports uploads up to 5 GB per request (axum body limit).

## References

### AWS S3
- [Developer Guide](https://docs.aws.amazon.com/AmazonS3/latest/userguide/s3-userguide.pdf)
- [API Reference](https://docs.aws.amazon.com/AmazonS3/latest/API/s3-api.pdf)

### Amazon SNS
- [Developer Guide](https://docs.aws.amazon.com/sns/latest/dg/sns-dg.pdf)
- [API Reference](https://docs.aws.amazon.com/sns/latest/api/sns-api.pdf)
