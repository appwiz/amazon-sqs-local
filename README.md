# aws-inmemory-services

In-memory implementations of seventeen AWS services written in Rust: Amazon S3, Amazon SNS, Amazon SQS, Amazon DynamoDB, AWS Lambda, Amazon Data Firehose, Amazon MemoryDB, Amazon Cognito, Amazon API Gateway, AWS KMS, AWS Secrets Manager, Amazon Kinesis Data Streams, Amazon EventBridge, AWS Step Functions, AWS Systems Manager Parameter Store, Amazon CloudWatch Logs, and Amazon SES. All services run as a single binary on separate ports, are compatible with the AWS CLI and SDKs, and require no external dependencies.

All state is held in memory — there is no disk persistence. Restarting the server clears all data.

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

All services start on their default ports:

| Service | Default Port |
|---------|-------------|
| S3 | `9000` |
| SNS | `9911` |
| SQS | `9324` |
| DynamoDB | `8000` |
| Lambda | `9001` |
| Firehose | `4573` |
| MemoryDB | `6379` |
| Cognito | `9229` |
| API Gateway | `4567` |
| KMS | `7600` |
| Secrets Manager | `7700` |
| Kinesis Data Streams | `4568` |
| EventBridge | `9195` |
| Step Functions | `8083` |
| SSM Parameter Store | `9100` |
| CloudWatch Logs | `9201` |
| SES | `9300` |

### CLI Options

| Flag | Default | Description |
|------|---------|-------------|
| `--s3-port` | `9000` | Port for the S3 service |
| `--sns-port` | `9911` | Port for the SNS service |
| `--sqs-port` | `9324` | Port for the SQS service |
| `--dynamodb-port` | `8000` | Port for the DynamoDB service |
| `--lambda-port` | `9001` | Port for the Lambda service |
| `--firehose-port` | `4573` | Port for the Firehose service |
| `--memorydb-port` | `6379` | Port for the MemoryDB service |
| `--cognito-port` | `9229` | Port for the Cognito service |
| `--apigateway-port` | `4567` | Port for the API Gateway service |
| `--kms-port` | `7600` | Port for the KMS service |
| `--secretsmanager-port` | `7700` | Port for the Secrets Manager service |
| `--kinesis-port` | `4568` | Port for the Kinesis Data Streams service |
| `--eventbridge-port` | `9195` | Port for the EventBridge service |
| `--stepfunctions-port` | `8083` | Port for the Step Functions service |
| `--ssm-port` | `9100` | Port for the SSM Parameter Store service |
| `--cloudwatchlogs-port` | `9201` | Port for the CloudWatch Logs service |
| `--ses-port` | `9300` | Port for the SES service |
| `--region` | `us-east-1` | AWS region used in ARNs |
| `--account-id` | `000000000000` | AWS account ID used in ARNs |

```bash
./target/release/aws-inmemory-services --region eu-west-1 --account-id 123456789012
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

## SQS Service

### Usage with the AWS CLI

```bash
# Create a queue
aws sqs create-queue \
  --queue-name my-queue \
  --endpoint-url http://localhost:9324 \
  --no-sign-request

# Send a message
aws sqs send-message \
  --queue-url http://localhost:9324/000000000000/my-queue \
  --message-body "Hello, world!" \
  --endpoint-url http://localhost:9324 \
  --no-sign-request

# Receive messages
aws sqs receive-message \
  --queue-url http://localhost:9324/000000000000/my-queue \
  --endpoint-url http://localhost:9324 \
  --no-sign-request

# Create a FIFO queue
aws sqs create-queue \
  --queue-name my-fifo.fifo \
  --attributes FifoQueue=true,ContentBasedDeduplication=true \
  --endpoint-url http://localhost:9324 \
  --no-sign-request

# Delete a queue
aws sqs delete-queue \
  --queue-url http://localhost:9324/000000000000/my-queue \
  --endpoint-url http://localhost:9324 \
  --no-sign-request
```

### Usage with AWS SDKs

```javascript
import { SQSClient, SendMessageCommand } from "@aws-sdk/client-sqs";

const client = new SQSClient({
  endpoint: "http://localhost:9324",
  region: "us-east-1",
  credentials: { accessKeyId: "test", secretAccessKey: "test" },
});

await client.send(new SendMessageCommand({
  QueueUrl: "http://localhost:9324/000000000000/my-queue",
  MessageBody: "Hello, world!",
}));
```

### Wire Protocol

SQS uses the **AWS JSON 1.0** protocol over HTTP POST:

- **Content-Type**: `application/x-amz-json-1.0`
- **Action routing**: `X-Amz-Target: AmazonSQS.<ActionName>` header
- **Request/response body**: JSON
- **Endpoint**: `http://localhost:<port>/`

### Supported Operations (23)

#### Queue Management

| Operation | Target | Description |
|-----------|--------|-------------|
| CreateQueue | `AmazonSQS.CreateQueue` | Create a standard or FIFO queue |
| DeleteQueue | `AmazonSQS.DeleteQueue` | Delete a queue and all its messages |
| GetQueueUrl | `AmazonSQS.GetQueueUrl` | Look up a queue URL by name |
| ListQueues | `AmazonSQS.ListQueues` | List queues with optional prefix filter |
| GetQueueAttributes | `AmazonSQS.GetQueueAttributes` | Retrieve queue attributes |
| SetQueueAttributes | `AmazonSQS.SetQueueAttributes` | Modify queue attributes |
| PurgeQueue | `AmazonSQS.PurgeQueue` | Delete all messages without deleting the queue |

#### Messaging

| Operation | Target | Description |
|-----------|--------|-------------|
| SendMessage | `AmazonSQS.SendMessage` | Send a single message |
| SendMessageBatch | `AmazonSQS.SendMessageBatch` | Send up to 10 messages |
| ReceiveMessage | `AmazonSQS.ReceiveMessage` | Receive messages with long polling support |
| DeleteMessage | `AmazonSQS.DeleteMessage` | Delete a processed message |
| DeleteMessageBatch | `AmazonSQS.DeleteMessageBatch` | Delete up to 10 messages |
| ChangeMessageVisibility | `AmazonSQS.ChangeMessageVisibility` | Extend/shorten visibility timeout |
| ChangeMessageVisibilityBatch | `AmazonSQS.ChangeMessageVisibilityBatch` | Change visibility for up to 10 messages |

#### Tags and Permissions

| Operation | Target | Description |
|-----------|--------|-------------|
| TagQueue | `AmazonSQS.TagQueue` | Add or update queue tags |
| UntagQueue | `AmazonSQS.UntagQueue` | Remove queue tags |
| ListQueueTags | `AmazonSQS.ListQueueTags` | List all tags on a queue |
| AddPermission | `AmazonSQS.AddPermission` | Add a permission statement |
| RemovePermission | `AmazonSQS.RemovePermission` | Remove a permission statement |

#### Dead-Letter Queues and Message Move Tasks

| Operation | Target | Description |
|-----------|--------|-------------|
| ListDeadLetterSourceQueues | `AmazonSQS.ListDeadLetterSourceQueues` | List queues using this queue as DLQ |
| StartMessageMoveTask | `AmazonSQS.StartMessageMoveTask` | Move messages between queues |
| CancelMessageMoveTask | `AmazonSQS.CancelMessageMoveTask` | Cancel an in-progress move task |
| ListMessageMoveTasks | `AmazonSQS.ListMessageMoveTasks` | List move tasks for a source queue |

### Queue Types

**Standard Queues**: At-least-once delivery, best-effort ordering, unlimited throughput.

**FIFO Queues**: Exactly-once processing within a 5-minute deduplication window, strict ordering within message groups. Queue names must end with `.fifo`. Require `MessageGroupId` on every send. Support optional `ContentBasedDeduplication`.

### Queue Attributes

| Attribute | Default | Range | Notes |
|-----------|---------|-------|-------|
| `VisibilityTimeout` | 30s | 0--43200 | Time a received message is hidden |
| `MessageRetentionPeriod` | 345600s (4d) | 60--1209600 | How long messages are retained |
| `DelaySeconds` | 0 | 0--900 | Default delivery delay |
| `MaximumMessageSize` | 262144 (256KB) | 1024--262144 | Maximum message body size |
| `ReceiveMessageWaitTimeSeconds` | 0 | 0--20 | Default long-poll wait time |
| `RedrivePolicy` | none | -- | DLQ config (JSON: `deadLetterTargetArn`, `maxReceiveCount`) |
| `RedriveAllowPolicy` | none | -- | Controls which queues can use this as DLQ |
| `FifoQueue` | false | -- | Immutable after creation |
| `ContentBasedDeduplication` | false | -- | FIFO only. SHA-256 body hash as dedup ID |
| `SqsManagedSseEnabled` | true | -- | SSE with SQS-managed keys (stored, not enforced) |

### SQS Error Response Format

```json
{
  "__type": "com.amazonaws.sqs#QueueDoesNotExist",
  "message": "The specified queue does not exist."
}
```

| HTTP Status | Meaning |
|-------------|---------|
| 200 | Success |
| 400 | Client error (invalid parameters, missing fields) |
| 404 | Resource not found |
| 409 | Conflict (queue exists with different attributes, purge in progress) |

---

## DynamoDB Service

### Usage with the AWS CLI

