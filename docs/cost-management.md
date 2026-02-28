# Cost Management

## CostExplorer

| | |
|---|---|
| **Port** | `10131` |
| **Protocol** | JSON RPC (AWSInsightsIndexService) |
| **Endpoint** | `http://localhost:10131` |

### Supported Operations (4)

| Operation | Description |
|-----------|-------------|
| CreateCostCategory | Create a new cost category |
| DescribeCostCategory | Describe a specific cost category |
| ListCostCategories | List all cost categories |
| DeleteCostCategory | Delete a cost category |

### Usage with AWS CLI

```bash
# Create
aws ce create-cost-category-definition --name my-category --rules '[]' --rule-version CostCategoryExpression.v1 --endpoint-url http://localhost:10131 --no-sign-request

# List
aws ce list-cost-category-definitions --endpoint-url http://localhost:10131 --no-sign-request
```

---

## Budgets

| | |
|---|---|
| **Port** | `10130` |
| **Protocol** | JSON RPC (AWSBudgetServiceGateway) |
| **Endpoint** | `http://localhost:10130` |

### Supported Operations (4)

| Operation | Description |
|-----------|-------------|
| CreateBudget | Create a new budget |
| DescribeBudget | Describe a specific budget |
| ListBudgets | List all budgets |
| DeleteBudget | Delete a budget |

### Usage with AWS CLI

```bash
# Create
aws budgets create-budget --account-id 012345678901 --budget '{"BudgetName":"my-budget","BudgetLimit":{"Amount":"100","Unit":"USD"},"TimeUnit":"MONTHLY","BudgetType":"COST"}' --endpoint-url http://localhost:10130 --no-sign-request

# List
aws budgets describe-budgets --account-id 012345678901 --endpoint-url http://localhost:10130 --no-sign-request
```

---

## BillingConductor

| | |
|---|---|
| **Port** | `10129` |
| **Protocol** | JSON RPC (AWSBillingConductor) |
| **Endpoint** | `http://localhost:10129` |

### Supported Operations (4)

| Operation | Description |
|-----------|-------------|
| CreatePricingPlan | Create a new pricing plan |
| DescribePricingPlan | Describe a specific pricing plan |
| ListPricingPlans | List all pricing plans |
| DeletePricingPlan | Delete a pricing plan |

### Usage with AWS CLI

```bash
# Create
aws billingconductor create-pricing-plan --name my-plan --endpoint-url http://localhost:10129 --no-sign-request

# List
aws billingconductor list-pricing-plans --endpoint-url http://localhost:10129 --no-sign-request
```
