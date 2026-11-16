use csvsentry::domain::validation_configuration::Row;
use csvsentry::validation::validation_result::validate_row;

mod common;
#[test]
pub fn matching_types_accepts_rows() {
    let (csv, config) = common::matching_types();
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
pub fn swapped_mail_and_phone_rejects_those_cells() {
    let (csv, config) = common::swapped_mail_and_phone();
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
pub fn matching_types_rejects_bad_cells() {
    let (csv, config) = common::matching_types_bad_cells();
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
