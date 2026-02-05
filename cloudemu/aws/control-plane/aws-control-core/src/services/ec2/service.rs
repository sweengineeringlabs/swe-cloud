use aws_data_core::StorageEngine;

#[derive(Clone)]
pub struct Ec2Service {
    pub storage: StorageEngine,
}

impl Ec2Service {
    pub fn new(storage: StorageEngine) -> Self {
        Self { storage }
    }
}
