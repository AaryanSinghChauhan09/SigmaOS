//! comm - compare two sorted files line by line
//! POSIX, GNU & BSD compatible comm implementation

use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::path::Path;

/// Options for comm execution to suppress specific output columns
#[derive(Debug, Clone, Copy, Default)]
pub struct CommOptions {
    pub suppress_col1: bool,
    pub suppress_col2: bool,
    pub suppress_col3: bool,
}

/// Compare two sorted files line by line
pub fn run(file1_path: &str, file2_path: &str, opts: CommOptions) -> Result<String, String> {
    let lines1 = read_file_lines(file1_path)?;
    let lines2 = read_file_lines(file2_path)?;

    let mut i = 0;
    let mut j = 0;
    let mut output = String::new();

    while i < lines1.len() && j < lines2.len() {
        if lines1[i] == lines2[j] {
            if !opts.suppress_col3 {
                let prefix = if opts.suppress_col1 && opts.suppress_col2 {
                    ""
                } else if opts.suppress_col1 || opts.suppress_col2 {
                    "\t"
                } else {
                    "\t\t"
                };
                output.push_str(&format!("{}{}\n", prefix, lines1[i]));
            }
            i += 1;
            j += 1;
        } else if lines1[i] < lines2[j] {
            if !opts.suppress_col1 {
                output.push_str(&format!("{}\n", lines1[i]));
            }
            i += 1;
        } else {
            if !opts.suppress_col2 {
                let prefix = if opts.suppress_col1 { "" } else { "\t" };
                output.push_str(&format!("{}{}\n", prefix, lines2[j]));
            }
            j += 1;
        }
    }

    while i < lines1.len() {
        if !opts.suppress_col1 {
            output.push_str(&format!("{}\n", lines1[i]));
        }
        i += 1;
    }

    while j < lines2.len() {
        if !opts.suppress_col2 {
            let prefix = if opts.suppress_col1 { "" } else { "\t" };
            output.push_str(&format!("{}{}\n", prefix, lines2[j]));
        }
        j += 1;
    }

    Ok(output)
}

fn read_file_lines(path: &str) -> Result<Vec<String>, String> {
    if path == "-" {
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        read_reader_lines(&mut handle)
    } else {
        let path_obj = Path::new(path);
        let file = File::open(path_obj).map_err(|e| format!("comm: {}: {}", path, e))?;
        let mut reader = BufReader::new(file);
        read_reader_lines(&mut reader)
    }
}

fn read_reader_lines<R: Read>(reader: &mut R) -> Result<Vec<String>, String> {
    let buf_reader = BufReader::new(reader);
    let mut lines = Vec::new();
    for line_res in buf_reader.lines() {
        let line = line_res.map_err(|e| format!("comm: {}", e))?;
        lines.push(line);
    }
    Ok(lines)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comm_options() {
        let opts = CommOptions::default();
        assert!(!opts.suppress_col1);
    }
}