```bash
# Create a table
aws dynamodb create-table \
  --table-name MyTable \
  --attribute-definitions AttributeName=pk,AttributeType=S \
  --key-schema AttributeName=pk,KeyType=HASH \
  --billing-mode PAY_PER_REQUEST \
  --endpoint-url http://localhost:8000 \
  --no-sign-request

# Put an item
aws dynamodb put-item \
  --table-name MyTable \
  --item '{"pk":{"S":"key1"},"data":{"S":"value1"}}' \
  --endpoint-url http://localhost:8000 \
  --no-sign-request

# Get an item
aws dynamodb get-item \
  --table-name MyTable \
  --key '{"pk":{"S":"key1"}}' \
  --endpoint-url http://localhost:8000 \
  --no-sign-request

# Query
aws dynamodb query \
  --table-name MyTable \
  --key-condition-expression "pk = :pk" \
  --expression-attribute-values '{":pk":{"S":"key1"}}' \
  --endpoint-url http://localhost:8000 \
  --no-sign-request

# Scan
aws dynamodb scan \
  --table-name MyTable \
  --endpoint-url http://localhost:8000 \
  --no-sign-request
```

### Usage with AWS SDKs

```javascript
import { DynamoDBClient, PutItemCommand, GetItemCommand } from "@aws-sdk/client-dynamodb";

const client = new DynamoDBClient({
  endpoint: "http://localhost:8000",
  region: "us-east-1",
  credentials: { accessKeyId: "test", secretAccessKey: "test" },
});

await client.send(new PutItemCommand({
  TableName: "MyTable",
  Item: { pk: { S: "key1" }, data: { S: "value1" } },
}));

const { Item } = await client.send(new GetItemCommand({
  TableName: "MyTable",
  Key: { pk: { S: "key1" } },
}));
```

### Wire Protocol

DynamoDB uses the **AWS JSON 1.0** protocol over HTTP POST:

- **Content-Type**: `application/x-amz-json-1.0`
- **Action routing**: `X-Amz-Target: DynamoDB_20120810.<ActionName>` header
- **Request/response body**: JSON
- **Endpoint**: `http://localhost:<port>/`

### Supported Operations (16)

#### Table Management

| Operation | Target | Description |
|-----------|--------|-------------|
| CreateTable | `DynamoDB_20120810.CreateTable` | Create a table with hash key or hash+range key |
| DeleteTable | `DynamoDB_20120810.DeleteTable` | Delete a table and all its items |
| DescribeTable | `DynamoDB_20120810.DescribeTable` | Get table metadata |
| ListTables | `DynamoDB_20120810.ListTables` | List all tables |
| UpdateTable | `DynamoDB_20120810.UpdateTable` | Update billing mode or provisioned throughput |

#### Item Operations

| Operation | Target | Description |
|-----------|--------|-------------|
| PutItem | `DynamoDB_20120810.PutItem` | Create or replace an item |
| GetItem | `DynamoDB_20120810.GetItem` | Retrieve an item by primary key |
| DeleteItem | `DynamoDB_20120810.DeleteItem` | Delete an item by primary key |
| UpdateItem | `DynamoDB_20120810.UpdateItem` | Update specific attributes with expressions |
| Query | `DynamoDB_20120810.Query` | Query items by key condition expression |
| Scan | `DynamoDB_20120810.Scan` | Scan all items with optional filter |
| BatchGetItem | `DynamoDB_20120810.BatchGetItem` | Get multiple items across tables |
| BatchWriteItem | `DynamoDB_20120810.BatchWriteItem` | Put or delete multiple items across tables |

#### Tagging

| Operation | Target | Description |
|-----------|--------|-------------|
| TagResource | `DynamoDB_20120810.TagResource` | Add tags to a table |
| UntagResource | `DynamoDB_20120810.UntagResource` | Remove tags from a table |
| ListTagsOfResource | `DynamoDB_20120810.ListTagsOfResource` | List tags on a table |

### DynamoDB Error Response Format

```json
{
  "__type": "com.amazonaws.dynamodb.v20120810#ResourceNotFoundException",
  "message": "Requested resource not found"
}
```

| HTTP Status | Code | Description |
|-------------|------|-------------|
| 200 | | Success |
| 400 | ResourceInUseException | Table already exists |
| 400 | ValidationException | Invalid parameters |
| 400 | SerializationException | Malformed request |
| 404 | ResourceNotFoundException | Table not found |

---

## Lambda Service

### Usage with the AWS CLI

```bash
# Create a function (with a dummy zip)
aws lambda create-function \
  --function-name my-func \
  --runtime python3.12 \
  --role arn:aws:iam::000000000000:role/test-role \
  --handler index.handler \
  --zip-file fileb://function.zip \
  --endpoint-url http://localhost:9001 \
  --no-sign-request

# Invoke a function
aws lambda invoke \
  --function-name my-func \
  output.json \
  --endpoint-url http://localhost:9001 \
  --no-sign-request

# List functions
aws lambda list-functions \
  --endpoint-url http://localhost:9001 \
  --no-sign-request

# Publish a version
aws lambda publish-version \
  --function-name my-func \
  --endpoint-url http://localhost:9001 \
  --no-sign-request

# Create an alias
aws lambda create-alias \
  --function-name my-func \
  --name prod \
  --function-version 1 \
  --endpoint-url http://localhost:9001 \
  --no-sign-request

# Delete a function
aws lambda delete-function \
  --function-name my-func \
  --endpoint-url http://localhost:9001 \
  --no-sign-request
```

### Usage with AWS SDKs

```javascript
import { LambdaClient, InvokeCommand } from "@aws-sdk/client-lambda";

const client = new LambdaClient({
  endpoint: "http://localhost:9001",
  region: "us-east-1",
  credentials: { accessKeyId: "test", secretAccessKey: "test" },
});

const response = await client.send(new InvokeCommand({
  FunctionName: "my-func",
}));
```

### Wire Protocol

Lambda uses a **REST API** with JSON — the HTTP method and path determine the operation:

- **Functions**: `GET/POST /2015-03-31/functions`, `GET/DELETE /2015-03-31/functions/{name}`
- **Code/Config**: `PUT /2015-03-31/functions/{name}/code`, `PUT /2015-03-31/functions/{name}/configuration`
- **Invoke**: `POST /2015-03-31/functions/{name}/invocations`
- **Versions**: `GET/POST /2015-03-31/functions/{name}/versions`
- **Aliases**: `GET/POST /2015-03-31/functions/{name}/aliases`
- **Policy**: `GET/POST /2015-03-31/functions/{name}/policy`
- **Tags**: `GET/POST/DELETE /2017-03-31/tags/{arn}`
- **Event Source Mappings**: `GET/POST /2015-03-31/event-source-mappings`

### Supported Operations (22)

#### Function Management

| Operation | Method | Path |
|-----------|--------|------|
| CreateFunction | POST | `/2015-03-31/functions` |
| GetFunction | GET | `/2015-03-31/functions/{name}` |
| DeleteFunction | DELETE | `/2015-03-31/functions/{name}` |
| ListFunctions | GET | `/2015-03-31/functions` |
| UpdateFunctionCode | PUT | `/2015-03-31/functions/{name}/code` |
| UpdateFunctionConfiguration | PUT | `/2015-03-31/functions/{name}/configuration` |

#### Invocation

| Operation | Method | Path |
|-----------|--------|------|
| Invoke | POST | `/2015-03-31/functions/{name}/invocations` |

#### Versions and Aliases

| Operation | Method | Path |
|-----------|--------|------|
| PublishVersion | POST | `/2015-03-31/functions/{name}/versions` |
| ListVersionsByFunction | GET | `/2015-03-31/functions/{name}/versions` |
| CreateAlias | POST | `/2015-03-31/functions/{name}/aliases` |
| GetAlias | GET | `/2015-03-31/functions/{name}/aliases/{alias}` |
| DeleteAlias | DELETE | `/2015-03-31/functions/{name}/aliases/{alias}` |
| ListAliases | GET | `/2015-03-31/functions/{name}/aliases` |

#### Permissions

| Operation | Method | Path |
|-----------|--------|------|
| AddPermission | POST | `/2015-03-31/functions/{name}/policy` |
| RemovePermission | DELETE | `/2015-03-31/functions/{name}/policy/{sid}` |
| GetPolicy | GET | `/2015-03-31/functions/{name}/policy` |

#### Event Source Mappings

| Operation | Method | Path |
|-----------|--------|------|
| CreateEventSourceMapping | POST | `/2015-03-31/event-source-mappings` |
| DeleteEventSourceMapping | DELETE | `/2015-03-31/event-source-mappings/{uuid}` |
| ListEventSourceMappings | GET | `/2015-03-31/event-source-mappings` |

#### Tagging

| Operation | Method | Path |
|-----------|--------|------|
| TagResource | POST | `/2017-03-31/tags/{arn}` |
| UntagResource | DELETE | `/2017-03-31/tags/{arn}` |
| ListTags | GET | `/2017-03-31/tags/{arn}` |

### Lambda Error Response Format

```json
{
  "Message": "Function not found: arn:aws:lambda:us-east-1:000000000000:function:my-func"
}
```

Response includes `x-amzn-ErrorType` header (e.g. `ResourceNotFoundException`).

| HTTP Status | Error Type | Description |
|-------------|------------|-------------|
| 200/202 | | Success |
| 400 | InvalidParameterValueException | Invalid parameters |
| 404 | ResourceNotFoundException | Function or resource not found |
| 409 | ResourceConflictException | Function already exists |

---

## Firehose Service

### Usage with the AWS CLI

