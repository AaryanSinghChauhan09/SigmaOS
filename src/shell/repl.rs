// SigmaOS Shell REPL (Read-Eval-Print Loop)
// Interactive shell for SigmaOS

use std::io::{self, BufRead, Write};

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
    FixAnomalies,
    UnixSocketBridge,
    SysfsViewer,
    Crash,
    Sysctl {
        mib_expression: Option<String>,
    },
    Ebpf {
        bytecode_spec: String,
    },
    Scheme {
        args: Vec<String>,
    },
    Secrets {
        args: Vec<String>,
    },
    Integrity {
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
}

impl ShellRepl {
    pub fn new() -> Self {
        Self {
            running: true,
            variables: std::collections::HashMap::new(),
            aliases: std::collections::HashMap::new(),
            prompt: "sigma-sh> ".to_string(),
            agent_engine: AgentAutomationEngine::new(),
        }
    }

    pub fn with_prompt(prompt: String) -> Self {
        Self {
            running: true,
            variables: std::collections::HashMap::new(),
            aliases: std::collections::HashMap::new(),
            prompt,
            agent_engine: AgentAutomationEngine::new(),
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
            "fix" => ShellCommand::FixAnomalies,
            "sock" => ShellCommand::UnixSocketBridge,
            "sysfs" => ShellCommand::SysfsViewer,
            "crash" => ShellCommand::Crash,
            "sysctl" => {
                if parts.len() >= 2 {
                    ShellCommand::Sysctl {
                        mib_expression: Some(parts[1..].join(" ")),
                    }
                } else {
                    ShellCommand::Sysctl {
                        mib_expression: None,
                    }
                }
            }
            "ebpf" => {
                if parts.len() >= 2 {
                    ShellCommand::Ebpf {
                        bytecode_spec: parts[1..].join(" "),
                    }
                } else {
                    ShellCommand::Unknown(input.to_string())
                }
            }
            "scheme" => {
                let args_vec = parts[1..].iter().map(|s| s.to_string()).collect();
                ShellCommand::Scheme { args: args_vec }
            }
            "secrets" => {
                let args_vec = parts[1..].iter().map(|s| s.to_string()).collect();
                ShellCommand::Secrets { args: args_vec }
            }
            "integrity" => {
                let args_vec = parts[1..].iter().map(|s| s.to_string()).collect();
                ShellCommand::Integrity { args: args_vec }
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
                   fix       - Run self-healing daemon to detect and remediate anomalies\n\
                   sock      - Run local UNIX Domain Sockets server-client IPC pipeline\n\
                   sysfs     - View Linux-inspired Sysfs (/sys) hardware and loop devices\n\
                   crash     - Trigger a simulated anonymized kernel panic oops report\n\
                   sysctl    - Query and modify BSD-style kernel parameter state\n\
                   ebpf      - Execute verified sandboxed Linux-grade bytecode\n\
                   scheme    - Open and manipulate Redox-inspired URL schemes\n\
                   secrets   - Access keyrings and vault secrets\n\
                   integrity - Monitor file hash configurations\n\
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
            ShellCommand::FixAnomalies => {
                use crate::resilience::{AutomatedFixerDaemon, SovereignProblemType, SelfHealingModule};
                let mut daemon = AutomatedFixerDaemon::new();
                let mut healing = SelfHealingModule::new();

                // 1. Remediate Memory Leak (Triggers standard Cache Clear)
                let r1 = daemon.detect_and_fix(SovereignProblemType::MemoryLeak, 5, &mut healing).unwrap();

                // 2. Remediate Socket Port Block (Triggers TCP stack flush)
                let r2 = daemon.detect_and_fix(SovereignProblemType::SocketPortBlocked, 8080, &mut healing).unwrap();

                // 3. Remediate Null Pointer Deref (Triggers Virtual Page remapping)
                let r3 = daemon.detect_and_fix(SovereignProblemType::NullPointerDeRef, 0, &mut healing).unwrap();

                Ok(format!(
                    "Self-Healing Anomaly Fixer Active!\n  \
                     - Remediated Anomaly 1: MemoryLeak -> Action={:?}\n  \
                     - Remediated Anomaly 2: SocketPortBlocked -> Action={:?}\n  \
                     - Remediated Anomaly 3: NullPointerDeRef -> Action={:?}\n  \
                     - Daemon Remediation Audit: Total Detected={}, Successful Fixes={}",
                    r1,
                    r2,
                    r3,
                    daemon.stats.total_problems_detected,
                    daemon.stats.successful_fixes
                ))
            }
            ShellCommand::UnixSocketBridge => {
                use crate::network::{UnixSocketRegistry, UnixSocketAddress};
                let mut registry = UnixSocketRegistry::new();
                let server_addr = UnixSocketAddress::Path("/var/run/server.sock".to_string());
                let client_addr = UnixSocketAddress::Path("/var/run/client.sock".to_string());

                // 1. Bind and listen
                registry.bind(server_addr.clone()).unwrap();
                registry.listen(&server_addr).unwrap();

                // 2. Connect
                registry.connect(client_addr.clone(), server_addr.clone()).unwrap();

                // 3. Write data from client
                let mut client = registry.sockets.get_mut(&client_addr).unwrap();
                client.write_data(b"IPC sovereign data stream!").unwrap();

                // 4. Pipe packets
                let peer_addr = UnixSocketAddress::Abstract(format!("peer-{:?}", server_addr));
                let len = registry.pipe_packets(&client_addr, &peer_addr).unwrap();

                // 5. Read data on server side peer
                let server_peer = registry.sockets.get_mut(&peer_addr).unwrap();
                let read_data = server_peer.read_data(100).unwrap();
                let read_str = std::str::from_utf8(&read_data).unwrap_or("");

                Ok(format!(
                    "UNIX Sockets Local Bridge active!\n  \
                     - Server path-bound: {:?}\n  \
                     - Client path-bound: {:?}\n  \
                     - Local IPC packet pipe transfer count: {} bytes\n  \
                     - Received payload: '{}'",
                    server_addr,
                    client_addr,
                    len,
                    read_str
                ))
            }
            ShellCommand::SysfsViewer => {
                use crate::process::SysfsRegistry;
                let mut registry = SysfsRegistry::new();

                // 1. Read battery supply status
                let bat_cap = registry.read_attribute("/sys/class/power_supply/BAT0/capacity").unwrap();
                let bat_stat = registry.read_attribute("/sys/class/power_supply/BAT0/status").unwrap();

                // 2. Read CPU 1 online state
                let cpu_online = registry.read_attribute("/sys/devices/system/cpu/cpu1/online").unwrap();

                // 3. Mount a loopback file
                registry.mount_loop_device(0, "/home/developer/sigma_dev_overlay.img").unwrap();
                let loop_dev = registry.loop_devices.get(&0).unwrap();

                Ok(format!(
                    "Sovereign Sysfs (/sys) Viewer Active!\n  \
                     - Battery Supply BAT0: {}% ({})\n  \
                     - CPU core cpu1 online state: {}\n  \
                     - Loopback Block Device Gated Mounts:\n    \
                       * /dev/loop0 back-mapped file: '{}'",
                    bat_cap,
                    bat_stat,
                    cpu_online,
                    loop_dev.backing_file_path
                ))
            }
            ShellCommand::Crash => {
                use crate::crash::{CrashReporter, OopsReport, CpuRegisterDump};
                let mut reporter = CrashReporter::new();
                let regs = CpuRegisterDump {
                    rip: 0xFFFFFFFF8100A52B,
                    rsp: 0xFFFF8800000A3000,
                    rbp: 0xFFFF8800000A3040,
                    rax: 0x0000000000000200,
                    rbx: 0x0000000000004000,
                    rcx: 0x0000000000000001,
                    rdx: 0x0000000000000055,
                    rflags: 0x0000000000010246,
                };
                let oops = OopsReport {
                    process_name: "sigma-core".to_string(),
                    pid: 45,
                    ppid: 1,
                    registers: regs,
                    stack_trace: vec![0xFFFFFFFF8100F2A3, 0xFFFFFFFF8101C5D8],
                    raw_panic_message: "Segmentation fault reading secrets token=secret_api_key_xyz from cache @ 192.168.1.55".to_string(),
                };
                let dump = reporter.generate_linux_grade_panic_dump(oops);
                Ok(dump)
            }
            ShellCommand::Sysctl { mib_expression } => {
                use crate::kernel::{SysctlRegistry, SysctlValue};
                let mut registry = SysctlRegistry::new();

                match mib_expression {
                    None => {
                        let mut out = "Sovereign BSD Sysctl MIB Nodes:\n".to_string();
                        let mut keys: Vec<&String> = registry.nodes.keys().collect();
                        keys.sort();
                        for key in keys {
                            let node = registry.nodes.get(key).unwrap();
                            let val_str = match &node.value {
                                SysctlValue::Int(v) => format!("{}", v),
                                SysctlValue::String(s) => s.clone(),
                                SysctlValue::Bool(b) => format!("{}", b),
                            };
                            out.push_str(&format!("  {} = {}  # {}\n", key, val_str, node.description));
                        }
                        Ok(out)
                    }
                    Some(expr) => {
                        if expr.contains('=') {
                            let parts: Vec<&str> = expr.split('=').collect();
                            if parts.len() == 2 {
                                let mib = parts[0].trim();
                                let raw_val = parts[1].trim();

                                let current_val = registry.get(mib).cloned();
                                match current_val {
                                    None => Err(format!("Sysctl node '{}' not found", mib)),
                                    Some(SysctlValue::Int(_)) => {
                                        if let Ok(v) = raw_val.parse::<i32>() {
                                            registry.set(mib, SysctlValue::Int(v))?;
                                            Ok(format!("Updated {} = {}", mib, v))
                                        } else {
                                            Err("Expected integer value!".to_string())
                                        }
                                    }
                                    Some(SysctlValue::String(_)) => {
                                        registry.set(mib, SysctlValue::String(raw_val.to_string()))?;
                                        Ok(format!("Updated {} = {}", mib, raw_val))
                                    }
                                    Some(SysctlValue::Bool(_)) => {
                                        if let Ok(b) = raw_val.parse::<bool>() {
                                            registry.set(mib, SysctlValue::Bool(b))?;
                                            Ok(format!("Updated {} = {}", mib, b))
                                        } else {
                                            Err("Expected boolean value (true/false)!".to_string())
                                        }
                                    }
                                }
                            } else {
                                Err("Invalid sysctl assignment expression. Usage: sysctl vm.swappiness=10".to_string())
                            }
                        } else {
                            let mib = expr.trim();
                            if let Some(val) = registry.get(mib) {
                                let val_str = match val {
                                    SysctlValue::Int(v) => format!("{}", v),
                                    SysctlValue::String(s) => s.clone(),
                                    SysctlValue::Bool(b) => format!("{}", b),
                                };
                                Ok(format!("{} = {}", mib, val_str))
                            } else {
                                Err(format!("Sysctl node '{}' not found", mib))
                            }
                        }
                    }
                }
            }
            ShellCommand::Ebpf { bytecode_spec } => {
                use crate::kernel::{EbpfInstruction, EbpfVerifier, EbpfEngine};
                use crate::kernel::ebpf::*;

                let mut program = Vec::new();
                for chunk in bytecode_spec.split(';') {
                    let chunk = chunk.trim();
                    if chunk.is_empty() {
                        continue;
                    }
                    let parts: Vec<&str> = chunk.split(':').collect();
                    if parts.is_empty() {
                        continue;
                    }
                    match parts[0] {
                        "ADD" => {
                            let dst = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
                            let src = parts.get(2).and_then(|s| s.parse().ok()).unwrap_or(0);
                            program.push(EbpfInstruction { opcode: EBPF_OP_ADD, dst, src, offset: 0, imm: 0 });
                        }
                        "ADDI" => {
                            let dst = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
                            let imm = parts.get(2).and_then(|s| s.parse().ok()).unwrap_or(0);
                            program.push(EbpfInstruction { opcode: EBPF_OP_ADDI, dst, src: 0, offset: 0, imm });
                        }
                        "SUB" => {
                            let dst = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
                            let src = parts.get(2).and_then(|s| s.parse().ok()).unwrap_or(0);
                            program.push(EbpfInstruction { opcode: EBPF_OP_SUB, dst, src, offset: 0, imm: 0 });
                        }
                        "LD" => {
                            let dst = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
                            let offset = parts.get(2).and_then(|s| s.parse().ok()).unwrap_or(0);
                            program.push(EbpfInstruction { opcode: EBPF_OP_LD, dst, src: 0, offset, imm: 0 });
                        }
                        "ST" => {
                            let dst = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
                            let offset = parts.get(2).and_then(|s| s.parse().ok()).unwrap_or(0);
                            program.push(EbpfInstruction { opcode: EBPF_OP_ST, dst, src: 0, offset, imm: 0 });
                        }
                        "MAP_LOOKUP" => {
                            let dst = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
                            let src = parts.get(2).and_then(|s| s.parse().ok()).unwrap_or(0);
                            program.push(EbpfInstruction { opcode: EBPF_OP_MAP_LOOKUP, dst, src, offset: 0, imm: 0 });
                        }
                        "DIV" => {
                            let dst = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
                            let src = parts.get(2).and_then(|s| s.parse().ok()).unwrap_or(0);
                            let imm = parts.get(3).and_then(|s| s.parse().ok()).unwrap_or(0);
                            program.push(EbpfInstruction { opcode: EBPF_OP_DIV, dst, src, offset: 0, imm });
                        }
                        "EXIT" => {
                            program.push(EbpfInstruction { opcode: EBPF_OP_EXIT, dst: 0, src: 0, offset: 0, imm: 0 });
                        }
                        _ => return Err(format!("Invalid opcode token: {}", parts[0])),
                    }
                }

                if let Err(verif_err) = EbpfVerifier::verify(&program) {
                    return Err(format!("eBPF Verifier Rejected Bytecode: {}", verif_err));
                }

                let mut engine = EbpfEngine::new();
                match engine.execute(&program) {
                    Ok(retval) => Ok(format!(
                        "eBPF Program Execution Successful!\n  Verifier Check: PASSED\n  Return Value (R0): {}",
                        retval
                    )),
                    Err(exec_err) => Err(format!("eBPF Execution Failed: {}", exec_err)),
                }
            }
            ShellCommand::Secrets { args } => {
                use crate::security::{SimpleKeyring, SimpleSecret, SecretType, SecretCapability, KeyringCapability, Keyring, Secret};
                let mut keyring = SimpleKeyring::new(KeyringCapability::full());

                if args.is_empty() {
                    return Ok("Sovereign Keyring vault usage:\n  \
                               secrets add <name> <data> - Add and encrypt a secret\n  \
                               secrets list              - List active vault IDs".to_string());
                }

                match args[0].as_str() {
                    "add" => {
                        if args.len() < 3 {
                            return Err("Usage: secrets add <name> <data>".to_string());
                        }
                        let name = &args[1];
                        let data = &args[2..].join(" ");
                        let mut secret = SimpleSecret::new(1, name.as_bytes(), SecretType::APIKey, SecretCapability::full());
                        secret.set_data(data.as_bytes());
                        secret.encrypt(b"sovereign_master_key").unwrap();

                        keyring.add_secret(Box::new(secret))?;
                        Ok(format!("Successfully encrypted and registered secret '{}' in keyring vault!", name))
                    }
                    "list" => {
                        let list = keyring.list_secrets();
                        Ok(format!("Active secrets count in secure keyring: {}", list.len()))
                    }
                    _ => Err(format!("Unknown secrets subcommand: {}", args[0])),
                }
            }
            ShellCommand::Integrity { args } => {
                use crate::security::{SimpleIntegrityMonitor, SimpleFile, FileCapability, MonitorCapability, IntegrityMonitor, IntegrityStatus};
                let mut monitor = SimpleIntegrityMonitor::new(MonitorCapability::full());

                if args.len() < 3 || args[0] != "verify" {
                    return Ok("Usage: integrity verify <path> <checksum_hex>".to_string());
                }

                let path = &args[1];
                let checksum = &args[2];
                let file = SimpleFile::new(1, path.as_bytes(), checksum.as_bytes(), FileCapability::full());
                let id = monitor.register_file(Box::new(file))?;
                let status = monitor.verify_file(id)?;

                Ok(format!("Integrity check for '{}': {:?}", path, status))
            }
            ShellCommand::Scheme { args } => {
                use crate::filesystem::SchemeRegistry;
                let mut registry = SchemeRegistry::new();

                if args.is_empty() {
                    return Ok("Redox OS URL Schemes Command usage:\n  \
                               scheme open <url>        - Open a scheme URL (e.g. shm://buffer, rand://stream)\n  \
                               scheme write <h> <data>  - Write to a registered handle\n  \
                               scheme read <h>          - Read from a registered handle\n  \
                               scheme close <h>         - Close a registered handle".to_string());
                }

                match args[0].as_str() {
                    "open" => {
                        if args.len() < 2 {
                            return Err("Usage: scheme open <url>".to_string());
                        }
                        let handle = registry.open(&args[1])?;
                        Ok(format!("Opened scheme URL successfully. Registered Handle: {}", handle))
                    }
                    "write" => {
                        if args.len() < 3 {
                            return Err("Usage: scheme write <handle> <data>".to_string());
                        }
                        let handle = args[1].parse::<usize>().map_err(|_| "Invalid handle format!")?;
                        let data = args[2..].join(" ");
                        let written = registry.write(handle, data.as_bytes())?;
                        Ok(format!("Successfully wrote {} bytes to handle {}", written, handle))
                    }
                    "read" => {
                        if args.len() < 2 {
                            return Err("Usage: scheme read <handle>".to_string());
                        }
                        let handle = args[1].parse::<usize>().map_err(|_| "Invalid handle format!")?;
                        let mut buf = [0u8; 1000];
                        let read_len = registry.read(handle, &mut buf)?;
                        let s = std::str::from_utf8(&buf[..read_len]).unwrap_or("[Non-UTF-8 resource data stream]");
                        Ok(format!("Read payload from handle {}:\n  '{}'", handle, s))
                    }
                    "close" => {
                        if args.len() < 2 {
                            return Err("Usage: scheme close <handle>".to_string());
                        }
                        let handle = args[1].parse::<usize>().map_err(|_| "Invalid handle format!")?;
                        registry.close(handle)?;
                        Ok(format!("Closed handle {} successfully", handle))
                    }
                    _ => Err(format!("Unknown scheme subcommand: {}", args[0])),
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

    #[test]
    fn test_crash_command() {
        let mut repl = ShellRepl::new();
        let cmd = repl.parse_command("crash");
        assert!(matches!(cmd, ShellCommand::Crash));

        let dump_res = repl.execute_command(cmd).unwrap();
        assert!(dump_res.contains("SIGMAOS KERNEL PANIC: OOPS EXCEPTION ENCOUNTERED"));
        assert!(dump_res.contains("Process: sigma-core (PID: 45, Parent PID: 1)"));
        assert!(dump_res.contains("token=XXXXXXXXXXXX")); // IP and token must be anonymized!
        assert!(!dump_res.contains("192.168.1.55"));
    }

    #[test]
    fn test_sysctl_command() {
        let mut repl = ShellRepl::new();

        // 1. List all sysctl parameters
        let cmd_list = repl.parse_command("sysctl");
        let list_res = repl.execute_command(cmd_list).unwrap();
        assert!(list_res.contains("kern.ostype = SigmaOS"));
        assert!(list_res.contains("vm.swappiness = 60"));

        // 2. Query specific parameter
        let cmd_query = repl.parse_command("sysctl vm.swappiness");
        let query_res = repl.execute_command(cmd_query).unwrap();
        assert_eq!(query_res, "vm.swappiness = 60");

        // 3. Update specific parameter
        let cmd_update = repl.parse_command("sysctl vm.swappiness=10");
        let update_res = repl.execute_command(cmd_update).unwrap();
        assert_eq!(update_res, "Updated vm.swappiness = 10");
    }

    #[test]
    fn test_ebpf_command() {
        let mut repl = ShellRepl::new();

        // Safe program: R1 = 40; R2 = 2; R0 = R1 + R2; EXIT
        let cmd_run = repl.parse_command("ebpf ADDI:1:40; ADDI:2:2; ADD:0:1; ADD:0:2; EXIT");
        let run_res = repl.execute_command(cmd_run).unwrap();
        assert!(run_res.contains("eBPF Program Execution Successful!"));
        assert!(run_res.contains("Return Value (R0): 42"));

        // Unsafe program: ADDI:1:10; DIV:1:0:0; EXIT -> division by zero
        let cmd_unsafe = repl.parse_command("ebpf ADDI:1:10; DIV:1:0:0; EXIT");
        let run_unsafe_res = repl.execute_command(cmd_unsafe);
        assert!(run_unsafe_res.is_err());
        assert!(run_unsafe_res.unwrap_err().contains("eBPF Verifier Rejected Bytecode: Static validation error"));
    }

    #[test]
    fn test_scheme_command() {
        let mut repl = ShellRepl::new();

        // Parse scheme command list
        let cmd_list = repl.parse_command("scheme");
        let list_res = repl.execute_command(cmd_list).unwrap();
        assert!(list_res.contains("Redox OS URL Schemes Command usage"));

        // Open shm URL
        let cmd_open = repl.parse_command("scheme open shm://shared_buff");
        let open_res = repl.execute_command(cmd_open).unwrap();
        assert!(open_res.contains("Registered Handle: 1000"));

        // Write to handle
        let cmd_write = repl.parse_command("scheme write 1000 RedoxRulesOS");
        let write_res = repl.execute_command(cmd_write).unwrap();
        assert!(write_res.contains("Successfully wrote 12 bytes"));

        // Read from handle
        let cmd_read = repl.parse_command("scheme read 1000");
        let read_res = repl.execute_command(cmd_read).unwrap();
        assert!(read_res.contains("'RedoxRulesOS'"));

        // Close handle
        let cmd_close = repl.parse_command("scheme close 1000");
        let close_res = repl.execute_command(cmd_close).unwrap();
        assert!(close_res.contains("Closed handle 1000 successfully"));
    }

    #[test]
    fn test_secrets_repl_command() {
        let mut repl = ShellRepl::new();

        // 1. Help message
        let cmd_help = repl.parse_command("secrets");
        let help_res = repl.execute_command(cmd_help).unwrap();
        assert!(help_res.contains("Sovereign Keyring vault usage"));

        // 2. Add secret
        let cmd_add = repl.parse_command("secrets add DB_PASS superpassword");
        let add_res = repl.execute_command(cmd_add).unwrap();
        assert!(add_res.contains("Successfully encrypted and registered secret"));

        // 3. List secrets
        let cmd_list = repl.parse_command("secrets list");
        let list_res = repl.execute_command(cmd_list).unwrap();
        assert!(list_res.contains("Active secrets count in secure keyring:"));
    }

    #[test]
    fn test_integrity_repl_command() {
        let mut repl = ShellRepl::new();

        let cmd_verify = repl.parse_command("integrity verify /bin/init abcd1234");
        let verify_res = repl.execute_command(cmd_verify).unwrap();
        assert!(verify_res.contains("Integrity check for '/bin/init': Valid"));
    }
}
