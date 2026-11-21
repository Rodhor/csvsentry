use crate::{
    csv_handling::csv_settings::CsvSettings, domain::validation_configuration::ValidationConfig,
};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

#[derive(Debug, Deserialize, Serialize)]
pub struct Settings {
    csv_settings: CsvSettings,
    validation_settings: ValidationConfig,
}

trait Store {
    fn save(&self, path: impl AsRef<Path>) -> Result<()>;

    fn load(path: impl AsRef<Path>) -> Result<Settings>;
}

impl Store for Settings {
    fn save(&self, path: impl AsRef<Path>) -> Result<()> {
        let path = path.as_ref();
        let content =
            serde_json::to_string_pretty(self).context("failed to serialize settings to JSON")?;
        fs::write(path, content)
            .with_context(|| format!("failed to write settings to file {}", path.display()))?;

        Ok(())
    }

    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let content = fs::read_to_string(path)
            .with_context(|| format!("failed to read settings file from {}", path.display()))?;
        let settings = serde_json::from_str(&content).with_context(|| {
            format!(
                "failed to parse settings file into struct {}",
                path.display()
            )
        })?;
        Ok(settings)
    }
}
