use crate::Emulator;
use crate::error::EmulatorError;
use axum::{
    extract::State,
    http::HeaderMap,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::{json, Value};
use std::sync::Arc;

pub async fn handle_request(
    State(emulator): State<Arc<Emulator>>,
    headers: HeaderMap,
    Json(body): Json<Value>,
) -> Response {
    let target = headers
        .get("x-amz-target")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("");
    
    let action = target.split('.').next_back().unwrap_or("");

    let result = match action {
        "CreateTable" => create_table(&emulator, body).await,
        "PutItem" => put_item(&emulator, body).await,
        "GetItem" => get_item(&emulator, body).await,
        "Query" => query(&emulator, body).await,
        "Scan" => scan(&emulator, body).await,
        "DescribeTable" => describe_table(&emulator, body).await,
        "ListTables" => list_tables(&emulator, body).await,
        _ => Err(EmulatorError::InvalidRequest(format!("Unsupported DynamoDB action: {}", action))),
    };

    match result {
        Ok(json_val) => Json::<Value>(json_val).into_response(),
        Err(e) => {
            let status = e.status_code();
            let json_err = json!({
                "__type": e.code(),
                "message": e.message()
            });
            (status, Json::<Value>(json_err)).into_response()
        }
    }
}

async fn create_table(emulator: &Emulator, body: Value) -> Result<Value, EmulatorError> {
    let name = body["TableName"].as_str().ok_or_else(|| EmulatorError::InvalidArgument("Missing TableName".into()))?;
    let attr_defs = serde_json::to_string(&body["AttributeDefinitions"]).unwrap_or_default();
    let key_schema = serde_json::to_string(&body["KeySchema"]).unwrap_or_default();
    
    let table = emulator.storage.create_table(
        name,
        &attr_defs,
        &key_schema,
        &emulator.config.account_id,
        &emulator.config.region
    )?;

    Ok(json!({
        "TableDescription": {
            "TableName": table.name,
            "TableArn": table.arn,
            "TableStatus": table.status,
            "CreationDateTime": 1234567890.0,
            "ItemCount": 0,
            "TableSizeBytes": 0
        }
    }))
}

async fn put_item(emulator: &Emulator, body: Value) -> Result<Value, EmulatorError> {
    let table_name = body["TableName"].as_str().ok_or_else(|| EmulatorError::InvalidArgument("Missing TableName".into()))?;
    let item = &body["Item"];
    
    // Retrieve table metadata to find PK name
    let table = emulator.storage.get_table(table_name)?;
    let key_schema: Vec<Value> = serde_json::from_str(&table.key_schema).unwrap_or_default();
    
    let pk_name = key_schema.iter()
        .find(|k| k["KeyType"] == "HASH")
        .and_then(|k| k["AttributeName"].as_str())
        .ok_or_else(|| EmulatorError::Internal("Table has no HASH key".into()))?;

    let pk_val_obj = item.get(pk_name).ok_or_else(|| EmulatorError::InvalidArgument(format!("Item missing partition key {}", pk_name)))?;
    
    // Extract actual value (S, N, etc)
    let pk_val = pk_val_obj.as_object()
        .and_then(|o| o.values().next())
        .and_then(|v| v.as_str())
        .ok_or_else(|| EmulatorError::InvalidArgument("Invalid partition key format".into()))?;

    // Handle Sort Key
    let sk_name = key_schema.iter()
        .find(|k| k["KeyType"] == "RANGE")
        .and_then(|k| k["AttributeName"].as_str());
        
    let sk_val = if let Some(sk) = sk_name {
         item.get(sk).and_then(|v| v.as_object().and_then(|o| o.values().next()).and_then(|v| v.as_str()))
    } else {
        None
    };
    
    emulator.storage.put_item(table_name, pk_val, sk_val, &item.to_string())?;
    
    Ok(json!({}))
}

async fn get_item(emulator: &Emulator, body: Value) -> Result<Value, EmulatorError> {
    let table_name = body["TableName"].as_str().ok_or_else(|| EmulatorError::InvalidArgument("Missing TableName".into()))?;
    let key = &body["Key"];

    // Retrieve table metadata to find PK name
    let table = emulator.storage.get_table(table_name)?;
    let key_schema: Vec<Value> = serde_json::from_str(&table.key_schema).unwrap_or_default();
    
    let pk_name = key_schema.iter()
        .find(|k| k["KeyType"] == "HASH")
        .and_then(|k| k["AttributeName"].as_str())
        .ok_or_else(|| EmulatorError::Internal("Table has no HASH key".into()))?;

    let pk_val_obj = key.get(pk_name).ok_or_else(|| EmulatorError::InvalidArgument(format!("Key missing partition key {}", pk_name)))?;
    
    // Extract actual value (S, N, etc)
    let pk_val = pk_val_obj.as_object()
        .and_then(|o| o.values().next())
        .and_then(|v| v.as_str())
        .ok_or_else(|| EmulatorError::InvalidArgument("Invalid partition key format".into()))?;

    // Handle Sort Key
    let sk_name = key_schema.iter()
        .find(|k| k["KeyType"] == "RANGE")
        .and_then(|k| k["AttributeName"].as_str());
        
    let sk_val = if let Some(sk) = sk_name {
         key.get(sk).and_then(|v| v.as_object().and_then(|o| o.values().next()).and_then(|v| v.as_str()))
    } else {
        None
    };

    let item_json = emulator.storage.get_item(table_name, pk_val, sk_val)?;
    
    match item_json {
        Some(json_str) => {
            let item: Value = serde_json::from_str(&json_str).unwrap_or(Value::Null);
            Ok(json!({ "Item": item }))
        }
        None => Ok(json!({})),
    }
}

async fn describe_table(_emulator: &Emulator, body: Value) -> Result<Value, EmulatorError> {
     let name = body["TableName"].as_str().unwrap_or("unknown");
     Ok(json!({
        "Table": {
            "TableName": name,
            "TableStatus": "ACTIVE",
            "CreationDateTime": 1234567890.0
        }
     }))
}

async fn list_tables(_emulator: &Emulator, _body: Value) -> Result<Value, EmulatorError> {
    Ok(json!({
        "TableNames": []
    }))
}

async fn query(emulator: &Emulator, body: Value) -> Result<Value, EmulatorError> {
    let table_name = body["TableName"].as_str().ok_or_else(|| EmulatorError::InvalidArgument("Missing TableName".into()))?;
    
    // Parse KeyConditionExpression to find PK placeholder
    // Format: "PK = :pk" or "PK = :pk AND SK > :sk"
    let condition = body["KeyConditionExpression"].as_str().ok_or_else(|| EmulatorError::InvalidArgument("Missing KeyConditionExpression".into()))?;
    
    // Extract equality condition
    // Split by " AND " to get conditions. Find one with "=".
    let eq_cond = condition.split(" AND ").find(|s| s.contains(" = ")).ok_or_else(|| EmulatorError::InvalidArgument("No equality condition found".into()))?;
    
    let parts: Vec<&str> = eq_cond.split(" = ").collect();
    if parts.len() != 2 {
        return Err(EmulatorError::InvalidArgument("Invalid condition format".into()));
    }
    
    let placeholder = parts[1].trim(); // e.g. ":pk"
    
    let values = body["ExpressionAttributeValues"].as_object().ok_or_else(|| EmulatorError::InvalidArgument("Missing ExpressionAttributeValues".into()))?;
    
    let val_obj = values.get(placeholder).ok_or_else(|| EmulatorError::InvalidArgument(format!("Missing value for placeholder {}", placeholder)))?;
    
    // Extract S or N
    let pk_val = if let Some(s) = val_obj["S"].as_str() { 
        s.to_string() 
    } else if let Some(n) = val_obj["N"].as_str() { 
        n.to_string() 
    } else {
        return Err(EmulatorError::InvalidArgument("Unsupported PK type".into()));
    };

    let items_json = emulator.storage.query_items(table_name, &pk_val)?;
    
    let items: Vec<Value> = items_json.into_iter().map(|s| serde_json::from_str(&s).unwrap_or(Value::Null)).collect();

    Ok(json!({
        "Items": items,
        "Count": items.len(),
        "ScannedCount": items.len()
    }))
}

async fn scan(emulator: &Emulator, body: Value) -> Result<Value, EmulatorError> {
    let table_name = body["TableName"].as_str().ok_or_else(|| EmulatorError::InvalidArgument("Missing TableName".into()))?;
    
    let items_json = emulator.storage.scan_items(table_name)?;
    let items: Vec<Value> = items_json.into_iter().map(|s| serde_json::from_str(&s).unwrap_or(Value::Null)).collect();

    Ok(json!({
        "Items": items,
        "Count": items.len(),
        "ScannedCount": items.len()
    }))
}
