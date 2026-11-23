use crate::validation::column_types::ColumnType;
use serde::{Deserialize, Serialize};

pub struct Row {
    pub number: usize,
    pub cells: Vec<String>,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, Default)]
pub enum RowIndex {
    #[default]
    CsvIndex,
    CsvIndexHeaderOffset,
    ExcelIndex,
}

impl RowIndex {
    pub fn from_csv_reader(&self, index: usize, has_header: bool) -> usize {
        match self {
            RowIndex::CsvIndex => index,
            RowIndex::CsvIndexHeaderOffset => index + 1,
            RowIndex::ExcelIndex => index + if has_header { 2 } else { 1 },
        }
    }
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
    #[serde(default)]
    pub prefered_index: RowIndex,
    pub columns: Vec<ColumnConfig>,
}
