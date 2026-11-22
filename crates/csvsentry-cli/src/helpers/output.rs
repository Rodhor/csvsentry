use serde_json;
use std::{fs, path::Path};

use anyhow::{Ok, Result};
use clap::ValueEnum;
use csvsentry_core::validation::validation_result::ValidationResult;

#[derive(Clone, Copy, ValueEnum)]
pub enum OutputFormat {
    Json,
    Jsonl,
}

pub fn write_results(
    results: Vec<ValidationResult>,
    path: &Path,
    format: OutputFormat,
) -> Result<()> {
    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&results)?,
        OutputFormat::Jsonl => results
            .iter()
            .map(serde_json::to_string)
            .collect::<Result<Vec<_>, _>>()?
            .join("\n"),
    };
    fs::write(path, content)?;
    Ok(())
}
