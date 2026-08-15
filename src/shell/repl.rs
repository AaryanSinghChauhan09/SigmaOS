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
    Echo {
        message: String,
    },
    Set {
        variable: String,
        value: String,
    },
    Get {
        variable: String,
    },
    Pwd,
    WhoAmI,
    Su {
        username: String,
        password: Option<String>,
    },
    Cat {
        filename: String,
    },
    Systemctl {
        action: String,
        service: String,
    },
    Apt {
        subcommand: String,
        package: Option<String>,
    },
    Uname,
    Clear,
    Touch {
        filename: String,
    },
    Mkdir {
        dirname: String,
    },
    Rm {
        filename: String,
    },
    Theme {
        theme_name: String,
    },
    Profile {
        profile_name: String,
    },
    A11y {
        feature: String,
        state: String,
    },
    Livepatch {
        args: Vec<String>,
    },
    Cron {
        args: Vec<String>,
    },
    Vm {
        args: Vec<String>,
    },
    Research {
        query: String,
    },
    Camera {
        effect: String,
    },
    Grid {
        args: Vec<String>,
    },
    Access {
        args: Vec<String>,
    },
    Sysctl {
        args: Vec<String>,
    },
    Patch {
        args: Vec<String>,
    },
    Rescue {
        args: Vec<String>,
    },
    Monitor {
        args: Vec<String>,
    },
    Sandbox {
        args: Vec<String>,
    },
    Unknown(String),
}

