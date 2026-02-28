# Compute Services

## EC2

| | |
|---|---|
| **Port** | `10001` |
| **Protocol** | Query/XML (Action parameter) |
| **Endpoint** | `http://localhost:10001` |

### Supported Operations (9)

| Operation | Description |
|-----------|-------------|
| CreateInstance | Create a new EC2 instance |
| DescribeInstances | List and describe EC2 instances |
| DeleteInstance | Terminate an EC2 instance |
| CreateVpc | Create a new VPC |
| DescribeVpcs | List and describe VPCs |
| DeleteVpc | Delete a VPC |
| CreateSecurityGroup | Create a new security group |
| DescribeSecurityGroups | List and describe security groups |
| DeleteSecurityGroup | Delete a security group |

### Usage with AWS CLI

```bash
# Create an instance
aws ec2 run-instances \
  --image-id ami-12345678 \
  --instance-type t2.micro \
  --endpoint-url http://localhost:10001 \
  --no-sign-request

# Describe instances
aws ec2 describe-instances \
  --endpoint-url http://localhost:10001 \
  --no-sign-request

# Create a VPC
aws ec2 create-vpc \
  --cidr-block 10.0.0.0/16 \
  --endpoint-url http://localhost:10001 \
  --no-sign-request

# Create a security group
aws ec2 create-security-group \
  --group-name my-sg \
  --description "My security group" \
  --endpoint-url http://localhost:10001 \
  --no-sign-request
```

---

## ECS

| | |
|---|---|
| **Port** | `10003` |
| **Protocol** | JSON RPC (`AmazonEC2ContainerServiceV20141113`) |
| **Endpoint** | `http://localhost:10003` |

### Supported Operations (12)

| Operation | Description |
|-----------|-------------|
| CreateCluster | Create a new ECS cluster |
| DescribeCluster | Describe an ECS cluster |
| ListClusters | List all ECS clusters |
| DeleteCluster | Delete an ECS cluster |
| CreateService | Create a new ECS service |
| DescribeService | Describe an ECS service |
| ListServices | List all ECS services |
| DeleteService | Delete an ECS service |
| CreateTaskDefinition | Register a new task definition |
| DescribeTaskDefinition | Describe a task definition |
| ListTaskDefinitions | List all task definitions |
| DeleteTaskDefinition | Deregister a task definition |

### Usage with AWS CLI

```bash
# Create a cluster
aws ecs create-cluster \
  --cluster-name my-cluster \
  --endpoint-url http://localhost:10003 \
  --no-sign-request

# List clusters
aws ecs list-clusters \
  --endpoint-url http://localhost:10003 \
  --no-sign-request

# Register a task definition
aws ecs register-task-definition \
  --family my-task \
  --container-definitions '[{"name":"app","image":"nginx","memory":256}]' \
  --endpoint-url http://localhost:10003 \
  --no-sign-request
```

---

## EKS

| | |
|---|---|
| **Port** | `10004` |
| **Protocol** | REST JSON |
| **Endpoint** | `http://localhost:10004` |

### Supported Operations (4)

| Operation | Description |
|-----------|-------------|
| CreateCluster | Create a new EKS cluster |
| GetCluster | Describe an EKS cluster |
| ListClusters | List all EKS clusters |
| DeleteCluster | Delete an EKS cluster |

### Usage with AWS CLI

```bash
# Create a cluster
aws eks create-cluster \
  --name my-cluster \
  --role-arn arn:aws:iam::000000000000:role/eks-role \
  --resources-vpc-config subnetIds=subnet-1,securityGroupIds=sg-1 \
  --endpoint-url http://localhost:10004 \
  --no-sign-request

# List clusters
aws eks list-clusters \
  --endpoint-url http://localhost:10004 \
  --no-sign-request
```

---

## ECR

| | |
|---|---|
| **Port** | `10002` |
| **Protocol** | JSON RPC (`AmazonEC2ContainerRegistry_V20150921`) |
| **Endpoint** | `http://localhost:10002` |

### Supported Operations (4)

| Operation | Description |
|-----------|-------------|
| CreateRepository | Create a new ECR repository |
| DescribeRepository | Describe an ECR repository |
| ListRepositorys | List all ECR repositories |
| DeleteRepository | Delete an ECR repository |

