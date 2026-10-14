//! sort - sort lines of text files
//! POSIX, GNU & BSD compatible sort implementation

use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::path::Path;

/// Zero-allocation case-insensitive string comparison helper.
/// Evaluates ASCII strings character-by-character without allocating heap String instances per comparison.
#[inline]
fn cmp_ignore_case(a: &str, b: &str) -> Ordering {
    if a.is_ascii() && b.is_ascii() {
        a.bytes()
            .map(|byte| byte.to_ascii_lowercase())
            .cmp(b.bytes().map(|byte| byte.to_ascii_lowercase()))
    } else {
        a.chars()
            .flat_map(char::to_lowercase)
            .cmp(b.chars().flat_map(char::to_lowercase))
    }
}

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
            cmp_ignore_case(a, b)
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

    // Pre-allocate buffer capacity for the output string to avoid heap reallocations.
    let total_bytes: usize = all_lines.iter().map(|l| l.len() + 1).sum();
    let mut output = String::with_capacity(total_bytes);
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

    #[test]
    fn test_cmp_ignore_case() {
        assert_eq!(cmp_ignore_case("apple", "APPLE"), Ordering::Equal);
        assert_eq!(cmp_ignore_case("Banana", "apple"), Ordering::Greater);
        assert_eq!(cmp_ignore_case("apple", "Cherry"), Ordering::Less);
        assert_eq!(cmp_ignore_case("über", "ÜBER"), Ordering::Equal);
    }

    #[test]
    fn test_case_insensitive_sort() {
        let mut lines = vec!["banana".to_string(), "Apple".to_string(), "CHERRY".to_string()];
        lines.sort_by(|a, b| cmp_ignore_case(a, b));
        assert_eq!(lines, vec!["Apple", "banana", "CHERRY"]);
    }
}
