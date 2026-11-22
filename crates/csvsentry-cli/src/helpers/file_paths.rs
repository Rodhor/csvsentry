use std::{
    env,
    path::{Path, PathBuf},
};

use anyhow::Result;

pub fn resolve_json_output_path(
    filepath: Option<PathBuf>,
    default_name: String,
) -> Result<PathBuf> {
    let mut path = match filepath {
        Some(path) => path,
        None => env::current_dir()?.join(format!("{default_name}.json")),
    };

    if path.is_dir() || has_trailing_separater(&path) {
        path.push(format!("{default_name}.json"));
        return Ok(path);
    }

    if path.extension().is_none() {
        path.set_extension("json");
    }
    Ok(path)
}

fn has_trailing_separater(path: &Path) -> bool {
    let raw = path.to_string_lossy();
    raw.ends_with('/') || raw.ends_with('\\')
}
