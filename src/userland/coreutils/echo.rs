//! echo - Display a line of text
//!
//! Display the TEXT on standard output.

use std::env;
use std::io::Write;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    match run(&args) {
        Ok(()) => std::process::exit(0),
        Err(e) => {
            eprintln!("echo: error: {}", e);
            std::process::exit(1);
        }
    }
}

pub fn run(args: &[String]) -> Result<(), String> {
    let mut suppress_newline = false;
    let mut enable_escape = false;
    let mut output_args = Vec::new();
    
    // Parse options
    for arg in args.iter().skip(1) {
        if arg == "-n" {
            suppress_newline = true;
        } else if arg == "-e" {
            enable_escape = true;
        } else if arg == "-E" {
            enable_escape = false;
        } else if arg.starts_with('-') {
            return Err(format!("invalid option: {}", arg));
        } else {
            output_args.push(arg.clone());
        }
    }
    
    // Join arguments with spaces
    let output = output_args.join(" ");
    
    // Handle escape sequences if enabled
    let output = if enable_escape {
        interpret_escapes(&output)
    } else {
        output
    };
    
    // Print output
    if suppress_newline {
        print!("{}", output);
    } else {
        println!("{}", output);
    }
    
    Ok(())
}

fn interpret_escapes(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars().peekable();
    
    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('n') => result.push('\n'),
                Some('t') => result.push('\t'),
                Some('r') => result.push('\r'),
                Some('\\') => result.push('\\'),
                Some('0') => result.push('\0'),
                Some(c) => {
                    result.push('\\');
                    result.push(c);
                }
                None => result.push('\\'),
            }
        } else {
            result.push(c);
        }
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_echo_basic() {
        let args = vec!["echo".to_string(), "hello".to_string()];
        let result = run(&args);
        assert!(result.is_ok());
    }

    #[test]
    fn test_echo_no_newline() {
        let args = vec!["echo".to_string(), "-n".to_string(), "hello".to_string()];
        let result = run(&args);
        assert!(result.is_ok());
    }

    #[test]
    fn test_echo_escape() {
        let args = vec!["echo".to_string(), "-e".to_string(), "hello\\nworld".to_string()];
        let result = run(&args);
        assert!(result.is_ok());
    }
}
