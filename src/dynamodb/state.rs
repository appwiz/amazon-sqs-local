use std::collections::HashMap;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use serde_json::Value;
use tokio::sync::Mutex;
use uuid::Uuid;

use super::error::DynamoDbError;
use super::table::Table;
use super::types::*;

struct DynamoDbStateInner {
    tables: HashMap<String, Table>,
    account_id: String,
    region: String,
}

pub struct DynamoDbState {
    inner: Arc<Mutex<DynamoDbStateInner>>,
}

impl DynamoDbState {
    pub fn new(account_id: String, region: String) -> Self {
        DynamoDbState {
            inner: Arc::new(Mutex::new(DynamoDbStateInner {
                tables: HashMap::new(),
                account_id,
                region,
            })),
        }
    }

    fn now_epoch() -> f64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f64()
    }

    // --- Table operations ---

    pub async fn create_table(
        &self,
        req: CreateTableRequest,
    ) -> Result<CreateTableResponse, DynamoDbError> {
        let mut inner = self.inner.lock().await;

        if inner.tables.contains_key(&req.table_name) {
            return Err(DynamoDbError::ResourceInUseException(format!(
                "Table already exists: {}",
                req.table_name
            )));
        }

        // Validate key schema
        let has_hash = req.key_schema.iter().any(|k| k.key_type == "HASH");
        if !has_hash {
            return Err(DynamoDbError::ValidationException(
                "No HASH key defined in KeySchema".into(),
            ));
        }

        let billing_mode = req.billing_mode.unwrap_or_else(|| "PROVISIONED".to_string());
        let provisioned_throughput = match &req.provisioned_throughput {
            Some(pt) => ProvisionedThroughputDescription {
                read_capacity_units: pt.read_capacity_units,
                write_capacity_units: pt.write_capacity_units,
                last_increase_date_time: None,
                last_decrease_date_time: None,
                number_of_decreases_today: 0,
            },
            None => ProvisionedThroughputDescription {
                read_capacity_units: 0,
                write_capacity_units: 0,
                last_increase_date_time: None,
                last_decrease_date_time: None,
                number_of_decreases_today: 0,
            },
        };

        let table_id = Uuid::new_v4().to_string();
        let table_arn = format!(
            "arn:aws:dynamodb:{}:{}:table/{}",
            inner.region, inner.account_id, req.table_name
        );

        let mut tags = HashMap::new();
        if let Some(tag_list) = req.tags {
            for tag in tag_list {
                tags.insert(tag.key, tag.value);
            }
        }

        let table = Table {
            table_name: req.table_name.clone(),
            table_arn,
            table_id,
            key_schema: req.key_schema,
            attribute_definitions: req.attribute_definitions,
            billing_mode,
            provisioned_throughput,
            creation_date_time: Self::now_epoch(),
            table_status: "ACTIVE".to_string(),
            items: Vec::new(),
            tags,
        };

        let description = table.to_description();
        inner.tables.insert(req.table_name, table);

        Ok(CreateTableResponse {
            table_description: description,
        })
    }

    pub async fn delete_table(
        &self,
        req: DeleteTableRequest,
    ) -> Result<DeleteTableResponse, DynamoDbError> {
        let mut inner = self.inner.lock().await;

        let table = inner.tables.remove(&req.table_name).ok_or_else(|| {
            DynamoDbError::ResourceNotFoundException(format!(
                "Requested resource not found: Table: {} not found",
                req.table_name
            ))
        })?;

        let mut description = table.to_description();
        description.table_status = "DELETING".to_string();

        Ok(DeleteTableResponse {
            table_description: description,
        })
    }

    pub async fn describe_table(
        &self,
        req: DescribeTableRequest,
    ) -> Result<DescribeTableResponse, DynamoDbError> {
        let inner = self.inner.lock().await;

        let table = inner.tables.get(&req.table_name).ok_or_else(|| {
            DynamoDbError::ResourceNotFoundException(format!(
                "Requested resource not found: Table: {} not found",
                req.table_name
            ))
        })?;

        Ok(DescribeTableResponse {
            table: table.to_description(),
        })
    }

    pub async fn list_tables(
        &self,
        req: ListTablesRequest,
    ) -> Result<ListTablesResponse, DynamoDbError> {
        let inner = self.inner.lock().await;

        let mut names: Vec<String> = inner.tables.keys().cloned().collect();
        names.sort();

        // Apply ExclusiveStartTableName
        if let Some(ref start) = req.exclusive_start_table_name {
            names.retain(|n| n.as_str() > start.as_str());
        }

        let limit = req.limit.unwrap_or(100) as usize;
        let last_evaluated = if names.len() > limit {
            let last = names[limit - 1].clone();
            names.truncate(limit);
            Some(last)
        } else {
            None
        };

        Ok(ListTablesResponse {
            table_names: names,
            last_evaluated_table_name: last_evaluated,
        })
    }

    pub async fn update_table(
        &self,
        req: UpdateTableRequest,
    ) -> Result<UpdateTableResponse, DynamoDbError> {
        let mut inner = self.inner.lock().await;

        let table = inner.tables.get_mut(&req.table_name).ok_or_else(|| {
            DynamoDbError::ResourceNotFoundException(format!(
                "Requested resource not found: Table: {} not found",
                req.table_name
            ))
        })?;

        if let Some(billing_mode) = req.billing_mode {
            table.billing_mode = billing_mode;
        }

        if let Some(pt) = req.provisioned_throughput {
            table.provisioned_throughput = ProvisionedThroughputDescription {
                read_capacity_units: pt.read_capacity_units,
                write_capacity_units: pt.write_capacity_units,
                last_increase_date_time: Some(Self::now_epoch()),
                last_decrease_date_time: None,
                number_of_decreases_today: 0,
            };
        }

        Ok(UpdateTableResponse {
            table_description: table.to_description(),
        })
    }

    // --- Item operations ---

    pub async fn put_item(
        &self,
        req: PutItemRequest,
    ) -> Result<PutItemResponse, DynamoDbError> {
        let mut inner = self.inner.lock().await;

        let table = inner.tables.get_mut(&req.table_name).ok_or_else(|| {
            DynamoDbError::ResourceNotFoundException(format!(
                "Requested resource not found: Table: {} not found",
                req.table_name
            ))
        })?;

        // Validate key attributes exist
        let hash_key = table.hash_key_name().to_string();
        if !req.item.contains_key(&hash_key) {
            return Err(DynamoDbError::ValidationException(format!(
                "One or more parameter values are not valid. Missing the key {} in the item",
                hash_key
            )));
        }

        if let Some(rk) = table.range_key_name().map(|s| s.to_string()) {
            if !req.item.contains_key(&rk) {
                return Err(DynamoDbError::ValidationException(format!(
                    "One or more parameter values are not valid. Missing the key {} in the item",
                    rk
                )));
            }
        }

        let old_item = match table.find_item_index(&req.item) {
            Some(idx) => Some(std::mem::replace(&mut table.items[idx], req.item)),
            None => {
                table.items.push(req.item);
                None
            }
        };

        let attributes = match req.return_values.as_deref() {
            Some("ALL_OLD") => old_item,
            _ => None,
        };

        Ok(PutItemResponse { attributes })
    }

    pub async fn get_item(
        &self,
        req: GetItemRequest,
    ) -> Result<GetItemResponse, DynamoDbError> {
        let inner = self.inner.lock().await;

        let table = inner.tables.get(&req.table_name).ok_or_else(|| {
            DynamoDbError::ResourceNotFoundException(format!(
                "Requested resource not found: Table: {} not found",
                req.table_name
            ))
        })?;

        let item = table
            .find_item_index(&req.key)
            .map(|idx| table.items[idx].clone());

        let item = item.map(|i| {
            apply_projection(
                i,
                req.projection_expression.as_deref(),
                req.expression_attribute_names.as_ref(),
            )
        });

        Ok(GetItemResponse { item })
    }

    pub async fn delete_item(
        &self,
        req: DeleteItemRequest,
    ) -> Result<DeleteItemResponse, DynamoDbError> {
        let mut inner = self.inner.lock().await;

        let table = inner.tables.get_mut(&req.table_name).ok_or_else(|| {
            DynamoDbError::ResourceNotFoundException(format!(
                "Requested resource not found: Table: {} not found",
                req.table_name
            ))
        })?;

        let old_item = table.find_item_index(&req.key).map(|idx| table.items.remove(idx));

        let attributes = match req.return_values.as_deref() {
            Some("ALL_OLD") => old_item,
            _ => None,
        };

        Ok(DeleteItemResponse { attributes })
    }

    pub async fn update_item(
        &self,
        req: UpdateItemRequest,
    ) -> Result<UpdateItemResponse, DynamoDbError> {
        let mut inner = self.inner.lock().await;

        let table = inner.tables.get_mut(&req.table_name).ok_or_else(|| {
            DynamoDbError::ResourceNotFoundException(format!(
                "Requested resource not found: Table: {} not found",
                req.table_name
            ))
        })?;

        let idx = table.find_item_index(&req.key);
        let existed = idx.is_some();

        // For UpdateItem, if the item doesn't exist, create it from the key
        let item_idx = match idx {
            Some(i) => i,
            None => {
                let mut new_item = req.key.clone();
                // Ensure hash and range keys are set
                let hash_key = table.hash_key_name().to_string();
                if !new_item.contains_key(&hash_key) {
                    if let Some(v) = req.key.get(&hash_key) {
                        new_item.insert(hash_key, v.clone());
                    }
                }
                if let Some(rk) = table.range_key_name().map(|s| s.to_string()) {
                    if !new_item.contains_key(&rk) {
                        if let Some(v) = req.key.get(&rk) {
                            new_item.insert(rk, v.clone());
                        }
                    }
                }
                table.items.push(new_item);
                table.items.len() - 1
            }
        };

        let old_item = if existed {
            Some(table.items[item_idx].clone())
        } else {
            None
        };

        // Apply update expression
        if let Some(ref expr) = req.update_expression {
            apply_update_expression(
                &mut table.items[item_idx],
                expr,
                req.expression_attribute_names.as_ref(),
                req.expression_attribute_values.as_ref(),
            )?;
        }

        let attributes = match req.return_values.as_deref() {
            Some("ALL_NEW") => Some(table.items[item_idx].clone()),
            Some("ALL_OLD") => old_item,
            Some("UPDATED_OLD") => old_item, // Simplified: return all old attrs
            Some("UPDATED_NEW") => Some(table.items[item_idx].clone()), // Simplified
            _ => None,
        };

        Ok(UpdateItemResponse { attributes })
    }

    // --- Query ---

    pub async fn query(
        &self,
        req: QueryRequest,
    ) -> Result<QueryResponse, DynamoDbError> {
        let inner = self.inner.lock().await;

        let table = inner.tables.get(&req.table_name).ok_or_else(|| {
            DynamoDbError::ResourceNotFoundException(format!(
                "Requested resource not found: Table: {} not found",
                req.table_name
            ))
        })?;

        let key_condition = req.key_condition_expression.as_deref().ok_or_else(|| {
            DynamoDbError::ValidationException(
                "KeyConditionExpression is required for Query".into(),
            )
        })?;

        let conditions = parse_key_condition_expression(
            key_condition,
            req.expression_attribute_names.as_ref(),
            req.expression_attribute_values.as_ref(),
        )?;

        // Filter items by key conditions
        let mut matched_items: Vec<Item> = table
            .items
            .iter()
            .filter(|item| evaluate_key_conditions(item, &conditions))
            .cloned()
            .collect();

        // Sort by sort key if present
        if let Some(range_key) = table.range_key_name() {
            let ascending = req.scan_index_forward.unwrap_or(true);
            let rk = range_key.to_string();
            matched_items.sort_by(|a, b| {
                let va = a.get(&rk);
                let vb = b.get(&rk);
                let cmp = compare_attribute_values(va, vb);
                if ascending { cmp } else { cmp.reverse() }
            });
        }

        let scanned_count = matched_items.len() as i64;

        // Apply filter expression
        if let Some(ref filter_expr) = req.filter_expression {
            matched_items = matched_items
                .into_iter()
                .filter(|item| {
                    evaluate_filter_expression(
                        item,
                        filter_expr,
                        req.expression_attribute_names.as_ref(),
                        req.expression_attribute_values.as_ref(),
                    )
                })
                .collect();
        }

        // Apply pagination
        if let Some(ref start_key) = req.exclusive_start_key {
            let start_pk = table.build_primary_key(start_key);
            if let Some(pos) = matched_items
                .iter()
                .position(|item| table.build_primary_key(item) == start_pk)
            {
                matched_items = matched_items.split_off(pos + 1);
            }
        }

        let mut last_evaluated_key = None;
        if let Some(limit) = req.limit {
            let limit = limit as usize;
            if matched_items.len() > limit {
                let last_item = matched_items[limit - 1].clone();
                matched_items.truncate(limit);
                last_evaluated_key = Some(extract_key(table, &last_item));
            }
        }

        // Apply projection
        if req.select.as_deref() == Some("COUNT") {
            let count = matched_items.len() as i64;
            return Ok(QueryResponse {
                items: vec![],
                count,
                scanned_count,
                last_evaluated_key,
            });
        }

        let matched_items: Vec<Item> = matched_items
            .into_iter()
            .map(|i| {
                apply_projection(
                    i,
                    req.projection_expression.as_deref(),
                    req.expression_attribute_names.as_ref(),
                )
            })
            .collect();

        let count = matched_items.len() as i64;

        Ok(QueryResponse {
            items: matched_items,
            count,
            scanned_count,
            last_evaluated_key,
        })
    }

    // --- Scan ---

    pub async fn scan(
        &self,
        req: ScanRequest,
    ) -> Result<ScanResponse, DynamoDbError> {
        let inner = self.inner.lock().await;

        let table = inner.tables.get(&req.table_name).ok_or_else(|| {
            DynamoDbError::ResourceNotFoundException(format!(
                "Requested resource not found: Table: {} not found",
                req.table_name
            ))
        })?;

        let mut items: Vec<Item> = table.items.clone();
        let scanned_count = items.len() as i64;

        // Apply filter expression
        if let Some(ref filter_expr) = req.filter_expression {
            items = items
                .into_iter()
                .filter(|item| {
                    evaluate_filter_expression(
                        item,
                        filter_expr,
                        req.expression_attribute_names.as_ref(),
                        req.expression_attribute_values.as_ref(),
                    )
                })
                .collect();
        }

        // Apply pagination
        if let Some(ref start_key) = req.exclusive_start_key {
            let start_pk = table.build_primary_key(start_key);
            if let Some(pos) = items
                .iter()
                .position(|item| table.build_primary_key(item) == start_pk)
            {
                items = items.split_off(pos + 1);
            }
        }

        let mut last_evaluated_key = None;
        if let Some(limit) = req.limit {
            let limit = limit as usize;
            if items.len() > limit {
                let last_item = items[limit - 1].clone();
                items.truncate(limit);
                last_evaluated_key = Some(extract_key(table, &last_item));
            }
        }

        if req.select.as_deref() == Some("COUNT") {
            let count = items.len() as i64;
            return Ok(ScanResponse {
                items: vec![],
                count,
                scanned_count,
                last_evaluated_key,
            });
        }

        let items: Vec<Item> = items
            .into_iter()
            .map(|i| {
                apply_projection(
                    i,
                    req.projection_expression.as_deref(),
                    req.expression_attribute_names.as_ref(),
                )
            })
            .collect();

        let count = items.len() as i64;

        Ok(ScanResponse {
            items,
            count,
            scanned_count,
            last_evaluated_key,
        })
    }

    // --- Batch operations ---

    pub async fn batch_get_item(
        &self,
        req: BatchGetItemRequest,
    ) -> Result<BatchGetItemResponse, DynamoDbError> {
        let inner = self.inner.lock().await;

        let mut responses: HashMap<String, Vec<Item>> = HashMap::new();

        for (table_name, keys_and_attrs) in &req.request_items {
            let table = inner.tables.get(table_name).ok_or_else(|| {
                DynamoDbError::ResourceNotFoundException(format!(
                    "Requested resource not found: Table: {} not found",
                    table_name
                ))
            })?;

            let mut items = Vec::new();
            for key in &keys_and_attrs.keys {
                if let Some(idx) = table.find_item_index(key) {
                    let item = apply_projection(
                        table.items[idx].clone(),
                        keys_and_attrs.projection_expression.as_deref(),
                        keys_and_attrs.expression_attribute_names.as_ref(),
                    );
                    items.push(item);
                }
            }

            responses.insert(table_name.clone(), items);
        }

        Ok(BatchGetItemResponse {
            responses,
            unprocessed_keys: HashMap::new(),
        })
    }

    pub async fn batch_write_item(
        &self,
        req: BatchWriteItemRequest,
    ) -> Result<BatchWriteItemResponse, DynamoDbError> {
        let mut inner = self.inner.lock().await;

        for (table_name, write_requests) in &req.request_items {
            let table = inner.tables.get_mut(table_name).ok_or_else(|| {
                DynamoDbError::ResourceNotFoundException(format!(
                    "Requested resource not found: Table: {} not found",
                    table_name
                ))
            })?;

            for write_req in write_requests {
                if let Some(ref put) = write_req.put_request {
                    match table.find_item_index(&put.item) {
                        Some(idx) => {
                            table.items[idx] = put.item.clone();
                        }
                        None => {
                            table.items.push(put.item.clone());
                        }
                    }
                }

                if let Some(ref delete) = write_req.delete_request {
                    if let Some(idx) = table.find_item_index(&delete.key) {
                        table.items.remove(idx);
                    }
                }
            }
        }

        Ok(BatchWriteItemResponse {
            unprocessed_items: HashMap::new(),
        })
    }

    // --- Tag operations ---

    pub async fn tag_resource(&self, req: TagResourceRequest) -> Result<(), DynamoDbError> {
        let mut inner = self.inner.lock().await;

        let table = find_table_by_arn_mut(&mut inner.tables, &req.resource_arn)?;

        for tag in req.tags {
            table.tags.insert(tag.key, tag.value);
        }

        Ok(())
    }

    pub async fn untag_resource(&self, req: UntagResourceRequest) -> Result<(), DynamoDbError> {
        let mut inner = self.inner.lock().await;

        let table = find_table_by_arn_mut(&mut inner.tables, &req.resource_arn)?;

        for key in &req.tag_keys {
            table.tags.remove(key);
        }

        Ok(())
    }

    pub async fn list_tags_of_resource(
        &self,
        req: ListTagsOfResourceRequest,
    ) -> Result<ListTagsOfResourceResponse, DynamoDbError> {
        let inner = self.inner.lock().await;

        let table = find_table_by_arn(&inner.tables, &req.resource_arn)?;

        let tags: Vec<Tag> = table
            .tags
            .iter()
            .map(|(k, v)| Tag {
                key: k.clone(),
                value: v.clone(),
            })
            .collect();

        Ok(ListTagsOfResourceResponse {
            tags,
            next_token: None,
        })
    }
}

