use crate::{
    csv_handling::{self, csv_settings::CsvSettings},
    domain::validation_configuration::{ColumnConfig, ValidationConfig},
    validation::column_types::ColumnType,
};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

#[derive(Debug, Deserialize, Serialize)]
pub struct Settings {
    pub csv_settings: CsvSettings,
    pub validation_settings: ValidationConfig,
}

pub trait Store {
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

impl Settings {
    pub fn create_settings_from_csv_file(
        csv_path: impl AsRef<Path>,
        output_path: impl AsRef<Path>,
        delimiter: u8,
    ) -> Result<()> {
        let csv_path = csv_path.as_ref();
        let csv_header = csv_handling::read::read_csv_headers(csv_path, delimiter)?;

        let column_settings = csv_header
            .iter()
            .enumerate()
            .map(|(index, header)| ColumnConfig {
                header: header.to_owned(),
                index,
                expected_type: ColumnType::Text(None),
                required: true,
            })
            .collect();

        let settings = Settings {
            csv_settings: CsvSettings {
                file_path: csv_path.to_path_buf(),
                delimiter,
                has_header: true,
            },
            validation_settings: ValidationConfig {
                columns: column_settings,
            },
        };

        settings.save(output_path)
    }
}