```bash
# Create a delivery stream
aws firehose create-delivery-stream \
  --delivery-stream-name mystream \
  --endpoint-url http://localhost:4573 \
  --no-sign-request

# Put a record (base64-encoded data)
aws firehose put-record \
  --delivery-stream-name mystream \
  --record '{"Data":"SGVsbG8gV29ybGQ="}' \
  --endpoint-url http://localhost:4573 \
  --no-sign-request

# Put a batch of records
aws firehose put-record-batch \
  --delivery-stream-name mystream \
  --records '{"Data":"UmVjb3JkMQ=="}' '{"Data":"UmVjb3JkMg=="}' \
  --endpoint-url http://localhost:4573 \
  --no-sign-request

# Describe a delivery stream
aws firehose describe-delivery-stream \
  --delivery-stream-name mystream \
  --endpoint-url http://localhost:4573 \
  --no-sign-request

# List delivery streams
aws firehose list-delivery-streams \
  --endpoint-url http://localhost:4573 \
  --no-sign-request

# Delete a delivery stream
aws firehose delete-delivery-stream \
  --delivery-stream-name mystream \
  --endpoint-url http://localhost:4573 \
  --no-sign-request
```

### Usage with AWS SDKs

```javascript
import { FirehoseClient, PutRecordCommand } from "@aws-sdk/client-firehose";

const client = new FirehoseClient({
  endpoint: "http://localhost:4573",
  region: "us-east-1",
  credentials: { accessKeyId: "test", secretAccessKey: "test" },
});

await client.send(new PutRecordCommand({
  DeliveryStreamName: "mystream",
  Record: { Data: Buffer.from("Hello World") },
}));
```

### Wire Protocol

Firehose uses the **AWS JSON 1.1** protocol over HTTP POST:

- **Content-Type**: `application/x-amz-json-1.1`
- **Action routing**: `X-Amz-Target: Firehose_20150804.<ActionName>` header
- **Request/response body**: JSON
- **Endpoint**: `http://localhost:<port>/`

### Supported Operations (10)

#### Stream Management

| Operation | Target | Description |
|-----------|--------|-------------|
| CreateDeliveryStream | `Firehose_20150804.CreateDeliveryStream` | Create a delivery stream |
| DeleteDeliveryStream | `Firehose_20150804.DeleteDeliveryStream` | Delete a delivery stream |
| DescribeDeliveryStream | `Firehose_20150804.DescribeDeliveryStream` | Get stream metadata and status |
| ListDeliveryStreams | `Firehose_20150804.ListDeliveryStreams` | List all delivery streams |
| UpdateDestination | `Firehose_20150804.UpdateDestination` | Update stream destination config |

#### Data Ingestion

| Operation | Target | Description |
|-----------|--------|-------------|
| PutRecord | `Firehose_20150804.PutRecord` | Put a single record |
| PutRecordBatch | `Firehose_20150804.PutRecordBatch` | Put multiple records (up to 500) |

#### Tagging

| Operation | Target | Description |
|-----------|--------|-------------|
| TagDeliveryStream | `Firehose_20150804.TagDeliveryStream` | Add tags to a stream |
| UntagDeliveryStream | `Firehose_20150804.UntagDeliveryStream` | Remove tags from a stream |
| ListTagsForDeliveryStream | `Firehose_20150804.ListTagsForDeliveryStream` | List tags on a stream |

### Firehose Error Response Format

```json
{
  "__type": "#ResourceNotFoundException",
  "message": "Delivery stream mystream under account 000000000000 not found."
}
```

| HTTP Status | Code | Description |
|-------------|------|-------------|
| 200 | | Success |
| 400 | InvalidArgumentException | Invalid parameters |
| 400 | ResourceInUseException | Stream already exists |
| 404 | ResourceNotFoundException | Stream not found |

---

## MemoryDB Service

### Usage with the AWS CLI

```bash
# Create a user
aws memorydb create-user \
  --user-name myuser \
  --access-string "on ~* +@all" \
  --authentication-mode Type=no-password \
  --endpoint-url http://localhost:6379 \
  --no-sign-request

# Create an ACL
aws memorydb create-acl \
  --acl-name myacl \
  --user-names myuser \
  --endpoint-url http://localhost:6379 \
  --no-sign-request

# Create a cluster
aws memorydb create-cluster \
  --cluster-name mycluster \
  --node-type db.t4g.small \
  --acl-name myacl \
  --endpoint-url http://localhost:6379 \
  --no-sign-request

# Describe clusters
aws memorydb describe-clusters \
  --endpoint-url http://localhost:6379 \
  --no-sign-request

# Create a snapshot
aws memorydb create-snapshot \
  --cluster-name mycluster \
  --snapshot-name mysnap \
  --endpoint-url http://localhost:6379 \
  --no-sign-request

# Delete a cluster
aws memorydb delete-cluster \
  --cluster-name mycluster \
  --endpoint-url http://localhost:6379 \
  --no-sign-request
```

### Usage with AWS SDKs

```javascript
import { MemoryDBClient, CreateClusterCommand } from "@aws-sdk/client-memorydb";

const client = new MemoryDBClient({
  endpoint: "http://localhost:6379",
  region: "us-east-1",
  credentials: { accessKeyId: "test", secretAccessKey: "test" },
});

await client.send(new CreateClusterCommand({
  ClusterName: "mycluster",
  NodeType: "db.t4g.small",
  ACLName: "myacl",
}));
```

### Wire Protocol

MemoryDB uses the **AWS JSON 1.1** protocol over HTTP POST:

- **Content-Type**: `application/x-amz-json-1.1`
- **Action routing**: `X-Amz-Target: AmazonMemoryDB.<ActionName>` header
- **Request/response body**: JSON
- **Endpoint**: `http://localhost:<port>/`

### Supported Operations (21)

#### Cluster Management

| Operation | Target | Description |
|-----------|--------|-------------|
| CreateCluster | `AmazonMemoryDB.CreateCluster` | Create a cluster |
| DeleteCluster | `AmazonMemoryDB.DeleteCluster` | Delete a cluster |
| DescribeClusters | `AmazonMemoryDB.DescribeClusters` | Describe one or all clusters |
| UpdateCluster | `AmazonMemoryDB.UpdateCluster` | Update cluster configuration |

#### Subnet Groups

| Operation | Target | Description |
|-----------|--------|-------------|
| CreateSubnetGroup | `AmazonMemoryDB.CreateSubnetGroup` | Create a subnet group |
| DeleteSubnetGroup | `AmazonMemoryDB.DeleteSubnetGroup` | Delete a subnet group |
| DescribeSubnetGroups | `AmazonMemoryDB.DescribeSubnetGroups` | Describe subnet groups |

#### Users

| Operation | Target | Description |
|-----------|--------|-------------|
| CreateUser | `AmazonMemoryDB.CreateUser` | Create a user |
| DeleteUser | `AmazonMemoryDB.DeleteUser` | Delete a user |
| DescribeUsers | `AmazonMemoryDB.DescribeUsers` | Describe users |
| UpdateUser | `AmazonMemoryDB.UpdateUser` | Update a user's access string |

#### ACLs

| Operation | Target | Description |
|-----------|--------|-------------|
| CreateACL | `AmazonMemoryDB.CreateACL` | Create an access control list |
| DeleteACL | `AmazonMemoryDB.DeleteACL` | Delete an ACL |
| DescribeACLs | `AmazonMemoryDB.DescribeACLs` | Describe ACLs |
| UpdateACL | `AmazonMemoryDB.UpdateACL` | Add or remove users from an ACL |

#### Snapshots

| Operation | Target | Description |
|-----------|--------|-------------|
| CreateSnapshot | `AmazonMemoryDB.CreateSnapshot` | Create a snapshot of a cluster |
| DeleteSnapshot | `AmazonMemoryDB.DeleteSnapshot` | Delete a snapshot |
| DescribeSnapshots | `AmazonMemoryDB.DescribeSnapshots` | Describe snapshots |

#### Tagging

| Operation | Target | Description |
|-----------|--------|-------------|
| TagResource | `AmazonMemoryDB.TagResource` | Add tags to a resource |
| UntagResource | `AmazonMemoryDB.UntagResource` | Remove tags from a resource |
| ListTags | `AmazonMemoryDB.ListTags` | List tags on a resource |

### MemoryDB Error Response Format

```json
{
  "__type": "ClusterNotFoundFault",
  "message": "Cluster mycluster not found"
}
```

| HTTP Status | Code | Description |
|-------------|------|-------------|
| 200 | | Success |
| 400 | ClusterAlreadyExistsFault | Cluster already exists |
| 400 | UserAlreadyExistsFault | User already exists |
| 400 | ACLAlreadyExistsFault | ACL already exists |
| 400 | InvalidParameterValue | Invalid parameters |
| 404 | ClusterNotFoundFault | Cluster not found |
| 404 | UserNotFoundFault | User not found |
| 404 | ACLNotFoundFault | ACL not found |

---

## Cognito Service

### Usage with the AWS CLI