// --- Helper functions ---

fn find_table_by_arn<'a>(
    tables: &'a HashMap<String, Table>,
    arn: &str,
) -> Result<&'a Table, DynamoDbError> {
    tables
        .values()
        .find(|t| t.table_arn == arn)
        .ok_or_else(|| {
            DynamoDbError::ResourceNotFoundException(format!(
                "Requested resource not found: {}",
                arn
            ))
        })
}

fn find_table_by_arn_mut<'a>(
    tables: &'a mut HashMap<String, Table>,
    arn: &str,
) -> Result<&'a mut Table, DynamoDbError> {
    tables
        .values_mut()
        .find(|t| t.table_arn == arn)
        .ok_or_else(|| {
            DynamoDbError::ResourceNotFoundException(format!(
                "Requested resource not found: {}",
                arn
            ))
        })
}

fn extract_key(table: &Table, item: &Item) -> Item {
    let mut key = Item::new();
    let hash_key = table.hash_key_name();
    if let Some(v) = item.get(hash_key) {
        key.insert(hash_key.to_string(), v.clone());
    }
    if let Some(range_key) = table.range_key_name() {
        if let Some(v) = item.get(range_key) {
            key.insert(range_key.to_string(), v.clone());
        }
    }
    key
}

fn apply_projection(
    item: Item,
    projection_expression: Option<&str>,
    expression_attribute_names: Option<&HashMap<String, String>>,
) -> Item {
    let expr = match projection_expression {
        Some(e) => e,
        None => return item,
    };

    let attrs: Vec<String> = expr
        .split(',')
        .map(|s| {
            let s = s.trim();
            resolve_name(s, expression_attribute_names)
        })
        .collect();

    let mut projected = Item::new();
    for attr in attrs {
        if let Some(v) = item.get(&attr) {
            projected.insert(attr, v.clone());
        }
    }
    projected
}

