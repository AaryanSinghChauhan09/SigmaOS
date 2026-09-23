//! mkdir - make directories
//! POSIX-compatible mkdir implementation

use std::fs;
use std::path::Path;
use std::os::unix::fs::PermissionsExt;

/// Create directories
pub fn run(path: &str, parents: bool, mode: Option<u32>) -> Result<(), String> {
    let path_obj = Path::new(path);

    if parents {
        if let Some(m) = mode {
            fs::create_dir_all(path_obj).map_err(|e| format!("mkdir: {}", e))?;
            set_permissions_recursive(path_obj, m)?;
        } else {
            fs::create_dir_all(path_obj).map_err(|e| format!("mkdir: {}", e))?;
        }
    } else {
        if let Some(m) = mode {
            fs::create_dir(path_obj).map_err(|e| format!("mkdir: {}", e))?;
            set_permissions(path_obj, m)?;
        } else {
            fs::create_dir(path_obj).map_err(|e| format!("mkdir: {}", e))?;
        }
    }

    Ok(())
}

/// Set permissions for a directory
fn set_permissions(path: &Path, mode: u32) -> Result<(), String> {
    let metadata = fs::metadata(path).map_err(|e| format!("mkdir: {}", e))?;
    let mut permissions = metadata.permissions();
    permissions.set_mode(mode);
    fs::set_permissions(path, permissions).map_err(|e| format!("mkdir: {}", e))
}

/// Set permissions recursively
fn set_permissions_recursive(path: &Path, mode: u32) -> Result<(), String> {
    set_permissions(path, mode)?;

    for entry in fs::read_dir(path).map_err(|e| format!("mkdir: {}", e))? {
        let entry = entry.map_err(|e| format!("mkdir: {}", e))?;
        let entry_path = entry.path();

        if entry_path.is_dir() {
            set_permissions_recursive(&entry_path, mode)?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mkdir() {
        let test_dir = format!("/tmp/test_mkdir_{}", std::process::id());
        assert!(run(&test_dir, false, None).is_ok());
        assert!(Path::new(&test_dir).exists());
        fs::remove_dir_all(&test_dir).unwrap();
    }

    #[test]
    fn test_mkdir_parents() {
        let test_dir = format!("/tmp/test_mkdir_parents_{}/subdir", std::process::id());
        assert!(run(&test_dir, true, None).is_ok());
        assert!(Path::new(&test_dir).exists());
        let parent = Path::new(&test_dir).parent().unwrap();
        fs::remove_dir_all(parent).unwrap();
    }
}
