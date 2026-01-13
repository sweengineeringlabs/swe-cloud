use async_trait::async_trait;
use cloudkit::api::{Condition, KeyValueStore, KvGetOptions, KvPutOptions, KvQueryOptions};
use cloudkit::common::{CloudError, CloudResult, ListResult, PaginationToken};
use cloudkit::core::CloudContext;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

/// AWS DynamoDB key-value store implementation.
pub struct DynamoDbStore {
    _context: Arc<CloudContext>,
    client: aws_sdk_dynamodb::Client,
}

impl DynamoDbStore {
    /// Create a new DynamoDB store.
    pub fn new(context: Arc<CloudContext>, sdk_config: aws_config::SdkConfig) -> Self {
        let client = aws_sdk_dynamodb::Client::new(&sdk_config);
        Self { _context: context, client }
    }

    fn to_attribute_value(val: serde_json::Value) -> aws_sdk_dynamodb::types::AttributeValue {
        match val {
            serde_json::Value::String(s) => aws_sdk_dynamodb::types::AttributeValue::S(s),
            serde_json::Value::Number(n) => aws_sdk_dynamodb::types::AttributeValue::N(n.to_string()),
            serde_json::Value::Bool(b) => aws_sdk_dynamodb::types::AttributeValue::Bool(b),
            serde_json::Value::Null => aws_sdk_dynamodb::types::AttributeValue::Null(true),
            serde_json::Value::Array(a) => {
                aws_sdk_dynamodb::types::AttributeValue::L(a.into_iter().map(Self::to_attribute_value).collect())
            }
            serde_json::Value::Object(o) => {
                aws_sdk_dynamodb::types::AttributeValue::M(
                    o.into_iter().map(|(k, v)| (k, Self::to_attribute_value(v))).collect(),
                )
            }
        }
    }

    fn from_attribute_value(val: aws_sdk_dynamodb::types::AttributeValue) -> serde_json::Value {
        match val {
            aws_sdk_dynamodb::types::AttributeValue::S(s) => serde_json::Value::String(s),
            aws_sdk_dynamodb::types::AttributeValue::N(n) => {
                if let Ok(i) = n.parse::<i64>() {
                    serde_json::Value::Number(i.into())
                } else if let Ok(f) = n.parse::<f64>() {
                    serde_json::value::Number::from_f64(f)
                        .map(serde_json::Value::Number)
                        .unwrap_or(serde_json::Value::String(n))
                } else {
                    serde_json::Value::String(n)
                }
            }
            aws_sdk_dynamodb::types::AttributeValue::Bool(b) => serde_json::Value::Bool(b),
            aws_sdk_dynamodb::types::AttributeValue::Null(_) => serde_json::Value::Null,
            aws_sdk_dynamodb::types::AttributeValue::L(l) => {
                serde_json::Value::Array(l.into_iter().map(Self::from_attribute_value).collect())
            }
            aws_sdk_dynamodb::types::AttributeValue::M(m) => {
                serde_json::Value::Object(
                    m.into_iter().map(|(k, v)| (k, Self::from_attribute_value(v))).collect(),
                )
            }
            _ => serde_json::Value::Null,
        }
    }

