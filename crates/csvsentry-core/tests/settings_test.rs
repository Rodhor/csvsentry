mod common;

use csvsentry_core::domain::settings::{Settings, Store};
use csvsentry_core::validation::column_types::ColumnType;
use tempfile::TempDir;

use crate::common::{prepare_bytes, prepare_file, regular_csv};

#[test]
fn generate_without_explore_uses_text() {
    let (dir, path) = prepare_file(regular_csv());
    let json = dir.path().join("settings.json");

    Settings::create_settings_from_csv_file(&path, &json, b',').unwrap();
    let s = Settings::load(&json).unwrap();

    assert_eq!(s.validation_settings.columns[0].expected_type, ColumnType::Text(None));
    assert_eq!(s.validation_settings.columns[1].expected_type, ColumnType::Text(None));
}

#[test]
fn explore_picks_integer_for_numeric_column() {
    let (dir, path) = prepare_file(regular_csv());
    let json = dir.path().join("settings.json");

    Settings::create_settings_from_csv_file_with_options(&path, &json, b',', true, Some(3)).unwrap();
    let s = Settings::load(&json).unwrap();

    assert_eq!(s.validation_settings.columns[0].expected_type, ColumnType::Integer);
}

#[test]
fn explore_empty_column_falls_back_to_text() {
    let (dir, path) = prepare_bytes(b"id,name\n1,\n2,\n");
    let json = dir.path().join("settings.json");

    Settings::create_settings_from_csv_file_with_options(&path, &json, b',', true, Some(2)).unwrap();
    let s = Settings::load(&json).unwrap();

    assert_eq!(s.validation_settings.columns[1].expected_type, ColumnType::Text(None));
    assert!(!s.validation_settings.columns[1].required);
}

#[test]
fn explore_picks_email_for_email_column() {
    let (dir, path) = prepare_bytes(b"mail\na@b.com\nc@d.com\n");
    let json = dir.path().join("settings.json");

    Settings::create_settings_from_csv_file_with_options(&path, &json, b',', true, Some(2)).unwrap();
    let s = Settings::load(&json).unwrap();

    assert_eq!(s.validation_settings.columns[0].expected_type, ColumnType::Email);
}

#[test]
fn explore_picks_boolean_for_boolean_column() {
    let (dir, path) = prepare_bytes(b"flag\ntrue\nfalse\nyes\n");
    let json = dir.path().join("settings.json");

    Settings::create_settings_from_csv_file_with_options(&path, &json, b',', true, Some(3)).unwrap();
    let s = Settings::load(&json).unwrap();

    assert_eq!(s.validation_settings.columns[0].expected_type, ColumnType::Boolean);
}

#[test]
fn explore_missing_file_is_error() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("nope.csv");
    let json = dir.path().join("settings.json");

    assert!(Settings::create_settings_from_csv_file(&path, &json, b',').is_err());
}