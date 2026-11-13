use crate::validation::column_types::ColumnType;

pub struct ColumnRule{
    pub expected_type: ColumnType,
    pub required: bool,
}