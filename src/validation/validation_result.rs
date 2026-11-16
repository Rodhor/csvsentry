use crate::domain::validation_configuration::{Row, ValidationConfig};
use std::collections::HashSet;

pub enum ValidationStatus {
    Valid,
    Invalid,
}

#[derive(Debug)]
pub struct InvalidValue {
    pub row: usize,
    pub column: usize,
    pub header: String,
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

pub fn validate_row(row: &Row, config: &ValidationConfig) -> Vec<InvalidValue> {
    let mut invalids = Vec::new();
    for col in &config.columns {
        let value = row.cells.get(col.index).map(String::as_str).unwrap_or("");
        if let Err(error) = col.expected_type.validate(value) {
            invalids.push(InvalidValue {
                row: row.number,
                column: col.index,
                header: col.header.clone(),
                value: value.to_string(),
                error,
            })
        }
    }
    invalids
}
