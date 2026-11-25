use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;
use csvsentry_core::csv_handling::csv_settings::CsvSettings;
use csvsentry_core::csv_handling::read::{read_csv, read_csv_headers, read_sample};
use csvsentry_core::domain::validation_configuration::{RowIndex, ValidationConfig};

fn create_large_csv(rows: usize) -> String {
    let mut s = String::from("id,name,age,mail,phone\n");
    for i in 0..rows {
        use std::fmt::Write as _;
        writeln!(s,"{i},User{i},{},user{i}@example.com,+49 123 456789", 20 + (i % 60)).unwrap();
    }
    s
}

fn regular_csv() -> &'static str {
    "\
id,name,age,mail,phone
1,Alice,30,alice@example.com,+49 123 456789
2,Bob,25,bob@example.com,+49 123 456789
3,Charlie,35,charlie@example.com,555-0200
"
}

fn custom_delimiter_csv() -> &'static str {
    "\
id^name^age^mail^phone
1^Alice^30^alice@example.com^+49 123 456789
2^Bob^25^bob@example.com^+49 123 456789
3^Charlie^35^charlie@example.com^555-0200
"
}

fn custom_delimiter_no_header_csv() -> &'static str {
    "\
1^Alice^30^alice@example.com^+49 123 456789
2^Bob^25^bob@example.com^+49 123 456789
3^Charlie^35^charlie@example.com^555-0200
"
}

fn prepare_file(content: &'static str) -> (TempDir,PathBuf) {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("data.csv");
    fs::write(&path, content).unwrap();

    (dir,path)
}

fn prepare_bytes(content: &[u8]) -> (TempDir, PathBuf) {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("data.csv");
    fs::write(&path, content).unwrap();
    (dir,path)
}

#[test]
fn read_csv_parses_rows_and_numbers() {
    let (_temp_dir, path) = prepare_file(regular_csv());

    let settings = CsvSettings{
        file_path: path,
        delimiter: b',',
        has_header: true,
    };
    let config = ValidationConfig { prefered_index: RowIndex::CsvIndex, columns: vec![] };
    let rows = read_csv(settings, &config).unwrap();
    assert_eq!(rows.len(), 3);
    assert_eq!(rows[0].number, 0);
    assert_eq!(rows[0].cells, vec!["1","Alice", "30", "alice@example.com", "+49 123 456789"]);
    assert_eq!(rows[1].cells, vec!["2","Bob", "25", "bob@example.com", "+49 123 456789"]);
}

#[test]
fn read_csv_headers_returns_header_row() {
    let (_dir, path) = prepare_file(regular_csv());
    let headers = read_csv_headers(&path, b',').unwrap();
    assert_eq!(headers, vec!["id", "name", "age", "mail", "phone"])
}

#[test]
fn read_sample_limits_rows() {
    let (_dir, path) = prepare_file(regular_csv());
    let settings = CsvSettings{
        file_path: path,
        delimiter: b',',
        has_header: true,
    };
    let rows = read_sample(settings, RowIndex::CsvIndexHeaderOffset, Some(2)).unwrap();
    assert_eq!(rows.len(), 2)
}


#[test]
fn read_with_row_csv_index() {
    let (_dir, path) = prepare_file(regular_csv());
    let settings = CsvSettings{
        file_path: path,
        delimiter: b',',
        has_header: true,
    };
    let config = ValidationConfig { prefered_index: RowIndex::CsvIndex, columns: vec![] };
    let rows = read_csv(settings, &config).unwrap();
    assert_eq!(rows[0].number, 0)
}

#[test]
fn read_with_row_csv_index_header_offset() {
    let (_dir, path) = prepare_file(regular_csv());
    let settings = CsvSettings{
        file_path: path,
        delimiter: b',',
        has_header: true,
    };
    let config = ValidationConfig { prefered_index: RowIndex::CsvIndexHeaderOffset, columns: vec![] };
    let rows = read_csv(settings, &config).unwrap();
    assert_eq!(rows[0].number, 1)
}

