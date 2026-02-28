# Machine Learning & AI

This category covers AWS machine learning and artificial intelligence services. All services store state in memory with no persistence.

---

## SageMaker

| | |
|---|---|
| **Port** | `10102` |
| **Protocol** | JSON RPC (`SageMaker`) |
| **Endpoint** | `http://localhost:10102` |

### Supported Operations (4)

| Operation | Description |
|-----------|-------------|
| CreateNotebookInstance | Create a new SageMaker notebook instance |
| DescribeNotebookInstance | Describe a notebook instance |
| ListNotebookInstances | List all notebook instances |
| DeleteNotebookInstance | Delete a notebook instance |

### Usage with AWS CLI

```bash
# Create a notebook instance
aws sagemaker create-notebook-instance \
  --notebook-instance-name my-notebook \
  --instance-type ml.t2.medium \
  --role-arn arn:aws:iam::000000000000:role/SageMakerRole \
  --endpoint-url http://localhost:10102 \
  --no-sign-request

# List notebook instances
aws sagemaker list-notebook-instances \
  --endpoint-url http://localhost:10102 \
  --no-sign-request

# Delete a notebook instance
aws sagemaker delete-notebook-instance \
  --notebook-instance-name my-notebook \
  --endpoint-url http://localhost:10102 \
  --no-sign-request
```

---

## Bedrock

| | |
|---|---|
| **Port** | `10093` |
| **Protocol** | REST JSON |
| **Endpoint** | `http://localhost:10093` |

### Supported Operations (4)

| Operation | Description |
|-----------|-------------|
| CreateModelCustomizationJob | Create a model customization job |
| GetModelCustomizationJob | Get details of a model customization job |
| ListModelCustomizationJobs | List all model customization jobs |
| DeleteModelCustomizationJob | Delete a model customization job |

### Usage with AWS CLI

```bash
# Create a model customization job
aws bedrock create-model-customization-job \
  --job-name my-job \
  --custom-model-name my-model \
  --role-arn arn:aws:iam::000000000000:role/BedrockRole \
  --base-model-identifier anthropic.claude-v2 \
  --training-data-config '{"s3Uri":"s3://bucket/data"}' \
  --output-data-config '{"s3Uri":"s3://bucket/output"}' \
  --hyper-parameters '{}' \
  --endpoint-url http://localhost:10093 \
  --no-sign-request

# List model customization jobs
aws bedrock list-model-customization-jobs \
  --endpoint-url http://localhost:10093 \
  --no-sign-request
```

---

## Comprehend

| | |
|---|---|
| **Port** | `10094` |
| **Protocol** | JSON RPC (`Comprehend_20171127`) |
| **Endpoint** | `http://localhost:10094` |

### Supported Operations (4)

| Operation | Description |
|-----------|-------------|
| CreateDocumentClassifier | Create a document classifier |
| DescribeDocumentClassifier | Describe a document classifier |
| ListDocumentClassifiers | List all document classifiers |
| DeleteDocumentClassifier | Delete a document classifier |

### Usage with AWS CLI

```bash
# List document classifiers
aws comprehend list-document-classifiers \
  --endpoint-url http://localhost:10094 \
  --no-sign-request
```

---

## Rekognition

| | |
|---|---|
| **Port** | `10101` |
| **Protocol** | JSON RPC (`RekognitionService`) |
| **Endpoint** | `http://localhost:10101` |

### Supported Operations (4)

| Operation | Description |
|-----------|-------------|
| CreateCollection | Create a face collection |
| DescribeCollection | Describe a face collection |
| ListCollections | List all face collections |
| DeleteCollection | Delete a face collection |

### Usage with AWS CLI

```bash
# Create a collection
aws rekognition create-collection \
  --collection-id my-collection \
  --endpoint-url http://localhost:10101 \
  --no-sign-request

# List collections
aws rekognition list-collections \
  --endpoint-url http://localhost:10101 \
  --no-sign-request

# Delete a collection
aws rekognition delete-collection \
  --collection-id my-collection \
  --endpoint-url http://localhost:10101 \
  --no-sign-request
```

---

## Textract

| | |
|---|---|
| **Port** | `10103` |
| **Protocol** | JSON RPC (`Textract`) |
| **Endpoint** | `http://localhost:10103` |

### Supported Operations (4)

| Operation | Description |
|-----------|-------------|
| CreateAdapter | Create a Textract adapter |
| DescribeAdapter | Describe a Textract adapter |
| ListAdapters | List all Textract adapters |
| DeleteAdapter | Delete a Textract adapter |

### Usage with AWS CLI

```bash
# List adapters
aws textract list-adapters \
  --endpoint-url http://localhost:10103 \
  --no-sign-request
```

---

## Transcribe

| | |
|---|---|
| **Port** | `10104` |
| **Protocol** | JSON RPC (`Transcribe`) |
| **Endpoint** | `http://localhost:10104` |

### Supported Operations (4)

| Operation | Description |
|-----------|-------------|
| CreateTranscriptionJob | Create a transcription job |
| DescribeTranscriptionJob | Describe a transcription job |
| ListTranscriptionJobs | List all transcription jobs |
| DeleteTranscriptionJob | Delete a transcription job |

### Usage with AWS CLI

