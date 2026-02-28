# Storage Services

This category covers AWS storage services including object storage, file systems, backup, and gateway services. All data is held in memory with no disk persistence.

---

## S3

| | |
|---|---|
| **Port** | `9000` |
| **Protocol** | REST XML |
| **Endpoint** | `http://localhost:9000` |

### Supported Operations (26)

| Operation | Description |
|-----------|-------------|
| CreateBucket | Create a new S3 bucket |
| DeleteBucket | Delete an empty bucket |
| HeadBucket | Check if a bucket exists and return its region |
| ListBuckets | List all buckets owned by the account |
| ListObjectsV2 | List objects in a bucket with prefix, delimiter, and pagination support |
| GetBucketLocation | Get the region constraint of a bucket |
| PutBucketVersioning | Enable or suspend versioning on a bucket |
| GetBucketVersioning | Get the versioning status of a bucket |
| PutBucketTagging | Set tags on a bucket |
| GetBucketTagging | Get the tags on a bucket |
| DeleteBucketTagging | Remove all tags from a bucket |
| PutObject | Upload an object to a bucket (up to 5 GB) |
| GetObject | Retrieve an object, with support for range requests |
| DeleteObject | Delete an object from a bucket |
| HeadObject | Retrieve object metadata without the body |
| CopyObject | Copy an object between buckets or within a bucket |
| PutObjectTagging | Set tags on an object |
| GetObjectTagging | Get the tags on an object |
| DeleteObjectTagging | Remove all tags from an object |
| DeleteObjects | Batch delete up to 1000 objects in a single request |
| CreateMultipartUpload | Initiate a multipart upload |
| UploadPart | Upload a part in a multipart upload |
| CompleteMultipartUpload | Complete a multipart upload by assembling parts |
| AbortMultipartUpload | Abort a multipart upload and discard parts |
| ListMultipartUploads | List in-progress multipart uploads for a bucket |
| ListParts | List uploaded parts for a multipart upload |

### Wire Protocol Details

S3 uses path-style REST XML over HTTP. Bucket operations use `/{bucket}` paths and object operations use `/{bucket}/{key}` paths. The service supports:

- **Content-Type detection**: automatically set on upload via headers
- **Custom metadata**: stored via `x-amz-meta-*` headers
- **Range requests**: `Range: bytes=start-end` header for partial downloads (returns HTTP 206)
- **Copy source**: `x-amz-copy-source: /bucket/key` header with optional metadata directive
- **ETags**: MD5-based ETags returned on upload
- **Max body size**: 5 GB per request

### Usage with AWS CLI

```bash
# Create a bucket
aws s3api create-bucket \
  --bucket my-bucket \
  --endpoint-url http://localhost:9000 \
  --no-sign-request

# Upload an object
aws s3api put-object \
  --bucket my-bucket \
  --key hello.txt \
  --body hello.txt \
  --endpoint-url http://localhost:9000 \
  --no-sign-request

# Download an object
aws s3api get-object \
  --bucket my-bucket \
  --key hello.txt \
  output.txt \
  --endpoint-url http://localhost:9000 \
  --no-sign-request

# List objects
aws s3api list-objects-v2 \
  --bucket my-bucket \
  --prefix "folder/" \
  --endpoint-url http://localhost:9000 \
  --no-sign-request

# Copy an object
aws s3api copy-object \
  --bucket dest-bucket \
  --key copied.txt \
  --copy-source my-bucket/hello.txt \
  --endpoint-url http://localhost:9000 \
  --no-sign-request

# Delete an object
aws s3api delete-object \
  --bucket my-bucket \
  --key hello.txt \
  --endpoint-url http://localhost:9000 \
  --no-sign-request

# Delete a bucket
aws s3api delete-bucket \
  --bucket my-bucket \
  --endpoint-url http://localhost:9000 \
  --no-sign-request

# Bucket versioning
aws s3api put-bucket-versioning \
  --bucket my-bucket \
  --versioning-configuration Status=Enabled \
  --endpoint-url http://localhost:9000 \
  --no-sign-request

# Bucket tagging
aws s3api put-bucket-tagging \
  --bucket my-bucket \
  --tagging 'TagSet=[{Key=env,Value=test}]' \
  --endpoint-url http://localhost:9000 \
  --no-sign-request
```

### Usage with AWS SDK (JavaScript)

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

### Limitations

- Versioning status can be toggled but version history is not maintained. Only the latest version of each object is stored.
- No server-side encryption is performed (encryption attributes are accepted but not applied).
- No bucket policies or ACLs are enforced.

---

## EFS

| | |
|---|---|
| **Port** | `9600` |
| **Protocol** | REST JSON |
| **Endpoint** | `http://localhost:9600` |

### Supported Operations (15)

| Operation | Description |
|-----------|-------------|
| CreateFileSystem | Create a new EFS file system |
| DescribeFileSystems | List file systems, optionally filtered by ID or creation token |
| UpdateFileSystem | Update file system properties (e.g., throughput mode) |
| DeleteFileSystem | Delete a file system |
| CreateMountTarget | Create a mount target for a file system in a subnet |
| DescribeMountTargets | List mount targets, filtered by ID or file system |
| DeleteMountTarget | Delete a mount target |
| CreateAccessPoint | Create an access point for a file system |
| DescribeAccessPoints | List access points, filtered by ID or file system |
| DeleteAccessPoint | Delete an access point |
| TagResource | Add tags to an EFS resource |
| UntagResource | Remove tags from an EFS resource |
| ListTagsForResource | List all tags on an EFS resource |
| PutLifecycleConfiguration | Set lifecycle policies on a file system |
| DescribeLifecycleConfiguration | Get lifecycle policies for a file system |

### Wire Protocol Details

