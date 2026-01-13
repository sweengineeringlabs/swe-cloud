use super::engine::{StorageEngine, StateMachineMetadata, ExecutionMetadata};
use crate::error::{EmulatorError, Result};
use rusqlite::params;

impl StorageEngine {
    // ==================== Step Functions Operations ====================
    
    pub fn create_state_machine(&self, name: &str, definition: &str, role_arn: &str, machine_type: &str, account_id: &str, region: &str) -> Result<StateMachineMetadata> {
        let db = self.db.lock();
        let arn = format!("arn:aws:states:{}:{}:stateMachine:{}", region, account_id, name);
        let now = chrono::Utc::now().to_rfc3339();
        
        db.execute(
            "INSERT INTO sf_state_machines (arn, name, definition, role_arn, type, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![arn, name, definition, role_arn, machine_type, now],
        ).map_err(|e| {
            if e.to_string().contains("UNIQUE constraint") {
                EmulatorError::AlreadyExists(format!("State machine {} already exists", name))
            } else {
                EmulatorError::Database(e.to_string())
            }
        })?;
        
        Ok(StateMachineMetadata {
            arn,
            name: name.to_string(),
            definition: definition.to_string(),
            role_arn: role_arn.to_string(),
            machine_type: machine_type.to_string(),
            created_at: now,
        })
    }

    pub fn list_state_machines(&self) -> Result<Vec<StateMachineMetadata>> {
        let db = self.db.lock();
        let mut stmt = db.prepare("SELECT arn, name, definition, role_arn, type, created_at FROM sf_state_machines")?;
        let machines = stmt.query_map([], |row| Ok(StateMachineMetadata {
            arn: row.get(0)?,
            name: row.get(1)?,
            definition: row.get(2)?,
            role_arn: row.get(3)?,
            machine_type: row.get(4)?,
            created_at: row.get(5)?,
        }))?
        .filter_map(|r| r.ok())
        .collect();
        Ok(machines)
    }

    pub fn delete_state_machine(&self, arn: &str) -> Result<()> {
        let db = self.db.lock();
        let rows = db.execute("DELETE FROM sf_state_machines WHERE arn = ?1", params![arn])?;
        if rows == 0 {
            return Err(EmulatorError::NotFound("StateMachine".into(), arn.into()));
        }
        Ok(())
    }

    pub fn start_execution(&self, state_machine_arn: &str, name: Option<&str>, input: Option<&str>, account_id: &str, region: &str) -> Result<ExecutionMetadata> {
        let db = self.db.lock();
        let exec_name = name.map(|s| s.to_string()).unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let arn = format!("arn:aws:states:{}:{}:execution:{}:{}", region, account_id, state_machine_arn.split(':').next_back().unwrap_or("unknown"), exec_name);
        let now = chrono::Utc::now().to_rfc3339();
        
        db.execute(
            "INSERT INTO sf_executions (arn, state_machine_arn, name, input, start_date) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![arn, state_machine_arn, exec_name, input, now],
        )?;
        
        Ok(ExecutionMetadata {
            arn,
            state_machine_arn: state_machine_arn.to_string(),
            name: exec_name,
            status: "RUNNING".to_string(),
            input: input.map(|s| s.to_string()),
            output: None,
            start_date: now,
            stop_date: None,
        })
    }

    pub fn describe_execution(&self, arn: &str) -> Result<ExecutionMetadata> {
        let db = self.db.lock();
        db.query_row(
            "SELECT arn, state_machine_arn, name, status, input, output, start_date, stop_date FROM sf_executions WHERE arn = ?1",
            params![arn],
            |row| Ok(ExecutionMetadata {
                arn: row.get(0)?,
                state_machine_arn: row.get(1)?,
                name: row.get(2)?,
                status: row.get(3)?,
                input: row.get(4)?,
                output: row.get(5)?,
                start_date: row.get(6)?,
                stop_date: row.get(7)?,
            })
        ).map_err(|_| EmulatorError::NotFound("Execution".into(), arn.into()))
    }

    pub fn update_execution_status(&self, arn: &str, status: &str, output: Option<&str>) -> Result<()> {
        let stop_date = if status == "SUCCEEDED" || status == "FAILED" || status == "ABORTED" {
            Some(chrono::Utc::now().to_rfc3339())
        } else {
            None
        };

        let db = self.db.lock();
        db.execute(
            "UPDATE sf_executions SET status = ?1, output = ?2, stop_date = ?3 WHERE arn = ?4",
            params![status, output, stop_date, arn],
        )?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_machine_workflow() {
        let engine = StorageEngine::in_memory().unwrap();
        
        let def = r#"{"StartAt": "Pass", "States": {"Pass": {"Type": "Pass", "End": true}}}"#;
        
        // Create Machine
        let machine = engine.create_state_machine("my-machine", def, "role:arn", "STANDARD", "123", "us-east-1").unwrap();
        assert_eq!(machine.name, "my-machine");
        
        // List Machines
        let machines = engine.list_state_machines().unwrap();
        assert_eq!(machines.len(), 1);
        
        // Start Execution
        let exec = engine.start_execution(&machine.arn, Some("exec-1"), Some("{}"), "123", "us-east-1").unwrap();
        assert_eq!(exec.status, "RUNNING");
        
        // Update Status
        engine.update_execution_status(&exec.arn, "SUCCEEDED", Some("{\"done\": true}")).unwrap();
        
        // Describe Execution
        let fetched_exec = engine.describe_execution(&exec.arn).unwrap();
        assert_eq!(fetched_exec.status, "SUCCEEDED");
        assert_eq!(fetched_exec.output, Some("{\"done\": true}".to_string()));
    }
}