fn resolve_name(name: &str, names: Option<&HashMap<String, String>>) -> String {
    if let Some(map) = names {
        if let Some(resolved) = map.get(name) {
            return resolved.clone();
        }
    }
    name.to_string()
}

fn resolve_value<'a>(
    token: &str,
    values: Option<&'a HashMap<String, Value>>,
) -> Option<&'a Value> {
    values.and_then(|map| map.get(token))
}

// --- Key condition expression parsing ---

#[derive(Debug)]
struct KeyCondition {
    attribute_name: String,
    operator: KeyConditionOp,
    value: Value,
    value2: Option<Value>,
}

#[derive(Debug)]
enum KeyConditionOp {
    Eq,
    Lt,
    Le,
    Gt,
    Ge,
    BeginsWith,
    Between,
}

fn parse_key_condition_expression(
    expr: &str,
    names: Option<&HashMap<String, String>>,
    values: Option<&HashMap<String, Value>>,
) -> Result<Vec<KeyCondition>, DynamoDbError> {
    let mut conditions = Vec::new();

    // Split by AND (case-insensitive)
    let parts = split_by_and(expr);

    for part in parts {
        let part = part.trim();
        let condition = parse_single_condition(part, names, values)?;
        conditions.push(condition);
    }

    Ok(conditions)
}