```bash
# Create a user pool
aws cognito-idp create-user-pool \
  --pool-name mypool \
  --endpoint-url http://localhost:9229 \
  --no-sign-request

# Create a user pool client
aws cognito-idp create-user-pool-client \
  --user-pool-id us-east-1_abc123 \
  --client-name myclient \
  --endpoint-url http://localhost:9229 \
  --no-sign-request

# Create a user
aws cognito-idp admin-create-user \
  --user-pool-id us-east-1_abc123 \
  --username testuser \
  --endpoint-url http://localhost:9229 \
  --no-sign-request

# List users
aws cognito-idp list-users \
  --user-pool-id us-east-1_abc123 \
  --endpoint-url http://localhost:9229 \
  --no-sign-request

# Create a group
aws cognito-idp create-group \
  --user-pool-id us-east-1_abc123 \
  --group-name mygroup \
  --endpoint-url http://localhost:9229 \
  --no-sign-request

# Add user to group
aws cognito-idp admin-add-user-to-group \
  --user-pool-id us-east-1_abc123 \
  --username testuser \
  --group-name mygroup \
  --endpoint-url http://localhost:9229 \
  --no-sign-request

# Initiate auth
aws cognito-idp initiate-auth \
  --auth-flow USER_PASSWORD_AUTH \
  --auth-parameters USERNAME=testuser,PASSWORD=Test1234! \
  --client-id <client-id> \
  --endpoint-url http://localhost:9229 \
  --no-sign-request
```

### Usage with AWS SDKs

```javascript
import { CognitoIdentityProviderClient, CreateUserPoolCommand, AdminCreateUserCommand } from "@aws-sdk/client-cognito-identity-provider";

const client = new CognitoIdentityProviderClient({
  endpoint: "http://localhost:9229",
  region: "us-east-1",
  credentials: { accessKeyId: "test", secretAccessKey: "test" },
});

const { UserPool } = await client.send(new CreateUserPoolCommand({
  PoolName: "mypool",
}));

await client.send(new AdminCreateUserCommand({
  UserPoolId: UserPool.Id,
  Username: "testuser",
}));
```

### Wire Protocol

Cognito uses the **AWS JSON 1.1** protocol over HTTP POST:

- **Content-Type**: `application/x-amz-json-1.1`
- **Action routing**: `X-Amz-Target: AWSCognitoIdentityProviderService.<ActionName>` header
- **Request/response body**: JSON
- **Endpoint**: `http://localhost:<port>/`

### Supported Operations (33)

#### User Pool Management

| Operation | Target | Description |
|-----------|--------|-------------|
| CreateUserPool | `AWSCognitoIdentityProviderService.CreateUserPool` | Create a user pool |
| DeleteUserPool | `AWSCognitoIdentityProviderService.DeleteUserPool` | Delete a user pool |
| DescribeUserPool | `AWSCognitoIdentityProviderService.DescribeUserPool` | Get user pool details |
| ListUserPools | `AWSCognitoIdentityProviderService.ListUserPools` | List user pools |
| UpdateUserPool | `AWSCognitoIdentityProviderService.UpdateUserPool` | Update user pool settings |

#### User Management

| Operation | Target | Description |
|-----------|--------|-------------|
| AdminCreateUser | `AWSCognitoIdentityProviderService.AdminCreateUser` | Create a user |
| AdminDeleteUser | `AWSCognitoIdentityProviderService.AdminDeleteUser` | Delete a user |
| AdminGetUser | `AWSCognitoIdentityProviderService.AdminGetUser` | Get user details |
| AdminSetUserPassword | `AWSCognitoIdentityProviderService.AdminSetUserPassword` | Set a user's password |
| AdminEnableUser | `AWSCognitoIdentityProviderService.AdminEnableUser` | Enable a user |
| AdminDisableUser | `AWSCognitoIdentityProviderService.AdminDisableUser` | Disable a user |
| AdminResetUserPassword | `AWSCognitoIdentityProviderService.AdminResetUserPassword` | Reset a user's password |
| AdminUpdateUserAttributes | `AWSCognitoIdentityProviderService.AdminUpdateUserAttributes` | Update user attributes |
| ListUsers | `AWSCognitoIdentityProviderService.ListUsers` | List users in a pool |

#### User Pool Clients

| Operation | Target | Description |
|-----------|--------|-------------|
| CreateUserPoolClient | `AWSCognitoIdentityProviderService.CreateUserPoolClient` | Create an app client |
| DeleteUserPoolClient | `AWSCognitoIdentityProviderService.DeleteUserPoolClient` | Delete an app client |
| DescribeUserPoolClient | `AWSCognitoIdentityProviderService.DescribeUserPoolClient` | Get client details |
| ListUserPoolClients | `AWSCognitoIdentityProviderService.ListUserPoolClients` | List app clients |
| UpdateUserPoolClient | `AWSCognitoIdentityProviderService.UpdateUserPoolClient` | Update an app client |

#### Groups

| Operation | Target | Description |
|-----------|--------|-------------|
| CreateGroup | `AWSCognitoIdentityProviderService.CreateGroup` | Create a group |
| DeleteGroup | `AWSCognitoIdentityProviderService.DeleteGroup` | Delete a group |
| GetGroup | `AWSCognitoIdentityProviderService.GetGroup` | Get group details |
| ListGroups | `AWSCognitoIdentityProviderService.ListGroups` | List groups in a pool |
| AdminAddUserToGroup | `AWSCognitoIdentityProviderService.AdminAddUserToGroup` | Add a user to a group |
| AdminRemoveUserFromGroup | `AWSCognitoIdentityProviderService.AdminRemoveUserFromGroup` | Remove a user from a group |
| AdminListGroupsForUser | `AWSCognitoIdentityProviderService.AdminListGroupsForUser` | List groups for a user |
| ListUsersInGroup | `AWSCognitoIdentityProviderService.ListUsersInGroup` | List users in a group |

#### Authentication

| Operation | Target | Description |
|-----------|--------|-------------|
| InitiateAuth | `AWSCognitoIdentityProviderService.InitiateAuth` | Start an auth flow |
| AdminInitiateAuth | `AWSCognitoIdentityProviderService.AdminInitiateAuth` | Start an admin auth flow |
| SignUp | `AWSCognitoIdentityProviderService.SignUp` | Self-register a new user |
| ConfirmSignUp | `AWSCognitoIdentityProviderService.ConfirmSignUp` | Confirm a self-registration |
| ForgotPassword | `AWSCognitoIdentityProviderService.ForgotPassword` | Initiate password reset |
| ConfirmForgotPassword | `AWSCognitoIdentityProviderService.ConfirmForgotPassword` | Complete password reset |

### Cognito Error Response Format

```json
{
  "__type": "#ResourceNotFoundException",
  "message": "User pool us-east-1_abc123 does not exist."
}
```

| HTTP Status | Code | Description |
|-------------|------|-------------|
| 200 | | Success |
| 400 | ResourceNotFoundException | User pool, user, or client not found |
| 400 | InvalidParameterException | Invalid or missing parameter |
| 400 | UsernameExistsException | Username already exists |
| 400 | UserNotFoundException | User not found |
| 400 | GroupExistsException | Group already exists |
| 400 | NotAuthorizedException | Authentication failed |
| 500 | InternalErrorException | Internal server error |

---

## API Gateway Service

### Usage with the AWS CLI

```bash
# Create a REST API
aws apigateway create-rest-api \
  --name myapi \
  --endpoint-url http://localhost:4567 \
  --no-sign-request

# Get resources
aws apigateway get-resources \
  --rest-api-id <api-id> \
  --endpoint-url http://localhost:4567 \
  --no-sign-request

# Create a resource
aws apigateway create-resource \
  --rest-api-id <api-id> \
  --parent-id <root-resource-id> \
  --path-part myresource \
  --endpoint-url http://localhost:4567 \
  --no-sign-request

# Add a method
aws apigateway put-method \
  --rest-api-id <api-id> \
  --resource-id <resource-id> \
  --http-method GET \
  --authorization-type NONE \
  --endpoint-url http://localhost:4567 \
  --no-sign-request

# Add an integration
aws apigateway put-integration \
  --rest-api-id <api-id> \
  --resource-id <resource-id> \
  --http-method GET \
  --type MOCK \
  --endpoint-url http://localhost:4567 \
  --no-sign-request

# Create a deployment
aws apigateway create-deployment \
  --rest-api-id <api-id> \
  --stage-name prod \
  --endpoint-url http://localhost:4567 \
  --no-sign-request

# List REST APIs
aws apigateway get-rest-apis \
  --endpoint-url http://localhost:4567 \
  --no-sign-request
```

### Usage with AWS SDKs

```javascript
import { APIGatewayClient, CreateRestApiCommand, GetResourcesCommand } from "@aws-sdk/client-api-gateway";

const client = new APIGatewayClient({
  endpoint: "http://localhost:4567",
  region: "us-east-1",
  credentials: { accessKeyId: "test", secretAccessKey: "test" },
});

const { id } = await client.send(new CreateRestApiCommand({
  name: "myapi",
}));

const { items } = await client.send(new GetResourcesCommand({
  restApiId: id,
}));
```

### Wire Protocol

API Gateway uses a **REST API** with JSON — the HTTP method and path determine the operation:

- **REST APIs**: `GET/POST /restapis`, `GET/PATCH/DELETE /restapis/{restApiId}`
- **Resources**: `GET /restapis/{restApiId}/resources`, `GET/POST/DELETE /restapis/{restApiId}/resources/{resourceId}`
- **Methods**: `GET/PUT/DELETE /restapis/{restApiId}/resources/{resourceId}/methods/{httpMethod}`
- **Integrations**: `GET/PUT/DELETE /restapis/{restApiId}/resources/{resourceId}/methods/{httpMethod}/integration`
- **Method Responses**: `GET/PUT/DELETE /restapis/{restApiId}/resources/{resourceId}/methods/{httpMethod}/responses/{statusCode}`
- **Integration Responses**: `PUT /restapis/{restApiId}/resources/{resourceId}/methods/{httpMethod}/integration/responses/{statusCode}`
- **Deployments**: `GET/POST /restapis/{restApiId}/deployments`, `GET /restapis/{restApiId}/deployments/{deploymentId}`
- **Stages**: `GET/POST /restapis/{restApiId}/stages`, `GET/PATCH/DELETE /restapis/{restApiId}/stages/{stageName}`
- **Tags**: `GET/POST/DELETE /tags/{restApiId}`
- **Endpoint**: `http://localhost:<port>/`

