//! ls - list directory contents
//! POSIX-compatible ls implementation

use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct FileInfo {
    pub name: String,
    pub is_dir: bool,
    pub size: u64,
    pub permissions: String,
}

/// List directory contents
pub fn run(path: &str, _long_format: bool, all: bool) -> Result<Vec<FileInfo>, String> {
    let dir_path = Path::new(path);

    if !dir_path.exists() {
        return Err(format!("ls: cannot access '{}': No such file or directory", path));
    }

    if dir_path.is_file() {
        // Single file
        let metadata = fs::metadata(dir_path).map_err(|e| format!("ls: {}", e))?;
        return Ok(vec![FileInfo {
            name: path.to_string(),
            is_dir: metadata.is_dir(),
            size: metadata.len(),
            permissions: format_permissions(&metadata),
        }]);
    }

    let mut entries = Vec::new();

    let read_dir = fs::read_dir(dir_path).map_err(|e| format!("ls: {}", e))?;

    for entry in read_dir {
        let entry = entry.map_err(|e| format!("ls: {}", e))?;
        let name = entry.file_name().to_string_lossy().to_string();

        // Skip hidden files unless -a flag
        if !all && name.starts_with('.') {
            continue;
        }

        let metadata = entry.metadata().map_err(|e| format!("ls: {}", e))?;

        entries.push(FileInfo {
            name,
            is_dir: metadata.is_dir(),
            size: metadata.len(),
            permissions: format_permissions(&metadata),
        });
    }

    // Sort entries alphabetically
    entries.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(entries)
}

/// Format file permissions (rwxrwxrwx style)
fn format_permissions(metadata: &fs::Metadata) -> String {
    let mut perms = String::new();

    // File type
    if metadata.is_dir() {
        perms.push('d');
    } else if metadata.is_symlink() {
        perms.push('l');
    } else {
        perms.push('-');
    }

    // Read/Write/Execute for user, group, others
    let mode = 0o644; // Placeholder - real implementation would read actual permissions
    let bits = ["r", "w", "x"];

    for i in 0..9 {
        if mode & (1 << (8 - i)) != 0 {
            perms.push_str(bits[i % 3]);
        } else {
            perms.push('-');
        }
    }

    perms
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_permissions() {
        // Test permission formatting
        let path = Path::new(".");
        if let Ok(metadata) = fs::metadata(path) {
            let perms = format_permissions(&metadata);
            assert!(perms.len() == 10);
            assert!(perms.starts_with('d') || perms.starts_with('-'));
        }
    }
}