fn split_by_and(expr: &str) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut last = 0;
    let bytes = expr.as_bytes();
    let len = bytes.len();

    let mut i = 0;
    while i < len {
        // Check for " AND " (case-insensitive) with word boundaries
        if i + 5 <= len {
            let slice = &expr[i..];
            if slice.starts_with(" AND ") || slice.starts_with(" and ") || slice.starts_with(" And ")
            {
                parts.push(&expr[last..i]);
                last = i + 5;
                i = last;
                continue;
            }
        }
        i += 1;
    }
    parts.push(&expr[last..]);
    parts
}

fn parse_single_condition(
    part: &str,
    names: Option<&HashMap<String, String>>,
    values: Option<&HashMap<String, Value>>,
) -> Result<KeyCondition, DynamoDbError> {
    let part = part.trim();

    // Check for begins_with(attr, val)
    if let Some(inner) = extract_function_args(part, "begins_with") {
        let args: Vec<&str> = inner.splitn(2, ',').collect();
        if args.len() != 2 {
            return Err(DynamoDbError::ValidationException(
                "Invalid begins_with expression".into(),
            ));
        }
        let attr = resolve_name(args[0].trim(), names);
        let val_token = args[1].trim();
        let val = resolve_value(val_token, values)
            .ok_or_else(|| {
                DynamoDbError::ValidationException(format!(
                    "Value {} not found in ExpressionAttributeValues",
                    val_token
                ))
            })?
            .clone();

        return Ok(KeyCondition {
            attribute_name: attr,
            operator: KeyConditionOp::BeginsWith,
            value: val,
            value2: None,
        });
    }

    // Check for BETWEEN: attr BETWEEN val1 AND val2
    let between_re = find_between(part);
    if let Some((attr_str, val1_str, val2_str)) = between_re {
        let attr = resolve_name(attr_str.trim(), names);
        let val1 = resolve_value(val1_str.trim(), values)
            .ok_or_else(|| {
                DynamoDbError::ValidationException(format!(
                    "Value {} not found in ExpressionAttributeValues",
                    val1_str.trim()
                ))
            })?
            .clone();
        let val2 = resolve_value(val2_str.trim(), values)
            .ok_or_else(|| {
                DynamoDbError::ValidationException(format!(
                    "Value {} not found in ExpressionAttributeValues",
                    val2_str.trim()
                ))
            })?
            .clone();

        return Ok(KeyCondition {
            attribute_name: attr,
            operator: KeyConditionOp::Between,
            value: val1,
            value2: Some(val2),
        });
    }

    // Simple comparison: attr OP val
    let operators = ["<=", ">=", "<>", "=", "<", ">"];
    for op_str in &operators {
        if op_str == &"<>" {
            continue; // Not valid for key conditions
        }
        if let Some(pos) = part.find(op_str) {
            let attr_str = part[..pos].trim();
            let val_str = part[pos + op_str.len()..].trim();

            let attr = resolve_name(attr_str, names);
            let val = resolve_value(val_str, values)
                .ok_or_else(|| {
                    DynamoDbError::ValidationException(format!(
                        "Value {} not found in ExpressionAttributeValues",
                        val_str
                    ))
                })?
                .clone();

            let operator = match *op_str {
                "=" => KeyConditionOp::Eq,
                "<" => KeyConditionOp::Lt,
                "<=" => KeyConditionOp::Le,
                ">" => KeyConditionOp::Gt,
                ">=" => KeyConditionOp::Ge,
                _ => unreachable!(),
            };

            return Ok(KeyCondition {
                attribute_name: attr,
                operator,
                value: val,
                value2: None,
            });
        }
    }

    Err(DynamoDbError::ValidationException(format!(
        "Invalid key condition expression: {}",
        part
    )))
}

