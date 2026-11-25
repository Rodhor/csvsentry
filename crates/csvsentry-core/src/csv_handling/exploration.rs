use anyhow::Result;
use strum::IntoEnumIterator;
use crate::csv_handling::csv_settings::CsvSettings;
use crate::csv_handling::read::read_sample;
use crate::domain::validation_configuration::RowIndex;
use crate::validation::column_types::ColumnType;

pub struct ExplorationResult {
    pub possible_types: Vec<ColumnType>,
    pub required: bool
}
pub fn explore_csv_column_types(settings: CsvSettings, sample_size: Option<usize>) -> Result<Vec<ExplorationResult>> {
    let row_index: RowIndex = if settings.has_header {RowIndex::CsvIndexHeaderOffset} else {RowIndex::CsvIndex};
    let sample = read_sample(settings, row_index, sample_size)?;
    let cells_count = sample.first().map(|r| r.cells.len()).unwrap_or(0);
    let mut result = Vec::with_capacity(cells_count);

    for col_idx in 0..cells_count {
        let mut possible: Vec<ColumnType> = ColumnType::iter().collect();
        let mut required = true;
        for row in &sample {
            let cell = row.cells.get(col_idx).map(String::as_str).unwrap_or("");
            if cell.trim().is_empty() {
                required = false;
                continue;
            }
            let cell_types = ColumnType::explore_possible_types(cell);
            possible.retain(|t|cell_types.contains(t))
        }
        result.push(ExplorationResult{ possible_types: possible, required })
    }
    Ok(result)
}
