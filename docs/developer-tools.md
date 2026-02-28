# Developer Tools

## CodeBuild

| Property | Value |
|----------|-------|
| Port | `10084` |
| Protocol | JSON RPC |
| Endpoint | `http://localhost:10084` |
| Target prefix | `CodeBuild_20161006` |

### Operations (4)

| Operation | Description |
|-----------|-------------|
| `CreateProject` | Create a new build project |
| `DescribeProject` | Describe a build project |
| `ListProjects` | List all build projects |
| `DeleteProject` | Delete a build project |

### CLI Example

```bash
aws codebuild list-projects \
  --endpoint-url http://localhost:10084 \
  --no-sign-request
```

---

## CodePipeline

| Property | Value |
|----------|-------|
| Port | `10087` |
| Protocol | JSON RPC |
| Endpoint | `http://localhost:10087` |
| Target prefix | `CodePipeline_20150709` |

### Operations (4)

| Operation | Description |
|-----------|-------------|
| `CreatePipeline` | Create a new pipeline |
| `DescribePipeline` | Describe a pipeline |
| `ListPipelines` | List all pipelines |
| `DeletePipeline` | Delete a pipeline |

### CLI Example

```bash
aws codepipeline list-pipelines \
  --endpoint-url http://localhost:10087 \
  --no-sign-request
```

---

## CodeCommit

| Property | Value |
|----------|-------|
| Port | `10085` |
| Protocol | JSON RPC |
| Endpoint | `http://localhost:10085` |
| Target prefix | `CodeCommit_20150413` |

### Operations (4)

| Operation | Description |
|-----------|-------------|
| `CreateRepository` | Create a new repository |
| `DescribeRepository` | Describe a repository |
| `ListRepositories` | List all repositories |
| `DeleteRepository` | Delete a repository |

### CLI Example

```bash
aws codecommit list-repositories \
  --endpoint-url http://localhost:10085 \
  --no-sign-request
```

---

## CodeDeploy

| Property | Value |
|----------|-------|
| Port | `10086` |
| Protocol | JSON RPC |
| Endpoint | `http://localhost:10086` |
| Target prefix | `CodeDeploy_20141006` |

### Operations (4)

| Operation | Description |
|-----------|-------------|
| `CreateApplication` | Create a new application |
| `DescribeApplication` | Describe an application |
| `ListApplications` | List all applications |
| `DeleteApplication` | Delete an application |

### CLI Example

```bash
aws deploy list-applications \
  --endpoint-url http://localhost:10086 \
  --no-sign-request
```

---

## CodeArtifact

| Property | Value |
|----------|-------|
| Port | `10083` |
| Protocol | REST JSON |
| Endpoint | `http://localhost:10083` |

### Operations (8)

| Operation | Method | Path |
|-----------|--------|------|
| `CreateDomain` | POST | `/v1/domain` |
| `GetDomain` | GET | `/v1/domain` |
| `ListDomains` | POST | `/v1/domains` |
| `DeleteDomain` | DELETE | `/v1/domain` |
| `CreateRepository` | POST | `/v1/repository` |
| `GetRepository` | GET | `/v1/repository` |
| `ListRepositories` | POST | `/v1/repositories` |
| `DeleteRepository` | DELETE | `/v1/repository` |

### CLI Example

```bash
aws codeartifact list-domains \
  --endpoint-url http://localhost:10083 \
  --no-sign-request
```

---

## CodeCatalyst

| Property | Value |
|----------|-------|
| Port | `10082` |
| Protocol | REST JSON |
| Endpoint | `http://localhost:10082` |

### Operations (4)

| Operation | Method | Path |
|-----------|--------|------|
| `CreateProject` | POST | `/v1/spaces/{spaceName}/projects` |
| `GetProject` | GET | `/v1/spaces/{spaceName}/projects/{projectName}` |
| `ListProjects` | POST | `/v1/spaces/{spaceName}/projects/list` |
| `DeleteProject` | DELETE | `/v1/spaces/{spaceName}/projects/{projectName}` |

### CLI Example

```bash
aws codecatalyst list-projects \
  --space-name my-space \
  --endpoint-url http://localhost:10082 \
  --no-sign-request
```

---

## FIS (Fault Injection Service)

| Property | Value |
|----------|-------|
| Port | `10088` |
| Protocol | REST JSON |
| Endpoint | `http://localhost:10088` |

### Operations (4)

| Operation | Method | Path |
|-----------|--------|------|
| `CreateExperimentTemplate` | POST | `/experimentTemplates` |
| `GetExperimentTemplate` | GET | `/experimentTemplates/{id}` |
| `ListExperimentTemplates` | GET | `/experimentTemplates` |
| `DeleteExperimentTemplate` | DELETE | `/experimentTemplates/{id}` |

### CLI Example

```bash
aws fis list-experiment-templates \
  --endpoint-url http://localhost:10088 \
  --no-sign-request
```
