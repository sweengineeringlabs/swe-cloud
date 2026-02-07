use oracle_data_core::StorageEngine;
use oracle_control_spi::{Request, Response, CloudResult};
use serde_json::{json, Value};
use std::sync::Arc;

pub struct NoSqlService {
    storage: Arc<StorageEngine>,
}

impl NoSqlService {
    pub fn new(storage: Arc<StorageEngine>) -> Self {
        Self { storage }
    }

    pub async fn handle_request(&self, req: Request) -> CloudResult<Response> {
        // /20190828/tables
        if req.path.contains("/tables") {
            if req.method == "POST" {
                return self.create_table(&req);
            }
        }
        // /20190828/rows
        if req.path.contains("/rows") && req.method == "PUT" {
            return self.put_row(&req);
        }
        Ok(Response::not_found("Not Found"))
    }

    fn create_table(&self, req: &Request) -> CloudResult<Response> {
        let body: Value = serde_json::from_slice(&req.body).unwrap_or(json!({}));
        let name = body["name"].as_str().unwrap_or("table1");
        let compartment = body["compartmentId"].as_str().unwrap_or("ocid1.compartment.oc1..test");
        let ddl = body["ddlStatement"].as_str().unwrap_or("CREATE TABLE table1 (id INTEGER, name STRING, PRIMARY KEY(id))");

        let table = self.storage.create_nosql_table(name, compartment, ddl).map_err(|e| oracle_control_spi::Error::Internal(e.to_string()))?;

        Ok(Response::json(json!({
            "id": format!("ocid1.nosqltable.oc1..{}", name),
            "name": table.name,
            "compartmentId": table.compartment_id,
            "lifecycleState": table.state
        })))
    }

    fn put_row(&self, req: &Request) -> CloudResult<Response> {
        let body: Value = serde_json::from_slice(&req.body).unwrap_or(json!({}));
        let table_name = body["tableNameOrId"].as_str().unwrap_or("table1");
        let value_obj = &body["value"]; // JSON object
        let key = value_obj["id"].to_string(); // Simple Assumption
        let value = value_obj.to_string();

        self.storage.put_nosql_row(table_name, &key, &value).map_err(|e| oracle_control_spi::Error::Internal(e.to_string()))?;

        Ok(Response::json(json!({
            "version": "1.0"
        })))
    }
}
