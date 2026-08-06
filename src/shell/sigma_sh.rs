// Sovereign Replacement System Utilities for SigmaOS
// This module implements zero-dependency, no-std compliant alternatives to standard Linux core utilities,
// init/supervision systems, syslog/journald, cron, capability-based sudo, and help manuals.
#[cfg(not(target_os = "none"))]
extern crate alloc as std_alloc;
#[cfg(not(target_os = "none"))]
use std_alloc::boxed::Box;

#![no_std]
#![no_main]

extern crate alloc;
/// OOP-based Sigma Shell for SigmaOS
/// Based on Ultimate Dominance Strategy: Stage 0 Milestone 0.1
/// Implements interactive shell with command parsing, echo, environment variables, aliases, and basic utilities

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};
use core::time::Duration;

// ==========================================
// 1. CORE UTILS (sigma-core-utils / BusyBox)
// ==========================================
pub struct SigmaCoreUtils;

impl SigmaCoreUtils {
    pub fn cat(&self, file_data: &[u8]) -> String {
        String::from_utf8_lossy(file_data).to_string()
    }

    pub fn echo(&self, args: &[&str]) -> String {
        args.join(" ")
    }

    pub fn ls(&self, files: &[&str]) -> String {
        files.join("\n")
    }

    pub fn cp(&self, source: &[u8]) -> Vec<u8> {
        source.to_vec()
    }

    pub fn mv(&self, source: &[u8]) -> Vec<u8> {
        source.to_vec()
    }

    pub fn rm(&self) -> bool {
        true
    }
}

// ==========================================
// 2. INIT SYSTEM (sigma-init / systemd / s6)
// ==========================================
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TargetState {
    Emergency = 0,
    SingleUser = 1,
    MultiUser = 2,
    Graphical = 3,
}

#[derive(Debug, Clone)]
pub struct Service {
    pub name: String,
    pub dependencies: Vec<String>,
    pub running: bool,
    pub restart_count: usize,
    pub max_restarts: usize,
    pub socket_activated: bool,
    pub startup_time_ms: u64,
}

pub struct SigmaInit {
    pub services: Vec<Service>,
    pub current_target: TargetState,
}

impl SigmaInit {
    pub fn new() -> Self {
        Self {
            services: Vec::new(),
            current_target: TargetState::MultiUser,
        }
    }

    pub fn set_target(&mut self, target: TargetState) {
        self.current_target = target;
    }

    pub fn register_service(&mut self, name: &str, dependencies: &[&str]) {
        let mut deps = Vec::new();
        for &dep in dependencies {
            deps.push(dep.to_string());
        }
        self.services.push(Service {
            name: name.to_string(),
            dependencies: deps,
            running: false,
            restart_count: 0,
            max_restarts: 3,
            socket_activated: false,
            startup_time_ms: 100, // mock startup time
        });
    }

    pub fn register_socket_activated_service(&mut self, name: &str, dependencies: &[&str]) {
        let mut deps = Vec::new();
        for &dep in dependencies {
            deps.push(dep.to_string());
        }
        self.services.push(Service {
            name: name.to_string(),
            dependencies: deps,
            running: false,
            restart_count: 0,
            max_restarts: 3,
            socket_activated: true,
            startup_time_ms: 150,
        });
    }

