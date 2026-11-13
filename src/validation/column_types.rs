use anyhow::Result;
pub enum ColumnType {
    Text(Vec<String>),
    Integer,
    Float,
    Email,
    PhoneNumber,
    Website,
    Salutation,
}

impl ColumnType {
    pub fn validate(&self, val: &str) -> Result<(), String> {
        if val.is_empty() {
            return Ok(());
        }
        match self {
            ColumnType::Text(invalid_strings) => {
                // If no strings are invalid for this text column, skip check entirely
                if invalid_strings.is_empty() {
                    return Ok(());
                }
                // Keep list of substrings present value
                let mut contains_invalid = Vec::<String>::new();
                for s in invalid_strings {
                    if val.contains(s) {
                        contains_invalid.push(s.clone());
                    }
                }
                if !contains_invalid.is_empty() {
                    return Err(format!(
                        "Entry ({val}) contains invalid substrings: {:?}",
                        contains_invalid
                    ));
                }
                Ok(())
            }
            ColumnType::Integer => {
                if val.parse::<i64>().is_err() {
                    return Err(format!("expected integer, found: {val}"));
                }
                Ok(())
            }
            ColumnType::Float => {
                if val.parse::<f64>().is_err() {
                    return Err(format!("expected float, found: {val}"));
                }
                Ok(())
            }
            ColumnType::Salutation => {
                let valid_salutations = vec![
                    String::from("herr"),
                    String::from("frau"),
                    String::from("firma"),
                ];
                if !valid_salutations.contains(&val.to_lowercase()) {
                    return Err(format!("invalid salutation: {val}"));
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }
}