fn extract_function_args<'a>(expr: &'a str, func_name: &str) -> Option<&'a str> {
    let lower = expr.to_lowercase();
    let prefix = format!("{}(", func_name);
    if let Some(start) = lower.find(&prefix) {
        let rest = &expr[start + prefix.len()..];
        if let Some(end) = rest.rfind(')') {
            return Some(&rest[..end]);
        }
    }
    None
}

fn find_between<'a>(expr: &'a str) -> Option<(&'a str, &'a str, &'a str)> {
    let lower = expr.to_lowercase();
    let between_pos = lower.find(" between ")?;
    let attr = &expr[..between_pos];
    let rest = &expr[between_pos + 9..]; // " between " is 9 chars
    let lower_rest = rest.to_lowercase();
    let and_pos = lower_rest.find(" and ")?;
    let val1 = &rest[..and_pos];
    let val2 = &rest[and_pos + 5..];
    Some((attr, val1, val2))
}

fn evaluate_key_conditions(item: &Item, conditions: &[KeyCondition]) -> bool {
    conditions.iter().all(|cond| {
        let item_value = match item.get(&cond.attribute_name) {
            Some(v) => v,
            None => return false,
        };

        match cond.operator {
            KeyConditionOp::Eq => attribute_value_equals(item_value, &cond.value),
            KeyConditionOp::Lt => compare_attribute_values(Some(item_value), Some(&cond.value))
                == std::cmp::Ordering::Less,
            KeyConditionOp::Le => {
                matches!(
                    compare_attribute_values(Some(item_value), Some(&cond.value)),
                    std::cmp::Ordering::Less | std::cmp::Ordering::Equal
                )
            }
            KeyConditionOp::Gt => compare_attribute_values(Some(item_value), Some(&cond.value))
                == std::cmp::Ordering::Greater,
            KeyConditionOp::Ge => {
                matches!(
                    compare_attribute_values(Some(item_value), Some(&cond.value)),
                    std::cmp::Ordering::Greater | std::cmp::Ordering::Equal
                )
            }
            KeyConditionOp::BeginsWith => {
                let item_str = extract_string_value(item_value).unwrap_or_default();
                let prefix_str = extract_string_value(&cond.value).unwrap_or_default();
                item_str.starts_with(&prefix_str)
            }
            KeyConditionOp::Between => {
                let cmp1 = compare_attribute_values(Some(item_value), Some(&cond.value));
                let cmp2 = compare_attribute_values(
                    Some(item_value),
                    cond.value2.as_ref(),
                );
                matches!(cmp1, std::cmp::Ordering::Greater | std::cmp::Ordering::Equal)
                    && matches!(cmp2, std::cmp::Ordering::Less | std::cmp::Ordering::Equal)
            }
        }
    })
}

fn attribute_value_equals(a: &Value, b: &Value) -> bool {
    a == b
}

fn extract_string_value(val: &Value) -> Option<String> {
    // AttributeValue is {"S": "string"} or {"N": "123"}
    if let Some(s) = val.get("S").and_then(|v| v.as_str()) {
        return Some(s.to_string());
    }
    if let Some(n) = val.get("N").and_then(|v| v.as_str()) {
        return Some(n.to_string());
    }
    if let Some(b) = val.get("B").and_then(|v| v.as_str()) {
        return Some(b.to_string());
    }
    None
}

fn extract_number_value(val: &Value) -> Option<f64> {
    val.get("N")
        .and_then(|v| v.as_str())
        .and_then(|s| s.parse::<f64>().ok())
}