#[test]
fn read_with_row_excel_index() {
    let (_dir, path) = prepare_file(regular_csv());
    let settings = CsvSettings{
        file_path: path,
        delimiter: b',',
        has_header: true,
    };
    let config = ValidationConfig { prefered_index: RowIndex::ExcelIndex, columns: vec![] };
    let rows = read_csv(settings, &config).unwrap();
    assert_eq!(rows[0].number, 2)
}


#[test]
fn read_csv_custom_delimiter_and_no_header() {
    let (_dir, path) = prepare_file(custom_delimiter_no_header_csv());
    let settings = CsvSettings{
        file_path: path,
        delimiter: b'^',
        has_header: false,
    };
    let config = ValidationConfig { prefered_index: RowIndex::CsvIndex, columns: vec![] };
    let rows = read_csv(settings, &config).unwrap();
    assert_eq!(rows.len(), 3);
    assert_eq!(rows[0].cells, vec!["1","Alice", "30", "alice@example.com", "+49 123 456789"]);
}

#[test]
fn read_csv_custom_delimiter_and_header() {
    let (_dir, path) = prepare_file(custom_delimiter_csv());
    let settings = CsvSettings{
        file_path: path,
        delimiter: b'^',
        has_header: true,
    };
    let config = ValidationConfig { prefered_index: RowIndex::CsvIndex, columns: vec![] };
    let rows = read_csv(settings, &config).unwrap();
    assert_eq!(rows.len(), 3);
    assert_eq!(rows[0].cells, vec!["1","Alice", "30", "alice@example.com", "+49 123 456789"]);
}
#[test]
fn read_csv_missing_file_is_error() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("nope.csv");
    let settings = CsvSettings { file_path: path, delimiter: b',', has_header: true };
    let config = ValidationConfig { prefered_index: RowIndex::CsvIndex, columns: vec![] };
    assert!(read_csv(settings, &config).is_err())
}

#[test]
fn empty_file_yields_no_rows() {
    let (_dir, path) = prepare_bytes(b"");
    let settings = CsvSettings{
        file_path: path,
        delimiter: b',',
        has_header: true,
    };
    let config = ValidationConfig { prefered_index: RowIndex::CsvIndex, columns: vec![] };
    let rows = read_csv(settings, &config).unwrap();
    assert!(rows.is_empty())
}

#[test]
fn bom_is_removed() {
    let mut bytes = vec![0xEF, 0xBB, 0xBF];
    bytes.extend_from_slice(regular_csv().as_bytes());
    let (_dir, path) = prepare_bytes(&bytes);
    let settings = CsvSettings{
        file_path: path.clone(),
        delimiter: b',',
        has_header: true,
    };
    let config = ValidationConfig { prefered_index: RowIndex::CsvIndex, columns: vec![] };
    let rows = read_csv(settings, &config).unwrap();
    assert_eq!(rows[0].cells[0], "1");

    let headers = read_csv_headers(&path, b',').unwrap();
    assert_eq!(headers[0], "id")
}

#[test]
fn read_csv_handles_many_rows() {
    let content = create_large_csv(10_000);
    let (_dir, path) = prepare_bytes(content.as_bytes());
    let settings = CsvSettings{
        file_path: path.clone(),
        delimiter: b',',
        has_header: true,
    };
    let config = ValidationConfig { prefered_index: RowIndex::CsvIndexHeaderOffset, columns: vec![] };
    let rows = read_csv(settings, &config).unwrap();

    assert_eq!(rows.len(), 10_000);
    assert_eq!(rows[0].cells[0], "0");
    assert_eq!(rows[9_999].cells[0], "9999");
}

