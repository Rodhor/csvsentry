use crate::validation::column_types::ColumnType;
use serde::{Deserialize, Serialize};

pub struct Row {
    pub number: usize,
    pub cells: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ColumnConfig {
    pub header: String,
    pub index: usize,
    pub expected_type: ColumnType,
    pub required: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ValidationConfig {
    pub columns: Vec<ColumnConfig>,
}
