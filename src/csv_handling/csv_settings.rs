use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct CsvSettings {
    pub file_path: PathBuf,
    pub delimiter: u8,
    pub has_header: bool,
}

impl CsvSettings {
    pub fn new() -> Self {
        Self {
            file_path: PathBuf::new(),
            delimiter: ",".parse().unwrap_or(44), // Default to "," if the delimiter cannot be read
            has_header: true,
        }
    }
}