### Supported Operations (30)

#### REST API Management

| Operation | Method | Path |
|-----------|--------|------|
| CreateRestApi | POST | `/restapis` |
| GetRestApis | GET | `/restapis` |
| GetRestApi | GET | `/restapis/{restApiId}` |
| UpdateRestApi | PATCH | `/restapis/{restApiId}` |
| DeleteRestApi | DELETE | `/restapis/{restApiId}` |

#### Resources

| Operation | Method | Path |
|-----------|--------|------|
| GetResources | GET | `/restapis/{restApiId}/resources` |
| GetResource | GET | `/restapis/{restApiId}/resources/{resourceId}` |
| CreateResource | POST | `/restapis/{restApiId}/resources/{parentId}` |
| DeleteResource | DELETE | `/restapis/{restApiId}/resources/{resourceId}` |

#### Methods

| Operation | Method | Path |
|-----------|--------|------|
| PutMethod | PUT | `/restapis/{restApiId}/resources/{resourceId}/methods/{httpMethod}` |
| GetMethod | GET | `/restapis/{restApiId}/resources/{resourceId}/methods/{httpMethod}` |
| DeleteMethod | DELETE | `/restapis/{restApiId}/resources/{resourceId}/methods/{httpMethod}` |

#### Integrations

| Operation | Method | Path |
|-----------|--------|------|
| PutIntegration | PUT | `/restapis/{restApiId}/resources/{resourceId}/methods/{httpMethod}/integration` |
| GetIntegration | GET | `/restapis/{restApiId}/resources/{resourceId}/methods/{httpMethod}/integration` |
| DeleteIntegration | DELETE | `/restapis/{restApiId}/resources/{resourceId}/methods/{httpMethod}/integration` |

#### Method Responses

| Operation | Method | Path |
|-----------|--------|------|
| PutMethodResponse | PUT | `/restapis/{restApiId}/resources/{resourceId}/methods/{httpMethod}/responses/{statusCode}` |
| GetMethodResponse | GET | `/restapis/{restApiId}/resources/{resourceId}/methods/{httpMethod}/responses/{statusCode}` |
| DeleteMethodResponse | DELETE | `/restapis/{restApiId}/resources/{resourceId}/methods/{httpMethod}/responses/{statusCode}` |

#### Integration Responses

| Operation | Method | Path |
|-----------|--------|------|
| PutIntegrationResponse | PUT | `/restapis/{restApiId}/resources/{resourceId}/methods/{httpMethod}/integration/responses/{statusCode}` |

#### Deployments

| Operation | Method | Path |
|-----------|--------|------|
| CreateDeployment | POST | `/restapis/{restApiId}/deployments` |
| GetDeployments | GET | `/restapis/{restApiId}/deployments` |
| GetDeployment | GET | `/restapis/{restApiId}/deployments/{deploymentId}` |

#### Stages

| Operation | Method | Path |
|-----------|--------|------|
| CreateStage | POST | `/restapis/{restApiId}/stages` |
| GetStages | GET | `/restapis/{restApiId}/stages` |
| GetStage | GET | `/restapis/{restApiId}/stages/{stageName}` |
| UpdateStage | PATCH | `/restapis/{restApiId}/stages/{stageName}` |
| DeleteStage | DELETE | `/restapis/{restApiId}/stages/{stageName}` |

#### Tagging

| Operation | Method | Path |
|-----------|--------|------|
| TagResource | POST | `/tags/{restApiId}` |
| UntagResource | DELETE | `/tags/{restApiId}` |
| GetTags | GET | `/tags/{restApiId}` |

### API Gateway Error Response Format

```json
{
  "message": "Invalid REST API identifier specified"
}
```

Response includes `x-amzn-ErrorType` header (e.g. `NotFoundException`).

| HTTP Status | Error Type | Description |
|-------------|------------|-------------|
| 200/201/202 | | Success |
| 400 | BadRequestException | Invalid parameters |
| 401 | UnauthorizedException | Not authorized |
| 404 | NotFoundException | REST API, resource, or stage not found |
| 409 | ConflictException | Resource already exists |
| 429 | TooManyRequestsException | Rate limit exceeded |

---

## KMS Service

### Usage with the AWS CLI

```bash
# Create a key
aws kms create-key \
  --description "My test key" \
  --endpoint-url http://localhost:7600 \
  --no-sign-request

# List keys
aws kms list-keys \
  --endpoint-url http://localhost:7600 \
  --no-sign-request

# Encrypt data
aws kms encrypt \
  --key-id <key-id> \
  --plaintext "SGVsbG8gV29ybGQ=" \
  --endpoint-url http://localhost:7600 \
  --no-sign-request

# Decrypt data
aws kms decrypt \
  --ciphertext-blob <ciphertext> \
  --endpoint-url http://localhost:7600 \
  --no-sign-request

# Generate a data key
aws kms generate-data-key \
  --key-id <key-id> \
  --key-spec AES_256 \
  --endpoint-url http://localhost:7600 \
  --no-sign-request

# Create an alias
aws kms create-alias \
  --alias-name alias/my-key \
  --target-key-id <key-id> \
  --endpoint-url http://localhost:7600 \
  --no-sign-request
```

### Wire Protocol

KMS uses the **AWS JSON 1.1** protocol over HTTP POST:

- **Content-Type**: `application/x-amz-json-1.1`
- **Action routing**: `X-Amz-Target: TrentService.<ActionName>` header
- **Request/response body**: JSON
- **Endpoint**: `http://localhost:<port>/`

### Supported Operations (21)

| Operation | Target | Description |
|-----------|--------|-------------|
| CreateKey | `TrentService.CreateKey` | Create a new KMS key |
| DescribeKey | `TrentService.DescribeKey` | Get key metadata |
| ListKeys | `TrentService.ListKeys` | List all keys |
| EnableKey | `TrentService.EnableKey` | Enable a disabled key |
| DisableKey | `TrentService.DisableKey` | Disable a key |
| ScheduleKeyDeletion | `TrentService.ScheduleKeyDeletion` | Schedule a key for deletion |
| CancelKeyDeletion | `TrentService.CancelKeyDeletion` | Cancel a scheduled deletion |
| Encrypt | `TrentService.Encrypt` | Encrypt plaintext (simulated) |
| Decrypt | `TrentService.Decrypt` | Decrypt ciphertext (simulated) |
| GenerateDataKey | `TrentService.GenerateDataKey` | Generate a data key |
| GenerateDataKeyWithoutPlaintext | `TrentService.GenerateDataKeyWithoutPlaintext` | Generate encrypted data key only |
| GenerateRandom | `TrentService.GenerateRandom` | Generate random bytes |
| Sign | `TrentService.Sign` | Sign a message (simulated) |
| Verify | `TrentService.Verify` | Verify a signature (simulated) |
| TagResource | `TrentService.TagResource` | Add tags to a key |
| UntagResource | `TrentService.UntagResource` | Remove tags from a key |
| ListResourceTags | `TrentService.ListResourceTags` | List tags on a key |
| CreateAlias | `TrentService.CreateAlias` | Create an alias for a key |
| DeleteAlias | `TrentService.DeleteAlias` | Delete an alias |
| ListAliases | `TrentService.ListAliases` | List aliases |
| GetKeyPolicy | `TrentService.GetKeyPolicy` | Get the key policy |
| PutKeyPolicy | `TrentService.PutKeyPolicy` | Set the key policy |

### KMS Error Response Format

```json
{
  "__type": "NotFoundException",
  "message": "Invalid keyId: nonexistent"
}
```

---

## Secrets Manager Service

### Usage with the AWS CLI

```bash
# Create a secret
aws secretsmanager create-secret \
  --name mydb/credentials \
  --secret-string '{"username":"admin","password":"s3cr3t"}' \
  --endpoint-url http://localhost:7700 \
  --no-sign-request

# Get a secret value
aws secretsmanager get-secret-value \
  --secret-id mydb/credentials \
  --endpoint-url http://localhost:7700 \
  --no-sign-request

# Update a secret
aws secretsmanager put-secret-value \
  --secret-id mydb/credentials \
  --secret-string '{"username":"admin","password":"newpass"}' \
  --endpoint-url http://localhost:7700 \
  --no-sign-request

# List secrets
aws secretsmanager list-secrets \
  --endpoint-url http://localhost:7700 \
  --no-sign-request

# Delete a secret
aws secretsmanager delete-secret \
  --secret-id mydb/credentials \
  --force-delete-without-recovery \
  --endpoint-url http://localhost:7700 \
  --no-sign-request
```

### Wire Protocol

Secrets Manager uses the **AWS JSON 1.1** protocol over HTTP POST:

- **Content-Type**: `application/x-amz-json-1.1`
- **Action routing**: `X-Amz-Target: secretsmanager.<ActionName>` header
- **Request/response body**: JSON
- **Endpoint**: `http://localhost:<port>/`

### Supported Operations (11)

