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
    Theme { name: String },
    Profile { name: String },
    A11y { feature: String, enabled: bool },
    Unknown(String),
}

/// Shell REPL
pub struct ShellRepl {
    running: bool,
    variables: HashMap<String, String>,
    prompt: String,
    pub current_theme: String,
    pub current_profile: String,
    pub a11y_features: std::collections::HashMap<String, bool>,
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
            prompt,
            current_theme: "default".to_string(),
            current_profile: "default".to_string(),
            a11y_features: a11y,
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
            "echo" => {
                let message = parts[1..].join(" ");
                ShellCommand::Echo { message }
            }
            "su" => {
                if parts.len() >= 2 {
                    let password = if parts.len() >= 3 {
                        Some(parts[2].to_string())
                    } else {
                        None
                    };
                    ShellCommand::Su {
                        username: parts[1].to_string(),
                        password,
                    }
                } else {
                    ShellCommand::Su {
                        username: "root".to_string(),
                        password: None,
                    }
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
                    let action = parts[1].to_string();
                    let service = if parts.len() >= 3 {
                        parts[2].to_string()
                    } else {
                        String::new()
                    };
                    ShellCommand::Systemctl { action, service }
                } else {
                    ShellCommand::Unknown(input.to_string())
                }
            }
            "apt" => {
                if parts.len() >= 2 {
                    let subcommand = parts[1].to_string();
                    let package = if parts.len() >= 3 {
                        Some(parts[2].to_string())
                    } else {
                        None
                    };
                    ShellCommand::Apt {
                        subcommand,
                        package,
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
            "theme" => {
                if parts.len() >= 2 {
                    ShellCommand::Theme {
                        name: parts[1].to_string(),
                    }
                } else {
                    ShellCommand::Unknown(input.to_string())
                }
            }
            "display" => {
                let subcommand = if parts.len() >= 2 {
                    parts[1].to_string()
                } else {
                    "list".to_string()
                };
                let args = parts[2..].iter().map(|s| s.to_string()).collect();
                ShellCommand::Display { subcommand, args }
            }
            "theme" => {
                let subcommand = if parts.len() >= 2 {
                    parts[1].to_string()
                } else {
                    "list".to_string()
                };
                let args = parts[2..].iter().map(|s| s.to_string()).collect();
                ShellCommand::Theme { subcommand, args }
            }
            "profile" => {
                let subcommand = if parts.len() >= 2 {
                    parts[1].to_string()
                } else {
                    "list".to_string()
                };
                let args = parts[2..].iter().map(|s| s.to_string()).collect();
                ShellCommand::Profile { subcommand, args }
            }
            "window" => {
                let subcommand = if parts.len() >= 2 {
                    parts[1].to_string()
                } else {
                    "list".to_string()
                };
                let args = parts[2..].iter().map(|s| s.to_string()).collect();
                ShellCommand::Window { subcommand, args }
            }
            "accessibility" | "acc" => {
                let subcommand = if parts.len() >= 2 {
                    parts[1].to_string()
                } else {
                    "status".to_string()
                };
                let args = parts[2..].iter().map(|s| s.to_string()).collect();
                ShellCommand::Accessibility { subcommand, args }
            }
            "screenshot" => {
                let subcommand = if parts.len() >= 2 {
                    parts[1].to_string()
                } else {
                    "capture".to_string()
                };
                let args = parts[2..].iter().map(|s| s.to_string()).collect();
                ShellCommand::Screenshot { subcommand, args }
            }
            "record" => {
                let subcommand = if parts.len() >= 2 {
                    parts[1].to_string()
                } else {
                    "start".to_string()
                };
                let args = parts[2..].iter().map(|s| s.to_string()).collect();
                ShellCommand::Record { subcommand, args }
            }
            "clipboard" => {
                let subcommand = if parts.len() >= 2 {
                    parts[1].to_string()
                } else {
                    "get".to_string()
                };
                let args = parts[2..].iter().map(|s| s.to_string()).collect();
                ShellCommand::Clipboard { subcommand, args }
            }
            _ => ShellCommand::Unknown(input.to_string()),
        }
    }

    pub fn execute_command(&mut self, command: ShellCommand) -> Result<String, String> {
        match command {
            ShellCommand::Help => Ok("Available commands:\n\
                   help             - Show this help message\n\
                   ps               - List running processes\n\
                   ls               - List files\n\
                   echo             - Print a message\n\
                   set              - Set a variable\n\
                   get              - Get a variable\n\
                   theme [name]     - Switch Zenith desktop theme\n\
                   profile [name]   - Switch Zenith user profile\n\
                   a11y [feat] [on] - Switch Zenith accessibility settings\n\
                   exit             - Exit the shell"
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
            ShellCommand::Theme { name } => {
                self.current_theme = name.clone();
                Ok(format!("Zenith Theme set to: {}", name))
            }
            ShellCommand::Profile { name } => {
                self.current_profile = name.clone();
                Ok(format!("Zenith Profile set to: {}", name))
            }
            ShellCommand::A11y { feature, enabled } => {
                self.a11y_features.insert(feature.clone(), enabled);
                Ok(format!(
                    "Zenith Accessibility [{}] set to: {}",
                    feature,
                    if enabled { "on" } else { "off" }
                ))
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
        assert_eq!(repl.prompt, "ubuntu@sigmaos:~$ ");
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
    fn test_theme_and_profile_commands() {
        let mut repl = ShellRepl::new();

        let theme_cmd = repl.parse_command("theme dark");
        let res = repl.execute_command(theme_cmd).unwrap();
        assert_eq!(repl.current_theme, "dark");
        assert!(res.contains("dark"));

        let profile_cmd = repl.parse_command("profile developer");
        let res = repl.execute_command(profile_cmd).unwrap();
        assert_eq!(repl.current_profile, "developer");
        assert!(res.contains("developer"));
    }

    #[test]
    fn test_a11y_commands() {
        let mut repl = ShellRepl::new();

        let a11y_cmd = repl.parse_command("a11y high_contrast on");
        let res = repl.execute_command(a11y_cmd).unwrap();
        assert_eq!(repl.a11y_features.get("high_contrast"), Some(&true));
        assert!(res.contains("on"));
    }

    #[test]
    fn test_exit() {
        let mut repl = ShellRepl::new();
        let command = ShellCommand::Exit;
        repl.execute_command(command).unwrap();
        assert!(!repl.running);
    }

    #[test]
    fn test_cli_customization() {
        let mut repl = ShellRepl::new();

        let list_cmd = repl.parse_command("theme list");
        assert!(matches!(list_cmd, ShellCommand::ThemeList));
        let list_res = repl.execute_command(list_cmd).unwrap();
        assert!(list_res.contains("Dark"));
        assert!(list_res.contains("Light"));

        let set_cmd = repl.parse_command("theme set Light");
        assert!(matches!(set_cmd, ShellCommand::ThemeSet { .. }));
        let set_res = repl.execute_command(set_cmd).unwrap();
        assert!(set_res.contains("Light"));

        let enable_cmd = repl.parse_command("routine enable work_mode");
        assert!(matches!(enable_cmd, ShellCommand::RoutineEnable { .. }));
        let enable_res = repl.execute_command(enable_cmd).unwrap();
        assert!(enable_res.contains("Work Mode"));
    }

    #[test]
    fn test_cli_accessibility() {
        let mut repl = ShellRepl::new();

        let set_cmd = repl.parse_command("a11y set screen_reader on");
        assert!(matches!(set_cmd, ShellCommand::A11ySet { .. }));
        let set_res = repl.execute_command(set_cmd).unwrap();
        assert!(set_res.contains("true"));

        let profile_cmd = repl.parse_command("a11y profile blind");
        assert!(matches!(profile_cmd, ShellCommand::A11yProfile { .. }));
        let profile_res = repl.execute_command(profile_cmd).unwrap();
        assert!(profile_res.contains("Vision Impaired"));
    }

    #[test]
    fn test_cli_telemetry() {
        let mut repl = ShellRepl::new();

        let show_cmd = repl.parse_command("monitor show");
        assert!(matches!(show_cmd, ShellCommand::MonitorShow));
        let show_res = repl.execute_command(show_cmd).unwrap();
        assert!(show_res.contains("System Telemetry Dashboard"));
        assert!(show_res.contains("CPU Usage"));
        assert!(show_res.contains("Memory Usage"));
    }

    #[test]
    fn test_cli_package_management() {
        let mut repl = ShellRepl::new();

        let list_cmd = repl.parse_command("pkg list");
        assert!(matches!(list_cmd, ShellCommand::PkgList));
        let list_res = repl.execute_command(list_cmd).unwrap();
        assert!(list_res.contains("Installed system packages"));

        let install_cmd = repl.parse_command("pkg install nano");
        assert!(matches!(install_cmd, ShellCommand::PkgInstall { .. }));
        let install_res = repl.execute_command(install_cmd).unwrap();
        assert!(install_res.contains("nano"));

        let remove_cmd = repl.parse_command("pkg remove nano");
        assert!(matches!(remove_cmd, ShellCommand::PkgRemove { .. }));
        let remove_res = repl.execute_command(remove_cmd).unwrap();
        assert!(remove_res.contains("nano"));
    }

    #[test]
    fn test_cli_virtualization() {
        let mut repl = ShellRepl::new();

        let list_cmd = repl.parse_command("vm list");
        assert!(matches!(list_cmd, ShellCommand::VmList));
        let list_res = repl.execute_command(list_cmd).unwrap();
        assert!(list_res.contains("Running Guest Virtual Machines"));

        let create_cmd = repl.parse_command("vm create guest-01 qemu");
        assert!(matches!(create_cmd, ShellCommand::VmCreate { .. }));
        let create_res = repl.execute_command(create_cmd).unwrap();
        assert!(create_res.contains("guest-01"));

        let container_cmd = repl.parse_command("container run web-c nginx-img");
        assert!(matches!(container_cmd, ShellCommand::ContainerRun { .. }));
        let container_res = repl.execute_command(container_cmd).unwrap();
        assert!(container_res.contains("web-c"));
    }

    #[test]
    fn test_cli_compatibility() {
        let mut repl = ShellRepl::new();
        assert!(repl
            .execute_command(ShellCommand::Display {
                subcommand: "list".to_string(),
                args: vec![]
            })
            .is_ok());
        assert!(repl
            .execute_command(ShellCommand::Display {
                subcommand: "set".to_string(),
                args: vec![]
            })
            .is_ok());
        assert!(repl
            .execute_command(ShellCommand::Display {
                subcommand: "scale".to_string(),
                args: vec!["DP-1".to_string(), "2.0".to_string()]
            })
            .is_ok());
    }

    #[test]
    fn test_cli_resilience() {
        let mut repl = ShellRepl::new();
        assert!(repl
            .execute_command(ShellCommand::Theme {
                subcommand: "list".to_string(),
                args: vec![]
            })
            .is_ok());
        assert_eq!(
            repl.execute_command(ShellCommand::Theme {
                subcommand: "set".to_string(),
                args: vec!["Banaras Gold".to_string()]
            })
            .unwrap(),
            "Theme changed to Banaras Gold."
        );
        assert_eq!(repl.active_theme, "Banaras Gold");
    }

    #[test]
    fn test_cli_defensive_auditing() {
        let mut repl = ShellRepl::new();
        assert!(repl
            .execute_command(ShellCommand::Profile {
                subcommand: "list".to_string(),
                args: vec![]
            })
            .is_ok());
        assert!(repl
            .execute_command(ShellCommand::Profile {
                subcommand: "switch".to_string(),
                args: vec!["developer".to_string()]
            })
            .is_ok());
        assert_eq!(repl.active_profile, "developer");
    }

    #[test]
    fn test_window_commands() {
        let mut repl = ShellRepl::new();
        assert!(repl
            .execute_command(ShellCommand::Window {
                subcommand: "list".to_string(),
                args: vec![]
            })
            .is_ok());
        assert!(repl
            .execute_command(ShellCommand::Window {
                subcommand: "create".to_string(),
                args: vec![
                    "SigmaBrowser".to_string(),
                    "sigma.browser".to_string(),
                    "10,10,600,400".to_string()
                ]
            })
            .is_ok());
        assert_eq!(repl.windows.len(), 3);
    }

    #[test]
    fn test_accessibility_commands() {
        let mut repl = ShellRepl::new();
        assert!(repl
            .execute_command(ShellCommand::Accessibility {
                subcommand: "status".to_string(),
                args: vec![]
            })
            .is_ok());
        assert!(repl
            .execute_command(ShellCommand::Accessibility {
                subcommand: "screen-reader".to_string(),
                args: vec!["true".to_string()]
            })
            .is_ok());
        assert!(repl.screen_reader_enabled);
    }

    #[test]
    fn test_clipboard_commands() {
        let mut repl = ShellRepl::new();
        assert!(repl
            .execute_command(ShellCommand::Clipboard {
                subcommand: "get".to_string(),
                args: vec![]
            })
            .is_ok());
        assert!(repl
            .execute_command(ShellCommand::Clipboard {
                subcommand: "set".to_string(),
                args: vec!["CopiedText".to_string()]
            })
            .is_ok());
        assert_eq!(repl.clipboard_content, "CopiedText");
    }
}