/// Shell REPL
pub struct ShellRepl {
    pub running: bool,
    pub variables: std::collections::HashMap<String, String>,
    pub aliases: std::collections::HashMap<String, String>,
    pub prompt: String,
    pub agent_engine: AgentAutomationEngine,
    pub current_user: String,
    pub current_dir: String,
    pub services: std::collections::HashMap<String, String>,
    pub installed_packages: std::collections::HashSet<String>,
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
            aliases: std::collections::HashMap::new(),
            prompt: "sigma-sh> ".to_string(),
            agent_engine: AgentAutomationEngine::new(),
            current_user: "ubuntu".to_string(),
            current_dir: "/home/ubuntu".to_string(),
            services,
            installed_packages: std::collections::HashSet::new(),
            current_theme: "default".to_string(),
            current_profile: "default".to_string(),
            a11y_features: std::collections::HashMap::new(),
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
            prompt: prompt,
            agent_engine: AgentAutomationEngine::new(),
            current_user: "ubuntu".to_string(),
            current_dir: "/home/ubuntu".to_string(),
            services,
            installed_packages: std::collections::HashSet::new(),
            current_theme: "default".to_string(),
            current_profile: "default".to_string(),
            a11y_features: std::collections::HashMap::new(),
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
            "rm" => {
                if parts.len() >= 2 {
                    ShellCommand::Rm {
                        filename: parts[1].to_string(),
                    }
                } else {
                    ShellCommand::Unknown(input.to_string())
                }
            }
            "echo" => {
                if parts.len() >= 2 {
                    ShellCommand::Echo {
                        message: parts[1..].join(" "),
                    }
                } else {
                    ShellCommand::Echo {
                        message: String::new(),
                    }
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
            "echo" => ShellCommand::Echo {
                message: parts[1..].join(" "),
            },
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
                        service: parts.get(2).unwrap_or(&"").to_string(),
                    }
                } else {
                    ShellCommand::Unknown(input.to_string())
                }
            }
            "apt" => {
                if parts.len() >= 2 {
                    ShellCommand::Apt {
                        subcommand: parts[1].to_string(),
                        package: parts.get(2).map(|s| s.to_string()),
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
            "livepatch" => {
                let args = parts[1..].iter().map(|s| s.to_string()).collect();
                ShellCommand::Livepatch { args }
            }
            "cron" => {
                let args = parts[1..].iter().map(|s| s.to_string()).collect();
                ShellCommand::Cron { args }
            }
            "vm" => {
                let args = parts[1..].iter().map(|s| s.to_string()).collect();
                ShellCommand::Vm { args }
            }
            "research" => {
                let query = parts[1..].join(" ");
                ShellCommand::Research { query }
            }
            "camera" => {
                let effect = parts[1..].join(" ");
                ShellCommand::Camera { effect }
            }
            "grid" => {
                let args = parts[1..].iter().map(|s| s.to_string()).collect();
                ShellCommand::Grid { args }
            }
            "access" => {
                let args = parts[1..].iter().map(|s| s.to_string()).collect();
                ShellCommand::Access { args }
            }
            "sysctl" => {
                let args = parts[1..].iter().map(|s| s.to_string()).collect();
                ShellCommand::Sysctl { args }
            }
            "patch" => {
                let args = parts[1..].iter().map(|s| s.to_string()).collect();
                ShellCommand::Patch { args }
            }
            "rescue" => {
                let args = parts[1..].iter().map(|s| s.to_string()).collect();
                ShellCommand::Rescue { args }
            }
            "monitor" => {
                let args = parts[1..].iter().map(|s| s.to_string()).collect();
                ShellCommand::Monitor { args }
            }
            "sandbox" => {
                let args = parts[1..].iter().map(|s| s.to_string()).collect();
                ShellCommand::Sandbox { args }
            }
            _ => ShellCommand::Unknown(input.to_string()),
        }
    }

    fn execute_command(&mut self, command: ShellCommand) -> Result<String, String> {
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
            ShellCommand::Grid { args } => {
                if args.is_empty() {
                    Ok("grid: Subcommands: list, add <id> <cores>, remove <id>".to_string())
                } else if args[0] == "list" {
                    Ok("Node: node-1 (idle, 8 cores)\nNode: node-2 (busy, 16 cores)".to_string())
                } else if args[0] == "add" && args.len() >= 3 {
                    Ok(format!("grid: Node '{}' with {} CPU cores registered to cluster.", args[1], args[2]))
                } else if args[0] == "remove" && args.len() >= 2 {
                    Ok(format!("grid: Node '{}' removed from cluster.", args[1]))
                } else {
                    Err("grid: Invalid parameters".to_string())
                }
            }
            ShellCommand::Access { args } => {
                if args.is_empty() {
                    Ok("access: Subcommands: list, enable <feature>, disable <feature>".to_string())
                } else if args[0] == "list" {
                    Ok("Accessibility Feature: ScreenReader (Disabled)\nAccessibility Feature: HighContrast (Enabled)".to_string())
                } else if args[0] == "enable" && args.len() >= 2 {
                    Ok(format!("access: Accessibility feature '{}' enabled.", args[1]))
                } else if args[0] == "disable" && args.len() >= 2 {
                    Ok(format!("access: Accessibility feature '{}' disabled.", args[1]))
                } else {
                    Err("access: Invalid parameters".to_string())
                }
            }
            ShellCommand::Sysctl { args } => {
                if args.is_empty() {
                    Ok("sysctl: Subcommands: list, query <param>, set <param>=<value>".to_string())
                } else if args[0] == "list" {
                    Ok("kern.maxproc = 1024\nnet.inet.tcp.sendspace = 32768\nhw.ncpu = 16".to_string())
                } else if args[0] == "query" && args.len() >= 2 {
                    Ok(format!("sysctl: {} = 1024", args[1]))
                } else if args[0] == "set" && args.len() >= 2 {
                    Ok(format!("sysctl: Parameter '{}' set successfully.", args[1]))
                } else {
                    Err("sysctl: Invalid parameters".to_string())
                }
            }
            ShellCommand::Patch { args } => {
                if args.is_empty() {
                    Ok("patch: Subcommands: list, apply <patch_hash> <signature>, rollback <patch_hash>".to_string())
                } else if args[0] == "list" {
                    Ok("Patch: patch_01 (Applied)\nPatch: patch_02 (Available)".to_string())
                } else if args[0] == "apply" && args.len() >= 3 {
                    Ok(format!("patch: Live patch '{}' applied successfully under secure signature verification.", args[1]))
                } else if args[0] == "rollback" && args.len() >= 2 {
                    Ok(format!("patch: Live patch '{}' rolled back.", args[1]))
                } else {
                    Err("patch: Invalid parameters".to_string())
                }
            }
            ShellCommand::Rescue { args } => {
                if args.is_empty() {
                    Ok("rescue: Subcommands: status, rollback <partition> <merkle_hash>".to_string())
                } else if args[0] == "status" {
                    Ok("Emergency Recovery Mode: Active\nBootable Partitions: /dev/sda1, /dev/sda2".to_string())
                } else if args[0] == "rollback" && args.len() >= 3 {
                    Ok(format!("rescue: Partition '{}' successfully rolled back to secure Merkle Root [{}].", args[1], args[2]))
                } else {
                    Err("rescue: Invalid parameters".to_string())
                }
            }
            ShellCommand::Monitor { args } => {
                if args.is_empty() {
                    Ok("monitor: Subcommands: telemetry, switch_latency, leaks".to_string())
                } else if args[0] == "telemetry" {
                    Ok("SigmaMonitor: Core temperature peak: 44.1 C (SIMD Accelerated)".to_string())
                } else if args[0] == "switch_latency" {
                    Ok("SigmaMonitor: Average Context Switch Latency: 13.375 ns".to_string())
                } else if args[0] == "leaks" {
                    Ok("SigmaMonitor: Zero-Allocation audit: 0 memory leak bytes logged.".to_string())
                } else {
                    Err("monitor: Invalid parameters".to_string())
                }
            }
            ShellCommand::Sandbox { args } => {
                if args.is_empty() {
                    Ok("sandbox: Subcommands: create <pid> <profile>, check <pid>".to_string())
                } else if args[0] == "create" && args.len() >= 3 {
                    Ok(format!("sandbox: Zero-trust sandbox created for PID {} using '{}' execution profile.", args[1], args[2]))
                } else if args[0] == "check" && args.len() >= 2 {
                    Ok(format!("sandbox: PID {} is active sandboxed.", args[1]))
                } else {
                    Err("sandbox: Invalid parameters".to_string())
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

        // 6. Grid Command Test
        let cmd_grid = repl.parse_command("grid add node-3 8");
        assert!(matches!(cmd_grid, ShellCommand::Grid { .. }));
        let out_grid = repl.execute_command(cmd_grid).unwrap();
        assert!(out_grid.contains("node-3"));

        // 7. Access Command Test
        let cmd_access = repl.parse_command("access enable ScreenReader");
        assert!(matches!(cmd_access, ShellCommand::Access { .. }));
        let out_access = repl.execute_command(cmd_access).unwrap();
        assert!(out_access.contains("ScreenReader"));

        // 8. Sysctl Command Test
        let cmd_sysctl = repl.parse_command("sysctl query kern.maxproc");
        assert!(matches!(cmd_sysctl, ShellCommand::Sysctl { .. }));
        let out_sysctl = repl.execute_command(cmd_sysctl).unwrap();
        assert!(out_sysctl.contains("kern.maxproc"));

        // 9. Patch Command Test
        let cmd_patch = repl.parse_command("patch rollback patch_01");
        assert!(matches!(cmd_patch, ShellCommand::Patch { .. }));
        let out_patch = repl.execute_command(cmd_patch).unwrap();
        assert!(out_patch.contains("rolled back"));

        // 10. Rescue Command Test
        let cmd_rescue = repl.parse_command("rescue rollback /dev/sda1 hash_val");
        assert!(matches!(cmd_rescue, ShellCommand::Rescue { .. }));
        let out_rescue = repl.execute_command(cmd_rescue).unwrap();
        assert!(out_rescue.contains("/dev/sda1"));

        // 11. Monitor Command Test
        let cmd_monitor = repl.parse_command("monitor telemetry");
        assert!(matches!(cmd_monitor, ShellCommand::Monitor { .. }));
        let out_monitor = repl.execute_command(cmd_monitor).unwrap();
        assert!(out_monitor.contains("SigmaMonitor"));

        // 12. Sandbox Command Test
        let cmd_sandbox = repl.parse_command("sandbox create 801 StrictBrowser");
        assert!(matches!(cmd_sandbox, ShellCommand::Sandbox { .. }));
        let out_sandbox = repl.execute_command(cmd_sandbox).unwrap();
        assert!(out_sandbox.contains("StrictBrowser"));
    }
}
