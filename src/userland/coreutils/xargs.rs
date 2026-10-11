//! xargs - build and execute command lines from standard input
//! POSIX-compatible xargs utility

use std::io::{self, BufRead};

#[derive(Debug, Clone, Default)]
pub struct XargsOptions {
    pub max_args: Option<usize>,
    pub delimiter: Option<char>,
}

pub fn run<F>(options: &XargsOptions, mut command_fn: F) -> Result<(), String>
where
    F: FnMut(&[String]) -> Result<(), String>,
{
    let stdin = io::stdin();
    let reader = stdin.lock();

    let mut current_batch = Vec::new();

    for line_res in reader.lines() {
        let line = line_res.map_err(|e| format!("xargs read error: {}", e))?;

        let items: Vec<&str> = match options.delimiter {
            Some(d) => line.split(d).collect(),
            None => line.split_whitespace().collect(),
        };

        for item in items {
            let trimmed = item.trim();
            if !trimmed.is_empty() {
                current_batch.push(trimmed.to_string());

                if let Some(max) = options.max_args {
                    if current_batch.len() >= max {
                        command_fn(&current_batch)?;
                        current_batch.clear();
                    }
                }
            }
        }
    }

    if !current_batch.is_empty() {
        command_fn(&current_batch)?;
    }

    Ok(())
}
