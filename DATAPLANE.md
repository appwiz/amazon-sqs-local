# AWS Data Plane API Reference

A catalog of AWS services classified by whether they expose **native AWS data plane APIs** — operations invoked via the AWS SDK using SigV4-signed requests to AWS service endpoints.

## Criteria

| Classification | Description |
|---|---|
| ✅ Native AWS Data Plane API | Data plane operations (read/write/process) are invoked through the AWS SDK using native AWS APIs. No separate client library or open-source protocol driver required. |
| ❌ Non-Native Data Plane | The control plane (create/delete/describe cluster) is AWS-native, but data plane operations require a separate, protocol-specific client (Redis client, Kafka client, MongoDB driver, etc.). |

---

## Storage

| Service | Developer Guide | API Reference | Native Data Plane API | Notes |
|---|---|---|---|---|
| Amazon S3 | [Guide](https://docs.aws.amazon.com/AmazonS3/latest/userguide/) | [API Ref](https://docs.aws.amazon.com/AmazonS3/latest/API/) | ✅ | `GetObject`, `PutObject`, `DeleteObject`, `CopyObject`, `ListObjectsV2` |
| Amazon S3 Glacier | [Guide](https://docs.aws.amazon.com/amazonglacier/latest/dev/) | [API Ref](https://docs.aws.amazon.com/amazonglacier/latest/dev/amazon-glacier-api.html) | ✅ | `InitiateJob`, `GetJobOutput`, `UploadArchive` |
| Amazon EBS Direct APIs | [Guide](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ebs-accessing-snapshot.html) | [API Ref](https://docs.aws.amazon.com/ebs/latest/APIReference/) | ✅ | Block-level snapshot access: `GetSnapshotBlock`, `PutSnapshotBlock`, `ListSnapshotBlocks` |
| Amazon EFS | [Guide](https://docs.aws.amazon.com/efs/latest/ug/) | [API Ref](https://docs.aws.amazon.com/efs/latest/ug/API_Reference.html) | ❌ | Data plane uses NFS protocol; AWS API is control plane only |
| Amazon FSx | [Guide](https://docs.aws.amazon.com/fsx/latest/ONTAPGuide/) | [API Ref](https://docs.aws.amazon.com/fsx/latest/APIReference/) | ❌ | Data plane uses Lustre POSIX, SMB, NFS, or NetApp ONTAP protocols |

---

## Databases

| Service | Developer Guide | API Reference | Native Data Plane API | Notes |
|---|---|---|---|---|
| Amazon DynamoDB | [Guide](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/) | [API Ref](https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/) | ✅ | `GetItem`, `PutItem`, `UpdateItem`, `DeleteItem`, `Query`, `Scan`, `TransactGetItems`, `TransactWriteItems` |
| Amazon QLDB | [Guide](https://docs.aws.amazon.com/qldb/latest/developerguide/) | [API Ref](https://docs.aws.amazon.com/qldb/latest/developerguide/API_Reference.html) | ✅ | Sessions API: `StartSession`, `ExecuteStatement`, `FetchPage`, `CommitTransaction` |
| Amazon Timestream (Write) | [Guide](https://docs.aws.amazon.com/timestream/latest/developerguide/) | [API Ref](https://docs.aws.amazon.com/timestream-write/latest/APIReference/) | ✅ | `WriteRecords` |
| Amazon Timestream (Query) | [Guide](https://docs.aws.amazon.com/timestream/latest/developerguide/) | [API Ref](https://docs.aws.amazon.com/timestream-query/latest/APIReference/) | ✅ | `Query` |
| Amazon Aurora (Data API) | [Guide](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/data-api.html) | [API Ref](https://docs.aws.amazon.com/rdsdataservice/latest/APIReference/) | ✅ | Optional HTTP endpoint: `ExecuteStatement`, `BatchExecuteStatement`, `BeginTransaction`, `CommitTransaction` |
| Amazon Aurora (standard) | [Guide](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/) | [API Ref](https://docs.aws.amazon.com/AmazonRDS/latest/APIReference/) | ❌ | Primary access via MySQL / PostgreSQL wire protocol; requires DB driver |
| Amazon RDS | [Guide](https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/) | [API Ref](https://docs.aws.amazon.com/AmazonRDS/latest/APIReference/) | ❌ | MySQL, PostgreSQL, Oracle, SQL Server wire protocols; requires DB driver |
| Amazon Redshift (Data API) | [Guide](https://docs.aws.amazon.com/redshift/latest/mgmt/data-api.html) | [API Ref](https://docs.aws.amazon.com/redshift-data/latest/APIReference/) | ✅ | Optional HTTP endpoint: `ExecuteStatement`, `BatchExecuteStatement`, `GetStatementResult` |
| Amazon Redshift (standard) | [Guide](https://docs.aws.amazon.com/redshift/latest/mgmt/) | [API Ref](https://docs.aws.amazon.com/redshift/latest/APIReference/) | ❌ | Primary access via PostgreSQL wire protocol (JDBC/ODBC); requires driver |
| Amazon Athena | [Guide](https://docs.aws.amazon.com/athena/latest/ug/) | [API Ref](https://docs.aws.amazon.com/athena/latest/APIReference/) | ✅ | `StartQueryExecution`, `GetQueryExecution`, `GetQueryResults`, `StopQueryExecution` |
| Amazon OpenSearch Service | [Guide](https://docs.aws.amazon.com/opensearch-service/latest/developerguide/) | [API Ref](https://docs.aws.amazon.com/opensearch-service/latest/APIReference/) | ❌ | Data plane uses the OpenSearch / Elasticsearch REST HTTP API; requires OpenSearch client |
| Amazon DocumentDB | [Guide](https://docs.aws.amazon.com/documentdb/latest/developerguide/) | [API Ref](https://docs.aws.amazon.com/documentdb/latest/developerguide/API_Reference.html) | ❌ | Uses MongoDB wire protocol; requires MongoDB driver |
| Amazon Neptune | [Guide](https://docs.aws.amazon.com/neptune/latest/userguide/) | [API Ref](https://docs.aws.amazon.com/neptune/latest/apiref/) | ❌ | Uses Gremlin, SPARQL, or openCypher protocols; requires graph client |
| Amazon Keyspaces | [Guide](https://docs.aws.amazon.com/keyspaces/latest/devguide/) | [API Ref](https://docs.aws.amazon.com/keyspaces/latest/APIReference/) | ❌ | Uses Cassandra CQL wire protocol; requires Cassandra driver |

---

## Messaging & Streaming

| Service | Developer Guide | API Reference | Native Data Plane API | Notes |
|---|---|---|---|---|
| Amazon SQS | [Guide](https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/) | [API Ref](https://docs.aws.amazon.com/AWSSimpleQueueService/latest/APIReference/) | ✅ | `SendMessage`, `ReceiveMessage`, `DeleteMessage`, `ChangeMessageVisibility`, `SendMessageBatch` |
| Amazon SNS | [Guide](https://docs.aws.amazon.com/sns/latest/dg/) | [API Ref](https://docs.aws.amazon.com/sns/latest/api/) | ✅ | `Publish`, `PublishBatch` |
| Amazon Kinesis Data Streams | [Guide](https://docs.aws.amazon.com/streams/latest/dev/) | [API Ref](https://docs.aws.amazon.com/kinesis/latest/APIReference/) | ✅ | `PutRecord`, `PutRecords`, `GetRecords`, `GetShardIterator`, `SubscribeToShard` |
| Amazon Data Firehose | [Guide](https://docs.aws.amazon.com/firehose/latest/dev/) | [API Ref](https://docs.aws.amazon.com/firehose/latest/APIReference/) | ✅ | `PutRecord`, `PutRecordBatch` |
| Amazon Kinesis Video Streams | [Guide](https://docs.aws.amazon.com/kinesisvideostreams/latest/dg/) | [API Ref](https://docs.aws.amazon.com/kinesisvideostreams/latest/APIReference/) | ✅ | `PutMedia`, `GetMedia`, `GetHLSStreamingSessionURL`, `GetDASHStreamingSessionURL` |
| Amazon EventBridge | [Guide](https://docs.aws.amazon.com/eventbridge/latest/userguide/) | [API Ref](https://docs.aws.amazon.com/eventbridge/latest/APIReference/) | ✅ | `PutEvents` |
| Amazon MSK (Managed Kafka) | [Guide](https://docs.aws.amazon.com/msk/latest/developerguide/) | [API Ref](https://docs.aws.amazon.com/msk/1.0/apireference/) | ❌ | Data plane uses Apache Kafka binary protocol; requires Kafka producer/consumer client |
| Amazon MQ | [Guide](https://docs.aws.amazon.com/amazon-mq/latest/developer-guide/) | [API Ref](https://docs.aws.amazon.com/amazon-mq/latest/api-reference/) | ❌ | Data plane uses AMQP, OpenWire, MQTT, STOMP; requires ActiveMQ / RabbitMQ client |

---

## Compute & Serverless

| Service | Developer Guide | API Reference | Native Data Plane API | Notes |
|---|---|---|---|---|
| AWS Lambda | [Guide](https://docs.aws.amazon.com/lambda/latest/dg/) | [API Ref](https://docs.aws.amazon.com/lambda/latest/api/) | ✅ | `Invoke`, `InvokeWithResponseStream` |
| AWS Step Functions | [Guide](https://docs.aws.amazon.com/step-functions/latest/dg/) | [API Ref](https://docs.aws.amazon.com/step-functions/latest/apireference/) | ✅ | `StartExecution`, `SendTaskSuccess`, `SendTaskFailure`, `SendTaskHeartbeat` |
| Amazon EKS | [Guide](https://docs.aws.amazon.com/eks/latest/userguide/) | [API Ref](https://docs.aws.amazon.com/eks/latest/APIReference/) | ❌ | Data plane uses the Kubernetes API; requires `kubectl` or k8s client libraries |
| Amazon EMR | [Guide](https://docs.aws.amazon.com/emr/latest/ManagementGuide/) | [API Ref](https://docs.aws.amazon.com/emr/latest/APIReference/) | ❌ | Data plane uses Spark, Hadoop, Hive, Presto/Trino APIs |

---

## AI / Machine Learning

| Service | Developer Guide | API Reference | Native Data Plane API | Notes |
|---|---|---|---|---|
| Amazon Bedrock Runtime | [Guide](https://docs.aws.amazon.com/bedrock/latest/userguide/) | [API Ref](https://docs.aws.amazon.com/bedrock/latest/APIReference/) | ✅ | `InvokeModel`, `InvokeModelWithResponseStream`, `Converse`, `ConverseStream` |
| Amazon Bedrock Agent Runtime | [Guide](https://docs.aws.amazon.com/bedrock/latest/userguide/) | [API Ref](https://docs.aws.amazon.com/bedrock/latest/APIReference/) | ✅ | `InvokeAgent`, `InvokeFlow`, `Retrieve`, `RetrieveAndGenerate` |
| Amazon SageMaker Runtime | [Guide](https://docs.aws.amazon.com/sagemaker/latest/dg/) | [API Ref](https://docs.aws.amazon.com/sagemaker/latest/APIReference/) | ✅ | `InvokeEndpoint`, `InvokeEndpointAsync`, `InvokeEndpointWithResponseStream` |
| Amazon SageMaker Feature Store Runtime | [Guide](https://docs.aws.amazon.com/sagemaker/latest/dg/) | [API Ref](https://docs.aws.amazon.com/sagemaker/latest/APIReference/) | ✅ | `GetRecord`, `PutRecord`, `DeleteRecord`, `BatchGetRecord` |
| Amazon Comprehend | [Guide](https://docs.aws.amazon.com/comprehend/latest/dg/) | [API Ref](https://docs.aws.amazon.com/comprehend/latest/APIReference/) | ✅ | `DetectSentiment`, `DetectEntities`, `DetectKeyPhrases`, `ClassifyDocument`, `DetectPiiEntities` |
| Amazon Comprehend Medical | [Guide](https://docs.aws.amazon.com/comprehend/latest/dg/) | [API Ref](https://docs.aws.amazon.com/comprehend-medical/latest/APIReference/) | ✅ | `DetectEntitiesV2`, `DetectPHI`, `InferICD10CM`, `InferRxNorm` |
| Amazon Rekognition | [Guide](https://docs.aws.amazon.com/rekognition/latest/dg/) | [API Ref](https://docs.aws.amazon.com/rekognition/latest/APIReference/) | ✅ | `DetectLabels`, `DetectFaces`, `SearchFacesByImage`, `DetectText`, `DetectModerationLabels` |
| Amazon Textract | [Guide](https://docs.aws.amazon.com/textract/latest/dg/) | [API Ref](https://docs.aws.amazon.com/textract/latest/APIReference/) | ✅ | `AnalyzeDocument`, `DetectDocumentText`, `AnalyzeExpense`, `AnalyzeID` |
| Amazon Polly | [Guide](https://docs.aws.amazon.com/polly/latest/dg/) | [API Ref](https://docs.aws.amazon.com/polly/latest/dg/API_Reference.html) | ✅ | `SynthesizeSpeech` |
| Amazon Transcribe | [Guide](https://docs.aws.amazon.com/transcribe/latest/dg/) | [API Ref](https://docs.aws.amazon.com/transcribe/latest/APIReference/) | ✅ | `StartTranscriptionJob`, `StartStreamTranscription` (real-time) |
| Amazon Translate | [Guide](https://docs.aws.amazon.com/translate/latest/dg/) | [API Ref](https://docs.aws.amazon.com/translate/latest/APIReference/) | ✅ | `TranslateText`, `TranslateDocument` |
| Amazon Lex V2 Runtime | [Guide](https://docs.aws.amazon.com/lexv2/latest/dg/) | [API Ref](https://docs.aws.amazon.com/lexv2/latest/APIReference/) | ✅ | `RecognizeText`, `RecognizeUtterance`, `StartConversation` |
| Amazon Kendra | [Guide](https://docs.aws.amazon.com/kendra/latest/dg/) | [API Ref](https://docs.aws.amazon.com/kendra/latest/APIReference/) | ✅ | `Query`, `Retrieve` |
| Amazon Personalize Runtime | [Guide](https://docs.aws.amazon.com/personalize/latest/dg/) | [API Ref](https://docs.aws.amazon.com/personalize/latest/APIReference/) | ✅ | `GetRecommendations`, `GetPersonalizedRanking` |
| Amazon Lookout for Vision | [Guide](https://docs.aws.amazon.com/lookout-for-vision/latest/developer-guide/) | [API Ref](https://docs.aws.amazon.com/lookout-for-vision/latest/APIReference/) | ✅ | `DetectAnomalies` |
| Amazon Forecast | [Guide](https://docs.aws.amazon.com/forecast/latest/dg/) | [API Ref](https://docs.aws.amazon.com/forecast/latest/APIReference/) | ✅ | `QueryForecast`, `QueryWhatIfForecast` |

---

## Security & Cryptography

| Service | Developer Guide | API Reference | Native Data Plane API | Notes |
|---|---|---|---|---|
| AWS KMS | [Guide](https://docs.aws.amazon.com/kms/latest/developerguide/) | [API Ref](https://docs.aws.amazon.com/kms/latest/APIReference/) | ✅ | `Encrypt`, `Decrypt`, `GenerateDataKey`, `ReEncrypt`, `Sign`, `Verify`, `GenerateMac`, `VerifyMac` |
| AWS Secrets Manager | [Guide](https://docs.aws.amazon.com/secretsmanager/latest/userguide/) | [API Ref](https://docs.aws.amazon.com/secretsmanager/latest/apireference/) | ✅ | `GetSecretValue` |
| AWS Payment Cryptography (Data Plane) | [Guide](https://docs.aws.amazon.com/payment-cryptography/latest/userguide/) | [API Ref](https://docs.aws.amazon.com/payment-cryptography/latest/DataAPIReference/) | ✅ | `EncryptData`, `DecryptData`, `GenerateMac`, `VerifyMac`, `TranslateKeyMaterial` |
| Amazon Verified Permissions | [Guide](https://docs.aws.amazon.com/verifiedpermissions/latest/userguide/) | [API Ref](https://docs.aws.amazon.com/verifiedpermissions/latest/apireference/) | ✅ | `IsAuthorized`, `IsAuthorizedWithToken`, `BatchIsAuthorized`, `BatchIsAuthorizedWithToken` |

---

## Configuration

| Service | Developer Guide | API Reference | Native Data Plane API | Notes |
|---|---|---|---|---|
| AWS Systems Manager Parameter Store | [Guide](https://docs.aws.amazon.com/systems-manager/latest/userguide/) | [API Ref](https://docs.aws.amazon.com/systems-manager/latest/APIReference/) | ✅ | `GetParameter`, `GetParameters`, `GetParametersByPath` |
| AWS AppConfig | [Guide](https://docs.aws.amazon.com/appconfig/latest/userguide/) | [API Ref](https://docs.aws.amazon.com/appconfig/latest/APIReference/) | ✅ | `StartConfigurationSession`, `GetLatestConfiguration` |

---

## In-Memory / Caching

| Service | Developer Guide | API Reference | Native Data Plane API | Notes |
|---|---|---|---|---|
| Amazon ElastiCache | [Guide](https://docs.aws.amazon.com/AmazonElastiCache/latest/dg/) | [API Ref](https://docs.aws.amazon.com/AmazonElastiCache/latest/APIReference/) | ❌ | Data plane uses Redis RESP protocol or Memcached protocol; requires Redis / Memcached client |
| Amazon MemoryDB | [Guide](https://docs.aws.amazon.com/memorydb/latest/devguide/) | [API Ref](https://docs.aws.amazon.com/memorydb/latest/APIReference/) | ❌ | Data plane uses Redis RESP protocol; requires Redis client |

---

## IoT

| Service | Developer Guide | API Reference | Native Data Plane API | Notes |
|---|---|---|---|---|
| AWS IoT Core (Data Plane REST) | [Guide](https://docs.aws.amazon.com/iot/latest/developerguide/) | [API Ref](https://docs.aws.amazon.com/iot/latest/apireference/API_Operations_AWS_IoT_Data_Plane.html) | ✅ | `GetThingShadow`, `UpdateThingShadow`, `DeleteThingShadow`, `Publish` (HTTPS), `GetRetainedMessage` |
| AWS IoT Jobs (Data Plane) | [Guide](https://docs.aws.amazon.com/iot/latest/developerguide/) | [API Ref](https://docs.aws.amazon.com/iot/latest/apireference/API_Operations_AWS_IoT_Jobs_Data_Plane.html) | ✅ | `GetPendingJobExecutions`, `StartNextPendingJobExecution`, `DescribeJobExecution`, `UpdateJobExecution` |
| AWS IoT SiteWise | [Guide](https://docs.aws.amazon.com/iot-sitewise/latest/userguide/) | [API Ref](https://docs.aws.amazon.com/iot-sitewise/latest/APIReference/) | ✅ | `BatchPutAssetPropertyValues`, `GetAssetPropertyValue`, `BatchGetAssetPropertyValue` |
| AWS IoT Core (MQTT) | [Guide](https://docs.aws.amazon.com/iot/latest/developerguide/) | [API Ref](https://docs.aws.amazon.com/iot/latest/apireference/) | ❌ | MQTT is an open protocol; requires an MQTT client (AWS IoT SDK wraps this) |

---

## Observability

| Service | Developer Guide | API Reference | Native Data Plane API | Notes |
|---|---|---|---|---|
| Amazon CloudWatch Metrics | [Guide](https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/) | [API Ref](https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference/) | ✅ | `PutMetricData`, `GetMetricData`, `GetMetricStatistics` |
| Amazon CloudWatch Logs | [Guide](https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/) | [API Ref](https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/) | ✅ | `PutLogEvents`, `GetLogEvents`, `FilterLogEvents`, `StartQuery`, `GetQueryResults` |
| AWS X-Ray | [Guide](https://docs.aws.amazon.com/xray/latest/devguide/) | [API Ref](https://docs.aws.amazon.com/xray/latest/api/) | ✅ | `PutTraceSegments`, `PutTelemetryRecords` |

---

## Networking (Infrastructure-Level Data Plane)

These services have data planes that operate at the network layer — they route/resolve traffic automatically and do not expose a client-invokable data API.

| Service | Developer Guide | API Reference | Native Data Plane API | Notes |
|---|---|---|---|---|
| Amazon Route 53 | [Guide](https://docs.aws.amazon.com/Route53/latest/DeveloperGuide/) | [API Ref](https://docs.aws.amazon.com/Route53/latest/APIReference/) | ❌ (infra) | Data plane = DNS resolution; no client-callable API — DNS clients resolve automatically |
| Elastic Load Balancing | [Guide](https://docs.aws.amazon.com/elasticloadbalancing/latest/userguide/) | [API Ref](https://docs.aws.amazon.com/elasticloadbalancing/latest/APIReference/) | ❌ (infra) | Data plane = request routing; no client-callable data plane API |
| Amazon CloudFront | [Guide](https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/) | [API Ref](https://docs.aws.amazon.com/cloudfront/latest/APIReference/) | ❌ (infra) | Data plane = content distribution; no client-callable data plane API |

---

## Geospatial

| Service | Developer Guide | API Reference | Native Data Plane API | Notes |
|---|---|---|---|---|
| Amazon Location Service | [Guide](https://docs.aws.amazon.com/location/latest/developerguide/) | [API Ref](https://docs.aws.amazon.com/location/latest/APIReference/) | ✅ | `SearchPlaceIndexForText`, `SearchPlaceIndexForPosition`, `CalculateRoute`, `GetMapTile` |

---

## Communication

| Service | Developer Guide | API Reference | Native Data Plane API | Notes |
|---|---|---|---|---|
| Amazon SES | [Guide](https://docs.aws.amazon.com/ses/latest/dg/) | [API Ref](https://docs.aws.amazon.com/ses/latest/APIReference/) | ✅ | `SendEmail`, `SendRawEmail`, `SendTemplatedEmail`, `SendBulkEmail` |
| Amazon Pinpoint | [Guide](https://docs.aws.amazon.com/pinpoint/latest/developerguide/) | [API Ref](https://docs.aws.amazon.com/pinpoint/latest/apireference/) | ✅ | `SendMessages`, `SendUsersMessages` |

---

## Identity

| Service | Developer Guide | API Reference | Native Data Plane API | Notes |
|---|---|---|---|---|
| Amazon Cognito User Pools | [Guide](https://docs.aws.amazon.com/cognito/latest/developerguide/) | [API Ref](https://docs.aws.amazon.com/cognito-user-identity-pools/latest/APIReference/) | ✅ | `InitiateAuth`, `RespondToAuthChallenge`, `GetUser`, `GlobalSignOut` |
| Amazon Cognito Identity | [Guide](https://docs.aws.amazon.com/cognito/latest/developerguide/) | [API Ref](https://docs.aws.amazon.com/cognitoidentity/latest/APIReference/) | ✅ | `GetId`, `GetCredentialsForIdentity` |

---

## Containers

| Service | Developer Guide | API Reference | Native Data Plane API | Notes |
|---|---|---|---|---|
| Amazon ECR | [Guide](https://docs.aws.amazon.com/AmazonECR/latest/userguide/) | [API Ref](https://docs.aws.amazon.com/AmazonECR/latest/APIReference/) | ✅ | `GetAuthorizationToken`, `BatchGetImage`, `GetDownloadUrlForLayer` |

---

## Application Recovery

| Service | Developer Guide | API Reference | Native Data Plane API | Notes |
|---|---|---|---|---|
| Amazon ARC (Application Recovery Controller) | [Guide](https://docs.aws.amazon.com/r53recovery/latest/dg/) | [API Ref](https://docs.aws.amazon.com/routing-control/latest/APIReference/) | ✅ | `GetRoutingControl`, `UpdateRoutingControl`, `BatchUpdateRoutingControl` — dedicated data plane cluster endpoint separate from control plane |

---

## Blockchain

| Service | Developer Guide | API Reference | Native Data Plane API | Notes |
|---|---|---|---|---|
| AWS Managed Blockchain | [Guide](https://docs.aws.amazon.com/managed-blockchain/latest/hyperledger-fabric-dev/) | [API Ref](https://docs.aws.amazon.com/managed-blockchain/latest/APIReference/) | ❌ | Data plane uses Hyperledger Fabric SDK or Ethereum Web3 API |

---

## Summary Count

| Classification | Count |
|---|---|
| ✅ Native AWS Data Plane API | 47 |
| ❌ Non-Native (open-source protocol or infrastructure-level) | 16 |