#[test]
fn read_csv_allows_bigger_sample() {
    let content = create_large_csv(10_000);
    let (_dir, path) = prepare_bytes(content.as_bytes());
    let settings = CsvSettings{
        file_path: path.clone(),
        delimiter: b',',
        has_header: true,
    };
    let rows = read_sample(settings, RowIndex::CsvIndexHeaderOffset, Some(5_000)).unwrap();

    assert_eq!(rows.len(), 5000);
    assert_eq!(rows[0].cells[0], "0");
    assert_eq!(rows[4_999].cells[0], "4999");
}

#[test]
fn header_only_file_yields_no_rows() {
    let (_dir, path) = prepare_bytes(b"id,name\n");
    let settings = CsvSettings { file_path: path, delimiter: b',', has_header: true };
    let config = ValidationConfig { prefered_index: RowIndex::CsvIndex, columns: vec![] };
    assert!(read_csv(settings, &config).unwrap().is_empty());
}

#[test]
fn read_sample_none_size_defaults() {
    let content = create_large_csv(100);
    let (_dir, path) = prepare_bytes(content.as_bytes());
    let settings = CsvSettings { file_path: path, delimiter: b',', has_header: true };
    let rows = read_sample(settings, RowIndex::CsvIndex, None).unwrap();
    assert_eq!(rows.len(), 50);
}

#[test]
fn read_sample_size_beyond_file_returns_all() {
    let (_dir, path) = prepare_file(regular_csv());
    let settings = CsvSettings { file_path: path, delimiter: b',', has_header: true };
    let rows = read_sample(settings, RowIndex::CsvIndex, Some(999)).unwrap();
    assert_eq!(rows.len(), 3);
}

#[test]
fn quoted_fields_with_commas_and_newlines() {
    let csv = "id,name\n1,\"Doe, John\"\n2,\"Line1\nLine2\"\n";
    let (_dir, path) = prepare_bytes(csv.as_bytes());
    let settings = CsvSettings { file_path: path, delimiter: b',', has_header: true };
    let config = ValidationConfig { prefered_index: RowIndex::CsvIndex, columns: vec![] };
    let rows = read_csv(settings, &config).unwrap();
    assert_eq!(rows[0].cells, vec!["1", "Doe, John"]);
    assert_eq!(rows[1].cells, vec!["2", "Line1\nLine2"]);
}


#[test]
fn broken_csv_ragged_rows_is_error() {
    let (_dir, path) = prepare_bytes(b"id,name\n1,Alice\n2,Bob,extra\n");
    let settings = CsvSettings { file_path: path, delimiter: b',', has_header: true };
    let config = ValidationConfig { prefered_index: RowIndex::CsvIndex, columns: vec![] };
    assert!(read_csv(settings, &config).is_err());
}

#[test]
fn broken_csv_invalid_utf8_is_error() {
    let mut bytes = b"id,name\n1,Alice\n".to_vec();
    bytes.push(0xFF);
    let (_dir, path) = prepare_bytes(&bytes);
    let settings = CsvSettings { file_path: path, delimiter: b',', has_header: true };
    let config = ValidationConfig { prefered_index: RowIndex::CsvIndex, columns: vec![] };
    assert!(read_csv(settings, &config).is_err());
}



#[test]
fn broken_csv_custom_delimiter_ragged_rows_is_error() {
    let (_dir, path) = prepare_bytes(b"id^name\n1^Alice\n2^Bob^extra\n");
    let settings = CsvSettings { file_path: path, delimiter: b'^', has_header: true };
    let config = ValidationConfig { prefered_index: RowIndex::CsvIndex, columns: vec![] };
    assert!(read_csv(settings, &config).is_err());
}



// Default behavior of csv crate at the time of testing - 11.2026
#[test]
fn unclosed_quote_is_accepted_as_field() {
    let (_dir, path) = prepare_bytes(b"id,name\n1,\"Alice\n");
    let settings = CsvSettings { file_path: path, delimiter: b',', has_header: true };
    let config = ValidationConfig { prefered_index: RowIndex::CsvIndex, columns: vec![] };
    let rows = read_csv(settings, &config).unwrap();
    assert_eq!(rows[0].cells, vec!["1", "Alice\n"]);
}