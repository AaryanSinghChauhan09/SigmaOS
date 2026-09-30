//! cp - copy files and directories
//! POSIX-compatible cp implementation

use std::fs;
use std::io::{Read, Write};
use std::path::Path;

/// Copy file from source to destination
pub fn run(src: &str, dst: &str, recursive: bool, preserve: bool) -> Result<(), String> {
    let src_path = Path::new(src);
    let dst_path = Path::new(dst);

    if !src_path.exists() {
        return Err(format!("cp: cannot stat '{}': No such file or directory", src));
    }

    if src_path.is_dir() {
        if !recursive {
            return Err(format!("cp: -r not specified; omitting directory '{}'", src));
        }
        copy_directory(src_path, dst_path, preserve)
    } else {
        copy_file(src_path, dst_path, preserve)
    }
}

/// Copy a single file
fn copy_file(src: &Path, dst: &Path, preserve: bool) -> Result<(), String> {
    let src_metadata = fs::metadata(src).map_err(|e| format!("cp: {}", e))?;

    let mut src_file = fs::File::open(src).map_err(|e| format!("cp: {}", e))?;
    let mut dst_file = fs::File::create(dst).map_err(|e| format!("cp: {}", e))?;

    let mut buffer = [0u8; 8192];
    loop {
        let n = src_file.read(&mut buffer).map_err(|e| format!("cp: {}", e))?;
        if n == 0 {
            break;
        }
        dst_file.write_all(&buffer[..n]).map_err(|e| format!("cp: {}", e))?;
    }

    if preserve {
        // Preserve permissions and timestamps
        if let Err(e) = fs::set_permissions(dst, src_metadata.permissions()) {
            return Err(format!("cp: failed to preserve permissions: {}", e));
        }
    }

    Ok(())
}

/// Copy directory recursively
fn copy_directory(src: &Path, dst: &Path, preserve: bool) -> Result<(), String> {
    if !dst.exists() {
        fs::create_dir_all(dst).map_err(|e| format!("cp: {}", e))?;
    }

    let src_metadata = fs::metadata(src).map_err(|e| format!("cp: {}", e))?;

    for entry in fs::read_dir(src).map_err(|e| format!("cp: {}", e))? {
        let entry = entry.map_err(|e| format!("cp: {}", e))?;
        let src_entry = entry.path();
        let dst_entry = dst.join(entry.file_name());

        if src_entry.is_dir() {
            copy_directory(&src_entry, &dst_entry, preserve)?;
        } else {
            copy_file(&src_entry, &dst_entry, preserve)?;
        }
    }

    if preserve {
        if let Err(e) = fs::set_permissions(dst, src_metadata.permissions()) {
            return Err(format!("cp: failed to preserve permissions: {}", e));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_copy_file() {
        let test_dir = format!("/tmp/test_cp_{}", std::process::id());
        let _ = fs::remove_dir_all(&test_dir);
        fs::create_dir_all(&test_dir).unwrap();

        let src_file = format!("{}/test_src.txt", test_dir);
        let dst_file = format!("{}/test_dst.txt", test_dir);

        let mut file = File::create(&src_file).unwrap();
        file.write_all(b"test content").unwrap();

        assert!(run(&src_file, &dst_file, false, false).is_ok());
        assert!(Path::new(&dst_file).exists());

        fs::remove_dir_all(&test_dir).unwrap();
    }
}
