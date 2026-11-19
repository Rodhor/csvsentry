use csvsentry::domain::validation_configuration::Row;
use csvsentry::validation::validation_result::{ValidationStatus, validate_row, validate_rows};

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
            .map(str::to_owned)
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
            .map(str::to_owned)
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
            .map(str::to_owned)
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
#[test]
pub fn matching_types_validation_result() {
    let (csv, config) = common::matching_types_rows();
    let results = validate_rows(&csv, &config);

    let got: Vec<_> = results
        .iter()
        .map(|r| {
            (
                r.column_header.as_str(),
                r.total_values,
                r.total_empty,
                r.unique_values.len(),
                r.invalid_values.len(),
            )
        })
        .collect();
    assert_eq!(
        got,
        vec![
            // header|total|empty|unique|invalid
            ("id", 3, 0, 3, 0),
            ("name", 3, 0, 3, 0),
            ("age", 3, 0, 3, 0),
            ("mail", 3, 0, 3, 0),
            ("phone", 3, 0, 2, 0),
        ]
    )
}
#[test]
pub fn miss_matching_types_validation_result() {
    let (csv, config) = common::matching_types_bad_cells_row();
    let results = validate_rows(&csv, &config);

    let got: Vec<_> = results
        .iter()
        .map(|r| {
            (
                r.column_header.as_str(),
                r.total_values,
                r.total_empty,
                r.unique_values.len(),
                r.invalid_values.len(),
            )
        })
        .collect();
    assert_eq!(
        got,
        vec![
            // header|total|empty|unique|invalid
            ("id", 3, 1, 2, 1),
            ("name", 3, 0, 3, 0),
            ("age", 3, 0, 3, 1),
            ("mail", 3, 0, 3, 1),
            ("phone", 3, 0, 3, 0),
        ]
    )
}
#[test]
pub fn miss_matching_column_types_validation_result() {
    let (csv, config) = common::swapped_mail_and_phone_row();
    let results = validate_rows(&csv, &config);

    let got: Vec<_> = results
        .iter()
        .map(|r| {
            (
                r.column_header.as_str(),
                r.total_values,
                r.total_empty,
                r.unique_values.len(),
                r.invalid_values.len(),
            )
        })
        .collect();
    assert_eq!(
        got,
        vec![
            // header|total|empty|unique|invalid
            ("id", 3, 0, 3, 0),
            ("name", 3, 0, 3, 0),
            ("age", 3, 0, 3, 0),
            ("mail", 3, 0, 3, 3),
            ("phone", 3, 0, 2, 3),
        ]
    )
}
#[test]
pub fn matching_types_return_valid_rows() {
    let (csv, config) = common::matching_types_rows();
    let results = validate_rows(&csv, &config);

    let got: Vec<_> = results.iter().map(|r| &r.status).collect();
    assert_eq!(
        got,
        vec![
            &ValidationStatus::Valid,
            &ValidationStatus::Valid,
            &ValidationStatus::Valid,
            &ValidationStatus::Valid,
            &ValidationStatus::Valid,
        ]
    )
}
#[test]
pub fn whitespace_and_empty_cells_required_and_not_required() {
    let (csv, config) = common::whitespace_and_missing_cells();
    let results = validate_rows(&csv, &config);

    let got: Vec<_> = results
        .iter()
        .map(|r| {
            (
                r.column_header.as_str(),
                r.total_values,
                r.total_empty,
                r.unique_values.len(),
                r.invalid_values.len(),
            )
        })
        .collect();
    assert_eq!(
        got,
        vec![
            // header|total|empty|unique|invalid
            ("id", 3, 0, 3, 0),
            ("name", 3, 0, 3, 0),
            ("age", 3, 2, 1, 0),
            ("mail", 3, 0, 3, 0),
            ("phone", 3, 2, 1, 2),
        ]
    );
}
#[test]
pub fn whitespace_and_empty_cells_required_and_not_required_part_2() {
    let (csv, config) = common::whitespace_and_missing_cells_part_2();
    let results = validate_rows(&csv, &config);

    let got: Vec<_> = results
        .iter()
        .map(|r| {
            (
                r.column_header.as_str(),
                r.total_values,
                r.total_empty,
                r.unique_values.len(),
                r.invalid_values.len(),
            )
        })
        .collect();
    assert_eq!(
        got,
        vec![
            // header|total|empty|unique|invalid
            ("id", 3, 0, 3, 0),
            ("name", 3, 0, 3, 0),
            ("age", 3, 2, 1, 2),
            ("mail", 3, 0, 3, 0),
            ("phone", 3, 2, 1, 0),
        ]
    );
}
#[test]
pub fn whitespace_and_empty_cells_required_and_not_required_status_validation() {
    let (csv, config) = common::whitespace_and_missing_cells_part_2();
    let results = validate_rows(&csv, &config);

    let got: Vec<_> = results.iter().map(|r| &r.status).collect();
    assert_eq!(
        got,
        vec![
            &ValidationStatus::Valid,
            &ValidationStatus::Valid,
            &ValidationStatus::Invalid,
            &ValidationStatus::Valid,
            &ValidationStatus::Valid,
        ]
    )
}
