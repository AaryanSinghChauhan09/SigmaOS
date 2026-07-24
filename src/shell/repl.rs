// SigmaOS Shell REPL (Read-Eval-Print Loop)
// Interactive shell with full desktop GUI-parity and defensive auditing commands

use std::collections::HashMap;
use std::io::{self, BufRead, Write};

use crate::accessibility::{
    AccessibilityCategory, AccessibilityFeature, AccessibilityFramework, AccessibilityProfile,
    AccessibilitySetting,
};
use crate::compatibility::{
    ApplicationBinary, BinaryFormat, CompatibilityManager, CompatibilityMode, TargetPlatform,
};
use crate::customization::{CustomizationEngine, Theme};
use crate::dashboard::{MetricType, SystemMonitor, UnifiedDashboard, WidgetType};
use crate::package::{PackageFormat, PackageSource, UnifiedPackage, UniversalPackageManager};
use crate::resilience::{RecoveryAction, RecoveryEventType, RecoveryRule, SelfHealingModule};
use crate::virtualization::{
    Container, ResourcePool, VirtualMachine, VirtualizationOrchestrator, VirtualizationTech,
    VmState,
};

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
    Alias { name: String, value: String },
    Unalias { name: String },
    Run { variable: String },
    Unknown(String),
}

/// Shell REPL
pub struct ShellRepl {
    running: bool,
    variables: std::collections::HashMap<String, String>,
    aliases: std::collections::HashMap<String, String>,
    prompt: String,

    // Keep internal instances of engines for persistent state during shell interaction
    pub customization: CustomizationEngine,
    pub accessibility: AccessibilityFramework,
    pub package_manager: UniversalPackageManager,
    pub virt_orchestrator: VirtualizationOrchestrator,
    pub compatibility: CompatibilityManager,
    pub self_healing: SelfHealingModule,
}

impl ShellRepl {
    pub fn new() -> Self {
        let mut services = std::collections::HashMap::new();
        services.insert("systemd-networkd".to_string(), "Running".to_string());
        services.insert("systemd-logind".to_string(), "Running".to_string());
        services.insert("cron".to_string(), "Running".to_string());

        Self {
            running: true,
            variables: std::collections::HashMap::new(),
            prompt: "sigma-sh> ".to_string(),
            current_user: "ubuntu".to_string(),
            current_dir: "/home/ubuntu".to_string(),
            services,
            installed_packages: std::collections::HashSet::new(),
        }
    }

    pub fn with_prompt(prompt: String) -> Self {
        let mut services = std::collections::HashMap::new();
        services.insert("systemd-networkd".to_string(), "Running".to_string());
        services.insert("systemd-logind".to_string(), "Running".to_string());
        services.insert("cron".to_string(), "Running".to_string());

        Self {
            running: true,
            variables: std::collections::HashMap::new(),
            prompt: "sigma-sh> ".to_string(),
            current_user: "ubuntu".to_string(),
            current_dir: "/home/ubuntu".to_string(),
            services,
            installed_packages: std::collections::HashSet::new(),
        }
    }

    pub fn with_prompt(prompt: String) -> Self {
        let mut services = std::collections::HashMap::new();
        services.insert("systemd-networkd".to_string(), "Running".to_string());
        services.insert("systemd-logind".to_string(), "Running".to_string());
        services.insert("cron".to_string(), "Running".to_string());

        Self {
            running: true,
            variables: std::collections::HashMap::new(),
            aliases: std::collections::HashMap::new(),
            prompt,
        }
    }

