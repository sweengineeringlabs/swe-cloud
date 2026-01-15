use super::engine::{StorageEngine, VirtualMachineMetadata};
use crate::error::{EmulatorError, Result};
use rusqlite::params;

impl StorageEngine {
    // ==================== Virtual Machine Operations ====================

    pub fn create_virtual_machine(&self, name: &str, location: &str, resource_group: &str, vm_size: &str, os_type: &str, admin_username: &str) -> Result<VirtualMachineMetadata> {
        let db = self.db.lock();
        let now = chrono::Utc::now().to_rfc3339();
        
        // Mock IP assignment
        let private_ip = format!("10.0.0.{}", rand::random::<u8>());
        let public_ip = format!("{}.{}.{}.{}", rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>());

        db.execute(
            r#"INSERT INTO az_virtual_machines 
               (name, location, resource_group, vm_size, os_type, admin_username, private_ip, public_ip, status, created_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, 'Running', ?)"#,
            params![name, location, resource_group, vm_size, os_type, admin_username, private_ip, public_ip, now],
        ).map_err(|e| {
            if e.to_string().contains("UNIQUE constraint") {
                EmulatorError::AlreadyExists(format!("Virtual Machine {} already exists", name))
            } else {
                EmulatorError::Database(e.to_string())
            }
        })?;

        Ok(VirtualMachineMetadata {
            name: name.to_string(),
            location: location.to_string(),
            resource_group: resource_group.to_string(),
            vm_size: vm_size.to_string(),
            os_type: os_type.to_string(),
            admin_username: admin_username.to_string(),
            private_ip: Some(private_ip),
            public_ip: Some(public_ip),
            status: "Running".to_string(),
            created_at: now,
        })
    }

    pub fn get_virtual_machine(&self, name: &str) -> Result<VirtualMachineMetadata> {
        let db = self.db.lock();
        db.query_row(
            r#"SELECT name, location, resource_group, vm_size, os_type, admin_username, private_ip, public_ip, status, created_at 
               FROM az_virtual_machines WHERE name = ?"#,
            params![name],
            |row| {
                Ok(VirtualMachineMetadata {
                    name: row.get(0)?,
                    location: row.get(1)?,
                    resource_group: row.get(2)?,
                    vm_size: row.get(3)?,
                    os_type: row.get(4)?,
                    admin_username: row.get(5)?,
                    private_ip: row.get(6)?,
                    public_ip: row.get(7)?,
                    status: row.get(8)?,
                    created_at: row.get(9)?,
                })
            },
        ).map_err(|_| EmulatorError::NotFound(format!("Virtual Machine {} not found", name)))
    }

    pub fn delete_virtual_machine(&self, name: &str) -> Result<()> {
        let db = self.db.lock();
        let count = db.execute("DELETE FROM az_virtual_machines WHERE name = ?", params![name])?;
        if count == 0 {
            return Err(EmulatorError::NotFound(format!("Virtual Machine {} not found", name)));
        }
        Ok(())
    }

    pub fn list_virtual_machines(&self, resource_group: Option<&str>) -> Result<Vec<VirtualMachineMetadata>> {
        let db = self.db.lock();
        
        // Build query based on optional resource group filter
        let (query, params) = if let Some(rg) = resource_group {
            ("SELECT name, location, resource_group, vm_size, os_type, admin_username, private_ip, public_ip, status, created_at FROM az_virtual_machines WHERE resource_group = ?", vec![rg.to_string()])
        } else {
            ("SELECT name, location, resource_group, vm_size, os_type, admin_username, private_ip, public_ip, status, created_at FROM az_virtual_machines", vec![])
        };

        let mut stmt = db.prepare(query)?;
        
        // This part is a bit tricky with dynamic params in rusqlite with params_from_iter, 
        // but for simplicity let's handle the two cases
        
        let map_row = |row: &rusqlite::Row| {
             Ok(VirtualMachineMetadata {
                name: row.get(0)?,
                location: row.get(1)?,
                resource_group: row.get(2)?,
                vm_size: row.get(3)?,
                os_type: row.get(4)?,
                admin_username: row.get(5)?,
                private_ip: row.get(6)?,
                public_ip: row.get(7)?,
                status: row.get(8)?,
                created_at: row.get(9)?,
            })
        };

        let vms = if let Some(rg) = resource_group {
            stmt.query_map(params![rg], map_row)?
        } else {
            stmt.query_map([], map_row)?
        }
        .filter_map(|r| r.ok())
        .collect();

        Ok(vms)
    }
}
