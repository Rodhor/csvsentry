use crate::validation::column_types::ColumnType;

pub struct Row {
    pub number: usize,
    pub cells: Vec<String>,
}

pub struct ColumnConfig {
    pub header: String,
    pub index: usize,
    pub expected_type: ColumnType,
    pub required: bool,
}
pub struct ValidationConfig {
    pub columns: Vec<ColumnConfig>,
}
