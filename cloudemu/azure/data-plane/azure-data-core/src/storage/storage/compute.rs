use chrono::Utc;
use super::StorageEngine;
use crate::error::Result;
use crate::storage::storage::engine::VirtualMachineMetadata;
use rusqlite::params;

impl StorageEngine {
    const TABLE_AZURE_VMS: &'static str = "azure_vms";

    pub fn init_compute_tables(&self) -> Result<()> {
        let conn = self.get_connection()?;
        
        conn.execute(&format!(
            "CREATE TABLE IF NOT EXISTS {} (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                resource_group TEXT NOT NULL,
                location TEXT NOT NULL,
                vm_size TEXT NOT NULL,
                os_type TEXT NOT NULL,
                admin_username TEXT NOT NULL,
                provisioning_state TEXT NOT NULL,
                created_at INTEGER,
                UNIQUE(name, resource_group)
            )", 
            Self::TABLE_AZURE_VMS
        ), [])?;

        Ok(())
    }

    pub fn create_virtual_machine(
        &self, 
        name: &str, 
        location: &str, 
        resource_group: &str, 
        vm_size: &str, 
        os_type: &str, 
        admin_username: &str
    ) -> Result<VirtualMachineMetadata> {
        let conn = self.get_connection()?;
        let id = format!("/subscriptions/sub-1/resourceGroups/{}/providers/Microsoft.Compute/virtualMachines/{}", resource_group, name);
        let now = chrono::Utc::now().timestamp();
        let status = "Succeeded".to_string();

        conn.execute(
            &format!("INSERT INTO {} (
                id, name, resource_group, location, vm_size, os_type, admin_username, provisioning_state, created_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)", Self::TABLE_AZURE_VMS),
            params![
                id, name, resource_group, location, vm_size, os_type, 
                admin_username, status, now
            ],
        )?;

        Ok(VirtualMachineMetadata {
            name: name.to_string(),
            location: location.to_string(),
            resource_group: resource_group.to_string(),
            vm_size: vm_size.to_string(),
            os_type: os_type.to_string(),
            admin_username: admin_username.to_string(),
            private_ip: Some("10.0.0.4".to_string()),
            public_ip: Some("20.30.40.50".to_string()),
            status,
            created_at: now.to_string(),
        })
    }

    pub fn list_vms(&self, resource_group: Option<&str>) -> Result<Vec<VirtualMachineMetadata>> {
        let conn = self.get_connection()?;
        
        let sql = if let Some(rg) = resource_group {
            format!("SELECT name, location, resource_group, vm_size, os_type, admin_username, provisioning_state, created_at FROM {} WHERE resource_group = '{}'", Self::TABLE_AZURE_VMS, rg)
        } else {
            format!("SELECT name, location, resource_group, vm_size, os_type, admin_username, provisioning_state, created_at FROM {}", Self::TABLE_AZURE_VMS)
        };

        let mut stmt = conn.prepare(&sql)?;
        let vms = stmt.query_map([], |row| {
            Ok(VirtualMachineMetadata {
                name: row.get(0)?,
                location: row.get(1)?,
                resource_group: row.get(2)?,
                vm_size: row.get(3)?,
                os_type: row.get(4)?,
                admin_username: row.get(5)?,
                private_ip: Some("10.0.0.4".to_string()),
                public_ip: Some("20.30.40.50".to_string()),
                status: row.get(6)?,
                created_at: row.get::<_, i64>(7)?.to_string(),
            })
        })?
        .collect::<std::result::Result<Vec<VirtualMachineMetadata>, _>>()?;

        Ok(vms)
    }

    pub fn get_virtual_machine(&self, name: &str) -> Result<VirtualMachineMetadata> {
         let conn = self.get_connection()?;
         let mut stmt = conn.prepare(&format!("SELECT name, location, resource_group, vm_size, os_type, admin_username, provisioning_state, created_at FROM {} WHERE name = ?1", Self::TABLE_AZURE_VMS))?;
         
         let vm = stmt.query_row(params![name], |row| {
            Ok(VirtualMachineMetadata {
                name: row.get(0)?,
                location: row.get(1)?,
                resource_group: row.get(2)?,
                vm_size: row.get(3)?,
                os_type: row.get(4)?,
                admin_username: row.get(5)?,
                private_ip: Some("10.0.0.4".to_string()),
                public_ip: Some("20.30.40.50".to_string()),
                status: row.get(6)?,
                created_at: row.get::<_, i64>(7)?.to_string(),
            })
         })?;
         
         Ok(vm)
    }
}
