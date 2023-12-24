#[cfg_attr(windows, path = "util/wallpaper_windows.rs")]
#[cfg_attr(not(windows), path = "util/wallpaper_unsupported.rs")]
pub(crate) mod wallpaper;

use std::{collections::BTreeSet, path::PathBuf};

use realpath::realpath;
use walkdir::{DirEntry, WalkDir};

pub(crate) fn scan_files(dirs: Vec<String>) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    // Use a set internally to prevent duplicate entries
    let mut files = BTreeSet::new();

    // Find all files under the specified directories
    // Follow symlinks
    // Ignore files and directories that start with '.'
    // Save the canonical paths
    for dir in dirs {
        for entry in WalkDir::new(dir)
            .follow_links(true)
            .into_iter()
            .filter_entry(|e| !is_hidden(e))
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file() {
                let file = realpath(entry.path())?;

                if matches!(imghdr::from_file(&file), Ok(Some(_))) {
                    files.insert(file);
                }
            }
        }
    }

    Ok(files.into_iter().collect())
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .path()
        .file_name()
        .and_then(|s| s.to_str())
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}
