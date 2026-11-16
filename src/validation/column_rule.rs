use crate::validation::column_types::ColumnType;

pub struct ColumnRule {
    pub expected_type: ColumnType,
    pub required: bool,
}

impl ColumnRule {
    pub fn validate(&self, value: &str) -> Result<(), String> {
        if self.required && value.trim().is_empty() {
            return Err("required, but value is missing".to_string());
        }

        self.expected_type.validate(value)
    }
}