### Usage with AWS CLI

```bash
# Create a repository
aws ecr create-repository \
  --repository-name my-repo \
  --endpoint-url http://localhost:10002 \
  --no-sign-request

# Describe repositories
aws ecr describe-repositories \
  --endpoint-url http://localhost:10002 \
  --no-sign-request
```

---

## Lambda

| | |
|---|---|
| **Port** | `9001` |
| **Protocol** | REST JSON |
| **Endpoint** | `http://localhost:9001` |

### Supported Operations (22)

| Operation | Description |
|-----------|-------------|
| CreateFunction | Create a new Lambda function |
| GetFunction | Get details of a Lambda function |
| ListFunctions | List all Lambda functions |
| DeleteFunction | Delete a Lambda function |
| UpdateFunctionCode | Update a function's code |
| UpdateFunctionConfiguration | Update a function's configuration |
| Invoke | Invoke a Lambda function (returns stub 200 response) |
| AddPermission | Add a resource-based policy statement |
| RemovePermission | Remove a resource-based policy statement |
| GetPolicy | Get the resource-based policy for a function |
| PublishVersion | Publish a new version of a function |
| ListVersionsByFunction | List all versions of a function |
| CreateAlias | Create an alias for a function version |
| GetAlias | Get details of an alias |
| ListAliases | List all aliases for a function |
| DeleteAlias | Delete an alias |
| CreateEventSourceMapping | Create an event source mapping |
| DeleteEventSourceMapping | Delete an event source mapping |
| ListEventSourceMappings | List event source mappings |
| TagResource | Add tags to a function |
| UntagResource | Remove tags from a function |
| ListTags | List tags for a function |

### Wire Protocol

Lambda uses REST JSON over HTTP. Requests are routed based on HTTP method and URL path:

- **Functions**: `POST /2015-03-31/functions` (create), `GET /2015-03-31/functions` (list)
- **Function by name**: `GET|DELETE /2015-03-31/functions/{name}`
- **Code**: `PUT /2015-03-31/functions/{name}/code`
- **Configuration**: `PUT /2015-03-31/functions/{name}/configuration`
- **Invoke**: `POST /2015-03-31/functions/{name}/invocations`
- **Policy**: `POST|GET /2015-03-31/functions/{name}/policy`, `DELETE /2015-03-31/functions/{name}/policy/{sid}`
- **Versions**: `POST|GET /2015-03-31/functions/{name}/versions`
- **Aliases**: `POST|GET /2015-03-31/functions/{name}/aliases`, `GET|DELETE /2015-03-31/functions/{name}/aliases/{alias_name}`
- **Event Source Mappings**: `POST|GET /2015-03-31/event-source-mappings`, `DELETE /2015-03-31/event-source-mappings/{uuid}`
- **Tags**: `POST|GET|DELETE /2017-03-31/tags/{arn}`

### Usage with AWS CLI

```bash
# Create a function
aws lambda create-function \
  --function-name my-func \
  --runtime python3.9 \
  --role arn:aws:iam::000000000000:role/lambda-role \
  --handler index.handler \
  --zip-file fileb://function.zip \
  --endpoint-url http://localhost:9001 \
  --no-sign-request

# List functions
aws lambda list-functions \
  --endpoint-url http://localhost:9001 \
  --no-sign-request

# Get function details
aws lambda get-function \
  --function-name my-func \
  --endpoint-url http://localhost:9001 \
  --no-sign-request

# Invoke a function
aws lambda invoke \
  --function-name my-func \
  --payload '{"key": "value"}' \
  output.json \
  --endpoint-url http://localhost:9001 \
  --no-sign-request

# Update function code
aws lambda update-function-code \
  --function-name my-func \
  --zip-file fileb://function.zip \
  --endpoint-url http://localhost:9001 \
  --no-sign-request

# Update function configuration
aws lambda update-function-configuration \
  --function-name my-func \
  --description "Updated function" \
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

# List aliases
aws lambda list-aliases \
  --function-name my-func \
  --endpoint-url http://localhost:9001 \
  --no-sign-request

# Add a permission
aws lambda add-permission \
  --function-name my-func \
  --statement-id stmt1 \
  --action lambda:InvokeFunction \
  --principal s3.amazonaws.com \
  --endpoint-url http://localhost:9001 \
  --no-sign-request

# Create event source mapping
aws lambda create-event-source-mapping \
  --function-name my-func \
  --event-source-arn arn:aws:sqs:us-east-1:000000000000:my-queue \
  --endpoint-url http://localhost:9001 \
  --no-sign-request

# Tag a function
aws lambda tag-resource \
  --resource arn:aws:lambda:us-east-1:000000000000:function:my-func \
  --tags env=test \
  --endpoint-url http://localhost:9001 \
  --no-sign-request

# Delete a function
aws lambda delete-function \
  --function-name my-func \
  --endpoint-url http://localhost:9001 \
  --no-sign-request
```

