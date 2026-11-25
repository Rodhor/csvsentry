use crate::csv_handling::csv_settings::CsvSettings;
use crate::domain::validation_configuration::{Row, RowIndex, ValidationConfig};
use crate::validation::validation_result::{ValidationResult, validate_rows};
use anyhow::{Ok, Result};
use std::fs::File;
use std::path::Path;

pub fn validate_csv(
    settings: CsvSettings,
    config: ValidationConfig,
) -> Result<Vec<ValidationResult>> {
    let rows = read_csv(settings, &config)?;
    Ok(validate_rows(rows.iter(), &config))
}

pub fn read_csv(settings: CsvSettings, config: &ValidationConfig) -> Result<Vec<Row>> {
    let file = File::open(settings.file_path)?;
    let index = config.prefered_index;
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(settings.has_header)
        .delimiter(settings.delimiter)
        .from_reader(file);

    reader
        .records()
        .enumerate()
        .map(|(number, record)| {
            let record = record?;
            Ok(Row {
                number: index.from_csv_reader(number, settings.has_header),
                cells: record.iter().map(str::to_owned).collect(),
            })
        })
        .collect()
}

pub fn read_csv_headers(path: &Path, delimiter: u8) -> Result<Vec<String>> {
    let file = File::open(path)?;
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(delimiter)
        .from_reader(file);

    let headers = reader.headers()?;
    Ok(headers.iter().map(str::to_owned).collect())
}

pub fn read_sample(
    settings: CsvSettings,
    row_index: RowIndex,
    size: Option<usize>,
) -> Result<Vec<Row>> {
    let file = File::open(settings.file_path)?;
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(settings.has_header)
        .delimiter(settings.delimiter)
        .from_reader(file);

    reader
        .records()
        .take(size.unwrap_or(50))
        .enumerate()
        .map(|(number, record)| {
            let record = record?;
            Ok(Row {
                number: row_index.from_csv_reader(number, settings.has_header),
                cells: record.iter().map(str::to_owned).collect(),
            })
        })
        .collect()
}
