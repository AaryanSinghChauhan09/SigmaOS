//! tee - read from standard input and write to standard output and files
//! POSIX, GNU & BSD compatible tee implementation

use std::fs::OpenOptions;
use std::io::{self, Read, Write};
use std::path::Path;

/// Options for tee execution
#[derive(Debug, Clone, Copy, Default)]
pub struct TeeOptions {
    pub append: bool,
}

/// Tee execution entrypoint
pub fn run(file_paths: &[String], opts: TeeOptions) -> Result<String, String> {
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    let mut files = Vec::new();
    for path in file_paths {
        let path_obj = Path::new(path);
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(opts.append)
            .truncate(!opts.append)
            .open(path_obj)
            .map_err(|e| format!("tee: {}: {}", path, e))?;
        files.push(file);
    }

    let mut buffer = [0u8; 8192];
    let mut captured_output = Vec::new();

    loop {
        let bytes_read = handle.read(&mut buffer).map_err(|e| format!("tee: {}", e))?;
        if bytes_read == 0 {
            break;
        }

        let chunk = &buffer[..bytes_read];
        captured_output.extend_from_slice(chunk);

        for file in &mut files {
            file.write_all(chunk).map_err(|e| format!("tee: {}", e))?;
        }
    }

    Ok(String::from_utf8_lossy(&captured_output).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tee_options_default() {
        let opts = TeeOptions::default();
        assert!(!opts.append);
    }
}