fn compare_attribute_values(a: Option<&Value>, b: Option<&Value>) -> std::cmp::Ordering {
    match (a, b) {
        (None, None) => std::cmp::Ordering::Equal,
        (None, Some(_)) => std::cmp::Ordering::Less,
        (Some(_), None) => std::cmp::Ordering::Greater,
        (Some(a), Some(b)) => {
            // Try numeric comparison first
            if let (Some(na), Some(nb)) = (extract_number_value(a), extract_number_value(b)) {
                return na.partial_cmp(&nb).unwrap_or(std::cmp::Ordering::Equal);
            }
            // String comparison
            let sa = extract_string_value(a).unwrap_or_default();
            let sb = extract_string_value(b).unwrap_or_default();
            sa.cmp(&sb)
        }
    }
}

// --- Filter expression evaluation ---

fn evaluate_filter_expression(
    item: &Item,
    expr: &str,
    names: Option<&HashMap<String, String>>,
    values: Option<&HashMap<String, Value>>,
) -> bool {
    // Support basic comparison expressions: attr = val, attr <> val, attr < val, etc.
    // Also support AND / OR and NOT, attribute_exists(), attribute_not_exists(), begins_with()

    // Parse and evaluate recursively
    evaluate_expr(item, expr.trim(), names, values)
}

fn evaluate_expr(
    item: &Item,
    expr: &str,
    names: Option<&HashMap<String, String>>,
    values: Option<&HashMap<String, Value>>,
) -> bool {
    let expr = expr.trim();

    // Handle OR (lower precedence than AND)
    if let Some((left, right)) = split_logical_op(expr, " OR ", " or ") {
        return evaluate_expr(item, left, names, values)
            || evaluate_expr(item, right, names, values);
    }

    // Handle AND
    if let Some((left, right)) = split_logical_op(expr, " AND ", " and ") {
        return evaluate_expr(item, left, names, values)
            && evaluate_expr(item, right, names, values);
    }

    // Handle NOT
    let lower = expr.to_lowercase();
    if lower.starts_with("not ") {
        return !evaluate_expr(item, &expr[4..], names, values);
    }

    // Handle parenthesized expressions
    if expr.starts_with('(') && expr.ends_with(')') {
        return evaluate_expr(item, &expr[1..expr.len() - 1], names, values);
    }

    // Handle functions
    if let Some(inner) = extract_function_args(expr, "attribute_exists") {
        let attr = resolve_name(inner.trim(), names);
        return item.contains_key(&attr);
    }

    if let Some(inner) = extract_function_args(expr, "attribute_not_exists") {
        let attr = resolve_name(inner.trim(), names);
        return !item.contains_key(&attr);
    }

    if let Some(inner) = extract_function_args(expr, "begins_with") {
        let args: Vec<&str> = inner.splitn(2, ',').collect();
        if args.len() == 2 {
            let attr = resolve_name(args[0].trim(), names);
            let val = resolve_value(args[1].trim(), values);
            if let Some(item_val) = item.get(&attr) {
                if let Some(cmp_val) = val {
                    let item_str = extract_string_value(item_val).unwrap_or_default();
                    let prefix_str = extract_string_value(cmp_val).unwrap_or_default();
                    return item_str.starts_with(&prefix_str);
                }
            }
            return false;
        }
    }

    if let Some(inner) = extract_function_args(expr, "contains") {
        let args: Vec<&str> = inner.splitn(2, ',').collect();
        if args.len() == 2 {
            let attr = resolve_name(args[0].trim(), names);
            let val = resolve_value(args[1].trim(), values);
            if let Some(item_val) = item.get(&attr) {
                if let Some(cmp_val) = val {
                    let item_str = extract_string_value(item_val).unwrap_or_default();
                    let search_str = extract_string_value(cmp_val).unwrap_or_default();
                    return item_str.contains(&search_str);
                }
            }
            return false;
        }
    }

    // Simple comparison operators
    let operators: &[(&str, fn(&Value, &Value) -> bool)] = &[
        ("<>", |a, b| !attribute_value_equals(a, b)),
        ("<=", |a, b| {
            matches!(
                compare_attribute_values(Some(a), Some(b)),
                std::cmp::Ordering::Less | std::cmp::Ordering::Equal
            )
        }),
        (">=", |a, b| {
            matches!(
                compare_attribute_values(Some(a), Some(b)),
                std::cmp::Ordering::Greater | std::cmp::Ordering::Equal
            )
        }),
        ("=", |a, b| attribute_value_equals(a, b)),
        ("<", |a, b| {
            compare_attribute_values(Some(a), Some(b)) == std::cmp::Ordering::Less
        }),
        (">", |a, b| {
            compare_attribute_values(Some(a), Some(b)) == std::cmp::Ordering::Greater
        }),
    ];

    for (op_str, op_fn) in operators {
        if let Some(pos) = find_operator_pos(expr, op_str) {
            let attr_str = expr[..pos].trim();
            let val_str = expr[pos + op_str.len()..].trim();

            let attr = resolve_name(attr_str, names);
            if let Some(item_val) = item.get(&attr) {
                if let Some(cmp_val) = resolve_value(val_str, values) {
                    return op_fn(item_val, cmp_val);
                }
            }
            return false;
        }
    }

    // Default: if we can't parse, don't filter
    true
}

fn find_operator_pos(expr: &str, op: &str) -> Option<usize> {
    // Find operator position that's not inside a function call
    let mut paren_depth = 0;
    let bytes = expr.as_bytes();
    let op_bytes = op.as_bytes();
    let op_len = op_bytes.len();

    if expr.len() < op_len {
        return None;
    }

    for i in 0..=(expr.len() - op_len) {
        match bytes[i] {
            b'(' => paren_depth += 1,
            b')' => paren_depth -= 1,
            _ => {}
        }
        if paren_depth == 0 && &bytes[i..i + op_len] == op_bytes {
            // Make sure it's not part of a larger operator (e.g. <= vs <)
            return Some(i);
        }
    }
    None
}

