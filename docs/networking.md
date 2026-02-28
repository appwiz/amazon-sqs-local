# Networking & Content Delivery

## API Gateway

| | |
|---|---|
| **Port** | `4567` |
| **Protocol** | REST JSON |
| **Endpoint** | `http://localhost:4567` |

### Supported Operations (30)

| Operation | Description |
|-----------|-------------|
| CreateRestApi | Create a new REST API |
| GetRestApis | List all REST APIs |
| GetRestApi | Get details of a REST API |
| DeleteRestApi | Delete a REST API |
| UpdateRestApi | Update a REST API via patch operations |
| GetResources | List all resources for a REST API |
| CreateResource | Create a new resource under a parent |
| GetResource | Get details of a resource |
| DeleteResource | Delete a resource |
| PutMethod | Add an HTTP method to a resource |
| GetMethod | Get details of a method |
| DeleteMethod | Remove a method from a resource |
| PutIntegration | Set up an integration for a method |
| GetIntegration | Get details of an integration |
| DeleteIntegration | Remove an integration |
| PutMethodResponse | Add a method response |
| GetMethodResponse | Get details of a method response |
| DeleteMethodResponse | Remove a method response |
| PutIntegrationResponse | Add an integration response |
| CreateDeployment | Create a deployment for a REST API |
| GetDeployments | List all deployments |
| GetDeployment | Get details of a deployment |
| CreateStage | Create a stage for a deployment |
| GetStages | List all stages for a REST API |
| GetStage | Get details of a stage |
| UpdateStage | Update a stage via patch operations |
| DeleteStage | Delete a stage |
| TagResource | Add tags to a REST API |
| UntagResource | Remove tags from a REST API |
| GetTags | List tags for a REST API |

### Wire Protocol

API Gateway uses REST JSON over HTTP. Requests are routed based on HTTP method and URL path:

- **REST APIs**: `POST /restapis` (create), `GET /restapis` (list)
- **REST API by ID**: `GET|DELETE|PATCH /restapis/{rest_api_id}`
- **Resources**: `GET /restapis/{rest_api_id}/resources` (list), `POST|GET|DELETE /restapis/{rest_api_id}/resources/{resource_id}`
- **Methods**: `PUT|GET|DELETE /restapis/{rest_api_id}/resources/{resource_id}/methods/{http_method}`
- **Integrations**: `PUT|GET|DELETE /restapis/{rest_api_id}/resources/{resource_id}/methods/{http_method}/integration`
- **Method Responses**: `PUT|GET|DELETE /restapis/{rest_api_id}/resources/{resource_id}/methods/{http_method}/responses/{status_code}`
- **Integration Responses**: `PUT /restapis/{rest_api_id}/resources/{resource_id}/methods/{http_method}/integration/responses/{status_code}`
- **Deployments**: `POST|GET /restapis/{rest_api_id}/deployments`, `GET /restapis/{rest_api_id}/deployments/{deployment_id}`
- **Stages**: `POST|GET /restapis/{rest_api_id}/stages`, `GET|PATCH|DELETE /restapis/{rest_api_id}/stages/{stage_name}`
- **Tags**: `POST|GET|DELETE /tags/{rest_api_id}`

### Usage with AWS CLI

```bash
# Create a REST API
aws apigateway create-rest-api \
  --name my-api \
  --endpoint-url http://localhost:4567 \
  --no-sign-request

# List REST APIs
aws apigateway get-rest-apis \
  --endpoint-url http://localhost:4567 \
  --no-sign-request

# Get a REST API
aws apigateway get-rest-api \
  --rest-api-id <api-id> \
  --endpoint-url http://localhost:4567 \
  --no-sign-request

# Get resources for a REST API
aws apigateway get-resources \
  --rest-api-id <api-id> \
  --endpoint-url http://localhost:4567 \
  --no-sign-request

# Create a resource
aws apigateway create-resource \
  --rest-api-id <api-id> \
  --parent-id <root-resource-id> \
  --path-part items \
  --endpoint-url http://localhost:4567 \
  --no-sign-request

# Put a method on a resource
aws apigateway put-method \
  --rest-api-id <api-id> \
  --resource-id <resource-id> \
  --http-method GET \
  --authorization-type NONE \
  --endpoint-url http://localhost:4567 \
  --no-sign-request

# Put an integration
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
  --endpoint-url http://localhost:4567 \
  --no-sign-request

# Create a stage
aws apigateway create-stage \
  --rest-api-id <api-id> \
  --stage-name prod \
  --deployment-id <deployment-id> \
  --endpoint-url http://localhost:4567 \
  --no-sign-request

# List stages
aws apigateway get-stages \
  --rest-api-id <api-id> \
  --endpoint-url http://localhost:4567 \
  --no-sign-request

# Delete a REST API
aws apigateway delete-rest-api \
  --rest-api-id <api-id> \
  --endpoint-url http://localhost:4567 \
  --no-sign-request
```

