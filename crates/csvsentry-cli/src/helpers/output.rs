use serde::Serialize;
use serde_json;
use std::{fs, path::Path};

use anyhow::{Ok, Result};
use clap::ValueEnum;
use csvsentry_core::validation::validation_result::{
    InvalidValue, ValidationResult, ValidationStatus,
};

#[derive(Serialize)]
struct ValidationSummary {
    column_header: String,
    total_values: u64,
    total_empty: u64,
    invalid_values: Vec<InvalidValue>,
    status: ValidationStatus,
}

impl From<ValidationResult> for ValidationSummary {
    fn from(r: ValidationResult) -> Self {
        Self {
            column_header: r.column_header,
            total_values: r.total_values,
            total_empty: r.total_empty,
            invalid_values: r.invalid_values,
            status: r.status,
        }
    }
}

#[derive(Clone, Copy, ValueEnum)]
pub enum OutputFormat {
    Json,
    Jsonl,
}

pub fn write_results(
    results: Vec<ValidationResult>,
    path: &Path,
    format: OutputFormat,
    verbose: bool,
) -> Result<()> {
    let content = if verbose {
        serialize(&results, format)?
    } else {
        let summaries: Vec<_> = results.into_iter().map(ValidationSummary::from).collect();
        serialize(&summaries, format)?
    };
    fs::write(path, content)?;
    Ok(())
}

fn serialize<T: Serialize>(items: &[T], format: OutputFormat) -> Result<String> {
    Ok(match format {
        OutputFormat::Json => serde_json::to_string_pretty(&items)?,
        OutputFormat::Jsonl => items
            .iter()
            .map(serde_json::to_string)
            .collect::<Result<Vec<_>, _>>()?
            .join("\n"),
    })
}
