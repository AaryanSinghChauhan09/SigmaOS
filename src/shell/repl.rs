// SigmaOS Shell REPL (Read-Eval-Print Loop)
// Interactive shell for SigmaOS

use std::io::{self, BufRead, Write};

#[derive(Debug, Clone)]
pub struct AgentAutomationEngine;

impl AgentAutomationEngine {
    pub fn new() -> Self {
        AgentAutomationEngine
    }
}

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
    variables: std::collections::HashMap<String, String>,
    prompt: String,
    pub current_theme: String,
    pub current_profile: String,
    pub a11y_features: std::collections::HashMap<String, bool>,
}

impl ShellRepl {
    pub fn new() -> Self {
        let mut a11y = std::collections::HashMap::new();
        a11y.insert("screen_reader".to_string(), false);
        a11y.insert("high_contrast".to_string(), false);
        a11y.insert("magnification".to_string(), false);

        Self {
            running: true,
            variables: std::collections::HashMap::new(),
            aliases: std::collections::HashMap::new(),
            prompt: "sigma-sh> ".to_string(),
            current_theme: "default".to_string(),
            current_profile: "default".to_string(),
            a11y_features: a11y,
        }
    }

    pub fn with_prompt(prompt: String) -> Self {
        let mut a11y = std::collections::HashMap::new();
        a11y.insert("screen_reader".to_string(), false);
        a11y.insert("high_contrast".to_string(), false);
        a11y.insert("magnification".to_string(), false);

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
            "theme" => {
                if parts.len() >= 2 {
                    ShellCommand::Theme {
                        name: parts[1].to_string(),
                    }
                } else {
                    ShellCommand::Unknown(input.to_string())
                }
            }
            "profile" => {
                if parts.len() >= 2 {
                    ShellCommand::Profile {
                        name: parts[1].to_string(),
                    }
                } else {
                    ShellCommand::Unknown(input.to_string())
                }
            }
            "a11y" => {
                if parts.len() >= 3 {
                    let enabled = match parts[2] {
                        "on" | "true" | "enable" => true,
                        _ => false,
                    };
                    ShellCommand::A11y {
                        feature: parts[1].to_string(),
                        enabled,
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
            ShellCommand::Theme { theme_name } => {
                self.current_theme = theme_name.clone();
                Ok(format!("Theme set to {}", theme_name))
            }
            ShellCommand::Profile { profile_name } => {
                self.current_profile = profile_name.clone();
                Ok(format!("Profile set to {}", profile_name))
            }
            ShellCommand::A11y { feature, state } => {
                let is_on = state == "on" || state == "true";
                self.a11y_features.insert(feature.clone(), is_on);
                Ok(format!("A11y feature {} set to {}", feature, state))
            }
            ShellCommand::Livepatch { args } => {
                if args.is_empty() {
                    Ok("livepatch: Subcommands: list, apply <symbol> <addr1> <addr2>".to_string())
                } else if args[0] == "list" {
                    Ok("sys_read -> 0xffffffffc0300100 (Active)".to_string())
                } else if args[0] == "apply" && args.len() >= 4 {
                    Ok(format!("Successfully registered livepatch redirect for '{}' from 0x{} to 0x{}", args[1], args[2], args[3]))
                } else {
                    Err("livepatch: Invalid parameters".to_string())
                }
            }
            ShellCommand::Cron { args } => {
                if args.is_empty() {
                    Ok("cron: Subcommands: list, add <name> <cmd> <schedule>".to_string())
                } else if args[0] == "list" {
                    Ok("backup_job  Daily  run_as_user=0  randomized_delay=300s  generation_id=42".to_string())
                } else if args[0] == "add" && args.len() >= 4 {
                    Ok(format!("Successfully added multi-distro cron job '{}' to execute '{}'", args[1], args[2]))
                } else {
                    Err("cron: Invalid parameters".to_string())
                }
            }
            ShellCommand::Vm { args } => {
                if args.is_empty() {
                    Ok("vm: Subcommands: list, start <name>, stop <name>".to_string())
                } else if args[0] == "list" {
                    Ok("Intel-VM  Intel VT-x (VMX)  Stopped  hpet=true  iommu_protection=AMD-Vi".to_string())
                } else if args[0] == "start" && args.len() >= 2 {
                    Ok(format!("Starting VM '{}' with hardware VT-x acceleration...", args[1]))
                } else if args[0] == "stop" && args.len() >= 2 {
                    Ok(format!("Stopping VM '{}'...", args[1]))
                } else {
                    Err("vm: Invalid parameters".to_string())
                }
            }
            ShellCommand::Research { query } => {
                if query.is_empty() {
                    Err("research: Please specify a research query".to_string())
                } else {
                    Ok(format!("SYNTHESIZED ANSWER (Evidence-Backed):\n - Claim supported by citation: [WANDR Wide and Deep Research] (Source: https://github.com/perplexityai/wandr) for query '{}'", query))
                }
            }
            ShellCommand::Camera { effect } => {
                if effect.is_empty() {
                    Ok("camera: Current effect: None. Supported effects: ChromaKey, Grayscale, Sepia, Negative".to_string())
                } else {
                    Ok(format!("Webcam effect successfully updated to '{}' (ManyCam/Snap Camera compatibility)", effect))
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
    fn test_pwd_whoami() {
        let mut repl = ShellRepl::new();
        assert_eq!(
            repl.execute_command(ShellCommand::Pwd).unwrap(),
            "/home/ubuntu"
        );
        assert_eq!(
            repl.execute_command(ShellCommand::WhoAmI).unwrap(),
            "ubuntu"
        );
    }

    #[test]
    fn test_su_root() {
        let mut repl = ShellRepl::new();
        assert!(repl
            .execute_command(ShellCommand::Su {
                username: "root".to_string(),
                password: Some("admin".to_string())
            })
            .is_ok());
        assert_eq!(repl.execute_command(ShellCommand::WhoAmI).unwrap(), "root");
        assert_eq!(repl.execute_command(ShellCommand::Pwd).unwrap(), "/root");
    }

    #[test]
    fn test_cat_command() {
        let mut repl = ShellRepl::new();
        assert!(repl
            .execute_command(ShellCommand::Cat {
                filename: "README.md".to_string()
            })
            .is_ok());
        assert!(repl
            .execute_command(ShellCommand::Cat {
                filename: "nonexistent.txt".to_string()
            })
            .is_err());
    }

    #[test]
    fn test_systemctl_commands() {
        let mut repl = ShellRepl::new();
        assert!(repl
            .execute_command(ShellCommand::Systemctl {
                action: "list".to_string(),
                service: String::new()
            })
            .is_ok());
        assert!(repl
            .execute_command(ShellCommand::Systemctl {
                action: "stop".to_string(),
                service: "cron".to_string()
            })
            .is_ok());
        assert!(repl
            .execute_command(ShellCommand::Systemctl {
                action: "start".to_string(),
                service: "cron".to_string()
            })
            .is_ok());
    }

    #[test]
    fn test_apt_commands() {
        let mut repl = ShellRepl::new();
        assert!(repl
            .execute_command(ShellCommand::Apt {
                subcommand: "update".to_string(),
                package: None
            })
            .is_ok());
        assert!(repl
            .execute_command(ShellCommand::Apt {
                subcommand: "search".to_string(),
                package: Some("vim".to_string())
            })
            .is_ok());
        assert!(repl
            .execute_command(ShellCommand::Apt {
                subcommand: "install".to_string(),
                package: Some("sigma-vim".to_string())
            })
            .is_ok());
        assert!(repl
            .execute_command(ShellCommand::Apt {
                subcommand: "list".to_string(),
                package: None
            })
            .is_ok());
    }

    #[test]
    fn test_uname_command() {
        let mut repl = ShellRepl::new();
        let cmd = repl.parse_command("uname");
        assert!(matches!(cmd, ShellCommand::Uname));
        let out = repl.execute_command(cmd).unwrap();
        assert!(out.contains("sigmaos"));
    }

    #[test]
    fn test_clear_command() {
        let mut repl = ShellRepl::new();
        let cmd = repl.parse_command("clear");
        assert!(matches!(cmd, ShellCommand::Clear));
        let out = repl.execute_command(cmd).unwrap();
        assert_eq!(out, "\x1B[2J\x1B[H");
    }

    #[test]
    fn test_touch_command() {
        let mut repl = ShellRepl::new();
        let cmd = repl.parse_command("touch testfile.txt");
        assert!(matches!(cmd, ShellCommand::Touch { .. }));
        let out = repl.execute_command(cmd).unwrap();
        assert_eq!(out, "Created empty file: testfile.txt");
    }

    #[test]
    fn test_mkdir_command() {
        let mut repl = ShellRepl::new();
        let cmd = repl.parse_command("mkdir testdir");
        assert!(matches!(cmd, ShellCommand::Mkdir { .. }));
        let out = repl.execute_command(cmd).unwrap();
        assert_eq!(out, "Created directory: testdir");
    }

    #[test]
    fn test_rm_command() {
        let mut repl = ShellRepl::new();
        let cmd = repl.parse_command("rm testfile.txt");
        assert!(matches!(cmd, ShellCommand::Rm { .. }));
        let out = repl.execute_command(cmd).unwrap();
        assert_eq!(out, "Removed file: testfile.txt");
    }

    #[test]
    fn test_extended_cli_commands() {
        let mut repl = ShellRepl::new();

        // 1. Livepatch Command Test
        let cmd_livepatch = repl.parse_command("livepatch apply sys_read 8122c400 c0300100");
        assert!(matches!(cmd_livepatch, ShellCommand::Livepatch { .. }));
        let out_livepatch = repl.execute_command(cmd_livepatch).unwrap();
        assert!(out_livepatch.contains("Successfully registered"));

        // 2. Cron Command Test
        let cmd_cron = repl.parse_command("cron list");
        assert!(matches!(cmd_cron, ShellCommand::Cron { .. }));
        let out_cron = repl.execute_command(cmd_cron).unwrap();
        assert!(out_cron.contains("backup_job"));

        // 3. VM Command Test
        let cmd_vm = repl.parse_command("vm start Intel-VM");
        assert!(matches!(cmd_vm, ShellCommand::Vm { .. }));
        let out_vm = repl.execute_command(cmd_vm).unwrap();
        assert!(out_vm.contains("Starting VM"));

        // 4. Research Command Test
        let cmd_res = repl.parse_command("research Perplexity");
        assert!(matches!(cmd_res, ShellCommand::Research { .. }));
        let out_res = repl.execute_command(cmd_res).unwrap();
        assert!(out_res.contains("SYNTHESIZED ANSWER"));

        // 5. Camera Command Test
        let cmd_cam = repl.parse_command("camera Sepia");
        assert!(matches!(cmd_cam, ShellCommand::Camera { .. }));
        let out_cam = repl.execute_command(cmd_cam).unwrap();
        assert!(out_cam.contains("Webcam effect successfully updated"));
    }
}