fn split_logical_op<'a>(
    expr: &'a str,
    op_upper: &str,
    op_lower: &str,
) -> Option<(&'a str, &'a str)> {
    let mut paren_depth = 0;
    let bytes = expr.as_bytes();
    let op_len = op_upper.len();

    if expr.len() < op_len {
        return None;
    }

    for i in 0..=(expr.len() - op_len) {
        match bytes[i] {
            b'(' => paren_depth += 1,
            b')' => {
                if paren_depth > 0 {
                    paren_depth -= 1;
                }
            }
            _ => {}
        }
        if paren_depth == 0 {
            let slice = &expr[i..];
            if slice.starts_with(op_upper) || slice.starts_with(op_lower) {
                return Some((&expr[..i], &expr[i + op_len..]));
            }
        }
    }
    None
}

// --- UpdateExpression parsing ---

fn apply_update_expression(
    item: &mut Item,
    expr: &str,
    names: Option<&HashMap<String, String>>,
    values: Option<&HashMap<String, Value>>,
) -> Result<(), DynamoDbError> {
    let expr = expr.trim();

    // Parse SET, REMOVE, ADD, DELETE actions
    // Format: SET attr1 = val1, attr2 = val2 REMOVE attr3, attr4 ADD attr5 val5 DELETE attr6 val6
    let actions = parse_update_actions(expr);

    for (action_type, action_body) in actions {
        match action_type.to_uppercase().as_str() {
            "SET" => apply_set_action(item, &action_body, names, values)?,
            "REMOVE" => apply_remove_action(item, &action_body, names)?,
            "ADD" => apply_add_action(item, &action_body, names, values)?,
            "DELETE" => apply_delete_action(item, &action_body, names, values)?,
            _ => {
                return Err(DynamoDbError::ValidationException(format!(
                    "Invalid update expression action: {}",
                    action_type
                )));
            }
        }
    }

    Ok(())
}

fn parse_update_actions(expr: &str) -> Vec<(String, String)> {
    let mut actions = Vec::new();
    let keywords = ["SET", "REMOVE", "ADD", "DELETE"];

    // Find all action positions
    let upper = expr.to_uppercase();
    let mut positions: Vec<(usize, &str)> = Vec::new();

    for kw in &keywords {
        let mut search_from = 0;
        while let Some(pos) = upper[search_from..].find(kw) {
            let abs_pos = search_from + pos;
            // Check word boundary: must be at start or preceded by space, and followed by space
            let at_start = abs_pos == 0 || expr.as_bytes()[abs_pos - 1] == b' ';
            let at_end = abs_pos + kw.len() >= expr.len()
                || expr.as_bytes()[abs_pos + kw.len()] == b' ';
            if at_start && at_end {
                positions.push((abs_pos, kw));
            }
            search_from = abs_pos + kw.len();
        }
    }

    positions.sort_by_key(|p| p.0);

    for i in 0..positions.len() {
        let (pos, kw) = positions[i];
        let body_start = pos + kw.len();
        let body_end = if i + 1 < positions.len() {
            positions[i + 1].0
        } else {
            expr.len()
        };
        let body = expr[body_start..body_end].trim().to_string();
        actions.push((kw.to_string(), body));
    }

    actions
}

fn apply_set_action(
    item: &mut Item,
    body: &str,
    names: Option<&HashMap<String, String>>,
    values: Option<&HashMap<String, Value>>,
) -> Result<(), DynamoDbError> {
    // SET attr1 = val1, attr2 = val2
    let assignments = split_set_assignments(body);

    for assignment in assignments {
        let assignment = assignment.trim();
        let eq_pos = assignment.find('=').ok_or_else(|| {
            DynamoDbError::ValidationException(format!(
                "Invalid SET expression: {}",
                assignment
            ))
        })?;

        let attr_str = assignment[..eq_pos].trim();
        let val_str = assignment[eq_pos + 1..].trim();

        let attr = resolve_name(attr_str, names);

        // Resolve value: could be a reference (:val) or a function like if_not_exists, list_append
        let value = resolve_set_value(item, val_str, names, values)?;
        item.insert(attr, value);
    }

    Ok(())
}

fn split_set_assignments(body: &str) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut last = 0;
    let mut paren_depth = 0;

    for (i, c) in body.char_indices() {
        match c {
            '(' => paren_depth += 1,
            ')' => {
                if paren_depth > 0 {
                    paren_depth -= 1;
                }
            }
            ',' if paren_depth == 0 => {
                parts.push(&body[last..i]);
                last = i + 1;
            }
            _ => {}
        }
    }
    parts.push(&body[last..]);
    parts
}

fn resolve_set_value(
    item: &Item,
    val_str: &str,
    names: Option<&HashMap<String, String>>,
    values: Option<&HashMap<String, Value>>,
) -> Result<Value, DynamoDbError> {
    let val_str = val_str.trim();

    // Check for if_not_exists(attr, val)
    if let Some(inner) = extract_function_args(val_str, "if_not_exists") {
        let args: Vec<&str> = inner.splitn(2, ',').collect();
        if args.len() == 2 {
            let attr = resolve_name(args[0].trim(), names);
            if let Some(existing) = item.get(&attr) {
                return Ok(existing.clone());
            }
            return resolve_set_value(item, args[1].trim(), names, values);
        }
    }

    // Check for list_append(list1, list2)
    if let Some(inner) = extract_function_args(val_str, "list_append") {
        let args: Vec<&str> = inner.splitn(2, ',').collect();
        if args.len() == 2 {
            let list1 = resolve_set_value(item, args[0].trim(), names, values)?;
            let list2 = resolve_set_value(item, args[1].trim(), names, values)?;

            let mut result = Vec::new();
            if let Some(l) = list1.get("L").and_then(|v| v.as_array()) {
                result.extend(l.iter().cloned());
            }
            if let Some(l) = list2.get("L").and_then(|v| v.as_array()) {
                result.extend(l.iter().cloned());
            }
            return Ok(serde_json::json!({"L": result}));
        }
    }

    // Check for arithmetic: attr + :val or attr - :val
    if let Some((left, right, is_add)) = parse_arithmetic(val_str) {
        let left_val = resolve_set_value(item, left.trim(), names, values)?;
        let right_val = resolve_set_value(item, right.trim(), names, values)?;
        let left_num = extract_number_value(&left_val).unwrap_or(0.0);
        let right_num = extract_number_value(&right_val).unwrap_or(0.0);
        let result = if is_add {
            left_num + right_num
        } else {
            left_num - right_num
        };
        // Format number without trailing zeros, like DynamoDB does
        let formatted = format_number(result);
        return Ok(serde_json::json!({"N": formatted}));
    }

    // Simple value reference (:val)
    if val_str.starts_with(':') {
        if let Some(v) = resolve_value(val_str, values) {
            return Ok(v.clone());
        }
        return Err(DynamoDbError::ValidationException(format!(
            "Value {} not found in ExpressionAttributeValues",
            val_str
        )));
    }

    // Attribute reference (#name or plain name)
    let attr = resolve_name(val_str, names);
    if let Some(v) = item.get(&attr) {
        return Ok(v.clone());
    }

    Err(DynamoDbError::ValidationException(format!(
        "Cannot resolve value: {}",
        val_str
    )))
}

