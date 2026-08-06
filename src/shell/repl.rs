// SigmaOS Shell REPL (Read-Eval-Print Loop)
// Interactive shell for SigmaOS

use std::io::{self, BufRead, Write}; // Userspace I/O is acceptable

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
    Touch {
        filename: String,
    },
    Mkdir {
        dirname: String,
    },
    Rm {
        filename: String,
    },
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
    Alias {
        name: String,
        value: String,
    },
    Unalias {
        name: String,
    },
    Run {
        variable: String,
    },
    AgentList,
    AgentRegister {
        description: String,
        commands: String,
    },
    AgentRun {
        task_id: usize,
    },
    Gst {
        basic_cost: f64,
        rate: u32,
    },
    Upi {
        vpa: String,
        amount: usize,
    },
    Translate {
        lang: String,
        key: String,
    },
    PeValidate {
        hex_stub: String,
    },
    VmStep {
        hex_bytecode: String,
    },
    Proc {
        args: Vec<String>,
    },
    Unknown(String),
}

/// Represents an automated action task executed by an AI agent
#[derive(Debug, Clone)]
pub struct AgentTask {
    pub task_id: usize,
    pub description: String,
    pub commands: Vec<String>,
}

/// AI Agent Automation Engine inside SigmaOS REPL
#[derive(Debug, Clone)]
pub struct AgentAutomationEngine {
    pub registered_tasks: std::collections::HashMap<usize, AgentTask>,
    pub next_task_id: usize,
}

impl AgentAutomationEngine {
    pub fn new() -> Self {
        AgentAutomationEngine {
            registered_tasks: std::collections::HashMap::new(),
            next_task_id: 1,
        }
    }

    pub fn register_task(&mut self, description: String, commands: Vec<String>) -> usize {
        let id = self.next_task_id;
        self.next_task_id += 1;
        self.registered_tasks.insert(
            id,
            AgentTask {
                task_id: id,
                description,
                commands,
            },
        );
        id
    }
}

impl Default for AgentAutomationEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Shell REPL
pub struct ShellRepl {
    running: bool,
    variables: std::collections::HashMap<String, String>,
    aliases: std::collections::HashMap<String, String>,
    prompt: String,
    agent_engine: AgentAutomationEngine,
    pub current_dir: String,
    pub current_user: String,
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
            current_dir: "/home/user".to_string(),
            current_user: "user".to_string(),
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
            prompt,
            agent_engine: AgentAutomationEngine::new(),
            current_dir: "/home/user".to_string(),
            current_user: "user".to_string(),
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

