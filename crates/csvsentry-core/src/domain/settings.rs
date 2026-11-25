use crate::{
    csv_handling::{self, csv_settings::CsvSettings},
    domain::validation_configuration::{ColumnConfig, RowIndex, ValidationConfig},
    validation::column_types::ColumnType,
};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};
use crate::csv_handling::exploration::explore_csv_column_types;

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
        Self::create_settings_from_csv_file_with_options(csv_path, output_path, delimiter, false, None)
    }

    pub fn create_settings_from_csv_file_with_options(
        csv_path: impl AsRef<Path>,
        output_path: impl AsRef<Path>,
        delimiter: u8,
        explore_types: bool,
        sample_size: Option<usize>
    ) -> Result<()> {
        let csv_path = csv_path.as_ref();
        let csv_header = csv_handling::read::read_csv_headers(csv_path, delimiter)?;

        let column_settings: Vec<ColumnConfig> = if explore_types {
            let csv_settings = CsvSettings { file_path: csv_path.to_path_buf(), delimiter, has_header: true };
            let explored = explore_csv_column_types(csv_settings, sample_size)?;
            csv_header
                .iter()
                .enumerate()
                .map(|(index, header)| {
                    let e = explored.get(index);

                    // Precedence follows ColumnType variant order (Text is skipped so it never
                    // wins for a non-empty column unless it is the only valid option);
                    // an all-empty column yields an empty
                    // possible_types and falls back to Text(None) below.
                    let expected_type = e
                        .and_then(|e| {
                            e.possible_types
                                .iter()
                                .find(|t| !matches!(t, ColumnType::Text(None)))
                                .cloned()
                        })
                        .unwrap_or(ColumnType::Text(None));
                    ColumnConfig {
                        header: header.to_owned(),
                        index,
                        expected_type,
                        required: e.map(|e| e.required).unwrap_or(true),
                    }
                })
                .collect()
        } else {
            csv_header
                .iter()
                .enumerate()
                .map(|(index, header)| ColumnConfig {
                    header: header.to_owned(),
                    index,
                    expected_type: ColumnType::Text(None),
                    required: true,
                })
                .collect()
        };

        let settings = Settings {
            csv_settings: CsvSettings {
                file_path: csv_path.to_path_buf(),
                delimiter,
                has_header: true,
            },
            validation_settings: ValidationConfig {
                prefered_index: RowIndex::default(),
                columns: column_settings,
            },
        };


        settings.save(output_path)
    }

}