    /// Simulate a connection attempt triggering standard socket activation for a service.
    pub fn socket_trigger(&mut self, service_name: &str) -> bool {
        if let Some(service) = self.services.iter_mut().find(|s| e_eq(&s.name, service_name)) {
            if service.socket_activated && !service.running {
                service.running = true;
                return true;
impl ShellCommand for EchoCommand {
    fn name(&self) -> &[u8] { b"echo" }
    fn execute(&mut self, args: &[&[u8]]) -> Result<(), ShellError> {
        for (i, arg) in args.iter().enumerate() {
            if i > 0 {
                // print spacer in actual implementation
            }
            for _byte in *arg {
                // write to stdout
            }
        }
        false
    }
    fn help(&self) -> &[u8] { b"echo [text] - Print text to output (supports variable expansion like $USER)" }
}

    /// Automatically identify failed/terminated services and trigger a restart supervision policy.
    pub fn check_and_restart_failed_services(&mut self) -> Vec<String> {
        let mut restarted = Vec::new();
        for service in &mut self.services {
            if !service.running && service.restart_count < service.max_restarts {
                service.restart_count += 1;
                service.running = true;
                restarted.push(service.name.clone());
            }
        }
        restarted
    }

    /// Systemd-analyze: return list of services ordered by startup duration time (descending).
    pub fn systemd_analyze(&self) -> Vec<(String, u64)> {
        let mut list = Vec::new();
        for service in &self.services {
            if service.running {
                list.push((service.name.clone(), service.startup_time_ms));
            }
        }
        list.sort_by(|a, b| b.1.cmp(&a.1));
        list
    }

    /// Supervise and resolve startup sequence under dependency constraints.
    pub fn start_services_ordered(&mut self) -> Vec<String> {
        let mut started = Vec::new();
        let mut attempts = 0;
        let total_services = self.services.len();

        while started.len() < total_services && attempts < 10 {
            attempts += 1;
            for i in 0..total_services {
                if self.services[i].running || self.services[i].socket_activated {
                    // Skip running or socket-activated units during initial bootstrap
                    if self.services[i].running && !started.contains(&self.services[i].name) {
                        started.push(self.services[i].name.clone());
                    }
                    continue;
                }
                let mut deps_satisfied = true;
                for dep in &self.services[i].dependencies {
                    if !started.contains(dep) {
                        deps_satisfied = false;
                        break;
                    }
                }
                if deps_satisfied {
                    self.services[i].running = true;
                    started.push(self.services[i].name.clone());
                }
            }
        }
        started
    }
}

/// Helper method for string matching
fn e_eq(a: &str, b: &str) -> bool {
    a == b
/// Linux-style built-in command to define aliases
pub struct AliasCommand {
    pub shell_ptr: *mut SimpleShell,
}

impl ShellCommand for AliasCommand {
    fn name(&self) -> &[u8] { b"alias" }
    fn execute(&mut self, args: &[&[u8]]) -> Result<(), ShellError> {
        if args.len() < 2 {
            return Err(ShellError::InvalidArgument);
        }
        unsafe {
            if !self.shell_ptr.is_null() {
                (*self.shell_ptr).set_alias(args[0], args[1]);
            }
        }
        Ok(())
    }
    fn help(&self) -> &[u8] { b"alias [shortcut] [command] - Define a shell alias" }
}

/// Linux-style built-in command to remove aliases
pub struct UnaliasCommand {
    pub shell_ptr: *mut SimpleShell,
}

impl ShellCommand for UnaliasCommand {
    fn name(&self) -> &[u8] { b"unalias" }
    fn execute(&mut self, args: &[&[u8]]) -> Result<(), ShellError> {
        if args.is_empty() {
            return Err(ShellError::InvalidArgument);
        }
        unsafe {
            if !self.shell_ptr.is_null() {
                (*self.shell_ptr).unset_alias(args[0]);
            }
        }
        Ok(())
    }
    fn help(&self) -> &[u8] { b"unalias [shortcut] - Remove a shell alias" }
}

pub trait Shell {
    fn register_command(&mut self, command: Box<dyn ShellCommand>) -> Result<CommandID, ShellError>;
    fn execute_line(&mut self, line: &[u8]) -> Result<(), ShellError>;
    fn get_prompt(&self) -> &[u8];
    fn set_prompt(&mut self, prompt: &[u8]);
}

// ==========================================
// 3. LOGGER SYSTEM (sigma-log / journald)
// ==========================================
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Debug = 0,
    Info = 1,
    Warning = 2,
    Error = 3,
#[repr(C)]
pub struct SimpleShell {
    pub commands: Vec<Option<Box<dyn ShellCommand>>>,
    pub next_id: AtomicUsize,
    pub prompt: [u8; 64],
    pub prompt_len: AtomicUsize,
    pub env: SimpleShellEnvironment,
    pub aliases: SimpleShellEnvironment, // Recycles environment implementation for alias maps
}

#[derive(Debug, Clone)]
pub struct LogEntry {
    pub level: LogLevel,
    pub tag: String,
    pub message: String,
    pub timestamp_ms: u64,
}

/// A lock-free circular journal log ring buffer.
pub struct SigmaLog {
    pub buffer: [Option<LogEntry>; 128],
    pub head: AtomicUsize,
    pub count: AtomicUsize,
}

impl SigmaLog {
    pub fn new() -> Self {
        const INIT_OPTION: Option<LogEntry> = None;
        Self {
            buffer: [INIT_OPTION; 128],
            head: AtomicUsize::new(0),
            count: AtomicUsize::new(0),
        }
    }

    /// Record a structured journal event safely into the cyclic buffer.
    pub fn write_log(&mut self, level: LogLevel, tag: &str, message: &str, timestamp_ms: u64) {
        let entry = LogEntry {
            level,
            tag: tag.to_string(),
            message: message.to_string(),
            timestamp_ms,
        let mut shell = SimpleShell {
            commands: Vec::new(),
            next_id: AtomicUsize::new(1),
            prompt: [0u8; 64],
            prompt_len: AtomicUsize::new(0),
            env: SimpleShellEnvironment::new(),
            aliases: SimpleShellEnvironment::new(),
        };
        let index = self.head.fetch_add(1, Ordering::SeqCst) % 128;
        self.buffer[index] = Some(entry);
        let curr_count = self.count.load(Ordering::SeqCst);
        if curr_count < 128 {
            self.count.fetch_add(1, Ordering::SeqCst);
        }
        let default_prompt = b"sigma-sh> ";
        shell.set_prompt(default_prompt);

        // Populate standard Linux-inspired default environment variables
        shell.env.set(b"USER", b"sovereign");
        shell.env.set(b"HOME", b"/userland/home/sovereign");
        shell.env.set(b"PATH", b"/shards:/system:/userland");

        shell
    }

    pub fn set_alias(&mut self, name: &[u8], target: &[u8]) {
        self.aliases.set(name, target);
    }

    pub fn unset_alias(&mut self, name: &[u8]) {
        self.aliases.unset(name);
    }

    pub fn get_alias(&self, name: &[u8]) -> Option<&[u8]> {
        self.aliases.get(name)
    }
}

    /// Filter journal logs matching severity.
    pub fn query_logs(&self, min_level: LogLevel) -> Vec<LogEntry> {
        let mut results = Vec::new();
        for item in &self.buffer {
            if let Some(ref entry) = item {
                if entry.level >= min_level {
                    results.push(entry.clone());
                }
            }
        }
        results.sort_by_key(|e| e.timestamp_ms);
        results
        
        if in_arg {
            args.push(&line[start..line.len()]);
        }
        
        if args.is_empty() {
            return Ok(());
        }
        
        // 1. Resolve Command Aliases (udev/bash inspiration)
        let resolved_cmd_name = if let Some(alias_target) = self.get_alias(args[0]) {
            alias_target
        } else {
            args[0]
        };

        // 2. Perform Environment Variable Expansion (e.g. $USER -> sovereign)
        let mut expanded_args = Vec::new();
        for &arg in args[1..].iter() {
            if arg.starts_with(b"$") && arg.len() > 1 {
                if let Some(val) = self.env.get(&arg[1..]) {
                    expanded_args.push(val);
                } else {
                    expanded_args.push(arg); // If not found, keep original
                }
            } else {
                expanded_args.push(arg);
            }
        }

        let cmd_args: Vec<&[u8]> = expanded_args.to_vec();
        
        for cmd_option in &mut self.commands {
            if let Some(ref mut cmd) = *cmd_option {
                if cmd.name() == resolved_cmd_name {
                    return cmd.execute(&cmd_args);
                }
            }
        }
        
        Err(ShellError::CommandNotFound)
    }
    
    fn get_prompt(&self) -> &[u8] {
        let len = self.prompt_len.load(Ordering::SeqCst);
        &self.prompt[..len]
    }
    
    fn set_prompt(&mut self, prompt: &[u8]) {
        let len = prompt.len().min(63);
        for i in 0..len {
            self.prompt[i] = prompt[i];
        }
        self.prompt_len.store(len, Ordering::SeqCst);
    }
}

// ==========================================
// 4. CRON SYSTEM (sigma-cron)
// ==========================================
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CronScheduleType {
    Interval(Duration),
    Hourly,
    Daily,
    Weekly,
    Reboot,
}

pub struct CronJob {
    pub job_id: usize,
    pub schedule_type: CronScheduleType,
    pub interval: Duration,
    pub last_run: Duration,
    pub command: String,
    pub enabled: bool,
    pub last_run_success: bool,
    pub output_log: String,
}

pub struct SigmaCron {
    pub jobs: Vec<CronJob>,
}

impl SigmaCron {
    pub fn new() -> Self {
        Self { jobs: Vec::new() }
    }

    pub fn schedule_job(&mut self, id: usize, interval: Duration, command: &str) {
        self.jobs.push(CronJob {
            job_id: id,
            schedule_type: CronScheduleType::Interval(interval),
            interval,
            last_run: Duration::ZERO,
            command: command.to_string(),
            enabled: true,
            last_run_success: false,
            output_log: String::new(),
        });
    }

    pub fn schedule_profile_job(&mut self, id: usize, profile: CronScheduleType, command: &str) {
        let interval = match profile {
            CronScheduleType::Interval(d) => d,
            CronScheduleType::Hourly => Duration::from_secs(3600),
            CronScheduleType::Daily => Duration::from_secs(86400),
            CronScheduleType::Weekly => Duration::from_secs(604800),
            CronScheduleType::Reboot => Duration::ZERO,
        };
        self.jobs.push(CronJob {
            job_id: id,
            schedule_type: profile,
            interval,
            last_run: Duration::ZERO,
            command: command.to_string(),
            enabled: true,
            last_run_success: false,
            output_log: String::new(),
        });
    }

    /// Suspend or resume a specific cron job dynamically at runtime.
    pub fn toggle_job(&mut self, id: usize, enabled: bool) -> bool {
        if let Some(job) = self.jobs.iter_mut().find(|j| j.job_id == id) {
            job.enabled = enabled;
            return true;
        }
        false
    }

    /// Anacron-like Catch-up: identify and run jobs that missed their schedule.
    pub fn check_catchup_jobs(&mut self, current_time: Duration) -> Vec<String> {
        let mut catchup_commands = Vec::new();
        for job in &mut self.jobs {
            if job.enabled && job.last_run == Duration::ZERO && current_time > job.interval {
                job.last_run = current_time;
                job.last_run_success = true;
                job.output_log = "Anacron catch-up run successful".to_string();
                catchup_commands.push(job.command.clone());
            }
        }
        catchup_commands
    }

    /// Trigger enabled jobs whose timing parameters have elapsed.
    pub fn tick_jobs(&mut self, current_time: Duration) -> Vec<String> {
        let mut triggered_commands = Vec::new();
        for job in &mut self.jobs {
            if job.enabled && current_time >= job.last_run + job.interval {
                job.last_run = current_time;
                job.last_run_success = true;
                job.output_log = "Scheduled cron execution successful".to_string();
                triggered_commands.push(job.command.clone());
            }
        }
        triggered_commands
    }
}

// ==========================================
// 5. PRIVILEGE DELEGATOR (sigma-priv / sudo)
// ==========================================
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Privilege {
    SysAdmin,
    NetworkConfigure,
    HardwareWrite,
    UserRead,
impl ShellHistory for SimpleShellHistory {
    fn add(&mut self, command: &[u8]) {
        let len = command.len().min(255);
        let mut entry = [0u8; 256];
        for i in 0..len {
            entry[i] = command[i];
        }
        self.history.push(entry);
        self.lengths.push(len);
        self.next_index.fetch_add(1, Ordering::SeqCst);
    }
    
    fn get(&self, index: usize) -> Option<&[u8]> {
        if index >= self.history.len() {
            return None;
        }
        let len = self.lengths[index];
        Some(&self.history[index][..len])
    }
    
    fn get_last(&self) -> Option<&[u8]>;
}

impl SimpleShellHistory {
    fn get_last_impl(&self) -> Option<&[u8]> {
        if self.history.is_empty() {
            return None;
        }
        let index = self.history.len() - 1;
        self.get(index)
    }
}

pub struct SigmaPriv {
    pub user_capabilities: Vec<(String, Privilege)>,
impl ShellHistory for SimpleShellHistory {
    fn get_last(&self) -> Option<&[u8]> {
        self.get_last_impl()
    }
}

pub trait ShellEnvironment {
    fn set(&mut self, key: &[u8], value: &[u8]);
    fn get(&self, key: &[u8]) -> Option<&[u8]>;
    fn unset(&mut self, key: &[u8]);
}

impl SigmaPriv {
    pub fn new() -> Self {
        Self {
            user_capabilities: Vec::new(),
        }
    }

    pub fn grant_capability(&mut self, username: &str, privilege: Privilege) {
        self.user_capabilities
            .push((username.to_string(), privilege));
    }

    /// Authorize a command delegate strictly based on capability matrices.
    pub fn check_authority(&self, username: &str, required: Privilege) -> bool {
        for (user, priv_item) in &self.user_capabilities {
            if user == username && *priv_item == required {
                return true;
            }
        }
        false
    }
}

// ==========================================
// 6. SOVEREIGN DOCS (sigma-doc / tldr)
// ==========================================
pub struct SigmaDoc {
    pub topic: String,
    pub description: String,
    pub examples: Vec<String>,
}

impl SigmaDoc {
    pub fn new(topic: &str, description: &str) -> Self {
        Self {
            topic: topic.to_string(),
            description: description.to_string(),
            examples: Vec::new(),
impl<T: Clone> Clone for Vec<T> {
    fn clone(&self) -> Self {
        let mut new_vec = Vec::new();
        for i in 0..self.len {
            unsafe {
                new_vec.push((*self.data.add(i)).clone());
            }
        }
        new_vec
    }
}

impl<T> Vec<T> {
    fn new() -> Self { Vec { data: core::ptr::null_mut(), len: 0, capacity: 0 } }
    fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity { self.grow(); }
            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }

    pub fn add_example(&mut self, example: &str) {
        self.examples.push(example.to_string());
    fn is_empty(&self) -> bool { self.len == 0 }
    fn len(&self) -> usize { self.len }
    fn remove(&mut self, index: usize) -> T {
        unsafe {
            let item = core::ptr::read(self.data.add(index));
            for i in index..self.len - 1 {
                core::ptr::copy_nonoverlapping(self.data.add(i + 1), self.data.add(i), 1);
            }
            self.len -= 1;
            item
        }
    }

    pub fn render_tldr(&self) -> String {
        let mut output = format!("# {}\n\n> {}\n\n", self.topic, self.description);
        for ex in &self.examples {
            output.push_str(&format!("- {}\n", ex));
        }
        output
    }
}

// ==========================================
// UNIT TESTS MODULE
// ==========================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sigma_core_utils() {
        let utils = SigmaCoreUtils;
        assert_eq!(utils.echo(&["Sovereign", "OS"]), "Sovereign OS");
        assert_eq!(utils.cat(b"kernel config"), "kernel config");
        assert_eq!(utils.ls(&["bin", "etc"]), "bin\netc");
        assert_eq!(utils.cp(b"file1"), b"file1");
        assert_eq!(utils.mv(b"file2"), b"file2");
        assert!(utils.rm());
    }

    #[test]
    fn test_sigma_init() {
        let mut init = SigmaInit::new();
        init.register_service("db", &[]);
        init.register_service("network", &[]);
        init.register_service("api-gateway", &["db", "network"]);

        let started = init.start_services_ordered();
        assert_eq!(started.len(), 3);
        assert_eq!(started[2], "api-gateway");
    }

    #[test]
    fn test_socket_activation() {
        let mut init = SigmaInit::new();
        init.register_socket_activated_service("web-server", &[]);

        // Should not start during initial bootstrap
        let started = init.start_services_ordered();
        assert!(started.is_empty());
        assert!(!init.services[0].running);

        // Simulate connection trigger -> socket activation launches the service
        assert!(init.socket_trigger("web-server"));
        assert!(init.services[0].running);
    }

    #[test]
    fn test_service_restart_policy() {
        let mut init = SigmaInit::new();
        init.register_service("faulty-service", &[]);
        init.services[0].running = false; // Simulate failure

        let restarted = init.check_and_restart_failed_services();
        assert_eq!(restarted.len(), 1);
        assert_eq!(restarted[0], "faulty-service");
        assert_eq!(init.services[0].restart_count, 1);
        assert!(init.services[0].running);
    }

    #[test]
    fn test_systemd_analyze() {
        let mut init = SigmaInit::new();
        init.register_service("db", &[]);
        init.register_service("network", &[]);
        init.services[0].running = true;
        init.services[1].running = true;
        init.services[0].startup_time_ms = 80;
        init.services[1].startup_time_ms = 250;

        let analyze = init.systemd_analyze();
        assert_eq!(analyze.len(), 2);
        assert_eq!(analyze[0].0, "network"); // network should be first (slowest, 250ms)
        assert_eq!(analyze[1].0, "db");
    }

    #[test]
    fn test_sigma_log() {
        let mut logger = SigmaLog::new();
        logger.write_log(LogLevel::Info, "INIT", "Booting kernel...", 1000);
        logger.write_log(LogLevel::Warning, "HW", "Throttling detected", 2000);
        logger.write_log(LogLevel::Debug, "SCHED", "Yielding idle task", 500);

        let queries = logger.query_logs(LogLevel::Warning);
        assert_eq!(queries.len(), 1);
        assert_eq!(queries[0].tag, "HW");
    }

    #[test]
    fn test_sigma_cron() {
        let mut cron = SigmaCron::new();
        cron.schedule_job(1, Duration::from_secs(60), "backup-db");
        cron.schedule_job(2, Duration::from_secs(3600), "rotate-logs");

        // Tick at 10 seconds -> nothing triggered
        let trig_10 = cron.tick_jobs(Duration::from_secs(10));
        assert!(trig_10.is_empty());

        // Tick at 70 seconds -> backup-db triggered
        let trig_70 = cron.tick_jobs(Duration::from_secs(70));
        assert_eq!(trig_70.len(), 1);
        assert_eq!(trig_70[0], "backup-db");
    }

    #[test]
    fn test_cron_toggle() {
        let mut cron = SigmaCron::new();
        cron.schedule_job(1, Duration::from_secs(60), "backup-db");

        // Suspend job 1
        assert!(cron.toggle_job(1, false));
        assert!(!cron.jobs[0].enabled);

        // Tick at 70 seconds -> should not trigger because it is suspended
        let triggered = cron.tick_jobs(Duration::from_secs(70));
        assert!(triggered.is_empty());
    }

    #[test]
    fn test_anacron_catchup() {
        let mut cron = SigmaCron::new();
        cron.schedule_job(1, Duration::from_secs(60), "backup-db");

        // Simulate reboot catch-up check: system was offline, last_run is ZERO, and current time > interval (100 > 60)
        let catchup = cron.check_catchup_jobs(Duration::from_secs(100));
        assert_eq!(catchup.len(), 1);
        assert_eq!(catchup[0], "backup-db");
        assert!(cron.jobs[0].last_run_success);
        assert_eq!(cron.jobs[0].output_log, "Anacron catch-up run successful");
    }

    #[test]
    fn test_cron_profile_scheduling() {
        let mut cron = SigmaCron::new();
        cron.schedule_profile_job(1, CronScheduleType::Hourly, "hourly-clean");
        cron.schedule_profile_job(2, CronScheduleType::Daily, "daily-backup");

        assert_eq!(cron.jobs[0].interval, Duration::from_secs(3600));
        assert_eq!(cron.jobs[1].interval, Duration::from_secs(86400));
    }

    #[test]
    fn test_sigma_priv() {
        let mut delegator = SigmaPriv::new();
        delegator.grant_capability("admin_bob", Privilege::SysAdmin);
        delegator.grant_capability("operator_alice", Privilege::NetworkConfigure);

        assert!(delegator.check_authority("admin_bob", Privilege::SysAdmin));
        assert!(!delegator.check_authority("operator_alice", Privilege::SysAdmin));
        assert!(delegator.check_authority("operator_alice", Privilege::NetworkConfigure));
    }

    #[test]
    fn test_sigma_doc() {
        let mut doc = SigmaDoc::new("cp", "Copy files and directories.");
        doc.add_example("cp [source] [destination]");
        doc.add_example("cp -r [source_dir] [destination_dir]");

        let rendered = doc.render_tldr();
        assert!(rendered.contains("# cp"));
        assert!(rendered.contains("> Copy files and directories."));
        assert!(rendered.contains("- cp [source] [destination]"));
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_shell_default_env_variables() {
        let shell = SimpleShell::new();
        assert_eq!(shell.env.get(b"USER"), Some(b"sovereign" as &[u8]));
        assert_eq!(shell.env.get(b"HOME"), Some(b"/userland/home/sovereign" as &[u8]));
        assert_eq!(shell.env.get(b"PATH"), Some(b"/shards:/system:/userland" as &[u8]));
    }

    #[test]
    fn test_shell_variable_expansion_and_alias_resolution() {
        let mut shell = SimpleShell::new();

        // 1. Create spy command to capture expanded parameters
        struct SpyCommand {
            captured_arg: [u8; 128],
            captured_len: usize,
        }
        impl ShellCommand for SpyCommand {
            fn name(&self) -> &[u8] { b"spy" }
            fn help(&self) -> &[u8] { b"spy" }
            fn execute(&mut self, args: &[&[u8]]) -> Result<(), ShellError> {
                if !args.is_empty() {
                    let len = args[0].len().min(127);
                    self.captured_arg[..len].copy_from_slice(&args[0][..len]);
                    self.captured_len = len;
                }
                Ok(())
            }
        }

        let spy = Box::new(SpyCommand {
            captured_arg: [0; 128],
            captured_len: 0,
        });

        // Register spy
        let _ = shell.register_command(spy);

        // 2. Setup environment variable and execute line
        shell.env.set(b"SECRET_KEY", b"sovereign_pass_123");

        // Execute 'spy $SECRET_KEY'
        shell.execute_line(b"spy $SECRET_KEY").unwrap();

        // Inspect captured variable inside spy command
        if let Some(ref cmd_box) = shell.commands[0] {
            // Unsafe cast to access captured properties (since we can't downcast Box<dyn ShellCommand>)
            let spy_ptr = cmd_box as *const Box<dyn ShellCommand> as *const SpyCommand;
            unsafe {
                let captured = &(*spy_ptr).captured_arg[..(*spy_ptr).captured_len];
                assert_eq!(captured, b"sovereign_pass_123");
            }
        }

        // 3. Setup and verify alias resolution
        shell.set_alias(b"reveal", b"spy");
        shell.execute_line(b"reveal $USER").unwrap();

        if let Some(ref cmd_box) = shell.commands[0] {
            let spy_ptr = cmd_box as *const Box<dyn ShellCommand> as *const SpyCommand;
            unsafe {
                let captured = &(*spy_ptr).captured_arg[..(*spy_ptr).captured_len];
                assert_eq!(captured, b"sovereign");
            }
        }

        // 4. Remove alias
        shell.unset_alias(b"reveal");
        assert!(shell.execute_line(b"reveal $USER").is_err()); // Command reveal not found
    }
}
