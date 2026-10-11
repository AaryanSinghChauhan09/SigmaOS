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
            let matches = match_reader(reader, &pat, options, if show_filename { Some(path) } else { None })?;
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
        let check_line = if options.ignore_case {
            line.to_lowercase()
        } else {
            line.clone()
        };

        let is_match = check_line.contains(pattern);
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
