#!/usr/bin/env bash
source "$(dirname "$0")/test_helpers.sh"

PORT=$(service_port servicecatalog)
ENDPOINT="http://localhost:${PORT}"

aws_sc() {
  aws servicecatalog "$@" \
    --endpoint-url "$ENDPOINT" \
    --region "$REGION" \
    --no-sign-request \
    --no-cli-pager \
    --output json 2>&1
}

ensure_server

# 1. CreatePortfolio
OUT=$(aws_sc create-portfolio \
  --display-name "TestPortfolio" \
  --provider-name "TestProvider" \
  --description "A test portfolio" \
  --idempotency-token "create-portfolio-token-1")
assert_contains "CreatePortfolio Id" "$OUT" "Id"
assert_contains "CreatePortfolio DisplayName" "$OUT" "DisplayName"

PORTFOLIO_ID=$(echo "$OUT" | python3 -c "import sys,json; print(json.load(sys.stdin)['PortfolioDetail']['Id'])" 2>/dev/null || echo "")

# 2. DescribePortfolio
OUT=$(aws_sc describe-portfolio --id "$PORTFOLIO_ID")
assert_contains "DescribePortfolio DisplayName" "$OUT" "TestPortfolio"

# 3. ListPortfolios
OUT=$(aws_sc list-portfolios)
assert_contains "ListPortfolios" "$OUT" "TestPortfolio"

# 4. UpdatePortfolio
OUT=$(aws_sc update-portfolio --id "$PORTFOLIO_ID" --description "Updated description")
assert_contains "UpdatePortfolio" "$OUT" "PortfolioDetail"

# 5. CreateProduct
OUT=$(aws_sc create-product \
  --name "TestProduct" \
  --owner "TestOwner" \
  --product-type CLOUD_FORMATION_TEMPLATE \
  --provisioning-artifact-parameters '{"Name":"v1","Description":"Initial version","Info":{"LoadTemplateFromURL":"https://s3.amazonaws.com/mybucket/mytemplate.template"},"Type":"CLOUD_FORMATION_TEMPLATE"}' \
  --idempotency-token "create-product-token-1")
assert_contains "CreateProduct ProductId" "$OUT" "ProductId"

PRODUCT_ID=$(echo "$OUT" | python3 -c "import sys,json; print(json.load(sys.stdin)['ProductViewDetail']['ProductViewSummary']['ProductId'])" 2>/dev/null || echo "")
ARTIFACT_ID=$(echo "$OUT" | python3 -c "import sys,json; print(json.load(sys.stdin)['ProvisioningArtifactDetail']['Id'])" 2>/dev/null || echo "")

# 6. DescribeProduct
OUT=$(aws_sc describe-product --id "$PRODUCT_ID")
assert_contains "DescribeProduct" "$OUT" "TestProduct"

# 7. UpdateProduct
OUT=$(aws_sc update-product --id "$PRODUCT_ID" --description "Updated product description")
assert_contains "UpdateProduct" "$OUT" "ProductViewDetail"

# 8. SearchProducts
OUT=$(aws_sc search-products)
assert_contains "SearchProducts" "$OUT" "ProductViewSummaries"

# 9. AssociateProductWithPortfolio
OUT=$(aws_sc associate-product-with-portfolio \
  --product-id "$PRODUCT_ID" \
  --portfolio-id "$PORTFOLIO_ID")
assert_contains "AssociateProductWithPortfolio" "$OUT" ""

# 10. DisassociateProductFromPortfolio
OUT=$(aws_sc disassociate-product-from-portfolio \
  --product-id "$PRODUCT_ID" \
  --portfolio-id "$PORTFOLIO_ID")
assert_contains "DisassociateProductFromPortfolio" "$OUT" ""

# 11. ProvisionProduct
OUT=$(aws_sc provision-product \
  --product-id "$PRODUCT_ID" \
  --provisioning-artifact-id "$ARTIFACT_ID" \
  --provisioned-product-name "TestProvisionedProduct" \
  --provision-token "provision-token-1")
assert_contains "ProvisionProduct" "$OUT" "RecordDetail"

PROVISIONED_PRODUCT_ID=$(echo "$OUT" | python3 -c "import sys,json; print(json.load(sys.stdin)['RecordDetail']['ProvisionedProductId'])" 2>/dev/null || echo "")

# 12. DescribeProvisionedProduct
OUT=$(aws_sc describe-provisioned-product --id "$PROVISIONED_PRODUCT_ID")
assert_contains "DescribeProvisionedProduct" "$OUT" "ProvisionedProductDetail"

# 13. SearchProvisionedProducts
OUT=$(aws_sc search-provisioned-products)
assert_contains "SearchProvisionedProducts" "$OUT" "ProvisionedProducts"

# 14. TerminateProvisionedProduct
OUT=$(aws_sc terminate-provisioned-product \
  --provisioned-product-id "$PROVISIONED_PRODUCT_ID" \
  --terminate-token "terminate-token-1")
assert_contains "TerminateProvisionedProduct" "$OUT" "RecordDetail"

# 15. DeleteProduct
OUT=$(aws_sc delete-product --id "$PRODUCT_ID")
assert_contains "DeleteProduct" "$OUT" ""

# 16. DeletePortfolio
OUT=$(aws_sc delete-portfolio --id "$PORTFOLIO_ID")
assert_contains "DeletePortfolio" "$OUT" ""

# 17. DescribePortfolio not found
OUT=$(aws_sc describe-portfolio --id "$PORTFOLIO_ID")
assert_contains "DescribePortfolio not found" "$OUT" "ResourceNotFoundException"

# ── report ───────────────────────────────────────────────────────────────

report_results "Service Catalog"
exit $?
