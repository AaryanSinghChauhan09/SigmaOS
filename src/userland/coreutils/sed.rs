//! sed - stream editor for filtering and transforming text
//! POSIX-compatible sed utility

use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

#[derive(Debug, Clone)]
pub enum SedCommand {
    Substitute {
        pattern: String,
        replacement: String,
        global: bool,
    },
    DeletePattern(String),
}

impl SedCommand {
    pub fn parse(expr: &str) -> Result<Self, String> {
        let trimmed = expr.trim();
        if trimmed.starts_with('s') && trimmed.len() >= 4 {
            let delimiter = trimmed.chars().nth(1).unwrap();
            let parts: Vec<&str> = trimmed[2..].split(delimiter).collect();
            if parts.len() >= 2 {
                let pattern = parts[0].to_string();
                let replacement = parts[1].to_string();
                let flags = parts.get(2).unwrap_or(&"");
                let global = flags.contains('g');
                return Ok(SedCommand::Substitute {
                    pattern,
                    replacement,
                    global,
                });
            }
        } else if trimmed.ends_with('d') {
            let pattern = trimmed[..trimmed.len() - 1].trim_matches('/').to_string();
            return Ok(SedCommand::DeletePattern(pattern));
        }

        Err(format!("sed: invalid command '{}'", expr))
    }
}

pub fn run(command: &SedCommand, paths: &[String]) -> Result<(), String> {
    if paths.is_empty() || (paths.len() == 1 && paths[0] == "-") {
        let stdin = io::stdin();
        let stdout = io::stdout();
        process_lines(stdin.lock(), &mut stdout.lock(), command)
    } else {
        for path in paths {
            let file = File::open(Path::new(path)).map_err(|e| format!("sed: {}: {}", path, e))?;
            let stdout = io::stdout();
            process_lines(BufReader::new(file), &mut stdout.lock(), command)?;
        }
        Ok(())
    }
}

fn process_lines<R: BufRead, W: Write>(
    reader: R,
    writer: &mut W,
    command: &SedCommand,
) -> Result<(), String> {
    for line_res in reader.lines() {
        let line = line_res.map_err(|e| format!("sed read error: {}", e))?;

        match command {
            SedCommand::Substitute {
                pattern,
                replacement,
                global,
            } => {
                let out = if *global {
                    line.replace(pattern, replacement)
                } else {
                    line.replacen(pattern, replacement, 1)
                };
                writeln!(writer, "{}", out).map_err(|e| format!("sed write error: {}", e))?;
            }
            SedCommand::DeletePattern(pattern) => {
                if !line.contains(pattern) {
                    writeln!(writer, "{}", line).map_err(|e| format!("sed write error: {}", e))?;
                }
            }
        }
    }
    Ok(())
}
