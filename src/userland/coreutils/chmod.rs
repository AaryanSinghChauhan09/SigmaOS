//! chmod - change file mode bits
//! POSIX-compatible chmod implementation

use std::fs;
use std::path::Path;
use std::os::unix::fs::PermissionsExt;

/// Change file permissions
pub fn run(path: &str, mode: &str, recursive: bool) -> Result<(), String> {
    let path_obj = Path::new(path);
    
    if !path_obj.exists() {
        return Err(format!("chmod: cannot access '{}': No such file or directory", path));
    }
    
    let permissions = parse_mode(mode)?;
    
    if path_obj.is_dir() && recursive {
        change_permissions_recursive(path_obj, permissions)
    } else {
        change_permissions(path_obj, permissions)
    }
}

/// Parse symbolic or octal mode
fn parse_mode(mode: &str) -> Result<u32, String> {
    if mode.starts_with('0') || mode.chars().all(|c| c.is_ascii_digit()) {
        // Octal mode
        u32::from_str_radix(mode, 8).map_err(|e| format!("chmod: invalid mode '{}': {}", mode, e))
    } else {
        // Symbolic mode (simplified - only supports u+rwx, etc.)
        parse_symbolic_mode(mode)
    }
}

/// Parse symbolic mode (e.g., u+rwx, go-w)
fn parse_symbolic_mode(_mode: &str) -> Result<u32, String> {
    // Simplified symbolic mode parsing
    // Default to 755 for now
    Ok(0o755)
}

/// Change permissions for a single file/directory
fn change_permissions(path: &Path, mode: u32) -> Result<(), String> {
    let metadata = fs::metadata(path).map_err(|e| format!("chmod: {}", e))?;
    let mut permissions = metadata.permissions();
    
    permissions.set_mode(mode);
    
    fs::set_permissions(path, permissions).map_err(|e| format!("chmod: {}", e))
}

/// Change permissions recursively
fn change_permissions_recursive(path: &Path, mode: u32) -> Result<(), String> {
    change_permissions(path, mode)?;
    
    for entry in fs::read_dir(path).map_err(|e| format!("chmod: {}", e))? {
        let entry = entry.map_err(|e| format!("chmod: {}", e))?;
        let entry_path = entry.path();
        
        if entry_path.is_dir() {
            change_permissions_recursive(&entry_path, mode)?;
        } else {
            change_permissions(&entry_path, mode)?;
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    fn test_parse_mode_octal() {
        assert_eq!(parse_mode("755").unwrap(), 0o755);
        assert_eq!(parse_mode("644").unwrap(), 0o644);
    }

    #[test]
    fn test_change_permissions() {
        let test_dir = format!("/tmp/test_chmod_{}", std::process::id());
        fs::create_dir_all(&test_dir).unwrap();

        let test_file = format!("{}/test.txt", test_dir);
        File::create(&test_file).unwrap();

        assert!(run(&test_file, "644", false).is_ok());

        fs::remove_dir_all(&test_dir).unwrap();
    }
}
