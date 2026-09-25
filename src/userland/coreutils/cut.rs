//! cut - remove sections from each line of files
//! POSIX, GNU & BSD compatible cut implementation

use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::path::Path;

/// Options for cut execution
#[derive(Debug, Clone)]
pub struct CutOptions {
    pub delimiter: char,
    pub fields: Vec<usize>,
    pub characters: Vec<usize>,
}

impl Default for CutOptions {
    fn default() -> Self {
        Self {
            delimiter: '\t',
            fields: Vec::new(),
            characters: Vec::new(),
        }
    }
}

/// Cut execution entrypoint
pub fn run(paths: &[String], opts: CutOptions) -> Result<String, String> {
    let mut output = String::new();

    for path in paths {
        if path == "-" {
            let stdin = io::stdin();
            let mut handle = stdin.lock();
            let res = process_reader(&mut handle, &opts)?;
            output.push_str(&res);
        } else {
            let path_obj = Path::new(path);
            let file = File::open(path_obj).map_err(|e| format!("cut: {}: {}", path, e))?;
            let mut reader = BufReader::new(file);
            let res = process_reader(&mut reader, &opts)?;
            output.push_str(&res);
        }
    }

    Ok(output)
}

/// Process lines from reader based on fields or characters
pub fn process_reader<R: Read>(reader: &mut R, opts: &CutOptions) -> Result<String, String> {
    let buf_reader = BufReader::new(reader);
    let mut result = String::new();

    for line_res in buf_reader.lines() {
        let line = line_res.map_err(|e| format!("cut: {}", e))?;

        if !opts.fields.is_empty() {
            let parts: Vec<&str> = line.split(opts.delimiter).collect();
            let mut selected = Vec::new();
            for &f in &opts.fields {
                if f > 0 && f <= parts.len() {
                    selected.push(parts[f - 1]);
                }
            }
            result.push_str(&selected.join(&opts.delimiter.to_string()));
            result.push('\n');
        } else if !opts.characters.is_empty() {
            let chars: Vec<char> = line.chars().collect();
            let mut selected = String::new();
            for &c in &opts.characters {
                if c > 0 && c <= chars.len() {
                    selected.push(chars[c - 1]);
                }
            }
            result.push_str(&selected);
            result.push('\n');
        } else {
            result.push_str(&line);
            result.push('\n');
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_cut_fields() {
        let input = "foo:bar:baz\none:two:three\n";
        let mut cursor = Cursor::new(input.as_bytes());
        let opts = CutOptions {
            delimiter: ':',
            fields: vec![1, 3],
            characters: Vec::new(),
        };

        let res = process_reader(&mut cursor, &opts).unwrap();
        assert_eq!(res, "foo:baz\none:three\n");
    }
}
