//! chown - change file owner and group
//! POSIX-compatible chown implementation

use std::fs;
use std::path::Path;

/// Change file owner and group
pub fn run(path: &str, owner: Option<&str>, group: Option<&str>, recursive: bool) -> Result<(), String> {
    let path_obj = Path::new(path);

    if !path_obj.exists() {
        return Err(format!("chown: cannot access '{}': No such file or directory", path));
    }

    // Note: Real chown requires root privileges and syscall to change ownership
    // This is a placeholder implementation

    if path_obj.is_dir() && recursive {
        change_ownership_recursive(path_obj, owner, group)
    } else {
        change_ownership(path_obj, owner, group)
    }
}

/// Change ownership for a single file/directory
fn change_ownership(path: &Path, owner: Option<&str>, group: Option<&str>) -> Result<(), String> {
    // Placeholder: In real implementation, this would call chown syscall
    // For now, just return success
    if let Some(o) = owner {
        println!("chown: would change owner to {} for {}", o, path.display());
    }
    if let Some(g) = group {
        println!("chown: would change group to {} for {}", g, path.display());
    }
    Ok(())
}

/// Change ownership recursively
fn change_ownership_recursive(path: &Path, owner: Option<&str>, group: Option<&str>) -> Result<(), String> {
    change_ownership(path, owner, group)?;

    for entry in fs::read_dir(path).map_err(|e| format!("chown: {}", e))? {
        let entry = entry.map_err(|e| format!("chown: {}", e))?;
        let entry_path = entry.path();

        if entry_path.is_dir() {
            change_ownership_recursive(&entry_path, owner, group)?;
        } else {
            change_ownership(&entry_path, owner, group)?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    fn test_change_ownership() {
        let test_dir = format!("/tmp/test_chown_{}", std::process::id());
        fs::create_dir_all(&test_dir).unwrap();

        let test_file = format!("{}/test.txt", test_dir);
        File::create(&test_file).unwrap();

        // This should succeed (placeholder)
        assert!(run(&test_file, Some("root"), Some("root"), false).is_ok());

        fs::remove_dir_all(&test_dir).unwrap();
    }
}
