use crate::csv_handling::csv_settings::CsvSettings;
use crate::domain::validation_configuration::{Row, ValidationConfig};
use crate::validation::validation_result::{ValidationResult, validate_rows};
use anyhow::Result;
use std::fs::File;

pub fn validate_csv(
    settings: CsvSettings,
    config: ValidationConfig,
) -> Result<Vec<ValidationResult>> {
    let rows = read_csv(settings)?;
    Ok(validate_rows(rows.iter(), &config))
}
pub fn read_csv(settings: CsvSettings) -> Result<Vec<Row>> {
    let file = File::open(settings.file_path)?;
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
                number,
                cells: record.iter().map(str::to_owned).collect(),
            })
        })
        .collect()
}