### Limitations

- The service manages REST API configuration but does not route or proxy actual HTTP requests.
- A root resource (`/`) is automatically created for each new REST API.

---

## CloudFront

| | |
|---|---|
| **Port** | `10021` |
| **Protocol** | REST JSON |
| **Endpoint** | `http://localhost:10021` |

### Supported Operations (4)

| Operation | Description |
|-----------|-------------|
| CreateDistribution | Create a CloudFront distribution |
| GetDistribution | Get details of a distribution |
| ListDistributions | List all distributions |
| DeleteDistribution | Delete a distribution |

### Usage with AWS CLI

```bash
# Create a distribution
aws cloudfront create-distribution \
  --distribution-config '{"CallerReference":"ref1","Origins":{"Quantity":1,"Items":[{"Id":"origin1","DomainName":"example.com","S3OriginConfig":{"OriginAccessIdentity":""}}]},"DefaultCacheBehavior":{"TargetOriginId":"origin1","ViewerProtocolPolicy":"allow-all","ForwardedValues":{"QueryString":false,"Cookies":{"Forward":"none"}},"TrustedSigners":{"Enabled":false,"Quantity":0},"MinTTL":0},"Comment":"","Enabled":true}' \
  --endpoint-url http://localhost:10021 \
  --no-sign-request

# List distributions
aws cloudfront list-distributions \
  --endpoint-url http://localhost:10021 \
  --no-sign-request
```

---

## Route 53

| | |
|---|---|
| **Port** | `10022` |
| **Protocol** | REST JSON |
| **Endpoint** | `http://localhost:10022` |

### Supported Operations (4)

| Operation | Description |
|-----------|-------------|
| CreateHostedZone | Create a hosted zone |
| GetHostedZone | Get details of a hosted zone |
| ListHostedZones | List all hosted zones |
| DeleteHostedZone | Delete a hosted zone |

### Usage with AWS CLI

```bash
# Create a hosted zone
aws route53 create-hosted-zone \
  --name example.com \
  --caller-reference ref1 \
  --endpoint-url http://localhost:10022 \
  --no-sign-request

# List hosted zones
aws route53 list-hosted-zones \
  --endpoint-url http://localhost:10022 \
  --no-sign-request
```

---

## ELB

| | |
|---|---|
| **Port** | `10027` |
| **Protocol** | Query/XML (Action parameter) |
| **Endpoint** | `http://localhost:10027` |

### Supported Operations (6)

| Operation | Description |
|-----------|-------------|
| CreateLoadBalancer | Create a load balancer |
| DescribeLoadBalancers | Describe load balancers |
| DeleteLoadBalancer | Delete a load balancer |
| CreateTargetGroup | Create a target group |
| DescribeTargetGroups | Describe target groups |
| DeleteTargetGroup | Delete a target group |

### Usage with AWS CLI

```bash
# Create a load balancer
aws elbv2 create-load-balancer \
  --name my-alb \
  --subnets subnet-1 subnet-2 \
  --endpoint-url http://localhost:10027 \
  --no-sign-request

# Describe load balancers
aws elbv2 describe-load-balancers \
  --endpoint-url http://localhost:10027 \
  --no-sign-request

# Create a target group
aws elbv2 create-target-group \
  --name my-targets \
  --protocol HTTP \
  --port 80 \
  --vpc-id vpc-1 \
  --endpoint-url http://localhost:10027 \
  --no-sign-request
```

---

## VPC Lattice

| | |
|---|---|
| **Port** | `10023` |
| **Protocol** | REST JSON |
| **Endpoint** | `http://localhost:10023` |

### Supported Operations (4)

| Operation | Description |
|-----------|-------------|
| CreateServiceNetwork | Create a service network |
| ListServiceNetworks | List all service networks |
| GetServiceNetwork | Get details of a service network |
| DeleteServiceNetwork | Delete a service network |

### Usage with AWS CLI

```bash
# Create a service network
aws vpc-lattice create-service-network \
  --name my-network \
  --endpoint-url http://localhost:10023 \
  --no-sign-request

# List service networks
aws vpc-lattice list-service-networks \
  --endpoint-url http://localhost:10023 \
  --no-sign-request
```

---

## Direct Connect

| | |
|---|---|
| **Port** | `10025` |
| **Protocol** | JSON RPC (`OvertureService`) |
| **Endpoint** | `http://localhost:10025` |

