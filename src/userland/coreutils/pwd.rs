//! pwd - Print working directory
//!
//! Print the name of the current working directory.

use std::env;
use std::io::{self, Write};

fn main() {
    match run() {
        Ok(()) => std::process::exit(0),
        Err(e) => {
            eprintln!("pwd: error: {}", e);
            std::process::exit(1);
        }
    }
}

fn run() -> Result<(), String> {
    let current_dir = env::current_dir()
        .map_err(|e| format!("failed to get current directory: {}", e))?;
    
    let path = current_dir.to_str()
        .ok_or_else(|| "working directory contains invalid UTF-8".to_string())?;
    
    println!("{}", path);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pwd_prints_directory() {
        let result = run();
        assert!(result.is_ok());
    }
}
