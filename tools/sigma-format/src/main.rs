//! sigma-format - Native Rust code formatter for SigmaOS
//! Replaces Prettier with zero-dependency Rust implementation
//! 
//! Usage: sigma-format <file> or sigma-format --check <file>

use std::env;
use std::fs;
use std::io::{self, Read, Write};
use std::path::Path;

const VERSION: &str = "1.0.0";

#[derive(Debug)]
enum FormatError {
    Io(io::Error),
    Parse(String),
}

impl From<io::Error> for FormatError {
    fn from(err: io::Error) -> Self {
        FormatError::Io(err)
    }
}

/// Simple code formatter - basic indentation and line ending normalization
/// This is a minimal implementation that can be expanded
fn format_code(input: &str) -> String {
    let mut output = String::new();
    let mut indent_level = 0;
    const INDENT: &str = "    ";
    
    for line in input.lines() {
        let trimmed = line.trim();
        
        // Decrease indent for closing braces
        if trimmed.starts_with('}') || trimmed.starts_with(']') || trimmed.starts_with(')') {
            indent_level = indent_level.saturating_sub(1);
        }
        
        // Add indentation
        if !trimmed.is_empty() {
            for _ in 0..indent_level {
                output.push_str(INDENT);
            }
            output.push_str(trimmed);
        }
        
        output.push('\n');
        
        // Increase indent for opening braces
        if trimmed.ends_with('{') || trimmed.ends_with('[') || trimmed.ends_with('(') {
            indent_level += 1;
        }
    }
    
    output
}

fn format_file(path: &Path, check_only: bool) -> Result<bool, FormatError> {
    let mut content = String::new();
    let mut file = fs::File::open(path)?;
    file.read_to_string(&mut content)?;
    
    let formatted = format_code(&content);
    let needs_formatting = content != formatted;
    
    if check_only {
        if needs_formatting {
            eprintln!("{} needs formatting", path.display());
        }
        Ok(needs_formatting)
    } else {
        if needs_formatting {
            let mut file = fs::File::create(path)?;
            file.write_all(formatted.as_bytes())?;
            println!("Formatted {}", path.display());
        }
        Ok(needs_formatting)
    }
}

fn print_usage() {
    println!("sigma-format v{}", VERSION);
    println!();
    println!("Native Rust code formatter for SigmaOS");
    println!();
    println!("USAGE:");
    println!("  sigma-format <file>           Format file in-place");
    println!("  sigma-format --check <file>   Check if file needs formatting");
    println!("  sigma-format --version        Print version");
    println!("  sigma-format --help           Show this help");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_usage();
        std::process::exit(1);
    }
    
    match args[1].as_str() {
        "--version" => {
            println!("sigma-format {}", VERSION);
        }
        "--help" | "-h" => {
            print_usage();
        }
        "--check" => {
            if args.len() < 3 {
                eprintln!("Error: --check requires a file argument");
                std::process::exit(1);
            }
            let path = Path::new(&args[2]);
            match format_file(path, true) {
                Ok(needs_formatting) => {
                    if needs_formatting {
                        std::process::exit(1);
                    }
                }
                Err(e) => {
                    eprintln!("Error: {:?}", e);
                    std::process::exit(1);
                }
            }
        }
        file => {
            let path = Path::new(file);
            match format_file(path, false) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Error: {:?}", e);
                    std::process::exit(1);
                }
            }
        }
    }
}