```bash
# List transcription jobs
aws transcribe list-transcription-jobs \
  --endpoint-url http://localhost:10104 \
  --no-sign-request
```

---

## Translate

| | |
|---|---|
| **Port** | `10105` |
| **Protocol** | JSON RPC (`AWSShineFrontendService_20170701`) |
| **Endpoint** | `http://localhost:10105` |

### Supported Operations (4)

| Operation | Description |
|-----------|-------------|
| CreateTerminology | Create a custom terminology |
| DescribeTerminology | Describe a custom terminology |
| ListTerminologys | List all custom terminologies |
| DeleteTerminology | Delete a custom terminology |

### Usage with AWS CLI

```bash
# List terminologies
aws translate list-terminologies \
  --endpoint-url http://localhost:10105 \
  --no-sign-request
```

---

## Polly

| | |
|---|---|
| **Port** | `10100` |
| **Protocol** | REST JSON |
| **Endpoint** | `http://localhost:10100` |

### Supported Operations (4)

| Operation | Description |
|-----------|-------------|
| CreateLexicon | Create a pronunciation lexicon |
| ListLexicons | List all pronunciation lexicons |
| GetLexicon | Get a pronunciation lexicon |
| DeleteLexicon | Delete a pronunciation lexicon |

### Usage with AWS CLI

```bash
# List lexicons
aws polly list-lexicons \
  --endpoint-url http://localhost:10100 \
  --no-sign-request
```

---

## Lex

| | |
|---|---|
| **Port** | `10098` |
| **Protocol** | REST JSON |
| **Endpoint** | `http://localhost:10098` |

### Supported Operations (4)

| Operation | Description |
|-----------|-------------|
| CreateBot | Create a Lex bot |
| ListBots | List all Lex bots |
| GetBot | Get details of a Lex bot |
| DeleteBot | Delete a Lex bot |

### Usage with AWS CLI

```bash
# List bots
aws lexv2-models list-bots \
  --endpoint-url http://localhost:10098 \
  --no-sign-request
```

---

## Kendra

| | |
|---|---|
| **Port** | `10097` |
| **Protocol** | JSON RPC (`AWSKendraFrontendService`) |
| **Endpoint** | `http://localhost:10097` |

### Supported Operations (4)

| Operation | Description |
|-----------|-------------|
| CreateIndex | Create a Kendra index |
| DescribeIndex | Describe a Kendra index |
| ListIndexs | List all Kendra indexes |
| DeleteIndex | Delete a Kendra index |

### Usage with AWS CLI

```bash
# List indexes
aws kendra list-indices \
  --endpoint-url http://localhost:10097 \
  --no-sign-request
```

---

## Personalize

| | |
|---|---|
| **Port** | `10099` |
| **Protocol** | JSON RPC (`AmazonPersonalize`) |
| **Endpoint** | `http://localhost:10099` |

### Supported Operations (4)

| Operation | Description |
|-----------|-------------|
| CreateDataset | Create a Personalize dataset |
| DescribeDataset | Describe a Personalize dataset |
| ListDatasets | List all Personalize datasets |
| DeleteDataset | Delete a Personalize dataset |

### Usage with AWS CLI

```bash
# List datasets
aws personalize list-datasets \
  --endpoint-url http://localhost:10099 \
  --no-sign-request
```

---

## Forecast

| | |
|---|---|
| **Port** | `10095` |
| **Protocol** | JSON RPC (`AmazonForecast`) |
| **Endpoint** | `http://localhost:10095` |

### Supported Operations (4)

| Operation | Description |
|-----------|-------------|
| CreateDataset | Create a Forecast dataset |
| DescribeDataset | Describe a Forecast dataset |
| ListDatasets | List all Forecast datasets |
| DeleteDataset | Delete a Forecast dataset |

### Usage with AWS CLI

```bash
# List datasets
aws forecast list-datasets \
  --endpoint-url http://localhost:10095 \
  --no-sign-request
```

---

## Fraud Detector

| | |
|---|---|
| **Port** | `10096` |
| **Protocol** | JSON RPC (`AWSHawksNestServiceFacade`) |
| **Endpoint** | `http://localhost:10096` |

### Supported Operations (4)

| Operation | Description |
|-----------|-------------|
| CreateDetector | Create a fraud detector |
| DescribeDetector | Describe a fraud detector |
| ListDetectors | List all fraud detectors |
| DeleteDetector | Delete a fraud detector |

### Usage with AWS CLI

```bash
# List detectors
aws frauddetector get-detectors \
  --endpoint-url http://localhost:10096 \
  --no-sign-request
```

---

## HealthLake

| | |
|---|---|
| **Port** | `10107` |
| **Protocol** | JSON RPC (`HealthLake`) |
| **Endpoint** | `http://localhost:10107` |

### Supported Operations (4)

| Operation | Description |
|-----------|-------------|
| CreateFHIRDatastore | Create a FHIR datastore |
| DescribeFHIRDatastore | Describe a FHIR datastore |
| ListFHIRDatastores | List all FHIR datastores |
| DeleteFHIRDatastore | Delete a FHIR datastore |

### Usage with AWS CLI

```bash
# List FHIR datastores
aws healthlake list-fhir-datastores \
  --endpoint-url http://localhost:10107 \
  --no-sign-request
```
