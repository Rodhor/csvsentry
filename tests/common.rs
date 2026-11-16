use csvsentry::domain::validation_configuration::{ColumnConfig, Row, ValidationConfig};
use csvsentry::validation::column_types::ColumnType;

pub fn people_csv() -> &'static str {
    "\
id,name,age,mail,phone
1,Alice,30,alice@example.com,+49 123 456789
2,Bob,25,bob@example.com,+49 123 456789
3,Charlie,35,charlie@example.com,555-0200
"
}

pub fn people_csv_bad_cells() -> &'static str {
    "\
id,name,age,mail,phone
1,Alice,thirty,alice@example.com,555-0100
2,Bob,25,invalid-email,555-0199
,Charlie,40,charlie@example.com,555-0200
"
}

fn rows_from(csv: &str) -> Vec<Row> {
    csv.lines()
        // Skip header
        .skip(1)
        .enumerate()
        .map(|(number, line)| Row {
            number,
            cells: line.split(',').map(str::to_owned).collect(),
        })
        .collect()
}
pub fn matching_types() -> (&'static str, ValidationConfig) {
    (
        people_csv(),
        ValidationConfig {
            columns: vec![
                ColumnConfig {
                    header: "id".to_string(),
                    index: 0,
                    expected_type: ColumnType::Integer,
                    required: true,
                },
                ColumnConfig {
                    header: "name".to_string(),
                    index: 1,
                    expected_type: ColumnType::Text(None),
                    required: true,
                },
                ColumnConfig {
                    header: "age".to_string(),
                    index: 2,
                    expected_type: ColumnType::Integer,
                    required: true,
                },
                ColumnConfig {
                    header: "mail".to_string(),
                    index: 3,
                    expected_type: ColumnType::Email,
                    required: true,
                },
                ColumnConfig {
                    header: "phone".to_string(),
                    index: 4,
                    expected_type: ColumnType::PhoneNumber,
                    required: true,
                },
            ],
        },
    )
}
pub fn swapped_mail_and_phone() -> (&'static str, ValidationConfig) {
    (
        people_csv(),
        ValidationConfig {
            columns: vec![
                ColumnConfig {
                    header: "id".to_string(),
                    index: 0,
                    expected_type: ColumnType::Text(None),
                    required: true,
                },
                ColumnConfig {
                    header: "name".to_string(),
                    index: 1,
                    expected_type: ColumnType::Text(None),
                    required: true,
                },
                ColumnConfig {
                    header: "age".to_string(),
                    index: 2,
                    expected_type: ColumnType::Float,
                    required: true,
                },
                ColumnConfig {
                    header: "mail".to_string(),
                    index: 3,
                    expected_type: ColumnType::PhoneNumber,
                    required: true,
                },
                ColumnConfig {
                    header: "phone".to_string(),
                    index: 4,
                    expected_type: ColumnType::Email,
                    required: true,
                },
            ],
        },
    )
}
pub fn matching_types_bad_cells() -> (&'static str, ValidationConfig) {
    (
        people_csv_bad_cells(),
        ValidationConfig {
            columns: vec![
                ColumnConfig {
                    header: "id".to_string(),
                    index: 0,
                    expected_type: ColumnType::Integer,
                    required: true,
                },
                ColumnConfig {
                    header: "name".to_string(),
                    index: 1,
                    expected_type: ColumnType::Text(None),
                    required: true,
                },
                ColumnConfig {
                    header: "age".to_string(),
                    index: 2,
                    expected_type: ColumnType::Integer,
                    required: true,
                },
                ColumnConfig {
                    header: "mail".to_string(),
                    index: 3,
                    expected_type: ColumnType::Email,
                    required: true,
                },
                ColumnConfig {
                    header: "phone".to_string(),
                    index: 4,
                    expected_type: ColumnType::PhoneNumber,
                    required: true,
                },
            ],
        },
    )
}
pub fn matching_types_rows() -> (Vec<Row>, ValidationConfig) {
    (
        rows_from(people_csv()),
        ValidationConfig {
            columns: vec![
                ColumnConfig {
                    header: "id".to_string(),
                    index: 0,
                    expected_type: ColumnType::Integer,
                    required: true,
                },
                ColumnConfig {
                    header: "name".to_string(),
                    index: 1,
                    expected_type: ColumnType::Text(None),
                    required: true,
                },
                ColumnConfig {
                    header: "age".to_string(),
                    index: 2,
                    expected_type: ColumnType::Integer,
                    required: true,
                },
                ColumnConfig {
                    header: "mail".to_string(),
                    index: 3,
                    expected_type: ColumnType::Email,
                    required: true,
                },
                ColumnConfig {
                    header: "phone".to_string(),
                    index: 4,
                    expected_type: ColumnType::PhoneNumber,
                    required: true,
                },
            ],
        },
    )
}
pub fn matching_types_bad_cells_row() -> (Vec<Row>, ValidationConfig) {
    (
        rows_from(people_csv_bad_cells()),
        ValidationConfig {
            columns: vec![
                ColumnConfig {
                    header: "id".to_string(),
                    index: 0,
                    expected_type: ColumnType::Integer,
                    required: true,
                },
                ColumnConfig {
                    header: "name".to_string(),
                    index: 1,
                    expected_type: ColumnType::Text(None),
                    required: true,
                },
                ColumnConfig {
                    header: "age".to_string(),
                    index: 2,
                    expected_type: ColumnType::Integer,
                    required: true,
                },
                ColumnConfig {
                    header: "mail".to_string(),
                    index: 3,
                    expected_type: ColumnType::Email,
                    required: true,
                },
                ColumnConfig {
                    header: "phone".to_string(),
                    index: 4,
                    expected_type: ColumnType::PhoneNumber,
                    required: true,
                },
            ],
        },
    )
}
pub fn swapped_mail_and_phone_row() -> (Vec<Row>, ValidationConfig) {
    (
        rows_from(people_csv()),
        ValidationConfig {
            columns: vec![
                ColumnConfig {
                    header: "id".to_string(),
                    index: 0,
                    expected_type: ColumnType::Text(None),
                    required: true,
                },
                ColumnConfig {
                    header: "name".to_string(),
                    index: 1,
                    expected_type: ColumnType::Text(None),
                    required: true,
                },
                ColumnConfig {
                    header: "age".to_string(),
                    index: 2,
                    expected_type: ColumnType::Float,
                    required: true,
                },
                ColumnConfig {
                    header: "mail".to_string(),
                    index: 3,
                    expected_type: ColumnType::PhoneNumber,
                    required: true,
                },
                ColumnConfig {
                    header: "phone".to_string(),
                    index: 4,
                    expected_type: ColumnType::Email,
                    required: true,
                },
            ],
        },
    )
}