| Operation | Target | Description |
|-----------|--------|-------------|
| CreateSecret | `secretsmanager.CreateSecret` | Create a new secret |
| GetSecretValue | `secretsmanager.GetSecretValue` | Retrieve the current secret value |
| PutSecretValue | `secretsmanager.PutSecretValue` | Update the secret value (creates a new version) |
| DescribeSecret | `secretsmanager.DescribeSecret` | Get secret metadata |
| ListSecrets | `secretsmanager.ListSecrets` | List all secrets |
| UpdateSecret | `secretsmanager.UpdateSecret` | Update secret description or KMS key |
| DeleteSecret | `secretsmanager.DeleteSecret` | Delete a secret |
| RestoreSecret | `secretsmanager.RestoreSecret` | Cancel a pending deletion |
| TagResource | `secretsmanager.TagResource` | Add tags to a secret |
| UntagResource | `secretsmanager.UntagResource` | Remove tags from a secret |
| ListSecretVersionIds | `secretsmanager.ListSecretVersionIds` | List all versions of a secret |

### Secrets Manager Error Response Format

```json
{
  "__type": "ResourceNotFoundException",
  "message": "Secrets Manager can't find the specified secret."
}
```

---

## Kinesis Data Streams Service

### Usage with the AWS CLI

```bash
# Create a stream
aws kinesis create-stream \
  --stream-name mystream \
  --shard-count 1 \
  --endpoint-url http://localhost:4568 \
  --no-sign-request

# Put a record
aws kinesis put-record \
  --stream-name mystream \
  --data "$(echo -n 'Hello Kinesis' | base64)" \
  --partition-key pk1 \
  --endpoint-url http://localhost:4568 \
  --no-sign-request

# Get a shard iterator
aws kinesis get-shard-iterator \
  --stream-name mystream \
  --shard-id shardId-000000000000 \
  --shard-iterator-type TRIM_HORIZON \
  --endpoint-url http://localhost:4568 \
  --no-sign-request

# Get records
aws kinesis get-records \
  --shard-iterator <iterator> \
  --endpoint-url http://localhost:4568 \
  --no-sign-request

# List streams
aws kinesis list-streams \
  --endpoint-url http://localhost:4568 \
  --no-sign-request

# Delete a stream
aws kinesis delete-stream \
  --stream-name mystream \
  --endpoint-url http://localhost:4568 \
  --no-sign-request
```

### Wire Protocol

Kinesis uses the **AWS JSON 1.1** protocol over HTTP POST:

- **Content-Type**: `application/x-amz-json-1.1`
- **Action routing**: `X-Amz-Target: Kinesis_20131202.<ActionName>` header
- **Request/response body**: JSON
- **Endpoint**: `http://localhost:<port>/`

### Supported Operations (15)

| Operation | Target | Description |
|-----------|--------|-------------|
| CreateStream | `Kinesis_20131202.CreateStream` | Create a stream with N shards |
| DeleteStream | `Kinesis_20131202.DeleteStream` | Delete a stream |
| DescribeStream | `Kinesis_20131202.DescribeStream` | Get stream details and shard list |
| DescribeStreamSummary | `Kinesis_20131202.DescribeStreamSummary` | Get a stream summary |
| ListStreams | `Kinesis_20131202.ListStreams` | List all streams |
| ListShards | `Kinesis_20131202.ListShards` | List shards for a stream |
| PutRecord | `Kinesis_20131202.PutRecord` | Put a single record |
| PutRecords | `Kinesis_20131202.PutRecords` | Put multiple records |
| GetShardIterator | `Kinesis_20131202.GetShardIterator` | Get a shard iterator (TRIM_HORIZON, LATEST, AT_SEQUENCE_NUMBER, AFTER_SEQUENCE_NUMBER) |
| GetRecords | `Kinesis_20131202.GetRecords` | Read records from a shard iterator |
| AddTagsToStream | `Kinesis_20131202.AddTagsToStream` | Add tags to a stream |
| RemoveTagsFromStream | `Kinesis_20131202.RemoveTagsFromStream` | Remove tags from a stream |
| ListTagsForStream | `Kinesis_20131202.ListTagsForStream` | List tags on a stream |
| IncreaseStreamRetentionPeriod | `Kinesis_20131202.IncreaseStreamRetentionPeriod` | Increase retention period |
| DecreaseStreamRetentionPeriod | `Kinesis_20131202.DecreaseStreamRetentionPeriod` | Decrease retention period |

### Kinesis Error Response Format

```json
{
  "__type": "ResourceNotFoundException",
  "message": "Stream mystream under account 000000000000 not found."
}
```

---

## EventBridge Service

### Usage with the AWS CLI

```bash
# Describe the default event bus
aws events describe-event-bus \
  --endpoint-url http://localhost:9195 \
  --no-sign-request

# Create a custom event bus
aws events create-event-bus \
  --name mybus \
  --endpoint-url http://localhost:9195 \
  --no-sign-request

# Put events
aws events put-events \
  --entries '[{"Source":"my.service","DetailType":"OrderPlaced","Detail":"{\"orderId\":\"123\"}"}]' \
  --endpoint-url http://localhost:9195 \
  --no-sign-request

# Create a rule
aws events put-rule \
  --name my-rule \
  --event-pattern '{"source":["my.service"]}' \
  --state ENABLED \
  --endpoint-url http://localhost:9195 \
  --no-sign-request

# Add a target
aws events put-targets \
  --rule my-rule \
  --targets Id=target1,Arn=arn:aws:lambda:us-east-1:000000000000:function:my-func \
  --endpoint-url http://localhost:9195 \
  --no-sign-request

# List rules
aws events list-rules \
  --endpoint-url http://localhost:9195 \
  --no-sign-request
```

### Wire Protocol

EventBridge uses the **AWS JSON 1.1** protocol over HTTP POST:

- **Content-Type**: `application/x-amz-json-1.1`
- **Action routing**: `X-Amz-Target: AmazonEventBridge.<ActionName>` header
- **Request/response body**: JSON
- **Endpoint**: `http://localhost:<port>/`

### Supported Operations (17)

| Operation | Target | Description |
|-----------|--------|-------------|
| CreateEventBus | `AmazonEventBridge.CreateEventBus` | Create a custom event bus |
| DeleteEventBus | `AmazonEventBridge.DeleteEventBus` | Delete a custom event bus |
| DescribeEventBus | `AmazonEventBridge.DescribeEventBus` | Describe an event bus |
| ListEventBuses | `AmazonEventBridge.ListEventBuses` | List all event buses |
| PutEvents | `AmazonEventBridge.PutEvents` | Publish events to a bus |
| PutRule | `AmazonEventBridge.PutRule` | Create or update a rule |
| DeleteRule | `AmazonEventBridge.DeleteRule` | Delete a rule |
| DescribeRule | `AmazonEventBridge.DescribeRule` | Describe a rule |
| ListRules | `AmazonEventBridge.ListRules` | List rules |
| PutTargets | `AmazonEventBridge.PutTargets` | Add targets to a rule |
| RemoveTargets | `AmazonEventBridge.RemoveTargets` | Remove targets from a rule |
| ListTargetsByRule | `AmazonEventBridge.ListTargetsByRule` | List targets for a rule |
| TagResource | `AmazonEventBridge.TagResource` | Add tags to a resource |
| UntagResource | `AmazonEventBridge.UntagResource` | Remove tags from a resource |
| ListTagsForResource | `AmazonEventBridge.ListTagsForResource` | List tags on a resource |

### EventBridge Error Response Format

```json
{
  "__type": "ResourceNotFoundException",
  "message": "Event bus mybus does not exist."
}
```

---

## Step Functions Service

### Usage with the AWS CLI

```bash
# Create a state machine
DEFINITION='{"Comment":"Test","StartAt":"Pass","States":{"Pass":{"Type":"Pass","End":true}}}'
aws stepfunctions create-state-machine \
  --name my-state-machine \
  --definition "$DEFINITION" \
  --role-arn arn:aws:iam::000000000000:role/StepFunctionsRole \
  --endpoint-url http://localhost:8083 \
  --no-sign-request

# Start an execution
aws stepfunctions start-execution \
  --state-machine-arn arn:aws:states:us-east-1:000000000000:stateMachine:my-state-machine \
  --name my-execution \
  --input '{"key":"value"}' \
  --endpoint-url http://localhost:8083 \
  --no-sign-request

# Describe an execution
aws stepfunctions describe-execution \
  --execution-arn arn:aws:states:us-east-1:000000000000:execution:my-state-machine:my-execution \
  --endpoint-url http://localhost:8083 \
  --no-sign-request

# List state machines
aws stepfunctions list-state-machines \
  --endpoint-url http://localhost:8083 \
  --no-sign-request

# Delete a state machine
aws stepfunctions delete-state-machine \
  --state-machine-arn arn:aws:states:us-east-1:000000000000:stateMachine:my-state-machine \
  --endpoint-url http://localhost:8083 \
  --no-sign-request
```

### Wire Protocol

Step Functions uses the **AWS JSON 1.1** protocol over HTTP POST:

- **Content-Type**: `application/x-amz-json-1.1`
- **Action routing**: `X-Amz-Target: AmazonStates.<ActionName>` header
- **Request/response body**: JSON with **camelCase** field names (e.g. `stateMachineArn`, `startDate`)
- **ARN format**: `arn:aws:states:<region>:<account>:stateMachine:<name>`
- **Endpoint**: `http://localhost:<port>/`

### Supported Operations (16)

