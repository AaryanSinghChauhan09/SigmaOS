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
    Npfctl {
        subcommand: String,
        arg: Option<String>,
    },
    Checksec {
        pid: usize,
    },
    Aimon,
    Aicontrol {
        subcommand: String,
        arg: String,
    },
    Aistat,
    Pledge {
        promises: String,
    },
    Jail {
        subcommand: String,
        jail_id: Option<u32>,
    },
    Runit {
        subcommand: String,
        service: Option<String>,
    },
    Useflags {
        flag: Option<String>,
        state: Option<bool>,
    },
    NixosGen {
        action: String,
        config: Option<String>,
    },
    Aur {
        subcommand: String,
        pkg: Option<String>,
    },
    Unknown(String),
}

/// Shell REPL
// ============================================================================
// API Hooking Manager (Linux / BSD Syscall & User-Mode API Interception)
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HookType {
    InlineJmp,
    ImportTable,
    Trampoline,
}

#[derive(Debug, Clone)]
pub struct ApiHook {
    pub target_api: String,
    pub hook_type: HookType,
    pub detoured_address: u64,
    pub original_bytes: Vec<u8>,
    pub is_enabled: bool,
}

#[derive(Debug, Default)]
pub struct ApiHookManager {
    pub hooks: std::collections::HashMap<String, ApiHook>,
}

impl ApiHookManager {
    pub fn new() -> Self {
        Self {
            hooks: std::collections::HashMap::new(),
        }
    }

    pub fn install_hook(&mut self, api_name: &str, hook_type: HookType, detour_addr: u64) -> Result<(), String> {
        let hook = ApiHook {
            target_api: api_name.to_string(),
            hook_type,
            detoured_address: detour_addr,
            original_bytes: vec![0xE9, 0x00, 0x00, 0x00, 0x00], // Mock 5-byte JMP
            is_enabled: true,
        };
        self.hooks.insert(api_name.to_string(), hook);
        Ok(())
    }

    pub fn is_hooked(&self, api_name: &str) -> bool {
        self.hooks.get(api_name).map_or(false, |h| h.is_enabled)
    }
}

// ============================================================================
// Script Alias Engine (Automatic, Fixed Name, User Named & Background & Ops)
// ============================================================================

#[derive(Debug, Clone)]
pub struct ScriptAliasEngine {
    pub automatic_aliases: std::collections::HashMap<String, String>,
    pub fixed_aliases: std::collections::HashMap<String, String>,
    pub user_aliases: std::collections::HashMap<String, String>,
}

impl ScriptAliasEngine {
    pub fn new() -> Self {
        let mut auto_map = std::collections::HashMap::new();
        auto_map.insert("ll".to_string(), "ls -la".to_string());
        auto_map.insert("la".to_string(), "ls -a".to_string());
        auto_map.insert("md".to_string(), "mkdir".to_string());

        let mut fixed_map = std::collections::HashMap::new();
        fixed_map.insert("@call".to_string(), "script_exec".to_string());

        Self {
            automatic_aliases: auto_map,
            fixed_aliases: fixed_map,
            user_aliases: std::collections::HashMap::new(),
        }
    }

    pub fn register_user_alias(&mut self, alias_name: &str, expansion: &str) {
        self.user_aliases.insert(alias_name.to_string(), expansion.to_string());
    }

    pub fn resolve_alias(&self, input: &str) -> (String, bool) {
        let trimmed = input.trim();
        let is_background = trimmed.ends_with('&');
        let clean_input = if is_background {
            trimmed[..trimmed.len() - 1].trim()
        } else {
            trimmed
        };

        let mut parts = clean_input.split_whitespace();
        if let Some(cmd) = parts.next() {
            let rest = parts.collect::<Vec<&str>>().join(" ");

            // 1. Check fixed aliases (@call)
            if let Some(fixed_cmd) = self.fixed_aliases.get(cmd) {
                let resolved = if rest.is_empty() {
                    fixed_cmd.clone()
                } else {
                    format!("{} {}", fixed_cmd, rest)
                };
                return (resolved, is_background);
            }

            // 2. Check user aliases
            if let Some(user_cmd) = self.user_aliases.get(cmd) {
                let resolved = if rest.is_empty() {
                    user_cmd.clone()
                } else {
                    format!("{} {}", user_cmd, rest)
                };
                return (resolved, is_background);
            }

            // 3. Check automatic aliases
            if let Some(auto_cmd) = self.automatic_aliases.get(cmd) {
                let resolved = if rest.is_empty() {
                    auto_cmd.clone()
                } else {
                    format!("{} {}", auto_cmd, rest)
                };
                return (resolved, is_background);
            }
        }

        (clean_input.to_string(), is_background)
    }

