//! uniq - report or omit repeated lines
//! POSIX, GNU & BSD compatible uniq implementation

use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::path::Path;

/// Options for uniq execution
#[derive(Debug, Clone, Copy, Default)]
pub struct UniqOptions {
    pub count: bool,
    pub repeated_only: bool,
    pub unique_only: bool,
    pub ignore_case: bool,
}

/// Uniq execution entrypoint
pub fn run(input_path: Option<&str>, opts: UniqOptions) -> Result<String, String> {
    let content = if let Some(path) = input_path {
        if path == "-" {
            let stdin = io::stdin();
            let mut handle = stdin.lock();
            process_reader(&mut handle, opts)?
        } else {
            let path_obj = Path::new(path);
            let file = File::open(path_obj).map_err(|e| format!("uniq: {}: {}", path, e))?;
            let mut reader = BufReader::new(file);
            process_reader(&mut reader, opts)?
        }
    } else {
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        process_reader(&mut handle, opts)?
    };

    Ok(content)
}

/// Filter repeated lines from reader
pub fn process_reader<R: Read>(reader: &mut R, opts: UniqOptions) -> Result<String, String> {
    let buf_reader = BufReader::new(reader);
    let mut lines = Vec::new();

    for line_res in buf_reader.lines() {
        let line = line_res.map_err(|e| format!("uniq: {}", e))?;
        lines.push(line);
    }

    let mut output = String::new();
    if lines.is_empty() {
        return Ok(output);
    }

    let mut current_line = &lines[0];
    let mut count = 1;

    for line in &lines[1..] {
        let matches = if opts.ignore_case {
            current_line.eq_ignore_ascii_case(line)
        } else {
            current_line == line
        };

        if matches {
            count += 1;
        } else {
            format_entry(&mut output, current_line, count, opts);
            current_line = line;
            count = 1;
        }
    }

    format_entry(&mut output, current_line, count, opts);

    Ok(output)
}

/// Format individual line entry based on options
fn format_entry(output: &mut String, line: &str, count: usize, opts: UniqOptions) {
    let is_repeated = count > 1;

    if opts.repeated_only && !is_repeated {
        return;
    }
    if opts.unique_only && is_repeated {
        return;
    }

    if opts.count {
        output.push_str(&format!("{:7} {}\n", count, line));
    } else {
        output.push_str(line);
        output.push('\n');
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_uniq_basic() {
        let input = "alpha\nalpha\nbeta\nbeta\nbeta\ngamma\n";
        let mut cursor = Cursor::new(input.as_bytes());
        let opts = UniqOptions::default();

        let result = process_reader(&mut cursor, opts).unwrap();
        assert_eq!(result, "alpha\nbeta\ngamma\n");
    }

    #[test]
    fn test_uniq_count() {
        let input = "alpha\nalpha\nbeta\n";
        let mut cursor = Cursor::new(input.as_bytes());
        let opts = UniqOptions {
            count: true,
            ..Default::default()
        };

        let result = process_reader(&mut cursor, opts).unwrap();
        assert_eq!(result, "      2 alpha\n      1 beta\n");
    }
}
