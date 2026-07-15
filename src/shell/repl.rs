// SigmaOS Shell REPL (Read-Eval-Print Loop)
// Interactive shell for SigmaOS

use std::io::{self, Write, BufRead};

/// Shell command type
#[derive(Debug, Clone)]
pub enum ShellCommand {
    Help,
    ListProcesses,
    ListFiles,
    Exit,
    Echo { message: String },
    Set { variable: String, value: String },
    Get { variable: String },
    Unknown(String),
}

/// Shell REPL
pub struct ShellRepl {
    running: bool,
    variables: std::collections::HashMap<String, String>,
    prompt: String,
}

impl ShellRepl {
    pub fn new() -> Self {
        Self {
            running: true,
            variables: std::collections::HashMap::new(),
            prompt: "sigma-sh> ".to_string(),
        }
    }

    pub fn with_prompt(prompt: String) -> Self {
        Self {
            running: true,
            variables: std::collections::HashMap::new(),
            prompt,
        }
    }

    pub fn run(&mut self) {
        println!("SigmaOS Shell v0.1.0");
        println!("Type 'help' for available commands\n");

        let stdin = io::stdin();
        let mut stdout = io::stdout();

        while self.running {
            print!("{}", self.prompt);
            stdout.flush().unwrap();

            let mut input = String::new();
            stdin.lock().read_line(&mut input).unwrap();

            let input = input.trim();
            if !input.is_empty() {
                self.execute_line(input);
            }
        }

        println!("Goodbye!");
    }

    fn execute_line(&mut self, line: &str) {
        let command = self.parse_command(line);
        let result = self.execute_command(command);

        match result {
            Ok(output) => {
                if !output.is_empty() {
                    println!("{}", output);
                }
            }
            Err(error) => {
                eprintln!("Error: {}", error);
            }
        }
    }

    fn parse_command(&self, input: &str) -> ShellCommand {
        let parts: Vec<&str> = input.split_whitespace().collect();
        
        if parts.is_empty() {
            return ShellCommand::Unknown(input.to_string());
        }

        match parts[0] {
            "help" => ShellCommand::Help,
            "ps" => ShellCommand::ListProcesses,
            "ls" => ShellCommand::ListFiles,
            "exit" | "quit" => ShellCommand::Exit,
            "echo" => {
                let message = parts[1..].join(" ");
                ShellCommand::Echo { message }
            }
            "set" => {
                if parts.len() >= 3 {
                    ShellCommand::Set {
                        variable: parts[1].to_string(),
                        value: parts[2..].join(" "),
                    }
                } else {
                    ShellCommand::Unknown(input.to_string())
                }
            }
            "get" => {
                if parts.len() >= 2 {
                    ShellCommand::Get {
                        variable: parts[1].to_string(),
                    }
                } else {
                    ShellCommand::Unknown(input.to_string())
                }
            }
            _ => ShellCommand::Unknown(input.to_string()),
        }
    }

    fn execute_command(&mut self, command: ShellCommand) -> Result<String, String> {
        match command {
            ShellCommand::Help => {
                Ok("Available commands:\n\
                   help    - Show this help message\n\
                   ps      - List running processes\n\
                   ls      - List files\n\
                   echo    - Print a message\n\
                   set     - Set a variable\n\
                   get     - Get a variable\n\
                   exit    - Exit the shell".to_string())
            }
            ShellCommand::ListProcesses => {
                Ok("PID  NAME        STATE\n\
                   1    sigma-sh    Running\n\
                   2    kernel      Running".to_string())
            }
            ShellCommand::ListFiles => {
                Ok("README.md\n\
                   Cargo.toml\n\
                   src/\n\
                   tests/".to_string())
            }
            ShellCommand::Exit => {
                self.running = false;
                Ok(String::new())
            }
            ShellCommand::Echo { message } => {
                Ok(message)
            }
            ShellCommand::Set { variable, value } => {
                self.variables.insert(variable.clone(), value.clone());
                Ok(format!("{} = {}", variable, value))
            }
            ShellCommand::Get { variable } => {
                match self.variables.get(&variable) {
                    Some(value) => Ok(value.clone()),
                    None => Err(format!("Variable '{}' not found", variable)),
                }
            }
            ShellCommand::Unknown(cmd) => {
                Err(format!("Unknown command: {}", cmd))
            }
        }
    }
}

impl Default for ShellRepl {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repl_creation() {
        let repl = ShellRepl::new();
        assert!(repl.running);
        assert_eq!(repl.prompt, "sigma-sh> ");
    }

    #[test]
    fn test_parse_help() {
        let repl = ShellRepl::new();
        let command = repl.parse_command("help");
        assert!(matches!(command, ShellCommand::Help));
    }

    #[test]
    fn test_parse_echo() {
        let repl = ShellRepl::new();
        let command = repl.parse_command("echo hello world");
        assert!(matches!(command, ShellCommand::Echo { .. }));
    }

    #[test]
    fn test_execute_echo() {
        let mut repl = ShellRepl::new();
        let command = ShellCommand::Echo { message: "test".to_string() };
        let result = repl.execute_command(command);
        assert_eq!(result.unwrap(), "test");
    }

    #[test]
    fn test_set_get_variable() {
        let mut repl = ShellRepl::new();
        let set_cmd = ShellCommand::Set {
            variable: "test".to_string(),
            value: "value".to_string(),
        };
        repl.execute_command(set_cmd).unwrap();

        let get_cmd = ShellCommand::Get {
            variable: "test".to_string(),
        };
        let result = repl.execute_command(get_cmd);
        assert_eq!(result.unwrap(), "value");
    }

    #[test]
    fn test_exit() {
        let mut repl = ShellRepl::new();
        let command = ShellCommand::Exit;
        repl.execute_command(command).unwrap();
        assert!(!repl.running);
    }
}
