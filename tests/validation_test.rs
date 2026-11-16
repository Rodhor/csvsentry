use csvsentry::domain::validation_configuration::Row;
use csvsentry::validation::validation_result::validate_row;

mod common;
#[test]
pub fn test_column_validation() {
    let csv = common::create_valid_csv();
    let config = common::create_validation_config();
    let mut rows = csv.lines();
    // Discard headers
    rows.next();

    for (number, row_values) in rows.enumerate() {
        let row_values: Vec<String> = row_values
            .split(',')
            .into_iter()
            .map(|s| s.to_owned())
            .collect();
        let row = Row {
            number,
            cells: row_values,
        };

        let result = validate_row(&row, &config);
        dbg!(&result);
        assert_eq!(result.len(), 0, "Expected 0 invalid values")
    }
}
