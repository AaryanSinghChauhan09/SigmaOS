//! mv - move (rename) files
//! POSIX-compatible mv implementation

use std::fs;
use std::path::Path;

/// Move/rename file or directory
pub fn run(src: &str, dst: &str) -> Result<(), String> {
    let src_path = Path::new(src);
    let dst_path = Path::new(dst);

    if !src_path.exists() {
        return Err(format!("mv: cannot stat '{}': No such file or directory", src));
    }

    fs::rename(src_path, dst_path).map_err(|e| format!("mv: {}", e))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_move_file() {
        let test_dir = format!("/tmp/test_mv_{}", std::process::id());
        let _ = fs::remove_dir_all(&test_dir);
        fs::create_dir_all(&test_dir).unwrap();

        let src_file = format!("{}/test_src.txt", test_dir);
        let dst_file = format!("{}/test_dst.txt", test_dir);

        let mut file = File::create(&src_file).unwrap();
        file.write_all(b"test content").unwrap();

        assert!(run(&src_file, &dst_file).is_ok());
        assert!(!Path::new(&src_file).exists());
        assert!(Path::new(&dst_file).exists());

        fs::remove_dir_all(&test_dir).unwrap();
    }
}