### Limitations

- `Invoke` returns a stub 200 response. Functions are not actually executed.
- Event source mappings are stored but events are not processed.

---

## Batch

| | |
|---|---|
| **Port** | `10007` |
| **Protocol** | REST JSON |
| **Endpoint** | `http://localhost:10007` |

### Supported Operations (8)

| Operation | Description |
|-----------|-------------|
| CreateComputeEnvironment | Create a compute environment |
| GetComputeEnvironment | Describe a compute environment |
| ListComputeEnvironments | List all compute environments |
| DeleteComputeEnvironment | Delete a compute environment |
| CreateJobQueue | Create a job queue |
| GetJobQueue | Describe a job queue |
| ListJobQueues | List all job queues |
| DeleteJobQueue | Delete a job queue |

### Usage with AWS CLI

```bash
# Create a compute environment
aws batch create-compute-environment \
  --compute-environment-name my-env \
  --type MANAGED \
  --endpoint-url http://localhost:10007 \
  --no-sign-request

# List compute environments
aws batch describe-compute-environments \
  --endpoint-url http://localhost:10007 \
  --no-sign-request

# Create a job queue
aws batch create-job-queue \
  --job-queue-name my-queue \
  --priority 1 \
  --compute-environment-order order=1,computeEnvironment=my-env \
  --endpoint-url http://localhost:10007 \
  --no-sign-request
```

---

## Lightsail

| | |
|---|---|
| **Port** | `10005` |
| **Protocol** | JSON RPC (`Lightsail_20161128`) |
| **Endpoint** | `http://localhost:10005` |

### Supported Operations (4)

| Operation | Description |
|-----------|-------------|
| CreateInstance | Create a Lightsail instance |
| DescribeInstance | Describe a Lightsail instance |
| ListInstances | List all Lightsail instances |
| DeleteInstance | Delete a Lightsail instance |

### Usage with AWS CLI

```bash
# Create an instance
aws lightsail create-instances \
  --instance-names my-instance \
  --availability-zone us-east-1a \
  --blueprint-id amazon_linux_2 \
  --bundle-id nano_2_0 \
  --endpoint-url http://localhost:10005 \
  --no-sign-request

# List instances
aws lightsail get-instances \
  --endpoint-url http://localhost:10005 \
  --no-sign-request
```

---

## Outposts

| | |
|---|---|
| **Port** | `10009` |
| **Protocol** | REST JSON |
| **Endpoint** | `http://localhost:10009` |

### Supported Operations (4)

| Operation | Description |
|-----------|-------------|
| CreateOutpost | Create an Outpost |
| ListOutposts | List all Outposts |
| GetOutpost | Get details of an Outpost |
| DeleteOutpost | Delete an Outpost |

### Usage with AWS CLI

```bash
# Create an outpost
aws outposts create-outpost \
  --name my-outpost \
  --site-id site-1 \
  --endpoint-url http://localhost:10009 \
  --no-sign-request

# List outposts
aws outposts list-outposts \
  --endpoint-url http://localhost:10009 \
  --no-sign-request
```

---

## Auto Scaling

| | |
|---|---|
| **Port** | `10011` |
| **Protocol** | Query/XML (Action parameter) |
| **Endpoint** | `http://localhost:10011` |

### Supported Operations (3)

| Operation | Description |
|-----------|-------------|
| CreateAutoScalingGroup | Create an Auto Scaling group |
| DescribeAutoScalingGroups | Describe Auto Scaling groups |
| DeleteAutoScalingGroup | Delete an Auto Scaling group |

