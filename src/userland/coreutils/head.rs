//! head - output the first part of files
//! POSIX, GNU & BSD compatible head implementation

use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::path::Path;

/// Options for head execution
#[derive(Debug, Clone, Copy)]
pub struct HeadOptions {
    pub lines: usize,
    pub bytes: Option<usize>,
}

impl Default for HeadOptions {
    fn default() -> Self {
        Self {
            lines: 10,
            bytes: None,
        }
    }
}

/// Head execution entrypoint
pub fn run(paths: &[String], opts: HeadOptions) -> Result<String, String> {
    let mut output = String::new();
    let multi_files = paths.len() > 1;

    for (idx, path) in paths.iter().enumerate() {
        if multi_files {
            if idx > 0 {
                output.push('\n');
            }
            output.push_str(&format!("==> {} <==\n", path));
        }

        if path == "-" {
            let stdin = io::stdin();
            let mut handle = stdin.lock();
            let content = process_reader(&mut handle, opts)?;
            output.push_str(&content);
        } else {
            let path_obj = Path::new(path);
            let file = File::open(path_obj).map_err(|e| format!("head: {}: {}", path, e))?;
            let mut reader = BufReader::new(file);
            let content = process_reader(&mut reader, opts)?;
            output.push_str(&content);
        }
    }

    Ok(output)
}

/// Process reader according to head options (lines or bytes)
fn process_reader<R: Read>(reader: &mut R, opts: HeadOptions) -> Result<String, String> {
    if let Some(num_bytes) = opts.bytes {
        let mut buffer = vec![0u8; num_bytes];
        let bytes_read = reader.read(&mut buffer).map_err(|e| format!("head: {}", e))?;
        Ok(String::from_utf8_lossy(&buffer[..bytes_read]).to_string())
    } else {
        let buf_reader = BufReader::new(reader);
        let mut result = String::new();
        for (i, line_res) in buf_reader.lines().enumerate() {
            if i >= opts.lines {
                break;
            }
            let line = line_res.map_err(|e| format!("head: {}", e))?;
            result.push_str(&line);
            result.push('\n');
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_head_lines() {
        let input = "line 1\nline 2\nline 3\nline 4\nline 5\n";
        let mut cursor = Cursor::new(input.as_bytes());
        let opts = HeadOptions {
            lines: 3,
            bytes: None,
        };
        let res = process_reader(&mut cursor, opts).unwrap();
        assert_eq!(res, "line 1\nline 2\nline 3\n");
    }

    #[test]
    fn test_head_bytes() {
        let input = "hello world from sigmaos";
        let mut cursor = Cursor::new(input.as_bytes());
        let opts = HeadOptions {
            lines: 10,
            bytes: Some(5),
        };
        let res = process_reader(&mut cursor, opts).unwrap();
        assert_eq!(res, "hello");
    }
}
