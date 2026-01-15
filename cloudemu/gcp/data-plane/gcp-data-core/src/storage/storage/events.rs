use super::engine::{StorageEngine, EventBusMetadata, EventRuleMetadata, EventTargetMetadata};
use crate::error::{EmulatorError, Result};
use rusqlite::params;

impl StorageEngine {
    // ==================== EventBridge Operations ====================
    
    pub fn create_event_bus(&self, name: &str, account_id: &str, region: &str) -> Result<EventBusMetadata> {
        let db = self.db.lock();
        let arn = format!("arn:aws:events:{}:{}:event-bus/{}", region, account_id, name);
        
        db.execute(
            "INSERT INTO event_buses (name, arn) VALUES (?1, ?2)",
            params![name, arn],
        ).map_err(|e| {
            if e.to_string().contains("UNIQUE constraint") {
                EmulatorError::AlreadyExists(format!("Event bus {} already exists", name))
            } else {
                EmulatorError::Database(e.to_string())
            }
        })?;
        
        Ok(EventBusMetadata {
            name: name.to_string(),
            arn,
            policy: None,
        })
    }
    
    pub fn get_event_bus(&self, name: &str) -> Result<EventBusMetadata> {
        let db = self.db.lock();
        db.query_row(
            "SELECT name, arn, policy FROM event_buses WHERE name = ?1",
            params![name],
            |row| Ok(EventBusMetadata {
                name: row.get(0)?,
                arn: row.get(1)?,
                policy: row.get(2)?,
            })
        ).map_err(|_| EmulatorError::NotFound("EventBus".into(), name.into()))
    }
    
    pub fn list_event_buses(&self) -> Result<Vec<EventBusMetadata>> {
        let db = self.db.lock();
        let mut stmt = db.prepare("SELECT name, arn, policy FROM event_buses")?;
        let buses = stmt.query_map([], |row| Ok(EventBusMetadata {
            name: row.get(0)?,
            arn: row.get(1)?,
            policy: row.get(2)?,
        }))?
        .filter_map(|r| r.ok())
        .collect();
        Ok(buses)
    }
    
    pub fn delete_event_bus(&self, name: &str) -> Result<()> {
        let db = self.db.lock();
        let rows = db.execute("DELETE FROM event_buses WHERE name = ?1", params![name])?;
        if rows == 0 {
            return Err(EmulatorError::NotFound("EventBus".into(), name.into()));
        }
        Ok(())
    }
    
    // NOTE: This function will be refactored with a builder pattern in upcoming storage refactor
    #[allow(clippy::too_many_arguments)]
    pub fn put_rule(&self, name: &str, bus_name: &str, pattern: Option<&str>, state: &str, description: Option<&str>, schedule: Option<&str>, account_id: &str, region: &str) -> Result<String> {
        let db = self.db.lock();
        let now = chrono::Utc::now().to_rfc3339();
        let arn = format!("arn:aws:events:{}:{}:rule/{}/{}", region, account_id, bus_name, name);
        
        db.execute(
            r#"INSERT INTO event_rules 
               (name, event_bus_name, arn, event_pattern, state, description, schedule_expression, created_at)
               VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
               ON CONFLICT(event_bus_name, name) DO UPDATE SET
               event_pattern = excluded.event_pattern,
               state = excluded.state,
               description = excluded.description,
               schedule_expression = excluded.schedule_expression"#,
            params![name, bus_name, arn, pattern, state, description, schedule, now],
        )?;
        
        Ok(arn)
    }
    
    pub fn list_rules(&self, bus_name: &str) -> Result<Vec<EventRuleMetadata>> {
        let db = self.db.lock();
        let mut stmt = db.prepare(
            "SELECT name, event_bus_name, arn, event_pattern, state, description, schedule_expression, created_at 
             FROM event_rules WHERE event_bus_name = ?1"
        )?;
        let rules = stmt.query_map(params![bus_name], |row| Ok(EventRuleMetadata {
            name: row.get(0)?,
            event_bus_name: row.get(1)?,
            arn: row.get(2)?,
            event_pattern: row.get(3)?,
            state: row.get(4)?,
            description: row.get(5)?,
            schedule_expression: row.get(6)?,
            created_at: row.get(7)?,
        }))?
        .filter_map(|r| r.ok())
        .collect();
        Ok(rules)
    }
    
