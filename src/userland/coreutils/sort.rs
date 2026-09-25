//! sort - sort lines of text files
//! POSIX, GNU & BSD compatible sort implementation

use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::path::Path;

/// Options for sort execution
#[derive(Debug, Clone, Copy, Default)]
pub struct SortOptions {
    pub reverse: bool,
    pub unique: bool,
    pub ignore_case: bool,
}

/// Sort execution entrypoint
pub fn run(paths: &[String], opts: SortOptions) -> Result<String, String> {
    let mut all_lines = Vec::new();

    for path in paths {
        if path == "-" {
            let stdin = io::stdin();
            let mut handle = stdin.lock();
            read_lines(&mut handle, &mut all_lines)?;
        } else {
            let path_obj = Path::new(path);
            let file = File::open(path_obj).map_err(|e| format!("sort: {}: {}", path, e))?;
            let mut reader = BufReader::new(file);
            read_lines(&mut reader, &mut all_lines)?;
        }
    }

    all_lines.sort_by(|a, b| {
        if opts.ignore_case {
            a.to_lowercase().cmp(&b.to_lowercase())
        } else {
            a.cmp(b)
        }
    });

    if opts.reverse {
        all_lines.reverse();
    }

    if opts.unique {
        all_lines.dedup();
    }

    let mut output = String::new();
    for line in all_lines {
        output.push_str(&line);
        output.push('\n');
    }

    Ok(output)
}

fn read_lines<R: Read>(reader: &mut R, lines: &mut Vec<String>) -> Result<(), String> {
    let buf_reader = BufReader::new(reader);
    for line_res in buf_reader.lines() {
        let line = line_res.map_err(|e| format!("sort: {}", e))?;
        lines.push(line);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_sort_lines() {
        let input = "banana\napple\ncherry\n";
        let mut cursor = Cursor::new(input.as_bytes());
        let mut lines = Vec::new();
        read_lines(&mut cursor, &mut lines).unwrap();
        lines.sort();

        assert_eq!(lines, vec!["apple", "banana", "cherry"]);
    }
}
