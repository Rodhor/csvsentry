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

pub fn validate_rows<'a>(
    rows: impl IntoIterator<Item = &'a Row>,
    config: &ValidationConfig,
) -> Vec<ValidationResult> {
    let mut results: Vec<ValidationResult> = config
        .columns
        .iter()
        .map(|col| ValidationResult {
            column: col.index as u64,
            column_header: col.header.clone(),
            unique_values: HashSet::new(),
            total_values: 0,
            total_empty: 0,
            invalid_values: Vec::new(),
            status: ValidationStatus::Valid,
        })
        .collect();

    for row in rows {
        for (i, col) in config.columns.iter().enumerate() {
            let value = row.cells.get(col.index).map(String::as_str).unwrap_or("");
            let slot = &mut results[i];
            slot.total_values += 1;
            if value.trim().is_empty() {
                slot.total_empty += 1;
            } else {
                slot.unique_values.insert(value.to_string());
            }
        }
        for error in validate_row(row, config) {
            let slot = results
                .iter_mut()
                .find(|r| r.column == error.column as u64)
                .expect("error column is one of config.columns");
            slot.status = ValidationStatus::Invalid;
            slot.invalid_values.push(error)
        }
    }
    results
}

pub fn validate_row(row: &Row, config: &ValidationConfig) -> Vec<InvalidValue> {
    let mut invalids = Vec::new();
    for col in &config.columns {
        let value = row.cells.get(col.index).map(String::as_str).unwrap_or("");

        if let Err(error) =
            validate_required(value, col.required).and_then(|_| col.expected_type.validate(value))
        {
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

fn validate_required(value: &str, required: bool) -> Result<(), String> {
    let trimmed = value.trim();
    if required && trimmed.is_empty() {
        return Err("required value is missing".to_string());
    }
    Ok(())
}