| Operation | Target | Description |
|-----------|--------|-------------|
| CreateStateMachine | `AmazonStates.CreateStateMachine` | Create a state machine |
| DeleteStateMachine | `AmazonStates.DeleteStateMachine` | Delete a state machine |
| DescribeStateMachine | `AmazonStates.DescribeStateMachine` | Describe a state machine |
| ListStateMachines | `AmazonStates.ListStateMachines` | List all state machines |
| StartExecution | `AmazonStates.StartExecution` | Start a new execution |
| StopExecution | `AmazonStates.StopExecution` | Stop a running execution |
| DescribeExecution | `AmazonStates.DescribeExecution` | Describe an execution |
| ListExecutions | `AmazonStates.ListExecutions` | List executions for a state machine |
| GetExecutionHistory | `AmazonStates.GetExecutionHistory` | Get execution event history |
| SendTaskSuccess | `AmazonStates.SendTaskSuccess` | Report task success |
| SendTaskFailure | `AmazonStates.SendTaskFailure` | Report task failure |
| SendTaskHeartbeat | `AmazonStates.SendTaskHeartbeat` | Send a task heartbeat |
| TagResource | `AmazonStates.TagResource` | Add tags to a resource |
| UntagResource | `AmazonStates.UntagResource` | Remove tags from a resource |
| ListTagsForResource | `AmazonStates.ListTagsForResource` | List tags on a resource |

### Step Functions Error Response Format

```json
{
  "__type": "StateMachineDoesNotExist",
  "message": "State machine does not exist: arn:aws:states:..."
}
```

---

## SSM Parameter Store Service

### Usage with the AWS CLI

```bash
# Put a parameter
aws ssm put-parameter \
  --name /myapp/db/host \
  --value "localhost" \
  --type String \
  --endpoint-url http://localhost:9100 \
  --no-sign-request

# Get a parameter
aws ssm get-parameter \
  --name /myapp/db/host \
  --endpoint-url http://localhost:9100 \
  --no-sign-request

# Get parameters by path
aws ssm get-parameters-by-path \
  --path /myapp \
  --recursive \
  --endpoint-url http://localhost:9100 \
  --no-sign-request

# Overwrite a parameter
aws ssm put-parameter \
  --name /myapp/db/host \
  --value "prod.example.com" \
  --type String \
  --overwrite \
  --endpoint-url http://localhost:9100 \
  --no-sign-request

# Delete a parameter
aws ssm delete-parameter \
  --name /myapp/db/host \
  --endpoint-url http://localhost:9100 \
  --no-sign-request

# Describe parameters
aws ssm describe-parameters \
  --endpoint-url http://localhost:9100 \
  --no-sign-request
```

### Wire Protocol

SSM uses the **AWS JSON 1.1** protocol over HTTP POST:

- **Content-Type**: `application/x-amz-json-1.1`
- **Action routing**: `X-Amz-Target: AmazonSSM.<ActionName>` header
- **Request/response body**: JSON
- **ARN format**: `arn:aws:ssm:<region>:<account>:parameter/<name>`
- **Endpoint**: `http://localhost:<port>/`

### Supported Operations (10)

| Operation | Target | Description |
|-----------|--------|-------------|
| PutParameter | `AmazonSSM.PutParameter` | Create or update a parameter |
| GetParameter | `AmazonSSM.GetParameter` | Get a single parameter |
| GetParameters | `AmazonSSM.GetParameters` | Get multiple parameters by name |
| GetParametersByPath | `AmazonSSM.GetParametersByPath` | Get parameters under a path prefix (with optional recursion) |
| DeleteParameter | `AmazonSSM.DeleteParameter` | Delete a single parameter |
| DeleteParameters | `AmazonSSM.DeleteParameters` | Delete multiple parameters |
| DescribeParameters | `AmazonSSM.DescribeParameters` | List all parameters with metadata |
| AddTagsToResource | `AmazonSSM.AddTagsToResource` | Add tags to a parameter |
| RemoveTagsFromResource | `AmazonSSM.RemoveTagsFromResource` | Remove tags from a parameter |
| ListTagsForResource | `AmazonSSM.ListTagsForResource` | List tags on a parameter |

### Parameter Types

| Type | Description |
|------|-------------|
| `String` | Plain text string value |
| `SecureString` | Encrypted string (stored as-is in local mode) |
| `StringList` | Comma-delimited list of values |

### SSM Error Response Format

```json
{
  "__type": "ParameterNotFound",
  "message": "Parameter /myapp/db/host not found."
}
```

---

## CloudWatch Logs Service

### Usage with the AWS CLI

```bash
# Create a log group
aws logs create-log-group \
  --log-group-name /myapp/service \
  --endpoint-url http://localhost:9201 \
  --no-sign-request

# Create a log stream
aws logs create-log-stream \
  --log-group-name /myapp/service \
  --log-stream-name stream-2024-01-01 \
  --endpoint-url http://localhost:9201 \
  --no-sign-request

# Put log events
NOW_MS=$(date +%s)000
aws logs put-log-events \
  --log-group-name /myapp/service \
  --log-stream-name stream-2024-01-01 \
  --log-events "[{\"timestamp\":${NOW_MS},\"message\":\"Application started\"}]" \
  --endpoint-url http://localhost:9201 \
  --no-sign-request

# Get log events
aws logs get-log-events \
  --log-group-name /myapp/service \
  --log-stream-name stream-2024-01-01 \
  --endpoint-url http://localhost:9201 \
  --no-sign-request

# Filter log events
aws logs filter-log-events \
  --log-group-name /myapp/service \
  --filter-pattern "ERROR" \
  --endpoint-url http://localhost:9201 \
  --no-sign-request

# Set retention policy
aws logs put-retention-policy \
  --log-group-name /myapp/service \
  --retention-in-days 7 \
  --endpoint-url http://localhost:9201 \
  --no-sign-request
```

### Wire Protocol

CloudWatch Logs uses the **AWS JSON 1.1** protocol over HTTP POST:

- **Content-Type**: `application/x-amz-json-1.1`
- **Action routing**: `X-Amz-Target: Logs_20140328.<ActionName>` header
- **Request/response body**: JSON with **camelCase** field names
- **Endpoint**: `http://localhost:<port>/`

### Supported Operations (17)

#### Log Groups

| Operation | Target | Description |
|-----------|--------|-------------|
| CreateLogGroup | `Logs_20140328.CreateLogGroup` | Create a log group |
| DeleteLogGroup | `Logs_20140328.DeleteLogGroup` | Delete a log group |
| DescribeLogGroups | `Logs_20140328.DescribeLogGroups` | List log groups with optional prefix filter |
| PutRetentionPolicy | `Logs_20140328.PutRetentionPolicy` | Set retention period for a log group |
| DeleteRetentionPolicy | `Logs_20140328.DeleteRetentionPolicy` | Remove retention policy |

#### Log Streams

| Operation | Target | Description |
|-----------|--------|-------------|
| CreateLogStream | `Logs_20140328.CreateLogStream` | Create a log stream |
| DeleteLogStream | `Logs_20140328.DeleteLogStream` | Delete a log stream |
| DescribeLogStreams | `Logs_20140328.DescribeLogStreams` | List log streams in a group |

#### Log Events

| Operation | Target | Description |
|-----------|--------|-------------|
| PutLogEvents | `Logs_20140328.PutLogEvents` | Write log events to a stream |
| GetLogEvents | `Logs_20140328.GetLogEvents` | Read log events from a stream |
| FilterLogEvents | `Logs_20140328.FilterLogEvents` | Search log events across streams with pattern matching |

#### Tagging

| Operation | Target | Description |
|-----------|--------|-------------|
| TagLogGroup | `Logs_20140328.TagLogGroup` | Add tags to a log group (legacy) |
| UntagLogGroup | `Logs_20140328.UntagLogGroup` | Remove tags from a log group (legacy) |
| ListTagsLogGroup | `Logs_20140328.ListTagsLogGroup` | List tags on a log group (legacy) |
| TagResource | `Logs_20140328.TagResource` | Add tags to a resource |
| UntagResource | `Logs_20140328.UntagResource` | Remove tags from a resource |
| ListTagsForResource | `Logs_20140328.ListTagsForResource` | List tags on a resource |

### CloudWatch Logs Error Response Format

```json
{
  "__type": "ResourceNotFoundException",
  "message": "The specified log group does not exist"
}
```

---

## SES Service

### Usage with the AWS CLI

```bash
# Create an email identity
aws sesv2 create-email-identity \
  --email-identity sender@example.com \
  --endpoint-url http://localhost:9300 \
  --no-sign-request

# List identities
aws sesv2 list-email-identities \
  --endpoint-url http://localhost:9300 \
  --no-sign-request

# Get identity details
aws sesv2 get-email-identity \
  --email-identity sender@example.com \
  --endpoint-url http://localhost:9300 \
  --no-sign-request

# Send an email
aws sesv2 send-email \
  --from-email-address sender@example.com \
  --destination '{"ToAddresses":["recipient@example.com"]}' \
  --content '{"Simple":{"Subject":{"Data":"Hello"},"Body":{"Text":{"Data":"Hello World"}}}}' \
  --endpoint-url http://localhost:9300 \
  --no-sign-request

# Delete an identity
aws sesv2 delete-email-identity \
  --email-identity sender@example.com \
  --endpoint-url http://localhost:9300 \
  --no-sign-request
```

### Wire Protocol

SES uses the **SES v2 REST API** with JSON:

- **Content-Type**: `application/json`
- **Action routing**: HTTP method + path
- **Request/response body**: JSON
- **Endpoint**: `http://localhost:<port>/`

### Supported Operations (5)

