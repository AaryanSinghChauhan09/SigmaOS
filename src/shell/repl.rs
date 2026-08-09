// SigmaOS Command-Line REPL (Read-Eval-Print Loop)
// Conforms to zero-dependency, safe Rust design patterns under freestanding environments

use crate::klib::{HashMap, Vec};

/// Shell command type
#[derive(Debug, Clone)]
pub enum ShellCommand {
    Help,
    ListProcesses,
    ListFiles,
    Exit,
    Pwd,
    WhoAmI,
    Uname,
    Clear,
    Touch { filename: String },
    Mkdir { dirname: String },
    Rm { filename: String },
    Su { username: String, password: Option<String> },
    Cat { filename: String },
    Systemctl { action: String, service: Option<String> },
    Apt { action: String, subcommand: String, package: Option<String> },
    Theme { theme_name: String },
    Profile { profile_name: String },
    A11y { feature: String, state: String },
    Echo { message: String },
    Set { variable: String, value: String },
    Get { variable: String },
    Unknown(String),
}

pub struct ShellRepl {
    pub running: bool,
    pub prompt: String,
    pub current_user: String,
    pub current_dir: String,
    pub variables: HashMap<String, String>,
}

impl ShellRepl {
    pub fn new() -> Self {
        ShellRepl {
            running: true,
            prompt: "sigma-sh> ".to_string(),
            current_user: "guest".to_string(),
            current_dir: "/home/guest".to_string(),
            variables: HashMap::new(),
        }
    }

    pub fn with_prompt(prompt: String) -> Self {
        let mut repl = Self::new();
        repl.prompt = prompt;
        repl
    }

