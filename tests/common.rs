use csvsentry::domain::validation_configuration::{ColumnConfig, ValidationConfig};
use csvsentry::validation::column_types::ColumnType;

pub fn create_valid_csv() -> &'static str {
    "\
id,name,age,mail,phone
1,Alice,30,alice@example.com,+49 123 456789
2,Bob,25,bob@example.com,+49 123 456789
3,Charlie,35,charlie@example.com,555-0200
"
}

pub fn create_invalid_csv() -> &'static str {
    "\
id,name,age,mail,phone
1,Alice,thirty,alice@example.com,555-0100
2,Bob,25,invalid-email,555-0199
,Charlie,40,charlie@example.com,555-0200
"
}

pub fn create_csv_with_caret_comma() -> &'static str {
    "\
id^name^age^mail^phone
1^Diana^28^diana@example.com^555-0300
2^Eve^22^eve@example.com^555-0311
"
}

pub fn create_validation_config() -> ValidationConfig {
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
    }
}