| Operation | Method | Path | Description |
|-----------|--------|------|-------------|
| CreateEmailIdentity | POST | `/v2/email/identities` | Create and auto-verify an email identity (address or domain) |
| ListEmailIdentities | GET | `/v2/email/identities` | List all email identities |
| GetEmailIdentity | GET | `/v2/email/identities/{emailIdentity}` | Get details for an identity |
| DeleteEmailIdentity | DELETE | `/v2/email/identities/{emailIdentity}` | Delete an identity |
| SendEmail | POST | `/v2/email/outbound-emails` | Send an email (accepted; not delivered) |

### SES Error Response Format

```json
{
  "message": "Email identity sender@example.com not found."
}
```

Response includes `x-amzn-ErrorType` header (e.g. `NotFoundException`, `AlreadyExistsException`).

---

## Running Tests

The integration test suites use the AWS CLI to exercise all API operations:

```bash
# Run S3 tests (49 assertions)
bash tests/s3_integration.sh

# Run SNS tests (42 assertions)
bash tests/sns_integration.sh

# Run SQS tests (70 assertions)
bash tests/sqs_integration.sh

# Run DynamoDB tests (30 assertions)
bash tests/dynamodb_integration.sh

# Run Lambda tests (28 assertions)
bash tests/lambda_integration.sh

# Run Firehose tests (18 assertions)
bash tests/firehose_integration.sh

# Run MemoryDB tests (29 assertions)
bash tests/memorydb_integration.sh

# Run Cognito tests (53 assertions)
bash tests/cognito_integration.sh

# Run API Gateway tests (43 assertions)
bash tests/apigateway_integration.sh

# Run KMS tests (27 assertions)
bash tests/kms_integration.sh

# Run Secrets Manager tests (17 assertions)
bash tests/secretsmanager_integration.sh

# Run Kinesis tests (18 assertions)
bash tests/kinesis_integration.sh

# Run EventBridge tests (17 assertions)
bash tests/eventbridge_integration.sh

# Run Step Functions tests (16 assertions)
bash tests/stepfunctions_integration.sh

# Run SSM tests (14 assertions)
bash tests/ssm_integration.sh

# Run CloudWatch Logs tests (23 assertions)
bash tests/cloudwatchlogs_integration.sh

# Run SES tests (10 assertions)
bash tests/ses_integration.sh
```

Each script builds the binary, starts the server on isolated ports, runs all test cases, and reports pass/fail counts.

---

## Differences from AWS

This is a local development tool, not a production replacement. Key differences:

- **In-memory only** -- all state is lost when the server stops. No disk persistence or replication.
- **No authentication** -- all requests are accepted without signature verification. Use `--no-sign-request`.
- **No TLS** -- the server speaks plain HTTP only.
- **Single-process** -- no distributed behavior.
- **S3 versioning** -- versioning status can be toggled but version history is not maintained. Only the latest version of each object is stored.
- **SNS subscriptions auto-confirm** -- all subscriptions are immediately confirmed without requiring endpoint verification.
- **SNS message delivery** -- messages are accepted and assigned IDs but not actually delivered to endpoints. Use this service for API compatibility testing, not delivery testing.
- **SQS permissions stored but not enforced** -- `AddPermission` / `RemovePermission` update the queue's policy, but no access checks are performed.
- **DynamoDB expressions** -- basic `KeyConditionExpression`, `UpdateExpression` (SET, REMOVE), `FilterExpression`, and `ProjectionExpression` are supported. Advanced features like condition expressions with complex operators, transactions, GSIs/LSIs, and streams are not implemented.
- **Lambda invocation** -- `Invoke` returns a stub 200 response. Functions are not actually executed. Use this for API compatibility testing.
- **Firehose delivery** -- records are accepted and stored in memory but not delivered to any destination. Use this for API compatibility testing.
- **MemoryDB clusters** -- clusters are created with simulated metadata (endpoints, shards, nodes) but no actual Redis instances are started.
- **Cognito authentication** -- `InitiateAuth`, `AdminInitiateAuth`, `SignUp`, and related flows return stub token responses. No actual JWT signing or token validation is performed. Use this for API compatibility testing.
- **Cognito subscriptions auto-confirm** -- `ConfirmSignUp` succeeds for any user regardless of the confirmation code provided.
- **API Gateway invocations** -- the service manages REST API configuration (resources, methods, integrations, deployments, stages) but does not route or proxy actual HTTP requests. Use this for infrastructure-as-code and API compatibility testing.
- **No CloudWatch metrics** -- no metrics integration.
- **Encryption attributes are accepted but not applied** -- KMS-related attributes are stored but data is not encrypted.
- **Upload size limit** -- S3 supports uploads up to 5 GB per request (axum body limit).
- **KMS cryptography is simulated** -- Encrypt/Decrypt, Sign/Verify, and GenerateDataKey operations produce deterministic fake outputs. No actual cryptographic operations are performed; do not use for security-sensitive testing.
- **Secrets Manager deletion is immediate** -- `DeleteSecret` with `--force-delete-without-recovery` removes the secret immediately. Without this flag the secret is marked deleted but still inaccessible.
- **Kinesis shard iterators expire on restart** -- iterators are stored in memory and are lost when the server restarts.
- **EventBridge rules do not evaluate events** -- `PutEvents` accepts and assigns IDs to events but does not match them against rules or invoke targets.
- **Step Functions executions do not run** -- `StartExecution` creates an execution in RUNNING state but does not evaluate the state machine definition or advance state.
- **SSM SecureString values are stored in plaintext** -- no KMS encryption is performed.
- **CloudWatch Logs FilterLogEvents uses substring matching** -- the filter pattern is matched as a plain substring, not as CloudWatch Logs filter syntax.
- **SES emails are not delivered** -- `SendEmail` accepts the request and returns a message ID but does not connect to an SMTP server or deliver email. All identities are auto-verified.

## References

### AWS S3
- [Developer Guide](https://docs.aws.amazon.com/AmazonS3/latest/userguide/s3-userguide.pdf)
- [API Reference](https://docs.aws.amazon.com/AmazonS3/latest/API/s3-api.pdf)

### Amazon SNS
- [Developer Guide](https://docs.aws.amazon.com/sns/latest/dg/sns-dg.pdf)
- [API Reference](https://docs.aws.amazon.com/sns/latest/api/sns-api.pdf)

### Amazon SQS
- [Developer Guide](https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-dg.pdf)
- [API Reference](https://docs.aws.amazon.com/AWSSimpleQueueService/latest/APIReference/sqs-api.pdf)

### Amazon DynamoDB
- [Developer Guide](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/dynamodb-dg.pdf)
- [API Reference](https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/dynamodb-api.pdf)

### AWS Lambda
- [Developer Guide](https://docs.aws.amazon.com/lambda/latest/dg/lambda-dg.pdf)
- [API Reference](https://docs.aws.amazon.com/lambda/latest/api/lambda-api.pdf)

### Amazon Data Firehose
- [Developer Guide](https://docs.aws.amazon.com/firehose/latest/dev/firehose-dg.pdf)
- [API Reference](https://docs.aws.amazon.com/firehose/latest/APIReference/firehose-api.pdf)

### Amazon MemoryDB
- [Developer Guide](https://docs.aws.amazon.com/memorydb/latest/devguide/memorydb-guide.pdf)
- [API Reference](https://docs.aws.amazon.com/memorydb/latest/APIReference/memorydb-api.pdf)

### Amazon Cognito
- [Developer Guide](https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-dg.pdf)
- [API Reference](https://docs.aws.amazon.com/cognito-user-identity-pools/latest/APIReference/cognito-userpools-api.pdf)

### Amazon API Gateway
- [Developer Guide](https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-dg.pdf)
- [API Reference](https://docs.aws.amazon.com/apigateway/latest/api/API_Operations.html)

### AWS KMS
- [Developer Guide](https://docs.aws.amazon.com/kms/latest/developerguide/kms-dg.pdf)
- [API Reference](https://docs.aws.amazon.com/kms/latest/APIReference/kms-api.pdf)

### AWS Secrets Manager
- [Developer Guide](https://docs.aws.amazon.com/secretsmanager/latest/userguide/secretsmanager-userguide.pdf)
- [API Reference](https://docs.aws.amazon.com/secretsmanager/latest/apireference/secretsmanager-api.pdf)

### Amazon Kinesis Data Streams
- [Developer Guide](https://docs.aws.amazon.com/streams/latest/dev/kinesis-dg.pdf)
- [API Reference](https://docs.aws.amazon.com/kinesis/latest/APIReference/kinesis-api.pdf)

### Amazon EventBridge
- [Developer Guide](https://docs.aws.amazon.com/eventbridge/latest/userguide/eb-ug.pdf)
- [API Reference](https://docs.aws.amazon.com/eventbridge/latest/APIReference/eb-api.pdf)

### AWS Step Functions
- [Developer Guide](https://docs.aws.amazon.com/step-functions/latest/dg/stepfunctions-dg.pdf)
- [API Reference](https://docs.aws.amazon.com/step-functions/latest/apireference/stepfunctions-api.pdf)

### AWS Systems Manager Parameter Store
- [Developer Guide](https://docs.aws.amazon.com/systems-manager/latest/userguide/systems-manager-guide.pdf)
- [API Reference](https://docs.aws.amazon.com/systems-manager/latest/APIReference/ssm-api.pdf)

### Amazon CloudWatch Logs
- [Developer Guide](https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/cloudwatch_logs_ug.pdf)
- [API Reference](https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/cwl-api.pdf)

### Amazon SES
- [Developer Guide](https://docs.aws.amazon.com/ses/latest/dg/ses-dg.pdf)
- [API Reference](https://docs.aws.amazon.com/ses/latest/APIReference/ses-api.pdf)
