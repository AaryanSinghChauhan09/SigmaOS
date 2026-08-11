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

/// Simulated Shell Job for Job Control
#[derive(Debug, Clone)]
pub struct ShellJob {
    pub job_id: usize,
    pub pid: usize,
    pub name: String,
    pub state: crate::process::linux_proc::LinuxProcessState,
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
    Kill {
        signal: Option<i32>,
        pid: usize,
    },
    Nice {
        nice_val: i32,
        command_line: String,
    },
    Renice {
        nice_val: i32,
        pid: usize,
    },
    Pgrep {
        pattern: String,
    },
    Pkill {
        pattern: String,
    },
    Top,
    Vmstat,
    Spawn {
        name: String,
        cmdline: String,
    },
    Jobs,
    Fg {
        job_id: usize,
    },
    Bg {
        job_id: usize,
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
    pub proc_fs: crate::process::linux_proc::ProcFileSystem,
    pub jobs: Vec<ShellJob>,
    pub next_job_id: usize,
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
            proc_fs: crate::process::linux_proc::ProcFileSystem::new(),
            jobs: Vec::new(),
            next_job_id: 1,
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
            agent_engine: AgentAutomationEngine::new(),
            current_user: "ubuntu".to_string(),
            current_dir: "/home/ubuntu".to_string(),
            services,
            installed_packages: std::collections::HashSet::new(),
            current_theme: "default".to_string(),
            current_profile: "default".to_string(),
            a11y_features: std::collections::HashMap::new(),
            proc_fs: crate::process::linux_proc::ProcFileSystem::new(),
            jobs: Vec::new(),
            next_job_id: 1,
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

    pub fn execute_line(&mut self, line: &str) {
        let trimmed = line.trim();
        let is_background = trimmed.ends_with('&');
        let clean_line = if is_background {
            trimmed[..trimmed.len() - 1].trim().to_string()
        } else {
            trimmed.to_string()
        };

        let command = self.parse_command(&clean_line);

        if is_background {
            match command {
                ShellCommand::Spawn { ref name, ref cmdline } => {
                    let pid = self.proc_fs.spawn_process(
                        name,
                        1,
                        crate::process::linux_proc::NiceValue::new(0),
                        "user.slice",
                        cmdline,
                    );
                    let job_id = self.next_job_id;
                    self.next_job_id += 1;
                    self.jobs.push(ShellJob {
                        job_id,
                        pid,
                        name: format!("spawn {} {}", name, cmdline),
                        state: crate::process::linux_proc::LinuxProcessState::Running,
                    });
                    println!("[{}] {}", job_id, pid);
                }
                ShellCommand::Nice { nice_val, ref command_line } => {
                    let parsed_inner = self.parse_command(command_line);
                    match parsed_inner {
                        ShellCommand::Spawn { ref name, ref cmdline } => {
                            let nice_obj = crate::process::linux_proc::NiceValue::new(nice_val);
                            let pid = self.proc_fs.spawn_process(
                                name,
                                1,
                                nice_obj,
                                "user.slice",
                                cmdline,
                            );
                            let job_id = self.next_job_id;
                            self.next_job_id += 1;
                            self.jobs.push(ShellJob {
                                job_id,
                                pid,
                                name: format!("nice {} spawn {} {}", nice_val, name, cmdline),
                                state: crate::process::linux_proc::LinuxProcessState::Running,
                            });
                            println!("[{}] {}", job_id, pid);
                        }
                        _ => {
                            println!("nice: commands other than spawn cannot be backgrounded");
                        }
                    }
                }
                _ => {
                    // For standard commands, we simulate backgrounding by creating a dummy PID
                    let dummy_pid = self.proc_fs.spawn_process(
                        "bg-job",
                        1,
                        crate::process::linux_proc::NiceValue::new(0),
                        "user.slice",
                        &clean_line,
                    );
                    let job_id = self.next_job_id;
                    self.next_job_id += 1;
                    self.jobs.push(ShellJob {
                        job_id,
                        pid: dummy_pid,
                        name: clean_line.clone(),
                        state: crate::process::linux_proc::LinuxProcessState::Running,
                    });
                    println!("[{}] {}", job_id, dummy_pid);
                }
            }
        } else {
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
            "top" => ShellCommand::Top,
            "vmstat" => ShellCommand::Vmstat,
            "jobs" => ShellCommand::Jobs,
            "fg" => {
                if parts.len() >= 2 {
                    let job_id = parts[1].trim_start_matches('%').parse::<usize>().unwrap_or(0);
                    ShellCommand::Fg { job_id }
                } else {
                    ShellCommand::Unknown(input.to_string())
                }
            }
            "bg" => {
                if parts.len() >= 2 {
                    let job_id = parts[1].trim_start_matches('%').parse::<usize>().unwrap_or(0);
                    ShellCommand::Bg { job_id }
                } else {
                    ShellCommand::Unknown(input.to_string())
                }
            }
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
            "cat" => {
                if parts.len() >= 2 {
                    ShellCommand::Cat {
                        filename: parts[1].to_string(),
                    }
                } else {
                    ShellCommand::Unknown(input.to_string())
                }
            }
            "echo" => {
                ShellCommand::Echo {
                    message: parts[1..].join(" "),
                }
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
                    ShellCommand::Apt { subcommand, package }
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
            "kill" => {
                if parts.len() == 3 && parts[1].starts_with('-') {
                    let sig_str = parts[1].trim_start_matches('-');
                    let signal = sig_str.parse::<i32>().ok();
                    let pid = parts[2].parse::<usize>().unwrap_or(0);
                    ShellCommand::Kill { signal, pid }
                } else if parts.len() == 2 {
                    let pid = parts[1].parse::<usize>().unwrap_or(0);
                    ShellCommand::Kill { signal: None, pid }
                } else {
                    ShellCommand::Unknown(input.to_string())
                }
            }
            "nice" => {
                if parts.len() >= 4 && parts[1] == "-n" {
                    let nice_val = parts[2].parse::<i32>().unwrap_or(0);
                    let command_line = parts[3..].join(" ");
                    ShellCommand::Nice { nice_val, command_line }
                } else if parts.len() >= 3 && parts[1].parse::<i32>().is_ok() {
                    let nice_val = parts[1].parse::<i32>().unwrap_or(0);
                    let command_line = parts[2..].join(" ");
                    ShellCommand::Nice { nice_val, command_line }
                } else if parts.len() >= 2 {
                    let command_line = parts[1..].join(" ");
                    ShellCommand::Nice { nice_val: 10, command_line }
                } else {
                    ShellCommand::Unknown(input.to_string())
                }
            }
            "renice" => {
                if parts.len() == 3 {
                    let nice_val = parts[1].parse::<i32>().unwrap_or(0);
                    let pid = parts[2].parse::<usize>().unwrap_or(0);
                    ShellCommand::Renice { nice_val, pid }
                } else {
                    ShellCommand::Unknown(input.to_string())
                }
            }
            "pgrep" => {
                if parts.len() >= 2 {
                    ShellCommand::Pgrep {
                        pattern: parts[1].to_string(),
                    }
                } else {
                    ShellCommand::Unknown(input.to_string())
                }
            }
            "pkill" => {
                if parts.len() >= 2 {
                    ShellCommand::Pkill {
                        pattern: parts[1].to_string(),
                    }
                } else {
                    ShellCommand::Unknown(input.to_string())
                }
            }
            "spawn" => {
                if parts.len() >= 3 {
                    let name = parts[1].to_string();
                    let cmdline = parts[2..].join(" ");
                    ShellCommand::Spawn { name, cmdline }
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
                   help                - Show this help message\n\
                   ps                  - List running processes dynamically\n\
                   ls                  - List files\n\
                   pwd                 - Print working directory\n\
                   whoami              - Print current logged-in user\n\
                   su <user>           - Switch user account (try 'su root' or 'su guest')\n\
                   cat <file>          - Display file contents\n\
                   systemctl           - Manage systemd services (try 'systemctl list' or 'systemctl status <service>')\n\
                   apt <cmd>           - Advanced Package Tool (try 'apt update', 'apt search <pkg>', or 'apt install <pkg>')\n\
                   echo                - Print a message\n\
                   set                 - Set a variable\n\
                   get                 - Get a variable\n\
                   kill [-<sig>] <pid> - Send signal to a simulated process\n\
                   nice <val> <cmd>    - Run a simulated command with modified nice priority\n\
                   renice <val> <pid>  - Modify nice value of a simulated process\n\
                   pgrep <pattern>     - Find simulated processes by name pattern\n\
                   pkill <pattern>     - Signal simulated processes by name pattern\n\
                   top                 - Display snapshot-style dynamic process monitor\n\
                   vmstat              - Display virtual memory and CPU core statistics\n\
                   spawn <name> <cmd>  - Spawn a custom simulated process\n\
                   jobs                - List active background jobs\n\
                   fg <job_id>         - Bring background job to foreground\n\
                   bg <job_id>         - Resume stopped job in the background\n\
                   exit                - Exit the shell"
                .to_string()),
            ShellCommand::ListProcesses => {
                let mut out = "PID   PPID  PGID  SID   NAME         STATE         NICE  CGROUP\n".to_string();
                let mut sorted_pids: Vec<&usize> = self.proc_fs.processes.keys().collect();
                sorted_pids.sort();
                for pid in sorted_pids {
                    if let Some(proc) = self.proc_fs.processes.get(pid) {
                        out.push_str(&format!(
                            "{:<5} {:<5} {:<5} {:<5} {:<12} {:<13} {:<5} {}\n",
                            proc.pid,
                            proc.ppid,
                            proc.pgid,
                            proc.sid,
                            proc.name,
                            proc.state.as_str(),
                            proc.nice.value(),
                            proc.cgroup_name
                        ));
                    }
                }
                Ok(out)
            }
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
            ShellCommand::Echo { message } => Ok(message),
            ShellCommand::Set { variable, value } => {
                self.variables.insert(variable.clone(), value.clone());
                Ok(format!("{} = {}", variable, value))
            }
            ShellCommand::Get { variable } => match self.variables.get(&variable) {
                Some(value) => Ok(value.clone()),
                None => Err(format!("Variable '{}' not found", variable)),
            },
            ShellCommand::Kill { signal, pid } => {
                let raw_sig = signal.unwrap_or(15);
                let native_sig = match raw_sig {
                    9 => crate::process::linux_proc::LinuxSignal::SigKill,
                    15 => crate::process::linux_proc::LinuxSignal::SigTerm,
                    2 => crate::process::linux_proc::LinuxSignal::SigInt,
                    _ => crate::process::linux_proc::LinuxSignal::SigTerm,
                };
                match self.proc_fs.send_signal(pid as i32, native_sig) {
                    Ok(_) => {
                        // Update background job state if matched
                        if let Some(job) = self.jobs.iter_mut().find(|j| j.pid == pid) {
                            if raw_sig == 9 || raw_sig == 15 {
                                job.state = crate::process::linux_proc::LinuxProcessState::Terminated;
                            } else if raw_sig == 2 {
                                job.state = crate::process::linux_proc::LinuxProcessState::Stopped;
                            }
                        }
                        Ok(format!("Sent signal {} to process {}", raw_sig, pid))
                    }
                    Err(e) => Err(format!("kill: failed to signal {}: {}", pid, e)),
                }
            }
            ShellCommand::Nice { nice_val, command_line } => {
                let parsed_inner = self.parse_command(&command_line);
                match parsed_inner {
                    ShellCommand::Spawn { name, cmdline } => {
                        let nice_obj = crate::process::linux_proc::NiceValue::new(nice_val);
                        let pid = self.proc_fs.spawn_process(&name, 1, nice_obj, "user.slice", &cmdline);
                        Ok(format!("nice: Spawned process {} (PID {}) with nice priority {}", name, pid, nice_val))
                    }
                    _ => {
                        // Simulate running standard command under nice
                        let result = self.execute_command(parsed_inner)?;
                        Ok(format!("nice ({}): {}", nice_val, result))
                    }
                }
            }
            ShellCommand::Renice { nice_val, pid } => {
                if let Some(proc) = self.proc_fs.processes.get_mut(&pid) {
                    proc.nice = crate::process::linux_proc::NiceValue::new(nice_val);
                    Ok(format!("Successfully reniced process {} to {}", pid, nice_val))
                } else {
                    Err(format!("renice: process {} not found", pid))
                }
            }
            ShellCommand::Pgrep { pattern } => {
                let mut matches = Vec::new();
                for proc in self.proc_fs.processes.values() {
                    if proc.name.contains(&pattern) {
                        matches.push(proc.pid.to_string());
                    }
                }
                if matches.is_empty() {
                    Ok(String::new())
                } else {
                    Ok(matches.join("\n"))
                }
            }
            ShellCommand::Pkill { pattern } => {
                let mut signaled = Vec::new();
                let pids: Vec<usize> = self.proc_fs.processes.keys().copied().collect();
                for pid in pids {
                    let name = {
                        if let Some(proc) = self.proc_fs.processes.get(&pid) {
                            proc.name.clone()
                        } else {
                            continue;
                        }
                    };
                    if name.contains(&pattern) {
                        if self.proc_fs.send_signal(pid as i32, crate::process::linux_proc::LinuxSignal::SigTerm).is_ok() {
                            signaled.push(format!("{} ({})", name, pid));
                            if let Some(job) = self.jobs.iter_mut().find(|j| j.pid == pid) {
                                job.state = crate::process::linux_proc::LinuxProcessState::Terminated;
                            }
                        }
                    }
                }
                if signaled.is_empty() {
                    Ok("No matching processes found to signal.".to_string())
                } else {
                    Ok(format!("Signaled processes: {}", signaled.join(", ")))
                }
            }
            ShellCommand::Top => {
                let mut out = String::new();
                out.push_str(&format!(
                    "Uptime: {} seconds | Cores: {} | Model: {}\n",
                    self.proc_fs.system_uptime, self.proc_fs.cpu_cores, self.proc_fs.cpu_model
                ));
                let free_mem = self.proc_fs.total_memory - self.proc_fs.used_memory;
                out.push_str(&format!(
                    "Memory: {} kB total, {} kB used, {} kB free\n\n",
                    self.proc_fs.total_memory / 1024, self.proc_fs.used_memory / 1024, free_mem / 1024
                ));
                out.push_str("PID   PPID  NAME         STATE         NICE  MEMORY_USAGE   CPU_TIME\n");
                let mut sorted_pids: Vec<&usize> = self.proc_fs.processes.keys().collect();
                sorted_pids.sort();
                for pid in sorted_pids {
                    if let Some(proc) = self.proc_fs.processes.get(pid) {
                        out.push_str(&format!(
                            "{:<5} {:<5} {:<12} {:<13} {:<5} {:<14} {}\n",
                            proc.pid,
                            proc.ppid,
                            proc.name,
                            proc.state.as_str(),
                            proc.nice.value(),
                            format!("{} B", proc.memory_usage),
                            proc.cpu_time
                        ));
                    }
                }
                Ok(out)
            }
            ShellCommand::Vmstat => {
                let free_kb = (self.proc_fs.total_memory - self.proc_fs.used_memory) / 1024;
                let out = format!(
                    "procs -----------memory---------- ---swap-- -----io---- -system-- ------cpu-----\n\
                     r  b   swpd   free   buff  cache   si   so    bi    bo   in   cs us sy id wa st\n\
                     1  0      0 {:<8} 131072 2097152   0    0    42     0    0    0 15  5 80  0  0\n",
                    free_kb
                );
                Ok(out)
            }
            ShellCommand::Spawn { name, cmdline } => {
                let pid = self.proc_fs.spawn_process(&name, 1, crate::process::linux_proc::NiceValue::new(0), "user.slice", &cmdline);
                Ok(format!("Spawned process {} (PID {}) successfully.", name, pid))
            }
            ShellCommand::Jobs => {
                let mut out = String::new();
                for job in &self.jobs {
                    out.push_str(&format!(
                        "[{}] {} ({})        {}\n",
                        job.job_id,
                        job.state.as_str(),
                        job.pid,
                        job.name
                    ));
                }
                if out.is_empty() {
                    Ok("No active background jobs.".to_string())
                } else {
                    Ok(out.trim_end().to_string())
                }
            }
            ShellCommand::Fg { job_id } => {
                let job_idx = self.jobs.iter().position(|j| job_id == j.job_id);
                if let Some(idx) = job_idx {
                    let mut job = self.jobs.remove(idx);
                    if job.state == crate::process::linux_proc::LinuxProcessState::Stopped {
                        if let Some(proc) = self.proc_fs.processes.get_mut(&job.pid) {
                            proc.state = crate::process::linux_proc::LinuxProcessState::Running;
                            for thread in &mut proc.threads {
                                thread.state = crate::process::linux_proc::LinuxProcessState::Running;
                            }
                        }
                        job.state = crate::process::linux_proc::LinuxProcessState::Running;
                    }
                    self.proc_fs.simulate_scheduler_tick();
                    Ok(format!("Brought job [{}] '{}' (PID {}) to the foreground. Process completed.", job.job_id, job.name, job.pid))
                } else {
                    Err(format!("fg: job [{}] not found", job_id))
                }
            }
            ShellCommand::Bg { job_id } => {
                let job = self.jobs.iter_mut().find(|j| job_id == j.job_id);
                if let Some(j) = job {
                    if j.state == crate::process::linux_proc::LinuxProcessState::Stopped {
                        if let Some(proc) = self.proc_fs.processes.get_mut(&j.pid) {
                            proc.state = crate::process::linux_proc::LinuxProcessState::Running;
                            for thread in &mut proc.threads {
                                thread.state = crate::process::linux_proc::LinuxProcessState::Running;
                            }
                        }
                        j.state = crate::process::linux_proc::LinuxProcessState::Running;
                        Ok(format!("Resumed job [{}] '{}' in the background.", j.job_id, j.name))
                    } else {
                        Ok(format!("job [{}] '{}' is already running in background.", j.job_id, j.name))
                    }
                } else {
                    Err(format!("bg: job [{}] not found", job_id))
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
    fn test_process_management_commands() {
        let mut repl = ShellRepl::new();

        // 1. Spawn a dynamic process
        let spawn_out = repl.execute_command(ShellCommand::Spawn {
            name: "test-daemon".to_string(),
            cmdline: "/bin/test-daemon --run".to_string(),
        }).unwrap();
        assert!(spawn_out.contains("test-daemon"));

        // 2. Query ps list and verify presence
        let ps_out = repl.execute_command(ShellCommand::ListProcesses).unwrap();
        assert!(ps_out.contains("test-daemon"));

        // 3. Renice process
        let renice_out = repl.execute_command(ShellCommand::Renice {
            nice_val: -12,
            pid: 3, // Default systemd: 1, kthreadd: 2, test-daemon: 3
        }).unwrap();
        assert!(renice_out.contains("reniced"));
        assert!(repl.execute_command(ShellCommand::ListProcesses).unwrap().contains("-12"));

        // 4. Test pgrep
        let pgrep_out = repl.execute_command(ShellCommand::Pgrep {
            pattern: "test".to_string(),
        }).unwrap();
        assert_eq!(pgrep_out.trim(), "3");

        // 5. Test nice prefix runner
        let nice_run_out = repl.execute_command(ShellCommand::Nice {
            nice_val: 5,
            command_line: "spawn nice-daemon nice_cmd".to_string(),
        }).unwrap();
        assert!(nice_run_out.contains("nice-daemon"));
        assert!(repl.execute_command(ShellCommand::ListProcesses).unwrap().contains("nice-daemon"));

        // 6. Test top snapshot
        let top_out = repl.execute_command(ShellCommand::Top).unwrap();
        assert!(top_out.contains("Memory:"));
        assert!(top_out.contains("nice-daemon"));

        // 7. Test vmstat snapshot
        let vmstat_out = repl.execute_command(ShellCommand::Vmstat).unwrap();
        assert!(vmstat_out.contains("swpd"));

        // 8. Test pkill / kill
        let pkill_out = repl.execute_command(ShellCommand::Pkill {
            pattern: "nice-daemon".to_string(),
        }).unwrap();
        assert!(pkill_out.contains("nice-daemon"));
    }

    #[test]
    fn test_job_control_and_bg_execution() {
        let mut repl = ShellRepl::new();

        // 1. Execute spawn command in background ending with '&'
        repl.execute_line("spawn background-daemon --background &");
        assert_eq!(repl.jobs.len(), 1);
        let first_job_pid = repl.jobs[0].pid;
        assert_eq!(repl.jobs[0].job_id, 1);
        assert_eq!(repl.jobs[0].state, crate::process::linux_proc::LinuxProcessState::Running);

        // 2. Query jobs list
        let jobs_out = repl.execute_command(ShellCommand::Jobs).unwrap();
        assert!(jobs_out.contains("background-daemon"));

        // 3. Send SIGINT simulation to stop the background process
        repl.execute_command(ShellCommand::Kill {
            signal: Some(2),
            pid: first_job_pid,
        }).unwrap();
        assert_eq!(repl.jobs[0].state, crate::process::linux_proc::LinuxProcessState::Stopped);

        // 4. Resume job in background using bg
        let bg_out = repl.execute_command(ShellCommand::Bg { job_id: 1 }).unwrap();
        assert!(bg_out.contains("Resumed"));
        assert_eq!(repl.jobs[0].state, crate::process::linux_proc::LinuxProcessState::Running);

        // 5. Bring job to foreground using fg
        let fg_out = repl.execute_command(ShellCommand::Fg { job_id: 1 }).unwrap();
        assert!(fg_out.contains("Brought"));
        // Brought to foreground removes it from background jobs queue
        assert_eq!(repl.jobs.len(), 0);
    }
}
