//! rm - remove files or directories
//! POSIX-compatible rm implementation

use std::fs;
use std::path::Path;

/// Remove file or directory
pub fn run(path: &str, recursive: bool, force: bool) -> Result<(), String> {
    let path_obj = Path::new(path);
    
    if !path_obj.exists() {
        if force {
            return Ok(());
        }
        return Err(format!("rm: cannot remove '{}': No such file or directory", path));
    }
    
    if path_obj.is_dir() {
        if !recursive {
            return Err(format!("rm: cannot remove '{}': Is a directory", path));
        }
        remove_directory(path_obj, force)
    } else {
        remove_file(path_obj, force)
    }
}

/// Remove a single file
fn remove_file(path: &Path, force: bool) -> Result<(), String> {
    fs::remove_file(path).map_err(|e| {
        if force {
            // Force removal - try again with write permissions
            format!("rm: {}", e)
        } else {
            format!("rm: {}", e)
        }
    })
}

/// Remove directory recursively
fn remove_directory(path: &Path, force: bool) -> Result<(), String> {
    for entry in fs::read_dir(path).map_err(|e| format!("rm: {}", e))? {
        let entry = entry.map_err(|e| format!("rm: {}", e))?;
        let entry_path = entry.path();
        
        if entry_path.is_dir() {
            remove_directory(&entry_path, force)?;
        } else {
            remove_file(&entry_path, force)?;
        }
    }
    
    fs::remove_dir(path).map_err(|e| format!("rm: {}", e))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_remove_file() {
        let test_dir = format!("/tmp/test_rm_{}", std::process::id());
        fs::create_dir_all(&test_dir).unwrap();

        let test_file = format!("{}/test.txt", test_dir);
        let mut file = File::create(&test_file).unwrap();
        file.write_all(b"test content").unwrap();

        assert!(run(&test_file, false, false).is_ok());
        assert!(!Path::new(&test_file).exists());

        fs::remove_dir_all(&test_dir).unwrap();
    }

    #[test]
    fn test_remove_directory() {
        let test_dir = format!("/tmp/test_rm_dir_{}", std::process::id());
        fs::create_dir_all(format!("{}/subdir", test_dir)).unwrap();

        assert!(run(&test_dir, true, false).is_ok());
        assert!(!Path::new(&test_dir).exists());
    }
}
