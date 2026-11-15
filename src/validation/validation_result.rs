use std::collections::{HashMap, HashSet};

pub enum ValidationStatus {
    Valid,
    Invalid,
}

pub struct InvalidValue {
    pub row: u64,
    pub column: HashMap<u64, String>,
    pub value: String,
    pub error: String,
}
pub struct ValidationResult {
    pub column: u64,
    pub column_header: String,
    pub unique_values: HashSet<String>,
    pub total_values: u64,
    pub total_empty: u64,
    pub invalid_values: Vec<InvalidValue>,
    pub status: ValidationStatus,
}
