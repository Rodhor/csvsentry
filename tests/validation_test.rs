use csvsentry::domain::validation_configuration::Row;
use csvsentry::validation::validation_result::validate_row;

mod common;
#[test]
pub fn test_column_validation_valid() {
    let (csv, config) = common::valid_configuration();
    let mut rows = csv.lines();
    let mut result = Vec::new();
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

        let row_errors = validate_row(&row, &config);
        if !row_errors.is_empty() {
            result.extend(row_errors);
        }
    }
    assert_eq!(result.len(), 0, "Expected 0 invalid values")
}
#[test]
pub fn test_column_validation_invalid() {
    let (csv, config) = common::invalid_configuration();
    let mut rows = csv.lines();
    let mut result = Vec::new();
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

        let row_errors = validate_row(&row, &config);
        if !row_errors.is_empty() {
            result.extend(row_errors);
        }
    }
    let got: Vec<_> = result
        .iter()
        .map(|e| (e.row, e.column, e.header.as_str(), e.value.as_str()))
        .collect();
    assert_eq!(
        got,
        vec![
            (0, 3, "mail", "alice@example.com"),
            (0, 4, "phone", "+49 123 456789"),
            (1, 3, "mail", "bob@example.com"),
            (1, 4, "phone", "+49 123 456789"),
            (2, 3, "mail", "charlie@example.com"),
            (2, 4, "phone", "555-0200"),
        ]
    );
}

#[test]
pub fn test_valid_configuration_invalid_csv() {
    let (csv, config) = common::valid_configuration_invalid_csv();
    let mut rows = csv.lines();
    let mut result = Vec::new();
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

        let row_errors = validate_row(&row, &config);
        if !row_errors.is_empty() {
            result.extend(row_errors);
        }
    }
    let got: Vec<_> = result
        .iter()
        .map(|e| (e.row, e.column, e.header.as_str(), e.value.as_str()))
        .collect();
    assert_eq!(
        got,
        vec![
            (0, 2, "age", "thirty"),
            (1, 3, "mail", "invalid-email"),
            (2, 0, "id", ""),
        ]
    );
}