EFS uses REST JSON with versioned URL paths prefixed with `/2015-02-01/`. File system operations use `/2015-02-01/file-systems`, mount target operations use `/2015-02-01/mount-targets`, and access point operations use `/2015-02-01/access-points`. Tag operations use `/2015-02-01/resource-tags/{ResourceId}`. Lifecycle configuration is managed at `/2015-02-01/file-systems/{FileSystemId}/lifecycle-configuration`.

### Usage with AWS CLI

```bash
# Create a file system
aws efs create-file-system \
  --creation-token my-fs \
  --performance-mode generalPurpose \
  --endpoint-url http://localhost:9600 \
  --no-sign-request

# Describe file systems
aws efs describe-file-systems \
  --endpoint-url http://localhost:9600 \
  --no-sign-request

# Create a mount target
aws efs create-mount-target \
  --file-system-id fs-12345678 \
  --subnet-id subnet-12345678 \
  --endpoint-url http://localhost:9600 \
  --no-sign-request

# Describe mount targets
aws efs describe-mount-targets \
  --file-system-id fs-12345678 \
  --endpoint-url http://localhost:9600 \
  --no-sign-request

# Create an access point
aws efs create-access-point \
  --file-system-id fs-12345678 \
  --client-token my-ap \
  --endpoint-url http://localhost:9600 \
  --no-sign-request

# Tag a resource
aws efs tag-resource \
  --resource-id fs-12345678 \
  --tags Key=env,Value=test \
  --endpoint-url http://localhost:9600 \
  --no-sign-request

# Set lifecycle configuration
aws efs put-lifecycle-configuration \
  --file-system-id fs-12345678 \
  --lifecycle-policies TransitionToIA=AFTER_30_DAYS \
  --endpoint-url http://localhost:9600 \
  --no-sign-request

# Delete a file system
aws efs delete-file-system \
  --file-system-id fs-12345678 \
  --endpoint-url http://localhost:9600 \
  --no-sign-request
```

### Limitations

- File systems are immediately available with no provisioning delay.
- Mount targets are simulated (no actual NFS endpoint is created).
- No actual file storage or NFS protocol support.

---

## FSx

| | |
|---|---|
| **Port** | `10147` |
| **Protocol** | JSON RPC (`AWSSimbaAPIService_v20180301`) |
| **Endpoint** | `http://localhost:10147` |

### Supported Operations (4)

| Operation | Description |
|-----------|-------------|
| CreateFileSystem | Create a new FSx file system |
| DescribeFileSystems | Describe one or more FSx file systems |
| ListFileSystems | List all FSx file systems |
| DeleteFileSystem | Delete an FSx file system |

### Usage with AWS CLI

```bash
# Create a file system
aws fsx create-file-system \
  --file-system-type LUSTRE \
  --storage-capacity 1200 \
  --subnet-ids subnet-12345678 \
  --endpoint-url http://localhost:10147 \
  --no-sign-request

# Describe file systems
aws fsx describe-file-systems \
  --endpoint-url http://localhost:10147 \
  --no-sign-request

# Delete a file system
aws fsx delete-file-system \
  --file-system-id fs-12345678 \
  --endpoint-url http://localhost:10147 \
  --no-sign-request
```

---

## Backup

| | |
|---|---|
| **Port** | `10146` |
| **Protocol** | REST JSON |
| **Endpoint** | `http://localhost:10146` |

### Supported Operations (8)

| Operation | Description |
|-----------|-------------|
| CreateBackupVault | Create a new backup vault |
| GetBackupVault | Get details of a backup vault |
| ListBackupVaults | List all backup vaults |
| DeleteBackupVault | Delete a backup vault |
| CreateBackupPlan | Create a new backup plan |
| GetBackupPlan | Get details of a backup plan |
| ListBackupPlans | List all backup plans |
| DeleteBackupPlan | Delete a backup plan |

### Usage with AWS CLI

```bash
# Create a backup vault
aws backup create-backup-vault \
  --backup-vault-name my-vault \
  --endpoint-url http://localhost:10146 \
  --no-sign-request

# List backup vaults
aws backup list-backup-vaults \
  --endpoint-url http://localhost:10146 \
  --no-sign-request

# Create a backup plan
aws backup create-backup-plan \
  --backup-plan '{"BackupPlanName":"my-plan","Rules":[{"RuleName":"daily","TargetBackupVaultName":"my-vault","ScheduleExpression":"cron(0 12 * * ? *)"}]}' \
  --endpoint-url http://localhost:10146 \
  --no-sign-request

# List backup plans
aws backup list-backup-plans \
  --endpoint-url http://localhost:10146 \
  --no-sign-request
```

---

## Storage Gateway

| | |
|---|---|
| **Port** | `10148` |
| **Protocol** | JSON RPC (`StorageGateway_20130630`) |
| **Endpoint** | `http://localhost:10148` |

### Supported Operations (4)

| Operation | Description |
|-----------|-------------|
| CreateGateway | Create a new storage gateway |
| DescribeGateway | Describe a storage gateway |
| ListGateways | List all storage gateways |
| DeleteGateway | Delete a storage gateway |

### Usage with AWS CLI

```bash
# Create a gateway
aws storagegateway create-gateway \
  --gateway-name my-gateway \
  --gateway-type FILE_S3 \
  --endpoint-url http://localhost:10148 \
  --no-sign-request

# List gateways
aws storagegateway list-gateways \
  --endpoint-url http://localhost:10148 \
  --no-sign-request

# Delete a gateway
aws storagegateway delete-gateway \
  --gateway-arn arn:aws:storagegateway:us-east-1:000000000000:gateway/sgw-12345678 \
  --endpoint-url http://localhost:10148 \
  --no-sign-request
```
