use serde::{Serialize, Deserialize};

/// FOCUS 1.0 Compatible Cost Item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FocusItem {
    pub provider: String,          // 'AWS', 'Azure', 'Google Cloud', 'Oracle'
    pub service_category: String,  // 'Compute', 'Storage', 'Database'
    pub resource_id: Option<String>,
    pub sku: String,
    pub billed_cost: f64,
    pub currency: String,
    pub usage_quantity: f64,
    pub usage_unit: String,
}
