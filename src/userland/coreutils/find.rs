//! find - search for files in a directory hierarchy
//! POSIX-compatible find utility

use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Default)]
pub struct FindOptions {
    pub name_pattern: Option<String>,
    pub file_type: Option<char>, // 'f' for file, 'd' for directory, 'l' for symlink
    pub max_depth: Option<usize>,
}

pub fn run(start_dir: &str, options: &FindOptions) -> Result<Vec<PathBuf>, String> {
    let mut results = Vec::new();
    let root = Path::new(start_dir);
    if !root.exists() {
        return Err(format!("find: '{}': No such file or directory", start_dir));
    }

    walk_dir(root, 0, options, &mut results)?;
    for path in &results {
        println!("{}", path.display());
    }
    Ok(results)
}

fn walk_dir(
    dir: &Path,
    current_depth: usize,
    options: &FindOptions,
    results: &mut Vec<PathBuf>,
) -> Result<(), String> {
    if let Some(max) = options.max_depth {
        if current_depth > max {
            return Ok(());
        }
    }

    let is_match = matches_filter(dir, options);
    if is_match {
        results.push(dir.to_path_buf());
    }

    if dir.is_dir() {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry_res in entries {
                if let Ok(entry) = entry_res {
                    let path = entry.path();
                    let _ = walk_dir(&path, current_depth + 1, options, results);
                }
            }
        }
    }

    Ok(())
}

fn matches_filter(path: &Path, options: &FindOptions) -> bool {
    if let Some(ref pattern) = options.name_pattern {
        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
            if !name.contains(pattern) {
                return false;
            }
        } else {
            return false;
        }
    }

    if let Some(t) = options.file_type {
        match t {
            'f' => {
                if !path.is_file() {
                    return false;
                }
            }
            'd' => {
                if !path.is_dir() {
                    return false;
                }
            }
            'l' => {
                if !path.is_symlink() {
                    return false;
                }
            }
            _ => {}
        }
    }

    true
}
