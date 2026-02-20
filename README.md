# aws-inmemory-services

In-memory implementations of nine AWS services written in Rust: Amazon S3, Amazon SNS, Amazon SQS, Amazon DynamoDB, AWS Lambda, Amazon Data Firehose, Amazon MemoryDB, Amazon Cognito, and Amazon API Gateway. All services run as a single binary on separate ports, are compatible with the AWS CLI and SDKs, and require no external dependencies.

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