    pub fn run(&mut self) {
        use std::io::{self, Write};

        println!("Welcome to SigmaOS Interactive Shell!");
        println!("Type 'help' to see available commands or 'exit' to exit.");

        while self.running {
            print!("{} {}", self.current_user, self.prompt);
            let _ = io::stdout().flush();

            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_err() {
                break;
            }

            let trimmed = input.trim();
            if trimmed.is_empty() {
                continue;
            }

            self.execute_line(trimmed);
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

    pub fn parse_command(&self, input: &str) -> ShellCommand {
        let parts: Vec<&str> = input.split_whitespace().collect();

        if parts.is_empty() {
            return ShellCommand::Unknown(input.to_string());
        }

        match parts[0] {
            "help" => ShellCommand::Help,
            "ps" => ShellCommand::ListProcesses,
            "ls" => ShellCommand::ListFiles,
            "exit" | "quit" => ShellCommand::Exit,
            "pwd" => ShellCommand::Pwd,
            "whoami" => ShellCommand::WhoAmI,
            "uname" => ShellCommand::Uname,
            "clear" => ShellCommand::Clear,
            "touch" => {
                if parts.len() >= 2 {
                    ShellCommand::Touch {
                        filename: parts[1].to_string(),
                    }
                } else {
                    ShellCommand::Unknown(input.to_string())
                }
            }
            "mkdir" => {
                if parts.len() >= 2 {
                    ShellCommand::Mkdir {
                        dirname: parts[1].to_string(),
                    }
                } else {
                    ShellCommand::Unknown(input.to_string())
                }
            }
            "rm" => {
                if parts.len() >= 2 {
                    ShellCommand::Rm {
                        filename: parts[1].to_string(),
                    }
                } else {
                    ShellCommand::Unknown(input.to_string())
                }
            }
            "su" => {
                if parts.len() >= 2 {
                    ShellCommand::Su {
                        username: parts[1].to_string(),
                        password: parts.get(2).map(|s| s.to_string()),
                    }
                } else {
                    ShellCommand::Unknown(input.to_string())
                }
            }
            "cat" => {
                if parts.len() >= 2 {
                    ShellCommand::Cat {
                        filename: parts[1].to_string(),
                    }
                } else {
                    ShellCommand::Unknown(input.to_string())
                }
            }
            "systemctl" => {
                if parts.len() >= 2 {
                    ShellCommand::Systemctl {
                        action: parts[1].to_string(),
                        service: parts.get(2).map(|s| s.to_string()),
                    }
                } else {
                    ShellCommand::Unknown(input.to_string())
                }
            }
            "apt" => {
                if parts.len() >= 2 {
                    ShellCommand::Apt {
                        action: parts[1].to_string(),
                        subcommand: parts[1].to_string(),
                        package: parts.get(2).map(|s| s.to_string()),
                    }
                } else {
                    ShellCommand::Unknown(input.to_string())
                }
            }
            "echo" => ShellCommand::Echo {
                message: parts[1..].join(" "),
            },
            "mkdir-vfs" => {
                if parts.len() >= 2 {
                    ShellCommand::Mkdir {
                        dirname: parts[1].to_string(),
                    }
                } else {
                    ShellCommand::Unknown(input.to_string())
                }
            }
            "theme" => {
                if parts.len() >= 2 {
                    ShellCommand::Theme {
                        theme_name: parts[1].to_string(),
                    }
                } else {
                    ShellCommand::Unknown(input.to_string())
                }
            }
            "profile" => {
                if parts.len() >= 2 {
                    ShellCommand::Profile {
                        profile_name: parts[1].to_string(),
                    }
                } else {
                    ShellCommand::Unknown(input.to_string())
                }
            }
            "a11y" => {
                if parts.len() >= 3 {
                    ShellCommand::A11y {
                        feature: parts[1].to_string(),
                        state: parts[2].to_string(),
                    }
                } else {
                    ShellCommand::Unknown(input.to_string())
                }
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

    pub fn execute_command(&mut self, command: ShellCommand) -> Result<String, String> {
        match command {
            ShellCommand::Help => Ok("Available commands:\n\
                   help         - Show this help message\n\
                   ps           - List running processes\n\
                   ls           - List files\n\
                   pwd          - Print working directory\n\
                   whoami       - Print current logged-in user\n\
                   su <user>    - Switch user account (try 'su root' or 'su guest')\n\
                   cat <file>   - Display file contents\n\
                   systemctl    - Manage systemd services (try 'systemctl list' or 'systemctl status <service>')\n\
                   apt <cmd>    - Advanced Package Tool (try 'apt update', 'apt search <pkg>', or 'apt install <pkg>')\n\
                   echo         - Print a message\n\
                   set          - Set a variable\n\
                   get          - Get a variable\n\
                   exit         - Exit the shell"
                .to_string()),
            ShellCommand::ListProcesses => Ok("PID  NAME        STATE\n\
                   1    sigma-sh    Running\n\
                   2    systemd     Running\n\
                   3    udevd       Running"
                .to_string()),
            ShellCommand::ListFiles => Ok("README.md\n\
                   Cargo.toml\n\
                   src/\n\
                   tests/"
                .to_string()),
            ShellCommand::Exit => {
                self.running = false;
                Ok(String::new())
            }
            ShellCommand::Pwd => Ok(self.current_dir.clone()),
            ShellCommand::WhoAmI => Ok(self.current_user.clone()),
            ShellCommand::Uname => Ok("Linux sigmaos 6.24.0-mainline #1 SMP PREEMPT_RT Sun Jul 19 2026 x86_64 x86_64 x86_64 GNU/Linux".to_string()),
            ShellCommand::Clear => Ok("\x1B[2J\x1B[H".to_string()),
            ShellCommand::Touch { filename } => Ok(format!("Created empty file: {}", filename)),
            ShellCommand::Mkdir { dirname } => Ok(format!("Created directory: {}", dirname)),
            ShellCommand::Rm { filename } => Ok(format!("Removed file: {}", filename)),
            ShellCommand::Su { username, password } => {
                if username == "root" {
                    let pwd = password.unwrap_or_default();
                    if pwd == "admin" || pwd == "root" {
                        self.current_user = "root".to_string();
                        self.current_dir = "/root".to_string();
                        Ok("Logged in as root".to_string())
                    } else {
                        Err("Authentication failed".to_string())
                    }
                } else if username == "guest" {
                    self.current_user = "guest".to_string();
                    self.current_dir = "/home/guest".to_string();
                    Ok("Logged in as guest".to_string())
                } else {
                    Err(format!("User '{}' not found", username))
                }
            }
            ShellCommand::Cat { filename } => {
                if filename == "README.md" {
                    Ok("# SigmaOS\nA secure microkernel OS".to_string())
                } else {
                    Err(format!("cat: {}: No such file or directory", filename))
                }
            }
            ShellCommand::Systemctl { action, service } => match action.as_str() {
                "list" => Ok("UNIT          ACTIVE\n\
                       network       active\n\
                       syslog        active\n\
                       sshd          inactive"
                    .to_string()),
                "status" => {
                    let srv = service.unwrap_or_default();
                    if srv == "network" {
                        Ok("network.service - L1 Loop Network Stack\nActive: active (running)".to_string())
                    } else {
                        Err(format!("systemctl: Service '{}' not found", srv))
                    }
                }
                _ => Err(format!("systemctl: Unknown action '{}'", action)),
            },
            ShellCommand::Apt { action: _, subcommand, package } => match subcommand.as_str() {
                "update" => Ok("Hit:1 https://pkg.sigmaos.org stable InRelease\nReading package lists... Done".to_string()),
                "search" => {
                    let pkg = package.unwrap_or_default();
                    Ok(format!("{} - Package matching query", pkg))
                }
                "install" => {
                    let pkg = package.unwrap_or_default();
                    if pkg.is_empty() {
                        Err("apt install: Package name is required".to_string())
                    } else {
                        Ok(format!("Installed package: {}", pkg))
                    }
                }
                _ => Err(format!("apt: Unknown command '{}'", subcommand)),
            },
            ShellCommand::Theme { theme_name } => {
                Ok(format!("Successfully switched theme to '{}'", theme_name))
            }
            ShellCommand::Profile { profile_name } => {
                Ok(format!("Successfully switched profile to '{}'", profile_name))
            }
            ShellCommand::A11y { feature, state } => {
                Ok(format!("Accessibility feature '{}' is now '{}'", feature, state))
            }
            ShellCommand::Echo { message } => Ok(message),
            ShellCommand::Set { variable, value } => {
                self.variables.insert(variable.clone(), value.clone());
                Ok(format!("{} = {}", variable, value))
            }
            ShellCommand::Get { variable } => match self.variables.get(&variable) {
                Some(value) => Ok(value.clone()),
                None => Err(format!("Variable '{}' not found", variable)),
            },
            ShellCommand::Unknown(cmd) => {
                let parts: Vec<&str> = cmd.split_whitespace().collect();
                if parts.is_empty() {
                    return Err("Unknown command".to_string());
                }
                let input_cmd = parts[0];
                let commands = &[
                    "help", "ps", "ls", "exit", "quit", "pwd", "whoami", "uname", "clear",
                    "touch", "mkdir", "rm", "su", "cat", "systemctl", "apt", "echo", "set",
                    "get", "theme", "profile", "a11y"
                ];
                let mut best_match: Option<&str> = None;
                let mut min_dist = usize::MAX;
                for &known_cmd in commands {
                    let dist = levenshtein(input_cmd, known_cmd);
                    if dist < min_dist {
                        min_dist = dist;
                        best_match = Some(known_cmd);
                    }
                }
                if let Some(suggestion) = best_match {
                    if min_dist <= 2 {
                        return Err(format!("Unknown command: '{}'. Did you mean '{}'?", input_cmd, suggestion));
                    }
                }
                Err(format!("Unknown command: '{}'", input_cmd))
            }
        }
    }
}

// Levenshtein distance helper function for command suggestions (Palette UX 🎨)
fn levenshtein(s1: &str, s2: &str) -> usize {
    let len2 = s2.chars().count();

    let mut row: Vec<usize> = (0..=len2).collect();
    for (i, c1) in s1.chars().enumerate() {
        let mut next_row = vec![0; len2 + 1];
        next_row[0] = i + 1;
        for (j, c2) in s2.chars().enumerate() {
            let cost = if c1 == c2 { 0 } else { 1 };
            next_row[j + 1] = core::cmp::min(
                row[j + 1] + 1, // deletion
                core::cmp::min(
                    next_row[j] + 1, // insertion
                    row[j] + cost // substitution
                )
            );
        }
        row = next_row;
    }
    row[len2]
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
    fn test_levenshtein_suggestions() {
        let mut repl = ShellRepl::new();

        // typo on "help"
        let cmd = repl.parse_command("hel");
        let result = repl.execute_command(cmd);
        assert_eq!(result.err().unwrap(), "Unknown command: 'hel'. Did you mean 'help'?");

        // typo on "clear"
        let cmd = repl.parse_command("cleer");
        let result = repl.execute_command(cmd);
        assert_eq!(result.err().unwrap(), "Unknown command: 'cleer'. Did you mean 'clear'?");

        // typo too far away
        let cmd = repl.parse_command("somethingtotallydifferent");
        let result = repl.execute_command(cmd);
        assert_eq!(result.err().unwrap(), "Unknown command: 'somethingtotallydifferent'");
    }

    #[test]
    fn test_parse_help() {
        let repl = ShellRepl::new();
        let command = repl.parse_command("help");
        assert!(matches!(command, ShellCommand::Help));
    }
}
