//! File search engine for SigmaOS-file-manager.

use std::path::{Path, PathBuf};
use crate::FileEntry;

#[derive(Debug, Clone)]
pub struct SearchQuery {
    pub name_pattern: Option<String>,
    pub content_pattern: Option<String>,
    pub mime_filter: Option<String>,
    pub case_sensitive: bool,
}

impl SearchQuery {
    pub fn by_name(pattern: impl Into<String>) -> Self {
        Self { name_pattern: Some(pattern.into()), content_pattern: None, mime_filter: None, case_sensitive: false }
    }

    pub fn by_content(pattern: impl Into<String>) -> Self {
        Self { name_pattern: None, content_pattern: Some(pattern.into()), mime_filter: None, case_sensitive: false }
    }
}

pub struct FileSearch;

impl FileSearch {
    /// Search a directory tree for files matching the query.
    pub fn search(root: &Path, query: &SearchQuery) -> Vec<PathBuf> {
        let mut results = Vec::new();
        Self::walk(root, query, &mut results);
        results
    }

    fn walk(dir: &Path, query: &SearchQuery, results: &mut Vec<PathBuf>) {
        let Ok(entries) = std::fs::read_dir(dir) else { return };
        for entry in entries.flatten() {
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();

            if let Some(pat) = &query.name_pattern {
                let matches = if query.case_sensitive {
                    name.contains(pat.as_str())
                } else {
                    name.to_lowercase().contains(&pat.to_lowercase())
                };
                if matches {
                    results.push(path.clone());
                }
            }

            if path.is_dir() {
                Self::walk(&path, query, results);
            }
        }
    }
}
