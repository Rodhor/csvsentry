use anyhow::Result;
use chrono::{DateTime, NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use strum::{EnumIter, IntoEnumIterator};
use url::Url;
use validator::ValidateEmail;

#[derive(Debug, Deserialize, Serialize, EnumIter)]
pub enum ColumnType {
    Text(Option<Vec<String>>),
    Integer,
    Float,
    Email,
    PhoneNumber,
    Website,
    Salutation,
    Boolean,
    Date,
    DateTime,
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
            ColumnType::Boolean => {
                const TRUE_VALUES: &[&str] = &["true", "wahr", "ja", "j", "1", "y", "yes"];
                const FALSE_VALUES: &[&str] = &["false", "falsch", "nein", "n", "0", "no"];

                let normalized = val.trim().to_lowercase();

                if TRUE_VALUES.contains(&normalized.as_str())
                    || FALSE_VALUES.contains(&normalized.as_str())
                {
                    Ok(())
                } else {
                    Err(format!("invalid boolean value: {:?}", val))
                }
            }
            ColumnType::Date => val
                .parse::<NaiveDate>()
                .map(|_| ())
                .map_err(|_| format!("expected date, found: {val}")),

            ColumnType::DateTime => {
                let naive_t = val.parse::<NaiveDateTime>().is_ok();
                let naive_space = NaiveDateTime::parse_from_str(val, "%Y-%m-%d %H:%M:%S").is_ok();
                let rfc3339 = DateTime::parse_from_rfc3339(val).is_ok();

                if naive_t || naive_space || rfc3339 {
                    Ok(())
                } else {
                    Err(format!("expected datetime, found: {val}"))
                }
            }
        }
    }

    pub fn explore_possible_types(val: &str) -> Vec<ColumnType> {
        if val.trim().is_empty() {
            return Vec::new();
        }

        Self::iter()
            .filter(|candidate| candidate.validate(val).is_ok())
            .collect()
    }
}