    pub fn put_targets(&self, bus_name: &str, rule_name: &str, targets: Vec<EventTargetMetadata>) -> Result<()> {
        let db = self.db.lock();
        for target in targets {
            db.execute(
                r#"INSERT INTO event_targets (id, rule_name, event_bus_name, arn, input, input_path)
                   VALUES (?1, ?2, ?3, ?4, ?5, ?6)
                   ON CONFLICT(event_bus_name, rule_name, id) DO UPDATE SET
                   arn = excluded.arn,
                   input = excluded.input,
                   input_path = excluded.input_path"#,
                params![target.id, rule_name, bus_name, target.arn, target.input, target.input_path],
            )?;
        }
        Ok(())
    }
    
    pub fn list_targets(&self, bus_name: &str, rule_name: &str) -> Result<Vec<EventTargetMetadata>> {
        let db = self.db.lock();
        let mut stmt = db.prepare(
            "SELECT id, rule_name, event_bus_name, arn, input, input_path 
             FROM event_targets WHERE event_bus_name = ?1 AND rule_name = ?2"
        )?;
        let targets = stmt.query_map(params![bus_name, rule_name], |row| Ok(EventTargetMetadata {
            id: row.get(0)?,
            rule_name: row.get(1)?,
            event_bus_name: row.get(2)?,
            arn: row.get(3)?,
            input: row.get(4)?,
            input_path: row.get(5)?,
        }))?
        .filter_map(|r| r.ok())
        .collect();
        Ok(targets)
    }

    pub fn record_event(&self, bus_name: &str, source: &str, detail_type: &str, detail: &str, resources: Option<&str>) -> Result<String> {
        let db = self.db.lock();
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().to_rfc3339();
        
        db.execute(
            r#"INSERT INTO event_history (id, event_bus_name, source, detail_type, detail, time, resources)
               VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)"#,
            params![id, bus_name, source, detail_type, detail, now, resources],
        )?;
        
        Ok(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_events_bus_lifecycle() {
        let engine = StorageEngine::in_memory().unwrap();
        
        // Create Bus
        let bus = engine.create_event_bus("test-bus", "123", "us-east-1").unwrap();
        assert_eq!(bus.name, "test-bus");
        
        // Get Bus
        let fetched = engine.get_event_bus("test-bus").unwrap();
        assert_eq!(fetched.arn, bus.arn);
        
        // List Buses
        let buses = engine.list_event_buses().unwrap();
        assert!(buses.iter().any(|b| b.name == "test-bus"));
        
        // Delete Bus
        engine.delete_event_bus("test-bus").unwrap();
        assert!(engine.get_event_bus("test-bus").is_err());
    }
    
    #[test]
    fn test_events_rules_and_targets() {
        let engine = StorageEngine::in_memory().unwrap();
        engine.create_event_bus("default", "123", "us-east-1").unwrap();
        
        // Put Rule
        let pattern = r#"{"source": ["aws.ec2"]}"#;
        let rule_arn = engine.put_rule("rule1", "default", Some(pattern), "ENABLED", None, None, "123", "us-east-1").unwrap();
        assert!(rule_arn.contains("rule1"));
        
        // Put Targets
        let targets = vec![EventTargetMetadata {
            id: "t1".to_string(),
            rule_name: "rule1".to_string(),
            event_bus_name: "default".to_string(),
            arn: "arn:aws:lambda:us-east-1:123:function:my-func".to_string(),
            input: None,
            input_path: None,
        }];
        engine.put_targets("default", "rule1", targets).unwrap();
        
        // List Targets
        let fetched_targets = engine.list_targets("default", "rule1").unwrap();
        assert_eq!(fetched_targets.len(), 1);
        assert_eq!(fetched_targets[0].id, "t1");
    }
}