    fn build_condition_expression(&self, condition: Condition) -> (String, HashMap<String, aws_sdk_dynamodb::types::AttributeValue>) {
        let mut values = HashMap::new();
        let expr = match condition {
            Condition::Equals(k, v) => {
                let p = format!(":v_{}", k.replace(".", "_"));
                values.insert(p.clone(), Self::to_attribute_value(v));
                format!("{} = {}", k, p)
            }
            Condition::NotEquals(k, v) => {
                let p = format!(":v_{}", k.replace(".", "_"));
                values.insert(p.clone(), Self::to_attribute_value(v));
                format!("{} <> {}", k, p)
            }
            Condition::GreaterThan(k, v) => {
                let p = format!(":v_{}", k.replace(".", "_"));
                values.insert(p.clone(), Self::to_attribute_value(v));
                format!("{} > {}", k, p)
            }
            Condition::GreaterThanOrEqual(k, v) => {
                let p = format!(":v_{}", k.replace(".", "_"));
                values.insert(p.clone(), Self::to_attribute_value(v));
                format!("{} >= {}", k, p)
            }
            Condition::LessThan(k, v) => {
                let p = format!(":v_{}", k.replace(".", "_"));
                values.insert(p.clone(), Self::to_attribute_value(v));
                format!("{} < {}", k, p)
            }
            Condition::LessThanOrEqual(k, v) => {
                let p = format!(":v_{}", k.replace(".", "_"));
                values.insert(p.clone(), Self::to_attribute_value(v));
                format!("{} <= {}", k, p)
            }
            Condition::And(conds) => {
                let mut exprs = Vec::new();
                for c in conds {
                    let (e, v) = self.build_condition_expression(c);
                    values.extend(v);
                    exprs.push(format!("({})", e));
                }
                exprs.join(" AND ")
            }
            Condition::Or(conds) => {
                let mut exprs = Vec::new();
                for c in conds {
                    let (e, v) = self.build_condition_expression(c);
                    values.extend(v);
                    exprs.push(format!("({})", e));
                }
                exprs.join(" OR ")
            }
            Condition::Not(c) => {
                let (e, v) = self.build_condition_expression(*c);
                values.extend(v);
                format!("NOT ({})", e)
            }
            Condition::Exists(k) => format!("attribute_exists({})", k),
            Condition::NotExists(k) => format!("attribute_not_exists({})", k),
            Condition::In(k, vals) => {
                let mut placeholders = Vec::new();
                for (i, v) in vals.into_iter().enumerate() {
                    let p = format!(":v{}_{}", i, k.replace(".", "_"));
                    values.insert(p.clone(), Self::to_attribute_value(v));
                    placeholders.push(p);
                }
                format!("{} IN ({})", k, placeholders.join(", "))
            }
            Condition::Between(k, v1, v2) => {
                let p1 = format!(":v1_{}", k.replace(".", "_"));
                let p2 = format!(":v2_{}", k.replace(".", "_"));
                values.insert(p1.clone(), Self::to_attribute_value(v1));
                values.insert(p2.clone(), Self::to_attribute_value(v2));
                format!("{} BETWEEN {} AND {}", k, p1, p2)
            }
            Condition::BeginsWith(k, v) => {
                let p = format!(":v_{}", k.replace(".", "_"));
                values.insert(p.clone(), Self::to_attribute_value(Value::String(v)));
                format!("begins_with({}, {})", k, p)
            }
            Condition::Contains(k, v) => {
                let p = format!(":v_{}", k.replace(".", "_"));
                values.insert(p.clone(), Self::to_attribute_value(Value::String(v)));
                format!("contains({}, {})", k, p)
            }
        };
        (expr, values)
    }
}

#[async_trait]
impl KeyValueStore for DynamoDbStore {
    async fn get<T: DeserializeOwned + Send>(
        &self,
        table: &str,
        key: &str,
    ) -> CloudResult<Option<T>> {
        self.get_with_options(table, key, KvGetOptions::default()).await
    }

    async fn get_with_options<T: DeserializeOwned + Send>(
        &self,
        table: &str,
        key: &str,
        options: KvGetOptions,
    ) -> CloudResult<Option<T>> {
        let mut req = self.client.get_item()
            .table_name(table)
            .key("id", aws_sdk_dynamodb::types::AttributeValue::S(key.to_string()));
            
        if options.consistent_read {
            req = req.consistent_read(true);
        }
        
        let resp = req.send().await.map_err(|e| CloudError::ServiceError(e.to_string()))?;
        
        if let Some(item) = resp.item {
            let json_val = serde_json::Value::Object(
                item.into_iter().map(|(k, v)| (k, Self::from_attribute_value(v))).collect()
            );
            let result = serde_json::from_value(json_val).map_err(|e| CloudError::Serialization(e.to_string()))?;
            Ok(Some(result))
        } else {
            Ok(None)
        }
    }

    async fn put<T: Serialize + Send + Sync>(
        &self,
        table: &str,
        key: &str,
        item: &T,
    ) -> CloudResult<()> {
        self.put_with_options(table, key, item, KvPutOptions::default()).await?;
        Ok(())
    }

    async fn put_with_options<T: Serialize + Send + Sync>(
        &self,
        table: &str,
        key: &str,
        item: &T,
        _options: KvPutOptions,
    ) -> CloudResult<Option<serde_json::Value>> {
        let json_item = serde_json::to_value(item).map_err(|e| CloudError::Serialization(e.to_string()))?;
        
        let mut put_req = self.client.put_item().table_name(table);
        
        if let serde_json::Value::Object(mut o) = json_item {
            // Ensure the key is present
            o.insert("id".to_string(), serde_json::Value::String(key.to_string()));
            
            for (k, v) in o {
                put_req = put_req.item(k, Self::to_attribute_value(v));
            }
        }
        
        put_req.send().await.map_err(|e| CloudError::ServiceError(e.to_string()))?;
        Ok(None)
    }

    async fn delete(&self, table: &str, key: &str) -> CloudResult<()> {
        self.client.delete_item()
            .table_name(table)
            .key("id", aws_sdk_dynamodb::types::AttributeValue::S(key.to_string()))
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
        Ok(())
    }

    async fn delete_with_condition(
        &self,
        table: &str,
        key: &str,
        condition: Condition,
    ) -> CloudResult<bool> {
        let (expr, values) = self.build_condition_expression(condition);
        
        let mut req = self.client.delete_item()
            .table_name(table)
            .key("id", aws_sdk_dynamodb::types::AttributeValue::S(key.to_string()))
            .condition_expression(expr);
            
        for (k, v) in values {
            req = req.expression_attribute_values(k, v);
        }
        
        match req.send().await {
            Ok(_) => Ok(true),
            Err(e) => {
                if e.to_string().contains("ConditionalCheckFailedException") {
                    Ok(false)
                } else {
                    Err(CloudError::ServiceError(e.to_string()))
                }
            }
        }
    }


