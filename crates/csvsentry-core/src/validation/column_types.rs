use anyhow::Result;
use serde::{Deserialize, Serialize};
use url::Url;
use validator::ValidateEmail;

#[derive(Debug, Deserialize, Serialize)]
pub enum ColumnType {
    Text(Option<Vec<String>>),
    Integer,
    Float,
    Email,
    PhoneNumber,
    Website,
    Salutation,
}

impl ColumnType {
    pub fn validate(&self, val: &str) -> Result<(), String> {
        if val.trim().is_empty() {
            return Ok(());
        }
        match self {
            ColumnType::Text(None) => Ok(()),
            ColumnType::Text(Some(invalid_strings)) => {
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
            ColumnType::Float => match val.parse::<f64>() {
                Ok(n) if n.is_finite() && !val.contains('e') && !val.contains('E') => Ok(()),
                _ => Err(format!("expected float, found: {val}")),
            },
            ColumnType::Salutation => {
                let valid_salutations = [
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
                let trimmed = val.trim();
                let valid_chars = trimmed.chars().enumerate().all(|(i, c)| {
                    c.is_ascii_digit() || matches!(c, ' ' | '-' | '(' | ')') || (i == 0 && c == '+')
                });
                let digit_count = trimmed.chars().filter(|c| c.is_ascii_digit()).count();
                if !valid_chars || digit_count < 5 {
                    return Err(format!("invalid phone number: {val}"));
                };
                Ok(())
            }
            ColumnType::Website => {
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
                if !parsed.username().is_empty() || parsed.password().is_some() {
                    return Err(format!("invalid URL: {val}"));
                }
                Ok(())
            }
        }
    }
}