### Supported Operations (4)

| Operation | Description |
|-----------|-------------|
| CreateConnection | Create a Direct Connect connection |
| DescribeConnection | Describe a connection |
| ListConnections | List all connections |
| DeleteConnection | Delete a connection |

### Usage with AWS CLI

```bash
# Create a connection
aws directconnect create-connection \
  --connection-name my-connection \
  --bandwidth 1Gbps \
  --location EqSe2 \
  --endpoint-url http://localhost:10025 \
  --no-sign-request

# List connections
aws directconnect describe-connections \
  --endpoint-url http://localhost:10025 \
  --no-sign-request
```

---

## Global Accelerator

| | |
|---|---|
| **Port** | `10026` |
| **Protocol** | JSON RPC (`GlobalAccelerator_V20180706`) |
| **Endpoint** | `http://localhost:10026` |

### Supported Operations (4)

| Operation | Description |
|-----------|-------------|
| CreateAccelerator | Create a Global Accelerator |
| DescribeAccelerator | Describe an accelerator |
| ListAccelerators | List all accelerators |
| DeleteAccelerator | Delete an accelerator |

### Usage with AWS CLI

```bash
# Create an accelerator
aws globalaccelerator create-accelerator \
  --name my-accelerator \
  --endpoint-url http://localhost:10026 \
  --no-sign-request

# List accelerators
aws globalaccelerator list-accelerators \
  --endpoint-url http://localhost:10026 \
  --no-sign-request
```

---

## Cloud Map

| | |
|---|---|
| **Port** | `10024` |
| **Protocol** | JSON RPC (`Route53AutoNaming_v20170314`) |
| **Endpoint** | `http://localhost:10024` |

### Supported Operations (4)

| Operation | Description |
|-----------|-------------|
| CreateNamespace | Create a namespace |
| DescribeNamespace | Describe a namespace |
| ListNamespaces | List all namespaces |
| DeleteNamespace | Delete a namespace |

### Usage with AWS CLI

```bash
# Create a namespace
aws servicediscovery create-http-namespace \
  --name my-namespace \
  --endpoint-url http://localhost:10024 \
  --no-sign-request

# List namespaces
aws servicediscovery list-namespaces \
  --endpoint-url http://localhost:10024 \
  --no-sign-request
```

---

## Network Firewall

| | |
|---|---|
| **Port** | `10048` |
| **Protocol** | JSON RPC (`NetworkFirewall_20201112`) |
| **Endpoint** | `http://localhost:10048` |

### Supported Operations (8)

| Operation | Description |
|-----------|-------------|
| CreateFirewall | Create a network firewall |
| DescribeFirewall | Describe a firewall |
| ListFirewalls | List all firewalls |
| DeleteFirewall | Delete a firewall |
| CreateFirewallPolicy | Create a firewall policy |
| DescribeFirewallPolicy | Describe a firewall policy |
| ListFirewallPolicys | List all firewall policies |
| DeleteFirewallPolicy | Delete a firewall policy |

### Usage with AWS CLI

```bash
# Create a firewall policy
aws network-firewall create-firewall-policy \
  --firewall-policy-name my-policy \
  --firewall-policy '{"StatelessDefaultActions":["aws:pass"],"StatelessFragmentDefaultActions":["aws:pass"]}' \
  --endpoint-url http://localhost:10048 \
  --no-sign-request

# Create a firewall
aws network-firewall create-firewall \
  --firewall-name my-firewall \
  --firewall-policy-arn arn:aws:network-firewall:us-east-1:000000000000:firewall-policy/my-policy \
  --vpc-id vpc-1 \
  --subnet-mappings SubnetId=subnet-1 \
  --endpoint-url http://localhost:10048 \
  --no-sign-request

# List firewalls
aws network-firewall list-firewalls \
  --endpoint-url http://localhost:10048 \
  --no-sign-request
```

---

## App Mesh

| | |
|---|---|
| **Port** | `10158` |
| **Protocol** | REST JSON |
| **Endpoint** | `http://localhost:10158` |

### Supported Operations (4)

| Operation | Description |
|-----------|-------------|
| CreateMesh | Create a service mesh |
| GetMesh | Get details of a mesh |
| ListMeshs | List all meshes |
| DeleteMesh | Delete a mesh |

### Usage with AWS CLI

```bash
# Create a mesh
aws appmesh create-mesh \
  --mesh-name my-mesh \
  --endpoint-url http://localhost:10158 \
  --no-sign-request

# List meshes
aws appmesh list-meshes \
  --endpoint-url http://localhost:10158 \
  --no-sign-request
```
