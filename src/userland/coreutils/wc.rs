//! wc - print newline, word, and byte counts for files
//! POSIX, GNU & BSD compatible word count implementation

use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::path::Path;

/// Options for wc execution
#[derive(Debug, Clone, Copy)]
pub struct WcOptions {
    pub lines: bool,
    pub words: bool,
    pub chars: bool,
    pub bytes: bool,
}

impl Default for WcOptions {
    fn default() -> Self {
        Self {
            lines: true,
            words: true,
            chars: false,
            bytes: true,
        }
    }
}

/// Statistics calculated by wc
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct WcStats {
    pub lines: usize,
    pub words: usize,
    pub chars: usize,
    pub bytes: usize,
}

/// Wc execution entrypoint
pub fn run(paths: &[String], opts: WcOptions) -> Result<String, String> {
    let mut output = String::new();
    let mut total_stats = WcStats::default();
    let multi_files = paths.len() > 1;

    for path in paths {
        let stats = if path == "-" {
            let stdin = io::stdin();
            let mut handle = stdin.lock();
            count_reader(&mut handle)?
        } else {
            let path_obj = Path::new(path);
            let file = File::open(path_obj).map_err(|e| format!("wc: {}: {}", path, e))?;
            let mut reader = BufReader::new(file);
            count_reader(&mut reader)?
        };

        total_stats.lines += stats.lines;
        total_stats.words += stats.words;
        total_stats.chars += stats.chars;
        total_stats.bytes += stats.bytes;

        output.push_str(&format_stats(&stats, opts, Some(path)));
    }

    if multi_files {
        output.push_str(&format_stats(&total_stats, opts, Some("total")));
    }

    Ok(output)
}

/// Count lines, words, chars, and bytes from a reader
pub fn count_reader<R: Read>(reader: &mut R) -> Result<WcStats, String> {
    let mut buf_reader = BufReader::new(reader);
    let mut stats = WcStats::default();
    let mut line_buf = String::new();

    loop {
        line_buf.clear();
        let bytes_read = buf_reader.read_line(&mut line_buf).map_err(|e| format!("wc: {}", e))?;
        if bytes_read == 0 {
            break;
        }

        stats.bytes += bytes_read;
        stats.chars += line_buf.chars().count();
        if line_buf.ends_with('\n') {
            stats.lines += 1;
        }
        stats.words += line_buf.split_whitespace().count();
    }

    Ok(stats)
}

/// Format statistics string according to options
fn format_stats(stats: &WcStats, opts: WcOptions, path: Option<&str>) -> String {
    let mut line = String::new();

    if opts.lines {
        line.push_str(&format!("{:8} ", stats.lines));
    }
    if opts.words {
        line.push_str(&format!("{:8} ", stats.words));
    }
    if opts.chars {
        line.push_str(&format!("{:8} ", stats.chars));
    }
    if opts.bytes {
        line.push_str(&format!("{:8} ", stats.bytes));
    }

    if let Some(p) = path {
        line.push_str(p);
    }
    line.push('\n');
    line
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_wc_counts() {
        let input = "hello world\nthis is sigmaos\n";
        let mut cursor = Cursor::new(input.as_bytes());
        let stats = count_reader(&mut cursor).unwrap();

        assert_eq!(stats.lines, 2);
        assert_eq!(stats.words, 5);
        assert_eq!(stats.bytes, input.len());
    }
}