### Usage with AWS CLI

```bash
# Create an Auto Scaling group
aws autoscaling create-auto-scaling-group \
  --auto-scaling-group-name my-asg \
  --min-size 1 --max-size 3 \
  --launch-template LaunchTemplateName=my-template \
  --endpoint-url http://localhost:10011 \
  --no-sign-request

# Describe Auto Scaling groups
aws autoscaling describe-auto-scaling-groups \
  --endpoint-url http://localhost:10011 \
  --no-sign-request
```

---

## Elastic Beanstalk

| | |
|---|---|
| **Port** | `10008` |
| **Protocol** | Query/XML (Action parameter) |
| **Endpoint** | `http://localhost:10008` |

### Supported Operations (3)

| Operation | Description |
|-----------|-------------|
| CreateApplication | Create an Elastic Beanstalk application |
| DescribeApplications | Describe Elastic Beanstalk applications |
| DeleteApplication | Delete an Elastic Beanstalk application |

### Usage with AWS CLI

```bash
# Create an application
aws elasticbeanstalk create-application \
  --application-name my-app \
  --endpoint-url http://localhost:10008 \
  --no-sign-request

# Describe applications
aws elasticbeanstalk describe-applications \
  --endpoint-url http://localhost:10008 \
  --no-sign-request
```

---

## App Runner

| | |
|---|---|
| **Port** | `10006` |
| **Protocol** | JSON RPC (`AppRunner`) |
| **Endpoint** | `http://localhost:10006` |

### Supported Operations (4)

| Operation | Description |
|-----------|-------------|
| CreateService | Create an App Runner service |
| DescribeService | Describe an App Runner service |
| ListServices | List all App Runner services |
| DeleteService | Delete an App Runner service |

### Usage with AWS CLI

```bash
# Create a service
aws apprunner create-service \
  --service-name my-service \
  --source-configuration '{"ImageRepository":{"ImageIdentifier":"nginx:latest","ImageRepositoryType":"ECR_PUBLIC"}}' \
  --endpoint-url http://localhost:10006 \
  --no-sign-request

# List services
aws apprunner list-services \
  --endpoint-url http://localhost:10006 \
  --no-sign-request
```

---

## EC2 Image Builder

| | |
|---|---|
| **Port** | `10010` |
| **Protocol** | REST JSON |
| **Endpoint** | `http://localhost:10010` |

### Supported Operations (4)

| Operation | Description |
|-----------|-------------|
| CreateImagePipeline | Create an image pipeline |
| ListImagePipelines | List all image pipelines |
| GetImagePipeline | Get details of an image pipeline |
| DeleteImagePipeline | Delete an image pipeline |

### Usage with AWS CLI

```bash
# Create an image pipeline
aws imagebuilder create-image-pipeline \
  --name my-pipeline \
  --image-recipe-arn arn:aws:imagebuilder:us-east-1:000000000000:image-recipe/my-recipe/1.0.0 \
  --infrastructure-configuration-arn arn:aws:imagebuilder:us-east-1:000000000000:infrastructure-configuration/my-config \
  --endpoint-url http://localhost:10010 \
  --no-sign-request

# List image pipelines
aws imagebuilder list-image-pipelines \
  --endpoint-url http://localhost:10010 \
  --no-sign-request
```

---

## GameLift

| | |
|---|---|
| **Port** | `10156` |
| **Protocol** | JSON RPC (`GameLift`) |
| **Endpoint** | `http://localhost:10156` |

### Supported Operations (4)

| Operation | Description |
|-----------|-------------|
| CreateFleet | Create a GameLift fleet |
| DescribeFleet | Describe a GameLift fleet |
| ListFleets | List all GameLift fleets |
| DeleteFleet | Delete a GameLift fleet |

### Usage with AWS CLI

```bash
# Create a fleet
aws gamelift create-fleet \
  --name my-fleet \
  --ec2-instance-type c5.large \
  --endpoint-url http://localhost:10156 \
  --no-sign-request

# List fleets
aws gamelift list-fleets \
  --endpoint-url http://localhost:10156 \
  --no-sign-request
```