fn parse_arithmetic(expr: &str) -> Option<(&str, &str, bool)> {
    // Look for + or - not inside a function call
    let mut paren_depth = 0;
    let bytes = expr.as_bytes();

    for (i, &b) in bytes.iter().enumerate() {
        match b {
            b'(' => paren_depth += 1,
            b')' => {
                if paren_depth > 0 {
                    paren_depth -= 1;
                }
            }
            b'+' if paren_depth == 0 && i > 0 => {
                return Some((&expr[..i], &expr[i + 1..], true));
            }
            b'-' if paren_depth == 0 && i > 0 => {
                // Make sure it's not a negative number or part of an attribute name
                let left = expr[..i].trim();
                if !left.is_empty() && !left.ends_with('(') {
                    return Some((&expr[..i], &expr[i + 1..], false));
                }
            }
            _ => {}
        }
    }
    None
}

fn format_number(n: f64) -> String {
    if n == n.floor() && n.abs() < 1e15 {
        format!("{}", n as i64)
    } else {
        format!("{}", n)
    }
}

fn apply_remove_action(
    item: &mut Item,
    body: &str,
    names: Option<&HashMap<String, String>>,
) -> Result<(), DynamoDbError> {
    // REMOVE attr1, attr2
    for attr_str in body.split(',') {
        let attr = resolve_name(attr_str.trim(), names);
        item.remove(&attr);
    }
    Ok(())
}

fn apply_add_action(
    item: &mut Item,
    body: &str,
    names: Option<&HashMap<String, String>>,
    values: Option<&HashMap<String, Value>>,
) -> Result<(), DynamoDbError> {
    // ADD attr val - for numbers, adds to existing; for sets, adds elements
    let parts: Vec<&str> = body.trim().splitn(2, ' ').collect();
    if parts.len() != 2 {
        return Err(DynamoDbError::ValidationException(
            "Invalid ADD expression".into(),
        ));
    }

    let attr = resolve_name(parts[0].trim(), names);
    let val_token = parts[1].trim();
    let val = resolve_value(val_token, values).ok_or_else(|| {
        DynamoDbError::ValidationException(format!(
            "Value {} not found in ExpressionAttributeValues",
            val_token
        ))
    })?;

    if let Some(existing) = item.get(&attr).cloned() {
        // Numeric ADD
        if let (Some(existing_num), Some(add_num)) =
            (extract_number_value(&existing), extract_number_value(val))
        {
            let result = existing_num + add_num;
            let formatted = format_number(result);
            item.insert(attr, serde_json::json!({"N": formatted}));
            return Ok(());
        }

        // Set ADD (SS, NS, BS)
        for set_type in &["SS", "NS", "BS"] {
            if let (Some(existing_set), Some(add_set)) = (
                existing.get(set_type).and_then(|v| v.as_array()),
                val.get(set_type).and_then(|v| v.as_array()),
            ) {
                let mut merged = existing_set.clone();
                for elem in add_set {
                    if !merged.contains(elem) {
                        merged.push(elem.clone());
                    }
                }
                let mut map = serde_json::Map::new();
                map.insert(set_type.to_string(), Value::Array(merged));
                item.insert(attr, Value::Object(map));
                return Ok(());
            }
        }
    } else {
        // Attribute doesn't exist, set it
        item.insert(attr, val.clone());
    }

    Ok(())
}

fn apply_delete_action(
    item: &mut Item,
    body: &str,
    names: Option<&HashMap<String, String>>,
    values: Option<&HashMap<String, Value>>,
) -> Result<(), DynamoDbError> {
    // DELETE attr val - remove elements from a set
    let parts: Vec<&str> = body.trim().splitn(2, ' ').collect();
    if parts.len() != 2 {
        return Err(DynamoDbError::ValidationException(
            "Invalid DELETE expression".into(),
        ));
    }

    let attr = resolve_name(parts[0].trim(), names);
    let val_token = parts[1].trim();
    let val = resolve_value(val_token, values).ok_or_else(|| {
        DynamoDbError::ValidationException(format!(
            "Value {} not found in ExpressionAttributeValues",
            val_token
        ))
    })?;

    if let Some(existing) = item.get(&attr).cloned() {
        for set_type in &["SS", "NS", "BS"] {
            if let (Some(existing_set), Some(remove_set)) = (
                existing.get(set_type).and_then(|v| v.as_array()),
                val.get(set_type).and_then(|v| v.as_array()),
            ) {
                let filtered: Vec<Value> = existing_set
                    .iter()
                    .filter(|elem| !remove_set.contains(elem))
                    .cloned()
                    .collect();
                if filtered.is_empty() {
                    item.remove(&attr);
                } else {
                    let mut map = serde_json::Map::new();
                    map.insert(set_type.to_string(), Value::Array(filtered));
                    item.insert(attr, Value::Object(map));
                }
                return Ok(());
            }
        }
    }

    Ok(())
}
