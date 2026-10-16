//! grep - print lines that match patterns
//! POSIX-compatible grep utility

use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

#[derive(Debug, Clone, Default)]
pub struct GrepOptions {
    pub ignore_case: bool,
    pub invert_match: bool,
    pub line_number: bool,
    pub count_only: bool,
}

pub fn run(pattern: &str, paths: &[String], options: &GrepOptions) -> Result<usize, String> {
    let mut total_matches = 0;
    let pat = if options.ignore_case {
        pattern.to_lowercase()
    } else {
        pattern.to_string()
    };

    if paths.is_empty() || (paths.len() == 1 && paths[0] == "-") {
        let stdin = io::stdin();
        let reader = stdin.lock();
        let matches = match_reader(reader, &pat, options, None)?;
        total_matches += matches;
    } else {
        for path in paths {
            let file = File::open(Path::new(path)).map_err(|e| format!("grep: {}: {}", path, e))?;
            let reader = BufReader::new(file);
            let show_filename = paths.len() > 1;
            let matches = match_reader(
                reader,
                &pat,
                options,
                if show_filename { Some(path) } else { None },
            )?;
            total_matches += matches;
        }
    }

    Ok(total_matches)
}

/// Zero-allocation case-insensitive string matching helper for line scanning.
/// For ASCII content (the vast majority of files and logs), window slice comparisons
/// are performed in-place without allocating temporary heap `String` instances.
fn contains_ignore_case(haystack: &str, needle: &str) -> bool {
    if needle.is_empty() {
        return true;
    }
    if haystack.len() < needle.len() {
        return false;
    }
    if haystack.is_ascii() && needle.is_ascii() {
        let needle_bytes = needle.as_bytes();
        haystack.as_bytes().windows(needle_bytes.len()).any(|window| {
            window
                .iter()
                .zip(needle_bytes.iter())
                .all(|(&h, &n)| h.to_ascii_lowercase() == n.to_ascii_lowercase())
        })
    } else {
        haystack.to_lowercase().contains(&needle.to_lowercase())
    }
}

fn match_reader<R: BufRead>(
    reader: R,
    pattern: &str,
    options: &GrepOptions,
    filename: Option<&str>,
) -> Result<usize, String> {
    let mut match_count = 0;

    for (line_idx, line_res) in reader.lines().enumerate() {
        let line = line_res.map_err(|e| format!("grep read error: {}", e))?;
        // Optimization: Avoid allocating a heap String (`line.clone()` or `line.to_lowercase()`) for every line read.
        let is_match = if options.ignore_case {
            contains_ignore_case(&line, pattern)
        } else {
            line.contains(pattern)
        };

        let passes = if options.invert_match {
            !is_match
        } else {
            is_match
        };

        if passes {
            match_count += 1;
            if !options.count_only {
                let mut prefix = String::new();
                if let Some(f) = filename {
                    prefix.push_str(&format!("{}:", f));
                }
                if options.line_number {
                    prefix.push_str(&format!("{}:", line_idx + 1));
                }
                println!("{}{}", prefix, line);
            }
        }
    }

    if options.count_only {
        if let Some(f) = filename {
            println!("{}:{}", f, match_count);
        } else {
            println!("{}", match_count);
        }
    }

    Ok(match_count)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_contains_ignore_case_ascii() {
        assert!(contains_ignore_case("Hello World", "world"));
        assert!(contains_ignore_case("Hello World", "World"));
        assert!(contains_ignore_case("SIGMAOS LINUX", "sigmaos"));
        assert!(!contains_ignore_case("Alpha Beta", "gamma"));
    }

    #[test]
    fn test_match_reader_case_sensitive() {
        let data = "line 1\nLine 2\nline 3\n";
        let cursor = Cursor::new(data);
        let opts = GrepOptions {
            count_only: true,
            ..Default::default()
        };
        let matches = match_reader(cursor, "line", &opts, None).unwrap();
        assert_eq!(matches, 2);
    }

    #[test]
    fn test_match_reader_case_insensitive() {
        let data = "line 1\nLine 2\nLINE 3\n";
        let cursor = Cursor::new(data);
        let opts = GrepOptions {
            ignore_case: true,
            count_only: true,
            ..Default::default()
        };
        let matches = match_reader(cursor, "line", &opts, None).unwrap();
        assert_eq!(matches, 3);
    }

    #[test]
    fn test_match_reader_invert_match() {
        let data = "apple\nbanana\ncherry\n";
        let cursor = Cursor::new(data);
        let opts = GrepOptions {
            invert_match: true,
            count_only: true,
            ..Default::default()
        };
        let matches = match_reader(cursor, "banana", &opts, None).unwrap();
        assert_eq!(matches, 2);
    }
}
