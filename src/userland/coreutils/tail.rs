//! tail - output the last part of files
//! POSIX, GNU & BSD compatible tail implementation

use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::path::Path;

/// Options for tail execution
#[derive(Debug, Clone, Copy)]
pub struct TailOptions {
    pub lines: usize,
    pub bytes: Option<usize>,
}

impl Default for TailOptions {
    fn default() -> Self {
        Self {
            lines: 10,
            bytes: None,
        }
    }
}

/// Tail execution entrypoint
pub fn run(paths: &[String], opts: TailOptions) -> Result<String, String> {
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
            let file = File::open(path_obj).map_err(|e| format!("tail: {}: {}", path, e))?;
            let mut reader = BufReader::new(file);
            let content = process_reader(&mut reader, opts)?;
            output.push_str(&content);
        }
    }

    Ok(output)
}

/// Process reader according to tail options (last N lines or bytes)
fn process_reader<R: Read>(reader: &mut R, opts: TailOptions) -> Result<String, String> {
    if let Some(num_bytes) = opts.bytes {
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer).map_err(|e| format!("tail: {}", e))?;
        let start = if buffer.len() > num_bytes {
            buffer.len() - num_bytes
        } else {
            0
        };
        Ok(String::from_utf8_lossy(&buffer[start..]).to_string())
    } else {
        let buf_reader = BufReader::new(reader);
        let lines: Vec<String> = buf_reader
            .lines()
            .collect::<Result<_, _>>()
            .map_err(|e| format!("tail: {}", e))?;

        let start = if lines.len() > opts.lines {
            lines.len() - opts.lines
        } else {
            0
        };

        let mut result = String::new();
        for line in &lines[start..] {
            result.push_str(line);
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
    fn test_tail_lines() {
        let input = "line 1\nline 2\nline 3\nline 4\nline 5\n";
        let mut cursor = Cursor::new(input.as_bytes());
        let opts = TailOptions {
            lines: 2,
            bytes: None,
        };
        let res = process_reader(&mut cursor, opts).unwrap();
        assert_eq!(res, "line 4\nline 5\n");
    }

    #[test]
    fn test_tail_bytes() {
        let input = "hello world from sigmaos";
        let mut cursor = Cursor::new(input.as_bytes());
        let opts = TailOptions {
            lines: 10,
            bytes: Some(7),
        };
        let res = process_reader(&mut cursor, opts).unwrap();
        assert_eq!(res, "sigmaos");
    }
}
