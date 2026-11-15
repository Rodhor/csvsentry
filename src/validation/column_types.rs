use anyhow::Result;
use url::Url;
use validator::ValidateEmail;

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

                // Iterate through the invalid substrings, checking if present in the value
                let contains_invalid: Vec<String> = invalid_strings
                    .iter()
                    .filter(|s| val.contains(s.as_str()))
                    .cloned()
                    .collect();

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
            ColumnType::Email => {
                if val.is_empty() {
                    return Ok(());
                }

                // Custom logic to define invalid states of the email that are not caught by the validator
                if val.contains("..")
                    || val.starts_with('.')
                    || val.starts_with('@')
                    || val.contains(".@")
                {
                    return Err(format!("invalid email: {val}"));
                }

                // Use Validator lib to check email with regex
                if !ValidateEmail::validate_email(&val) {
                    return Err(format!("invalid email: {val}"));
                }
                Ok(())
            }
            ColumnType::PhoneNumber => {
                if val.is_empty() {
                    return Ok(());
                }

                let is_valid = val
                    .chars()
                    .enumerate()
                    .all(|(i, c)| c.is_numeric() || c == ' ' || i == 0 && c == '+');

                if !is_valid {
                    return Err(format!("invalid phone number: {val}"));
                };
                Ok(())
            }
            ColumnType::Website => {
                if val.is_empty() {
                    return Ok(());
                }
                let candidate = if val.starts_with("www.") {
                    format!("https://{val}")
                } else {
                    val.to_string()
                };

                let parsed = Url::parse(&candidate).map_err(|_| format!("invalid URL: {val}"))?;

                if parsed.scheme() != "http" && parsed.scheme() != "https" {
                    return Err(format!("invalid URL scheme: {val}"));
                }

                let host = parsed
                    .host_str()
                    .ok_or_else(|| format!("invalid url: {val}"))?;
                if host.split('.').any(str::is_empty) {
                    return Err(format!("invalid URL: {val}"));
                }
                Ok(())
            }
        }
    }
}