    pub fn parse_command(&self, input: &str) -> ShellCommand {
        let mut expanded_input = input.to_string();
        let first_word = input.split_whitespace().next().unwrap_or("");
        if let Some(alias_value) = self.aliases.get(first_word) {
            let rest = if input.len() > first_word.len() {
                &input[first_word.len()..]
            } else {
                ""
            };
            expanded_input = format!("{}{}", alias_value, rest);
        }

        let parts: Vec<&str> = expanded_input.split_whitespace().collect();

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
            "agent" => {
                if parts.len() >= 2 {
                    match parts[1] {
                        "list" => ShellCommand::AgentList,
                        "run" => {
                            if parts.len() >= 3 {
                                if let Ok(id) = parts[2].parse::<usize>() {
                                    ShellCommand::AgentRun { task_id: id }
                                } else {
                                    ShellCommand::Unknown(input.to_string())
                                }
                            } else {
                                ShellCommand::Unknown(input.to_string())
                            }
                        }
                        "register" => {
                            if parts.len() >= 4 {
                                let desc = parts[2].to_string();
                                let cmds = parts[3..].join(" ");
                                ShellCommand::AgentRegister {
                                    description: desc,
                                    commands: cmds,
                                }
                            } else {
                                ShellCommand::Unknown(input.to_string())
                            }
                        }
                        _ => ShellCommand::Unknown(input.to_string()),
                    }
                } else {
                    ShellCommand::Unknown(input.to_string())
                }
            }
            "gst" => {
                if parts.len() >= 3 {
                    if let (Ok(cost), Ok(rate)) = (parts[1].parse::<f64>(), parts[2].parse::<u32>())
                    {
                        ShellCommand::Gst {
                            basic_cost: cost,
                            rate,
                        }
                    } else {
                        ShellCommand::Unknown(input.to_string())
                    }
                } else {
                    ShellCommand::Unknown(input.to_string())
                }
            }
            "upi" => {
                if parts.len() >= 3 {
                    if let Ok(amount) = parts[2].parse::<usize>() {
                        ShellCommand::Upi {
                            vpa: parts[1].to_string(),
                            amount,
                        }
                    } else {
                        ShellCommand::Unknown(input.to_string())
                    }
                } else {
                    ShellCommand::Unknown(input.to_string())
                }
            }
            "translate" => {
                if parts.len() >= 3 {
                    ShellCommand::Translate {
                        lang: parts[1].to_string(),
                        key: parts[2].to_string(),
                    }
                } else {
                    ShellCommand::Unknown(input.to_string())
                }
            }
            "pe" => {
                if parts.len() >= 2 {
                    ShellCommand::PeValidate {
                        hex_stub: parts[1].to_string(),
                    }
                } else {
                    ShellCommand::Unknown(input.to_string())
                }
            }
            "vm" => {
                if parts.len() >= 2 {
                    ShellCommand::VmStep {
                        hex_bytecode: parts[1].to_string(),
                    }
                } else {
                    ShellCommand::Unknown(input.to_string())
                }
            }
            "proc" => {
                let args_vec = parts[1..].iter().map(|s| s.to_string()).collect();
                ShellCommand::Proc { args: args_vec }
            }
            _ => ShellCommand::Unknown(input.to_string()),
        }
    }

    pub fn execute_command(&mut self, command: ShellCommand) -> Result<String, String> {
        match command {
            ShellCommand::Help => Ok("Available commands:\n\
                   help      - Show this help message\n\
                   ps        - List running processes\n\
                   proc      - Access Linux-inspired ProcFS (e.g. proc cat /proc/meminfo)\n\
                   ls        - List files\n\
                   echo      - Print a message\n\
                   set       - Set a variable\n\
                   get       - Get a variable\n\
                   alias     - Create a command shortcut/alias\n\
                   unalias   - Remove an alias\n\
                   run       - Execute an automated macro/script variable\n\
                   agent     - Interface for AI Agent Automation tasks (register, list, run)\n\
                   gst       - Perform sovereign India-first Goods and Services Tax Calculations\n\
                   upi       - Generate sovereign Indian UPI virtual payment paylink\n\
                   translate - Retrieve official Sanskrit/Hindi/Tamil Indic translations\n\
                   pe        - Validate Windows NT PE and DOS MZ binary headers\n\
                   vm        - Execute instruction-deterministic micro-virtualization steps\n\
                   exit      - Exit the shell"
                .to_string()),
            ShellCommand::ListProcesses => {
                use crate::process::ProcFileSystem;
                let pfs = ProcFileSystem::new();
                let mut out = "PID  NAME             STATE\n".to_string();
                let mut keys: Vec<&usize> = pfs.processes.keys().collect();
                keys.sort();
                for pid in keys {
                    let proc = pfs.processes.get(pid).unwrap();
                    out.push_str(&format!("{:<5} {:<16} {}\n", proc.pid, proc.name, proc.state.as_str()));
                }
                Ok(out)
            },
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
            ShellCommand::AgentRegister {
                description,
                commands,
            } => {
                let cmd_list: Vec<String> =
                    commands.split(';').map(|s| s.trim().to_string()).collect();
                let id = self
                    .agent_engine
                    .register_task(description.clone(), cmd_list);
                Ok(format!(
                    "Agent task #{} registered successfully: {}",
                    id, description
                ))
            }
            ShellCommand::AgentList => {
                if self.agent_engine.registered_tasks.is_empty() {
                    Ok("No agent automation tasks registered.".to_string())
                } else {
                    let mut list_str = "Registered Agent Automation Tasks:\n".to_string();
                    for (id, task) in &self.agent_engine.registered_tasks {
                        list_str.push_str(&format!(
                            "  [#{}] {} (Commands: {})\n",
                            id,
                            task.description,
                            task.commands.join("; ")
                        ));
                    }
                    Ok(list_str)
                }
            }
            ShellCommand::AgentRun { task_id } => {
                if let Some(task) = self.agent_engine.registered_tasks.get(&task_id).cloned() {
                    let mut result_str = format!("[Agent Automation Run #{}]\n", task_id);
                    result_str.push_str(&format!("Task Description: {}\n", task.description));
                    result_str.push_str("-----------------------------\n");
                    for (idx, cmd) in task.commands.iter().enumerate() {
                        result_str.push_str(&format!("Step {}: Executing '{}'...\n", idx + 1, cmd));
                        self.execute_line(cmd);
                    }
                    result_str.push_str("-----------------------------\n");
                    result_str.push_str("[Agent Automation Complete: Success]");
                    Ok(result_str)
                } else {
                    Err(format!("Agent task #{} not found", task_id))
                }
            }
            ShellCommand::Gst { basic_cost, rate } => {
                use crate::compatibility::india_stack::GstCalculator;
                match GstCalculator::calculate_gst(basic_cost, rate, false) {
                    Ok((cgst, sgst, _)) => Ok(format!(
                        "GST Details:\n  Basic Cost: INR {:.2}\n  CGST: INR {:.2}\n  SGST: INR {:.2}\n  Total: INR {:.2}",
                        basic_cost, cgst, sgst, basic_cost + cgst + sgst
                    )),
                    Err(_) => Err("Invalid GST rate".to_string()),
                }
            }
            ShellCommand::Upi { vpa, amount } => {
                use crate::compatibility::india_stack::MockUPIService;
                let upi = MockUPIService::new(amount);
                let mut buf = [0u8; 128];
                match upi.generate_upi_qr(vpa.as_bytes(), amount, &mut buf) {
                    Ok(len) => {
                        let qr_str = std::str::from_utf8(&buf[..len]).unwrap_or("");
                        Ok(format!("Generated UPI QR Paylink:\n  {}", qr_str))
                    }
                    Err(_) => Err("Invalid VPA address".to_string()),
                }
            }
            ShellCommand::Translate { lang, key } => {
                use crate::compatibility::india_stack::MultilingualSupport;
                match MultilingualSupport::translate(lang.as_bytes(), key.as_bytes()) {
                    Ok(translated_bytes) => {
                        let translated_str = std::str::from_utf8(translated_bytes).unwrap_or("");
                        Ok(format!("Translation [{}]: {}", lang, translated_str))
                    }
                    Err(_) => Err("Translation not found".to_string()),
                }
            }
            ShellCommand::PeValidate { hex_stub } => {
                use crate::compatibility::reactos::PortableExecutableLoader;
                let mut bytes = Vec::new();
                let mut i = 0;
                while i + 1 < hex_stub.len() {
                    if let Ok(b) = u8::from_str_radix(&hex_stub[i..i + 2], 16) {
                        bytes.push(b);
                    }
                    i += 2;
                }

                match PortableExecutableLoader::validate_pe_image(&bytes) {
                    Ok(()) => {
                        Ok("Valid Windows Portable Executable (PE) headers parsed!".to_string())
                    }
                    Err(_) => Err("Invalid PE image headers format".to_string()),
                }
            }
            ShellCommand::VmStep { hex_bytecode } => {
                use crate::virtualization::deterministic::DeterministicVirtualMachine;
                let mut vm = DeterministicVirtualMachine::new();
                let mut bytes = Vec::new();
                let mut i = 0;
                while i + 1 < hex_bytecode.len() {
                    if let Ok(b) = u8::from_str_radix(&hex_bytecode[i..i + 2], 16) {
                        bytes.push(b);
                    }
                    i += 2;
                }

                match vm.step_instruction(&bytes) {
                    Ok(()) => Ok(format!(
                        "VM Step Completed successfully!\n  RIP: {}\n  Reg[0]: {}",
                        vm.cpu.rip, vm.cpu.r[0]
                    )),
                    Err(_) => Err("VM instruction execution limit/error".to_string()),
                }
            }
            ShellCommand::Proc { args } => {
                use crate::process::ProcFileSystem;
                let pfs = ProcFileSystem::new();
                if args.is_empty() {
                    Ok("Usage: proc cat <file_path>\nAvailable files:\n  /proc/meminfo\n  /proc/cpuinfo\n  /proc/uptime\n  /proc/cgroups\n  /proc/<pid>/status\n  /proc/<pid>/cmdline\n  /proc/<pid>/stat".to_string())
                } else if args[0] == "cat" && args.len() >= 2 {
                    let path = &args[1];
                    match pfs.read_file(path) {
                        Ok(content) => Ok(content),
                        Err(err) => Err(err),
                    }
                } else {
                    Err(format!("Unknown proc action. Only 'cat' is supported."))
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

    #[test]
    fn test_agent_automation() {
        let mut repl = ShellRepl::new();

        // 1. Register an Agent Task
        let reg_cmd = ShellCommand::AgentRegister {
            description: "SysAudit".to_string(),
            commands: "echo audit_start; ps; echo audit_end".to_string(),
        };
        let reg_res = repl.execute_command(reg_cmd).unwrap();
        assert!(reg_res.contains("Agent task #1 registered successfully"));

        // 2. List registered tasks
        let list_cmd = ShellCommand::AgentList;
        let list_res = repl.execute_command(list_cmd).unwrap();
        assert!(list_res.contains("SysAudit"));

        // 3. Run the Agent Task
        let run_cmd = ShellCommand::AgentRun { task_id: 1 };
        let run_res = repl.execute_command(run_cmd).unwrap();
        assert!(run_res.contains("[Agent Automation Run #1]"));
        assert!(run_res.contains("[Agent Automation Complete: Success]"));
    }

    #[test]
    fn test_sovereign_utility_commands() {
        let mut repl = ShellRepl::new();

        // 1. Test GST Command
        let gst_cmd = ShellCommand::Gst {
            basic_cost: 1000.0,
            rate: 18,
        };
        let gst_res = repl.execute_command(gst_cmd).unwrap();
        assert!(gst_res.contains("CGST: INR 90.00"));

        // 2. Test UPI Command
        let upi_cmd = ShellCommand::Upi {
            vpa: "receiver@upi".to_string(),
            amount: 1500,
        };
        let upi_res = repl.execute_command(upi_cmd).unwrap();
        assert!(upi_res.contains("upi://pay?pa=receiver@upi&am=1500"));

        // 3. Test Translate Command
        let trans_cmd = ShellCommand::Translate {
            lang: "sa".to_string(),
            key: "welcome".to_string(),
        };
        let trans_res = repl.execute_command(trans_cmd).unwrap();
        assert!(trans_res.contains("स्वागतम्"));

        // 4. Test PE Validation Command (Valid minimal PE hex representation)
        let pe_cmd = ShellCommand::PeValidate { hex_stub: "4d5a000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000004000000050450000".to_string() };
        let pe_res = repl.execute_command(pe_cmd).unwrap();
        assert!(pe_res.contains("Valid Windows Portable Executable"));

        // 5. Test VM step command (0x10, 0x00, 0x05 -> ADDI r[0], 5)
        let vm_cmd = ShellCommand::VmStep {
            hex_bytecode: "100005".to_string(),
        };
        let vm_res = repl.execute_command(vm_cmd).unwrap();
        assert!(vm_res.contains("VM Step Completed"));
    }
}
