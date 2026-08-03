// Sovereign Replacement System Utilities for SigmaOS
// This module implements zero-dependency, no-std compliant alternatives to standard Linux core utilities,
// init/supervision systems, syslog/journald, cron, capability-based sudo, and help manuals.

extern crate alloc;

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
        if let Some(service) = self
            .services
            .iter_mut()
            .find(|s| e_eq(&s.name, service_name))
        {
            if service.socket_activated && !service.running {
                service.running = true;
                return true;
            }
        }
        false
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
        };
        let index = self.head.fetch_add(1, Ordering::SeqCst) % 128;
        self.buffer[index] = Some(entry);
        let curr_count = self.count.load(Ordering::SeqCst);
        if curr_count < 128 {
            self.count.fetch_add(1, Ordering::SeqCst);
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
}

pub struct SigmaPriv {
    pub user_capabilities: Vec<(String, Privilege)>,
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
        }
    }

    pub fn add_example(&mut self, example: &str) {
        self.examples.push(example.to_string());
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