    async fn exists(&self, table: &str, key: &str) -> CloudResult<bool> {
        let resp = self.client.get_item()
            .table_name(table)
            .key("id", aws_sdk_dynamodb::types::AttributeValue::S(key.to_string()))
            .projection_expression("id")
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
        Ok(resp.item.is_some())
    }

    async fn update(
        &self,
        table: &str,
        key: &str,
        updates: HashMap<String, serde_json::Value>,
    ) -> CloudResult<()> {
        let mut req = self.client.update_item()
            .table_name(table)
            .key("id", aws_sdk_dynamodb::types::AttributeValue::S(key.to_string()));
            
        for (k, v) in updates {
            req = req.attribute_updates(k, aws_sdk_dynamodb::types::AttributeValueUpdate::builder()
                .value(Self::to_attribute_value(v))
                .action(aws_sdk_dynamodb::types::AttributeAction::Put)
                .build());
        }
        
        req.send().await.map_err(|e| CloudError::ServiceError(e.to_string()))?;
        Ok(())
    }

    async fn query<T: DeserializeOwned + Send>(
        &self,
        table: &str,
        partition_key: &str,
        options: KvQueryOptions,
    ) -> CloudResult<ListResult<T>> {
        let mut req = self.client.query()
            .table_name(table)
            .key_condition_expression("id = :id")
            .expression_attribute_values(":id", aws_sdk_dynamodb::types::AttributeValue::S(partition_key.to_string()));
            
        if let Some(limit) = options.limit {
            req = req.limit(limit as i32);
        }
        
        if let Some(token) = options.continuation_token {
            req = req.exclusive_start_key("id", aws_sdk_dynamodb::types::AttributeValue::S(token));
        }
        
        let resp = req.send().await.map_err(|e| CloudError::ServiceError(e.to_string()))?;
        
        let mut items = Vec::new();
        if let Some(raw_items) = resp.items {
            for item in raw_items {
                let json_val = serde_json::Value::Object(
                    item.into_iter().map(|(k, v)| (k, Self::from_attribute_value(v))).collect()
                );
                let result = serde_json::from_value(json_val).map_err(|e| CloudError::Serialization(e.to_string()))?;
                items.push(result);
            }
        }
        
        let token = resp.last_evaluated_key
            .and_then(|m| m.get("id").cloned())
            .and_then(|v| match v {
                aws_sdk_dynamodb::types::AttributeValue::S(s) => Some(s),
                _ => None
            });
            
        Ok(ListResult::new(items, token.map(PaginationToken::some).unwrap_or(PaginationToken::none())))
    }

    async fn batch_get<T: DeserializeOwned + Send>(
        &self,
        table: &str,
        keys: &[&str],
    ) -> CloudResult<Vec<T>> {
        let mut keys_vec = Vec::new();
        for key in keys {
            let mut key_map = HashMap::new();
            key_map.insert("id".to_string(), aws_sdk_dynamodb::types::AttributeValue::S(key.to_string()));
            keys_vec.push(key_map);
        }
        
        let resp = self.client.batch_get_item()
            .request_items(table, aws_sdk_dynamodb::types::KeysAndAttributes::builder()
                .set_keys(Some(keys_vec))
                .build()
                .unwrap())
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
            
        let mut result = Vec::new();
        if let Some(responses) = resp.responses {
            if let Some(items) = responses.get(table) {
                for item in items {
                    let json_val = serde_json::Value::Object(
                        item.iter().map(|(k, v)| (k.clone(), Self::from_attribute_value(v.clone()))).collect()
                    );
                    let val = serde_json::from_value(json_val).map_err(|e| CloudError::Serialization(e.to_string()))?;
                    result.push(val);
                }
            }
        }
        
        Ok(result)
    }

    async fn batch_write<T: Serialize + Send + Sync>(
        &self,
        table: &str,
        items: &[(&str, &T)],
    ) -> CloudResult<()>{
        let mut requests = Vec::new();
        for (key, item) in items {
            let json_item = serde_json::to_value(item).map_err(|e| CloudError::Serialization(e.to_string()))?;
            let mut item_map = HashMap::new();
            if let serde_json::Value::Object(o) = json_item {
                for (k, v) in o {
                    item_map.insert(k, Self::to_attribute_value(v));
                }
            }
            item_map.insert("id".to_string(), aws_sdk_dynamodb::types::AttributeValue::S(key.to_string()));
            
            requests.push(aws_sdk_dynamodb::types::WriteRequest::builder()
                .put_request(aws_sdk_dynamodb::types::PutRequest::builder()
                    .set_item(Some(item_map))
                    .build()
                    .unwrap())
                .build());
        }
        
        self.client.batch_write_item()
            .request_items(table, requests)
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
            
        Ok(())
    }
}
