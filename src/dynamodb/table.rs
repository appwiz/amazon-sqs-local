use std::collections::HashMap;

use super::types::*;

#[derive(Debug, Clone)]
pub struct Table {
    pub table_name: String,
    pub table_arn: String,
    pub table_id: String,
    pub key_schema: Vec<KeySchemaElement>,
    pub attribute_definitions: Vec<AttributeDefinition>,
    pub billing_mode: String,
    pub provisioned_throughput: ProvisionedThroughputDescription,
    pub creation_date_time: f64,
    pub table_status: String,
    pub items: Vec<Item>,
    pub tags: HashMap<String, String>,
}

impl Table {
    pub fn hash_key_name(&self) -> &str {
        self.key_schema
            .iter()
            .find(|k| k.key_type == "HASH")
            .map(|k| k.attribute_name.as_str())
            .unwrap()
    }

    pub fn range_key_name(&self) -> Option<&str> {
        self.key_schema
            .iter()
            .find(|k| k.key_type == "RANGE")
            .map(|k| k.attribute_name.as_str())
    }

    pub fn build_primary_key(&self, item: &Item) -> String {
        let hash = item
            .get(self.hash_key_name())
            .map(|v| v.to_string())
            .unwrap_or_default();
        match self.range_key_name() {
            Some(rk) => {
                let range = item.get(rk).map(|v| v.to_string()).unwrap_or_default();
                format!("{hash}|{range}")
            }
            None => hash,
        }
    }

    pub fn find_item_index(&self, key: &Item) -> Option<usize> {
        let target = self.build_primary_key(key);
        self.items.iter().position(|item| {
            self.build_primary_key(item) == target
        })
    }

    pub fn to_description(&self) -> TableDescription {
        let billing_mode_summary = if self.billing_mode == "PAY_PER_REQUEST" {
            Some(BillingModeSummary {
                billing_mode: self.billing_mode.clone(),
                last_update_to_pay_per_request_date_time: Some(self.creation_date_time),
            })
        } else {
            None
        };

        TableDescription {
            table_name: self.table_name.clone(),
            table_status: self.table_status.clone(),
            table_arn: self.table_arn.clone(),
            table_id: self.table_id.clone(),
            creation_date_time: self.creation_date_time,
            key_schema: self.key_schema.clone(),
            attribute_definitions: self.attribute_definitions.clone(),
            provisioned_throughput: self.provisioned_throughput.clone(),
            billing_mode_summary,
            item_count: self.items.len() as i64,
            table_size_bytes: 0,
        }
    }
}
