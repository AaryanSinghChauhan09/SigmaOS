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

/// Bolt performance optimization: zero-allocation case-insensitive substring search.
/// Eliminates intermediate `String` heap allocations on every scanned line during case-insensitive grep.
pub fn contains_ignore_case(haystack: &str, needle: &str) -> bool {
    if needle.is_empty() {
        return true;
    }
    if needle.len() > haystack.len() {
        return false;
    }
    let needle_bytes = needle.as_bytes();
    haystack
        .as_bytes()
        .windows(needle_bytes.len())
        .any(|window| window.iter().zip(needle_bytes).all(|(&h, &n)| h.eq_ignore_ascii_case(&n)))
}

pub fn run(pattern: &str, paths: &[String], options: &GrepOptions) -> Result<usize, String> {
    let mut total_matches = 0;

    if paths.is_empty() || (paths.len() == 1 && paths[0] == "-") {
        let stdin = io::stdin();
        let reader = stdin.lock();
        let matches = match_reader(reader, pattern, options, None)?;
        total_matches += matches;
    } else {
        for path in paths {
            let file = File::open(Path::new(path)).map_err(|e| format!("grep: {}: {}", path, e))?;
            let reader = BufReader::new(file);
            let show_filename = paths.len() > 1;
            let matches = match_reader(reader, pattern, options, if show_filename { Some(path) } else { None })?;
            total_matches += matches;
        }
    }

    Ok(total_matches)
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
        let is_match = if options.ignore_case {
            contains_ignore_case(&line, pattern)
        } else {
            line.contains(pattern)
        };
        let passes = if options.invert_match { !is_match } else { is_match };

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
    fn test_contains_ignore_case() {
        assert!(contains_ignore_case("SigmaOS Linux Kernel", "sigmaos"));
        assert!(contains_ignore_case("SigmaOS Linux Kernel", "KERNEL"));
        assert!(contains_ignore_case("SigmaOS Linux Kernel", "Linux"));
        assert!(contains_ignore_case("anything", ""));
        assert!(!contains_ignore_case("Short", "LongerPattern"));
        assert!(!contains_ignore_case("SigmaOS", "FreeBSD"));
    }

    #[test]
    fn test_match_reader_case_insensitive() {
        let text = "Line 1: SigmaOS\nLine 2: linux\nLine 3: SIGMAOS\n";
        let reader = Cursor::new(text);
        let opts = GrepOptions {
            ignore_case: true,
            invert_match: false,
            line_number: false,
            count_only: true,
        };

        let matches = match_reader(reader, "sigmaos", &opts, None).unwrap();
        assert_eq!(matches, 2);
    }

    #[test]
    fn test_match_reader_invert_match() {
        let text = "Line 1: SigmaOS\nLine 2: linux\nLine 3: SIGMAOS\n";
        let reader = Cursor::new(text);
        let opts = GrepOptions {
            ignore_case: true,
            invert_match: true,
            line_number: false,
            count_only: true,
        };

        let matches = match_reader(reader, "sigmaos", &opts, None).unwrap();
        assert_eq!(matches, 1);
    }
}
