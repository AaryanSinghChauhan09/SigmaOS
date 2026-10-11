//! awk - pattern scanning and processing language
//! POSIX-compatible awk utility

use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct AwkConfig {
    pub field_separator: char,
    pub print_field_idx: Option<usize>, // 0 for all, 1 for $1, etc.
}

impl Default for AwkConfig {
    fn default() -> Self {
        Self {
            field_separator: ' ',
            print_field_idx: None,
        }
    }
}

pub fn run(config: &AwkConfig, paths: &[String]) -> Result<(), String> {
    if paths.is_empty() || (paths.len() == 1 && paths[0] == "-") {
        let stdin = io::stdin();
        let stdout = io::stdout();
        process_awk(stdin.lock(), &mut stdout.lock(), config)
    } else {
        for path in paths {
            let file = File::open(Path::new(path)).map_err(|e| format!("awk: {}: {}", path, e))?;
            let stdout = io::stdout();
            process_awk(BufReader::new(file), &mut stdout.lock(), config)?;
        }
        Ok(())
    }
}

fn process_awk<R: BufRead, W: Write>(
    reader: R,
    writer: &mut W,
    config: &AwkConfig,
) -> Result<(), String> {
    let mut nr = 0;

    for line_res in reader.lines() {
        let line = line_res.map_err(|e| format!("awk read error: {}", e))?;
        nr += 1;

        let fields: Vec<&str> = if config.field_separator == ' ' {
            line.split_whitespace().collect()
        } else {
            line.split(config.field_separator).collect()
        };

        match config.print_field_idx {
            Some(0) => {
                writeln!(writer, "{}", line).map_err(|e| format!("awk write error: {}", e))?;
            }
            Some(idx) => {
                if idx <= fields.len() {
                    writeln!(writer, "{}", fields[idx - 1])
                        .map_err(|e| format!("awk write error: {}", e))?;
                }
            }
            None => {
                writeln!(writer, "[NR={}] {}", nr, line)
                    .map_err(|e| format!("awk write error: {}", e))?;
            }
        }
    }

    Ok(())
}