    pub fn execute_script_file(&self, file_path: &str) -> Result<Vec<String>, String> {
        if file_path.is_empty() {
            return Err("Script file path is empty".to_string());
        }
        // Mock execution of .sig or script file commands
        Ok(vec![
            format!("Loaded script file: {}", file_path),
            "echo Script execution completed successfully.".to_string(),
        ])
    }
}

impl Default for ScriptAliasEngine {
    fn default() -> Self {
        Self::new()
    }
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
    pub alias_engine: ScriptAliasEngine,
    pub hook_manager: ApiHookManager,
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
            alias_engine: ScriptAliasEngine::new(),
            hook_manager: ApiHookManager::new(),
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
            alias_engine: ScriptAliasEngine::new(),
            hook_manager: ApiHookManager::new(),
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
            "npfctl" => {
                let subcommand = if parts.len() >= 2 { parts[1].to_string() } else { "status".to_string() };
                let arg = if parts.len() >= 3 { Some(parts[2].to_string()) } else { None };
                ShellCommand::Npfctl { subcommand, arg }
            }
            "checksec" => {
                let pid = if parts.len() >= 2 { parts[1].parse::<usize>().unwrap_or(1) } else { 1 };
                ShellCommand::Checksec { pid }
            }
            "aimon" => ShellCommand::Aimon,
            "aicontrol" => {
                let subcommand = if parts.len() >= 2 { parts[1].to_string() } else { "status".to_string() };
                let arg = if parts.len() >= 3 { parts[2].to_string() } else { "".to_string() };
                ShellCommand::Aicontrol { subcommand, arg }
            }
            "aistat" => ShellCommand::Aistat,
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
            "mkdir" => {
                if parts.len() >= 2 {
                    ShellCommand::Mkdir {
                        dirname: parts[1].to_string(),
                    }
                } else {
                    ShellCommand::Unknown(input.to_string())
                }
            }
            "pledge" => {
                let promises = parts[1..].join(" ");
                ShellCommand::Pledge { promises }
            }
            "jail" => {
                let subcommand = if parts.len() >= 2 { parts[1].to_string() } else { "list".to_string() };
                let jail_id = if parts.len() >= 3 { parts[2].parse::<u32>().ok() } else { None };
                ShellCommand::Jail { subcommand, jail_id }
            }
            "runit" => {
                let subcommand = if parts.len() >= 2 { parts[1].to_string() } else { "status".to_string() };
                let service = if parts.len() >= 3 { Some(parts[2].to_string()) } else { None };
                ShellCommand::Runit { subcommand, service }
            }
            "useflags" => {
                let flag = if parts.len() >= 2 { Some(parts[1].to_string()) } else { None };
                let state = if parts.len() >= 3 { Some(parts[2] == "on" || parts[2] == "true" || parts[2] == "1") } else { None };
                ShellCommand::Useflags { flag, state }
            }
            "nixos" => {
                let action = if parts.len() >= 2 { parts[1].to_string() } else { "status".to_string() };
                let config = if parts.len() >= 3 { Some(parts[2..].join(" ")) } else { None };
                ShellCommand::NixosGen { action, config }
            }
            "aur" => {
                let subcommand = if parts.len() >= 2 { parts[1].to_string() } else { "search".to_string() };
                let pkg = if parts.len() >= 3 { Some(parts[2].to_string()) } else { None };
                ShellCommand::Aur { subcommand, pkg }
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
            ShellCommand::Npfctl { subcommand, arg } => {
                if subcommand == "status" {
                    Ok("Filtering: ACTIVE\nConfiguration: /etc/npf.conf\nState tracking: Enabled (Conntrack)\nActive connections: 42\nTotal evaluated: 1542".to_string())
                } else if subcommand == "reload" {
                    Ok("npfctl: Reloaded /etc/npf.conf successfully. Active rules updated.".to_string())
                } else if subcommand == "stats" {
                    Ok("NPF Firewall Statistics:\nPassed: 1420\nBlocked: 122\nStateful matches: 890\nNAT translations: 310".to_string())
                } else {
                    Ok(format!("npfctl {}: Executed successfully.", subcommand))
                }
            }
            ShellCommand::Checksec { pid } => {
                let mgr = crate::security::binary_protection::BinaryProtectionManager::new();
                let report = mgr.checksec(pid);
                Ok(format!(
                    "RELRO           STACK CANARY      NX            PIE             FORTIFY  CFI\n\
                     {:?}      {:?}       {:?}          {:?}            {:?}     {:?}",
                    report.relro,
                    report.stack_canary_active,
                    report.nx_active,
                    report.pie_active,
                    report.fortify_source_active,
                    report.cfi_active
                ))
            }
            ShellCommand::Aimon => {
                let mut mem_mgr = crate::ai::tensor_memory::AiTensorMemoryManager::new(8 * 1024 * 1024 * 1024);
                let _ = mem_mgr.allocate_tensor("llama3.70b.q4", vec![8192, 8192], crate::ai::tensor_memory::TensorDtype::Int4, crate::ai::tensor_memory::MemoryPinMode::PinnedHostDma);
                let stats = mem_mgr.get_stats();
                Ok(format!(
                    "=== SigmaOS AI Accelerator & Tensor Memory Monitor ===\n\
                     Allocated Bytes: {} MB\n\
                     Pinned DMA Bytes: {} MB\n\
                     Coherent Shared Bytes: {} MB\n\
                     Active Tensor Buffers: {}\n\
                     UMA Cache Hits/Misses: {} / {}\n\
                     DMA Mapping Handles: {}",
                    stats.total_allocated_bytes / (1024 * 1024),
                    stats.pinned_dma_bytes / (1024 * 1024),
                    stats.coherent_shared_bytes / (1024 * 1024),
                    stats.buffer_count,
                    stats.cache_hits,
                    stats.cache_misses,
                    stats.dma_mappings
                ))
            }
            ShellCommand::Aicontrol { subcommand, arg } => {
                let mut sched = crate::ai::compute_scheduler::AiComputeScheduler::new(crate::ai::compute_scheduler::AiComputeQuota::default());
                let _ = sched.enqueue_task("realtime_completion", crate::ai::compute_scheduler::AiTaskPriority::RealTimeLLM, crate::ai::compute_scheduler::ComputeDeviceTarget::DiscreteGpu, 50, 16, 2 * 1024 * 1024 * 1024);
                sched.schedule_next_tick();
                let (pending, running, completed, tokens, preemptions) = sched.get_summary();
                Ok(format!(
                    "aicontrol [{} {}]: SCHED_ULE Scheduler Active\n\
                     Pending Tasks: {} | Running Tasks: {} | Completed Tasks: {}\n\
                     Total Tokens Processed: {} | Priority Preemptions: {}",
                    subcommand, arg, pending, running, completed, tokens, preemptions
                ))
            }
            ShellCommand::Aistat => {
                let weights = vec![1.0f32; 1024];
                let qmat = crate::ai::quantization::QuantizedMatrix::quantize_fp32_matrix("model.embed", 32, 32, &weights, crate::ai::tensor_memory::TensorDtype::Int8).unwrap();
                let mut dispatcher = crate::ai::quantization::AiExecutionDispatcher::new(true, true, true);
                let route = dispatcher.resolve_device_route(crate::ai::compute_scheduler::ComputeDeviceTarget::DiscreteGpu);
                let (ops, us) = dispatcher.execute_gemm(&qmat, &route);
                Ok(format!(
                    "=== SigmaOS AI Performance & Quantization Telemetry ===\n\
                     Model Weight Compression Ratio: {:.2}x\n\
                     Primary Device Target: {:?}\n\
                     Active Device Executed: {:?}\n\
                     Fallback Active: {}\n\
                     GEMM Ops Executed: {} ops in {} us",
                    qmat.compression_ratio, route.primary_device, route.active_device, route.is_fallback_active, ops, us
                ))
            }
            ShellCommand::Echo { message } => Ok(message),
            ShellCommand::Pledge { promises } => {
                let mut pledge = crate::kernel::linux_bsd_innovations::OpenBsdPledge::new();
                if promises.is_empty() {
                    Ok("OpenBSD Pledge: Active promises = [stdio rpath wpath cpath inet]".to_string())
                } else if pledge.pledge(&promises).is_ok() {
                    Ok(format!("OpenBSD Pledge: Syscall restrictions applied successfully -> [{}]", promises))
                } else {
                    Err("OpenBSD Pledge: Security escalation attempt rejected!".to_string())
                }
            }
            ShellCommand::Jail { subcommand, jail_id } => {
                if subcommand == "list" {
                    Ok("JID  IP ADDRESS     HOSTNAME            PATH\n\
                        1    192.168.1.101  web_jail            /usr/jails/web\n\
                        2    192.168.1.102  db_jail             /usr/jails/db".to_string())
                } else if subcommand == "create" {
                    let id = jail_id.unwrap_or(10);
                    let jail = crate::kernel::linux_bsd_innovations::FreeBsdJail::create(id);
                    Ok(format!("FreeBSD Jail: Created isolated sandbox jail JID {} (Isolated: {})", id, jail.is_isolated()))
                } else {
                    Ok(format!("jail [{}]: Execution successful.", subcommand))
                }
            }
            ShellCommand::Runit { subcommand, service } => {
                let mut runit = crate::kernel::linux_bsd_innovations::VoidRunitInit::new();
                runit.start_service("socklog");
                runit.start_service("dhcpcd");

                if subcommand == "status" {
                    Ok("Void runit init supervision:\n\
                        run: dhcpcd (pid 1401) 840s\n\
                        run: socklog (pid 1405) 840s\n\
                        run: udevd (pid 1410) 840s".to_string())
                } else if subcommand == "start" {
                    let svc = service.unwrap_or_else(|| "custom_svc".to_string());
                    runit.start_service(&svc);
                    Ok(format!("Void runit: Started service '{}' successfully.", svc))
                } else {
                    Ok(format!("runit [{}]: Service command complete.", subcommand))
                }
            }
            ShellCommand::Useflags { flag, state } => {
                let mut gentoo = crate::kernel::linux_bsd_innovations::GentooUseFlags::new();
                gentoo.set_flag("wayland", true);
                gentoo.set_flag("egl", true);
                gentoo.add_dependency("wayland", "egl");

                if let Some(f) = flag {
                    if let Some(s) = state {
                        gentoo.set_flag(&f, s);
                        Ok(format!("Gentoo USE-flags: Set flag '{}' = {}. Dependency check: {}", f, s, gentoo.check_dependencies()))
                    } else {
                        Ok(format!("Gentoo USE-flags: Flag '{}' = {}", f, gentoo.has_feature(&f)))
                    }
                } else {
                    Ok("Gentoo USE-flags active: [wayland, egl, unicode, pam, systemd]\nDependency resolution: PASS".to_string())
                }
            }
            ShellCommand::NixosGen { action, config } => {
                let mut nixos = crate::kernel::linux_bsd_innovations::NixOsDeclarativeManager::new();
                nixos.apply_configuration(&["services.nginx.enable = true;"]).unwrap();

                if action == "rollback" {
                    if nixos.rollback().is_ok() {
                        Ok("NixOS Declarative Manager: Rolled back to previous generation successfully.".to_string())
                    } else {
                        Ok("NixOS Declarative Manager: Already at earliest generation.".to_string())
                    }
                } else if action == "switch" {
                    let cfg = config.unwrap_or_else(|| "environment.systemPackages = [ pkgs.git ];".to_string());
                    nixos.apply_configuration(&[&cfg]).unwrap();
                    Ok(format!("NixOS Declarative Manager: Applied generation config [{}]", cfg))
                } else {
                    Ok(format!("NixOS Declarative Manager: Current generation config = {:?}", nixos.configuration))
                }
            }
            ShellCommand::Aur { subcommand, pkg } => {
                let mut aur = crate::sigpkg::aur_helper::AurHelper::new();
                if subcommand == "install" {
                    let package = pkg.unwrap_or_else(|| "neofetch-git".to_string());
                    if aur.install(&package).is_ok() {
                        Ok(format!("Arch AUR: Fetched PKGBUILD, compiled, and installed '{}' cleanly.", package))
                    } else {
                        Err(format!("Arch AUR: Failed to build '{}'", package))
                    }
                } else if subcommand == "info" {
                    let package = pkg.unwrap_or_else(|| "sigma-meta".to_string());
                    Ok(format!("Arch AUR Package Info: {}\nRepository: aur.archlinux.org\nMaintainer: Sovereign Core", package))
                } else {
                    Ok("Arch AUR helper active. Use 'aur install <pkg>' or 'aur info <pkg>'.".to_string())
                }
            }
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
    fn test_api_hook_trampoline() {
        let mut mgr = ApiHookManager::new();
        assert!(!mgr.is_hooked("sys_read"));

        mgr.install_hook("sys_read", HookType::Trampoline, 0x7FFF0010).unwrap();
        assert!(mgr.is_hooked("sys_read"));
    }

    #[test]
    fn test_script_alias_and_ampersand_operations() {
        let mut engine = ScriptAliasEngine::new();
        engine.register_user_alias("cls", "clear");

        // Test automatic alias with background &
        let (resolved, is_bg) = engine.resolve_alias("ll &");
        assert_eq!(resolved, "ls -la");
        assert!(is_bg);

        // Test user alias
        let (resolved_cls, is_bg_cls) = engine.resolve_alias("cls");
        assert_eq!(resolved_cls, "clear");
        assert!(!is_bg_cls);

        // Test fixed alias @call
        let (resolved_call, _) = engine.resolve_alias("@call setup.sig");
        assert_eq!(resolved_call, "script_exec setup.sig");

        // Test script file execution
        let res = engine.execute_script_file("setup.sig").unwrap();
        assert_eq!(res.len(), 2);
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
    fn test_distro_repl_commands() {
        let mut repl = ShellRepl::new();

        // OpenBSD Pledge
        let cmd_pledge = repl.parse_command("pledge stdio rpath");
        assert!(matches!(cmd_pledge, ShellCommand::Pledge { .. }));
        let out_pledge = repl.execute_command(cmd_pledge).unwrap();
        assert!(out_pledge.contains("OpenBSD Pledge"));

        // FreeBSD Jail
        let cmd_jail = repl.parse_command("jail list");
        assert!(matches!(cmd_jail, ShellCommand::Jail { .. }));
        let out_jail = repl.execute_command(cmd_jail).unwrap();
        assert!(out_jail.contains("web_jail"));

        // Void runit
        let cmd_runit = repl.parse_command("runit status");
        assert!(matches!(cmd_runit, ShellCommand::Runit { .. }));
        let out_runit = repl.execute_command(cmd_runit).unwrap();
        assert!(out_runit.contains("socklog"));

        // Gentoo USE-flags
        let cmd_use = repl.parse_command("useflags wayland on");
        assert!(matches!(cmd_use, ShellCommand::Useflags { .. }));
        let out_use = repl.execute_command(cmd_use).unwrap();
        assert!(out_use.contains("Gentoo USE-flags"));

        // NixOS Declarative Manager
        let cmd_nixos = repl.parse_command("nixos rollback");
        assert!(matches!(cmd_nixos, ShellCommand::NixosGen { .. }));
        let out_nixos = repl.execute_command(cmd_nixos).unwrap();
        assert!(out_nixos.contains("NixOS Declarative Manager"));

        // Arch AUR Helper
        let cmd_aur = repl.parse_command("aur install neofetch-git");
        assert!(matches!(cmd_aur, ShellCommand::Aur { .. }));
        let out_aur = repl.execute_command(cmd_aur).unwrap();
        assert!(out_aur.contains("Arch AUR"));
    }
}
