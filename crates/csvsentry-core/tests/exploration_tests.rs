use tempfile::TempDir;
use csvsentry_core::csv_handling::csv_settings::CsvSettings;
use csvsentry_core::csv_handling::exploration::{explore_csv_column_types};
use csvsentry_core::validation::column_types::ColumnType;
use crate::common::{create_large_csv, custom_delimiter_csv, custom_delimiter_no_header_csv, prepare_bytes, prepare_file, regular_csv};

mod common;

#[test]
fn explore_regular_csv_types() {
    let (_dir, path) = prepare_file(regular_csv());
    let settings = CsvSettings { file_path: path, delimiter: b',', has_header: true };
    let res = explore_csv_column_types(settings, Some(3)).unwrap();

    assert_eq!(res.len(), 5);
    assert!(res[0].possible_types.contains(&ColumnType::Integer));
    assert!(res[0].possible_types.contains(&ColumnType::Text(None)));
    assert_eq!(res[1].possible_types, vec![ColumnType::Text(None)]);
    assert!(res[3].possible_types.contains(&ColumnType::Email));
    assert!(res.iter().all(|r| r.required));
}

#[test]
fn explore_marks_required_false_on_empty_cells() {
    let csv = "id,name\n1,\n2,Bob\n";
    let (_dir, path) = prepare_bytes(csv.as_bytes());
    let settings = CsvSettings { file_path: path, delimiter: b',', has_header: true };
    let res = explore_csv_column_types(settings, Some(2)).unwrap();
    assert!(res[0].required);
    assert!(!res[1].required);
}

#[test]
fn explore_empty_file_returns_empty() {
    let (_dir, path) = prepare_bytes(b"");
    let settings = CsvSettings { file_path: path, delimiter: b',', has_header: true };
    let res = explore_csv_column_types(settings, Some(5)).unwrap();
    assert!(res.is_empty());
}

#[test]
fn explore_none_sample_size_defaults_to_50() {
    let content = create_large_csv(100);
    let (_dir, path) = prepare_bytes(content.as_bytes());
    let settings = CsvSettings { file_path: path, delimiter: b',', has_header: true };
    let res = explore_csv_column_types(settings, None).unwrap();
    assert_eq!(res.len(), 5);
}

#[test]
fn explore_custom_delimiter() {
    let (_dir, path) = prepare_file(custom_delimiter_csv());
    let settings = CsvSettings { file_path: path, delimiter: b'^', has_header: true };
    let res = explore_csv_column_types(settings, Some(3)).unwrap();
    assert_eq!(res.len(), 5);
    assert!(res[0].possible_types.contains(&ColumnType::Integer));
}

#[test]
fn explore_no_header() {
    let (_dir, path) = prepare_file(custom_delimiter_no_header_csv());
    let settings = CsvSettings { file_path: path, delimiter: b'^', has_header: false };
    let res = explore_csv_column_types(settings, Some(3)).unwrap();
    assert_eq!(res.len(), 5);
}

#[test]
fn explore_missing_file_is_error() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("nope.csv");
    let settings = CsvSettings { file_path: path, delimiter: b',', has_header: true };
    assert!(explore_csv_column_types(settings, Some(5)).is_err());
}

#[test]
fn explore_sample_size_zero_returns_empty() {
    let (_dir, path) = prepare_file(regular_csv());
    let settings = CsvSettings { file_path: path, delimiter: b',', has_header: true };
    let res = explore_csv_column_types(settings, Some(0)).unwrap();
    assert!(res.is_empty());
}

#[test]
fn required_stays_true_if_empty_outside_sample() {
    let csv = "id,name\n1,Alice\n2,Bob\n3,\n";
    let (_dir, path) = prepare_bytes(csv.as_bytes());
    let settings = CsvSettings { file_path: path, delimiter: b',', has_header: true };
    let res = explore_csv_column_types(settings, Some(2)).unwrap();
    assert!(res[1].required); // empty row wasn't sampled
}

#[test]
fn explore_errors_on_ragged_rows() {
    let (_dir, path) = prepare_bytes(b"id,name\n1,Alice\n2,Bob,extra\n");
    let settings = CsvSettings { file_path: path, delimiter: b',', has_header: true };
    assert!(explore_csv_column_types(settings, Some(5)).is_err());
}

#[test]
fn explore_errors_on_invalid_utf8() {
    let mut bytes = b"id,name\n1,Alice\n".to_vec();
    bytes.push(0xFF);
    let (_dir, path) = prepare_bytes(&bytes);
    let settings = CsvSettings { file_path: path, delimiter: b',', has_header: true };
    assert!(explore_csv_column_types(settings, Some(5)).is_err());
}

#[test]
fn explore_sample_larger_than_file() {
    let (_dir, path) = prepare_file(regular_csv());
    let settings = CsvSettings { file_path: path, delimiter: b',', has_header: true };
    let res = explore_csv_column_types(settings, Some(999)).unwrap();
    assert_eq!(res.len(), 5);
}
