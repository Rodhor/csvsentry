use std::collections::HashSet;

pub struct ValidationResult {
    column: u64,
    column_type: String,
    unique_values: HashSet<String>,
    total_values: u64,
    total_empty: u64
}