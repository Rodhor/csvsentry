use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};

use crate::{
    csv_handling::read::validate_csv,
    domain::settings::{Settings, Store},
    validation::validation_result::ValidationResult,
};

pub fn validate_from_settings(path: impl AsRef<Path>) -> Result<Vec<ValidationResult>> {
    let settings = Settings::load(path)?;
    validate_csv(settings.csv_settings, settings.validation_settings)
}

pub fn validate_from_settings_to_json_output(
    settings_path: impl AsRef<Path>,
    output_path: impl AsRef<Path>,
) -> Result<PathBuf> {
    let settings = Settings::load(settings_path)?;
    let result = validate_csv(settings.csv_settings, settings.validation_settings)?;

    // Export json
    let path = output_path.as_ref();
    let content =
        serde_json::to_string_pretty(&result).context("failed to serialize result to JSON")?;
    fs::write(path, content)
        .with_context(|| format!("failed to write result to file {}", path.display()))?;
    Ok(path.to_path_buf())
}
