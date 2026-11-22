use std::path::PathBuf;

use crate::helpers::delimiter_serialization::delimiter_as_char;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct CsvSettings {
    pub file_path: PathBuf,

    #[serde(with = "delimiter_as_char")]
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
