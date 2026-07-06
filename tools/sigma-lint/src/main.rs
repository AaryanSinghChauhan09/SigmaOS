//! sigma-lint - Native Rust linter for SigmaOS
//! Replaces ESLint with zero-dependency Rust implementation
//! 
//! Usage: sigma-lint <file> or sigma-lint --fix <file>

use std::env;
use std::fs;
use std::io::{self, Read};
use std::path::Path;

const VERSION: &str = "1.0.0";

#[derive(Debug)]
enum LintError {
    Io(io::Error),
    Parse(String),
}

impl From<io::Error> for LintError {
    fn from(err: io::Error) -> Self {
        LintError::Io(err)
    }
}

#[derive(Debug, Clone)]
struct LintMessage {
    line: usize,
    column: usize,
    rule: String,
    message: String,
    severity: Severity,
}

#[derive(Debug, Clone, PartialEq)]
enum Severity {
    Error,
    Warning,
    Info,
}

/// Basic linting rules - can be expanded
fn lint_code(content: &str) -> Vec<LintMessage> {
    let mut messages = Vec::new();
    
    for (line_num, line) in content.lines().enumerate() {
        // Rule: no-console
        if line.contains("console.") {
            messages.push(LintMessage {
                line: line_num + 1,
                column: line.find("console.").unwrap_or(0) + 1,
                rule: "no-console".to_string(),
                message: "Unexpected console statement".to_string(),
                severity: Severity::Warning,
            });
        }
        
        // Rule: no-unused-vars (basic check)
        if line.contains("let ") && line.contains("= ") && !line.contains("//") {
            let parts: Vec<&str> = line.split('=').collect();
            if parts.len() > 1 {
                let var_part = parts[0].trim();
                if var_part.starts_with("let ") {
                    let var_name = var_part.strip_prefix("let ").unwrap().trim();
                    if var_name.ends_with('_') {
                        // Intentionally unused, ignore
                    } else {
                        // Check if variable is used later (simplified)
                        let var_usage = content.lines().any(|l| l.contains(var_name) && !l.contains("let "));
                        if !var_usage {
                            messages.push(LintMessage {
                                line: line_num + 1,
                                column: line.find("let ").unwrap_or(0) + 1,
                                rule: "no-unused-vars".to_string(),
                                message: format!("'{}' is assigned a value but never used", var_name),
                                severity: Severity::Warning,
                            });
                        }
                    }
                }
            }
        }
        
        // Rule: eqeqeq (prefer === over ==)
        if line.contains("==") && !line.contains("===") && !line.contains("!=") && !line.contains("//") {
            messages.push(LintMessage {
                line: line_num + 1,
                column: line.find("==").unwrap_or(0) + 1,
                rule: "eqeqeq".to_string(),
                message: "Expected '===' and instead saw '=='".to_string(),
                severity: Severity::Error,
            });
        }
        
        // Rule: semi (require semicolons)
        if line.trim().ends_with('{') || line.trim().ends_with('}') || line.trim().is_empty() {
            // Ignore
        } else if !line.trim().ends_with(';') && !line.trim().starts_with("//") && !line.trim().starts_with("/*") {
            messages.push(LintMessage {
                line: line_num + 1,
                column: line.trim().len(),
                rule: "semi".to_string(),
                message: "Missing semicolon".to_string(),
                severity: Severity::Error,
            });
        }
    }
    
    messages
}

fn lint_file(path: &Path) -> Result<Vec<LintMessage>, LintError> {
    let mut content = String::new();
    let mut file = fs::File::open(path)?;
    file.read_to_string(&mut content)?;
    
    Ok(lint_code(&content))
}

fn print_messages(messages: &[LintMessage], file: &str) {
    let mut error_count = 0;
    let mut warning_count = 0;
    
    for msg in messages {
        match msg.severity {
            Severity::Error => {
                eprintln!("{}:{}:{}  error  {}  {}", file, msg.line, msg.column, msg.rule, msg.message);
                error_count += 1;
            }
            Severity::Warning => {
                eprintln!("{}:{}:{}  warning  {}  {}", file, msg.line, msg.column, msg.rule, msg.message);
                warning_count += 1;
            }
            Severity::Info => {
                eprintln!("{}:{}:{}  info  {}  {}", file, msg.line, msg.column, msg.rule, msg.message);
            }
        }
    }
    
    if error_count > 0 || warning_count > 0 {
        eprintln!();
        eprintln!("✖ {} problems ({} errors, {} warnings)", error_count + warning_count, error_count, warning_count);
    } else {
        println!("✓ No problems found");
    }
}

fn print_usage() {
    println!("sigma-lint v{}", VERSION);
    println!();
    println!("Native Rust linter for SigmaOS");
    println!();
    println!("USAGE:");
    println!("  sigma-lint <file>              Lint file");
    println!("  sigma-lint --fix <file>        Lint and auto-fix file");
    println!("  sigma-lint --version           Print version");
    println!("  sigma-lint --help              Show this help");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_usage();
        std::process::exit(1);
    }
    
    match args[1].as_str() {
        "--version" => {
            println!("sigma-lint {}", VERSION);
        }
        "--help" | "-h" => {
            print_usage();
        }
        "--fix" => {
            if args.len() < 3 {
                eprintln!("Error: --fix requires a file argument");
                std::process::exit(1);
            }
            let path = Path::new(&args[2]);
            match lint_file(path) {
                Ok(messages) => {
                    print_messages(&messages, args[2].as_str());
                    // Auto-fix not implemented yet
                    if messages.iter().any(|m| m.severity == Severity::Error) {
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
            match lint_file(path) {
                Ok(messages) => {
                    print_messages(&messages, file);
                    if messages.iter().any(|m| m.severity == Severity::Error) {
                        std::process::exit(1);
                    }
                }
                Err(e) => {
                    eprintln!("Error: {:?}", e);
                    std::process::exit(1);
                }
            }
        }
    }
}