    pub fn run(&mut self) {
        println!("SigmaOS Shell v0.1.0 (GUI-Parity & Security Auditing Enabled)");
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

    pub fn execute_line(&mut self, line: &str) {
        if line.contains(';') {
            let subcommands: Vec<&str> = line.split(';').collect();
            for sub in subcommands {
                let trimmed = sub.trim();
                if !trimmed.is_empty() {
                    self.execute_single_line(trimmed);
                }
            }
        } else {
            self.execute_single_line(line);
        }
    }

    fn execute_single_line(&mut self, line: &str) {
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
        // Optimized to minimize/avoid heap allocations by using iterators.
        // Single-word commands are completely allocation-free.
        let mut parts = input.split_whitespace();
        let cmd = match parts.next() {
            Some(c) => c,
            None => return ShellCommand::Unknown(input.to_string()),
        };

        match cmd {
            "help" => ShellCommand::Help,
            "ps" => ShellCommand::ListProcesses,
            "ls" => ShellCommand::ListFiles,
            "exit" | "quit" => ShellCommand::Exit,
            "pwd" => ShellCommand::Pwd,
            "whoami" => ShellCommand::WhoAmI,
            "uname" => ShellCommand::Uname,
            "clear" => ShellCommand::Clear,
            "touch" => {
                if let Some(filename) = parts.next() {
                    ShellCommand::Touch {
                        filename: filename.to_string(),
                    }
                } else {
                    ShellCommand::Unknown(input.to_string())
                }
            }
            "mkdir" => {
                if let Some(dirname) = parts.next() {
                    ShellCommand::Mkdir {
                        dirname: dirname.to_string(),
                    }
                } else {
                    ShellCommand::Unknown(input.to_string())
                }
            }
            "rm" => {
                if let Some(filename) = parts.next() {
                    ShellCommand::Rm {
                        filename: filename.to_string(),
                    }
                } else {
                    ShellCommand::Unknown(input.to_string())
                }
            }
            "echo" => {
                let message = parts.collect::<Vec<&str>>().join(" ");
                ShellCommand::Echo { message }
            }
            "set" => {
                if let Some(variable) = parts.next() {
                    let value = parts.collect::<Vec<&str>>().join(" ");
                    if !value.is_empty() {
                        ShellCommand::Set {
                            variable: variable.to_string(),
                            value,
                        }
                    } else {
                        ShellCommand::Unknown(input.to_string())
                    }
                } else {
                    ShellCommand::Unknown(input.to_string())
                }
            }
            "get" => {
                if let Some(variable) = parts.next() {
                    ShellCommand::Get {
                        variable: variable.to_string(),
                    }
                } else {
                    ShellCommand::Unknown(input.to_string())
                }
            }
            "alias" => {
                if parts.len() >= 3 {
                    ShellCommand::Alias {
                        name: parts[1].to_string(),
                        value: parts[2..].join(" "),
                    }
                } else {
                    ShellCommand::Unknown(input.to_string())
                }
            }
            "unalias" => {
                if parts.len() >= 2 {
                    ShellCommand::Unalias {
                        name: parts[1].to_string(),
                    }
                } else {
                    ShellCommand::Unknown(input.to_string())
                }
            }
            "run" | "exec" => {
                if parts.len() >= 2 {
                    ShellCommand::Run {
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
                   help    - Show this help message\n\
                   ps      - List running processes\n\
                   ls      - List files\n\
                   echo    - Print a message\n\
                   set     - Set a variable\n\
                   get     - Get a variable\n\
                   alias   - Create a command shortcut/alias\n\
                   unalias - Remove an alias\n\
                   run     - Execute an automated macro/script variable\n\
                   exit    - Exit the shell"
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
                        self.prompt = "root@sigmaos:# ".to_string();
                        Ok("Successfully logged in as root.".to_string())
                    } else {
                        Err("su: Authentication failure (hint: use 'su root admin')".to_string())
                    }
                } else {
                    self.current_user = username.clone();
                    self.current_dir = format!("/home/{}", username);
                    self.prompt = format!("{}@sigmaos:~$ ", username);
                    Ok(format!("Logged in as {}.", username))
                }
            }
            ShellCommand::Cat { filename } => {
                if filename == "README.md" {
                    Ok("# 🛡️ SigmaOS — Sovereign, AI-Native Operating System".to_string())
                } else if filename == "Cargo.toml" {
                    Ok("[package]\nname = \"sigmaos\"\nversion = \"0.1.0\"".to_string())
                } else {
                    Err(format!("cat: {}: No such file or directory", filename))
                }
            }
            ShellCommand::Systemctl { action, service } => {
                if action == "list" || action == "status" && service.is_empty() {
                    let mut list_str = "UNIT                ACTIVE   SUB\n".to_string();
                    for (s, st) in &self.services {
                        list_str.push_str(&format!("{:<20} {}  {}\n", s, if st == "Running" { "active" } else { "inactive" }, st));
                    }
                    Ok(list_str)
                } else if action == "start" {
                    if self.services.contains_key(&service) {
                        self.services.insert(service.clone(), "Running".to_string());
                        Ok(format!("Started {} service.", service))
                    } else {
                        Err(format!("Failed to start {}.service: Unit not found.", service))
                    }
                } else if action == "stop" {
                    if self.services.contains_key(&service) {
                        self.services.insert(service.clone(), "Stopped".to_string());
                        Ok(format!("Stopped {} service.", service))
                    } else {
                        Err(format!("Failed to stop {}.service: Unit not found.", service))
                    }
                } else if action == "status" {
                    if let Some(status) = self.services.get(&service) {
                        Ok(format!("● {}.service\n   Active: {} ({})\n   Main PID: 1234", service, if status == "Running" { "active" } else { "inactive" }, status))
                    } else {
                        Err(format!("Unit {}.service could not be found.", service))
                    }
                } else {
                    Err(format!("systemctl: Unknown action '{}'", action))
                }
            }
            ShellCommand::Apt { subcommand, package } => {
                if subcommand == "update" {
                    Ok("Hit:1 http://archive.ubuntu.com/ubuntu noble InRelease\n\
                        Get:2 http://security.ubuntu.com/ubuntu noble-security InRelease\n\
                        Reading package lists... Done\n\
                        Building dependency tree... Done\n\
                        All packages are up to date."
                        .to_string())
                } else if subcommand == "list" {
                    let mut list_str = "Listing installed packages...\n".to_string();
                    for pkg in &self.installed_packages {
                        list_str.push_str(&format!("{}/noble,now 1.0.0 amd64 [installed]\n", pkg));
                    }
                    Ok(list_str)
                } else if subcommand == "search" {
                    let query = package.unwrap_or_default();
                    if query.is_empty() {
                        Ok("sigma-sh - Sovereign Shell\n\
                            sigma-vim - High-fidelity Editor\n\
                            sigma-curl - Lightweight HTTP Client"
                            .to_string())
                    } else {
                        let mut results = Vec::new();
                        let all_packages = ["sigma-sh", "sigma-vim", "sigma-curl", "sigma-gcc", "sigma-git", "sigma-python"];
                        for pkg in &all_packages {
                            if pkg.contains(&query) {
                                results.push(format!("{} - Package matching query", pkg));
                            }
                        }
                        if results.is_empty() {
                            Ok("No matching packages found.".to_string())
                        } else {
                            Ok(results.join("\n"))
                        }
                    }
                } else if subcommand == "install" {
                    let pkg = package.ok_or_else(|| "apt: Please specify a package to install".to_string())?;
                    self.installed_packages.insert(pkg.clone());
                    Ok(format!("Reading package lists...\n\
                                Building dependency tree...\n\
                                The following NEW packages will be installed:\n\
                                  {}\n\
                                Preparing to unpack ...\n\
                                Unpacking {} ...\n\
                                Setting up {} ...\n\
                                Successfully installed.", pkg, pkg, pkg))
                } else {
                    Err(format!("apt: Unknown command '{}'", subcommand))
                }
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
            ShellCommand::Alias { name, value } => {
                self.aliases.insert(name.clone(), value.clone());
                Ok(format!("alias {} = {}", name, value))
            }
            ShellCommand::Unalias { name } => {
                if self.aliases.remove(&name).is_some() {
                    Ok(format!("Removed alias {}", name))
                } else {
                    Err(format!("Alias '{}' not found", name))
                }
            }
            ShellCommand::Run { variable } => {
                if let Some(val) = self.variables.get(&variable).cloned() {
                    self.execute_line(&val);
                    Ok(format!("Executed macro '{}'", variable))
                } else {
                    Err(format!("Variable/Macro '{}' not found", variable))
                }
            }
            ShellCommand::Unknown(cmd) => Err(format!("Unknown command: {}", cmd)),
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
        let command = ShellCommand::Echo {
            message: "test".to_string(),
        };
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

    #[test]
    fn test_alias_unalias() {
        let mut repl = ShellRepl::new();
        let alias_cmd = ShellCommand::Alias {
            name: "l".to_string(),
            value: "ls".to_string(),
        };
        repl.execute_command(alias_cmd).unwrap();

        let parsed = repl.parse_command("l");
        assert!(matches!(parsed, ShellCommand::ListFiles));

        let unalias_cmd = ShellCommand::Unalias {
            name: "l".to_string(),
        };
        repl.execute_command(unalias_cmd).unwrap();

        let parsed_after = repl.parse_command("l");
        assert!(matches!(parsed_after, ShellCommand::Unknown(..)));
    }

    #[test]
    fn test_macro_automation() {
        let mut repl = ShellRepl::new();
        let set_cmd = ShellCommand::Set {
            variable: "test_macro".to_string(),
            value: "echo running; ls".to_string(),
        };
        repl.execute_command(set_cmd).unwrap();

        let run_cmd = ShellCommand::Run {
            variable: "test_macro".to_string(),
        };
        let result = repl.execute_command(run_cmd);
        assert!(result.is_ok());
    }
}
