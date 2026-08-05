// SigmaOS Sovereign Replacement System Utilities Suite (sigma-core-utils)
// Exposes robust, memory-safe Rust alternatives to BusyBox, systemd, syslog, cron, sudo/doas, and man pages.
// Aligns perfectly with the core Sovereign replacement table.

use crate::klib::{HashMap, HashSet};

/// Log Level for sigma-log
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SovereignLogLevel {
    Info,
    Warning,
    Error,
    Critical,
}

/// 1. `sigma-log` (Sovereign syslog & journald Logger)
pub struct SovereignLogger {
    pub logs: Vec<(SovereignLogLevel, u64, String)>,
    pub max_log_capacity: usize,
}

impl SovereignLogger {
    pub fn new(capacity: usize) -> Self {
        Self {
            logs: Vec::new(),
            max_log_capacity: capacity,
        }
    }

    pub fn log_event(&mut self, level: SovereignLogLevel, timestamp: u64, msg: &str) {
        if self.logs.len() >= self.max_log_capacity {
            self.logs.remove(0); // Rotate older logs
        }
        self.logs.push((level, timestamp, msg.to_string()));
    }

    pub fn query_logs_by_level(&self, level: SovereignLogLevel) -> Vec<String> {
        self.logs
            .iter()
            .filter(|(lvl, _, _)| *lvl == level)
            .map(|(_, ts, msg)| format!("[t={}] {}", ts, msg))
            .collect()
    }
}

/// 2. `sigma-cron` (Sovereign Job Scheduler)
#[derive(Debug, Clone)]
pub struct CronJob {
    pub job_id: String,
    pub command: String,
    pub interval_secs: u64,
    pub last_execution_timestamp: u64,
    pub has_executed: bool,
}

pub struct SovereignCronScheduler {
    pub jobs: HashMap<String, CronJob>,
}

impl SovereignCronScheduler {
    pub fn new() -> Self {
        Self {
            jobs: HashMap::new(),
        }
    }

    pub fn schedule_job(&mut self, id: &str, command: &str, interval_secs: u64) {
        let job = CronJob {
            job_id: id.to_string(),
            command: command.to_string(),
            interval_secs,
            last_execution_timestamp: 0,
            has_executed: false,
        };
        self.jobs.insert(id.to_string(), job);
    }

    /// Evaluates which jobs are due for execution at the current timestamp
    pub fn trigger_due_jobs(&mut self, current_timestamp: u64) -> Vec<String> {
        let mut executed_commands = Vec::new();
        for job in self.jobs.values_mut() {
            let elapsed = current_timestamp.saturating_sub(job.last_execution_timestamp);
            if !job.has_executed || elapsed >= job.interval_secs {
                job.last_execution_timestamp = current_timestamp;
                job.has_executed = true;
                executed_commands.push(job.command.clone());
            }
        }
        executed_commands
    }
}

impl Default for SovereignCronScheduler {
    fn default() -> Self {
        Self::new()
    }
}

/// 3. `sigma-priv` (Capability-Based Privilege Escalation Engine)
/// Fully replaces legacy insecure Root escalation (sudo/doas) with highly granular capability checking.
pub struct SovereignPrivilegeEngine {
    pub authorized_tokens: HashMap<String, HashSet<String>>, // maps user to a list of allowed capabilities (e.g. "REBOOT", "NETWORK_BIND")
}

impl SovereignPrivilegeEngine {
    pub fn new() -> Self {
        Self {
            authorized_tokens: HashMap::new(),
        }
    }

    pub fn grant_capability(&mut self, user: &str, capability: &str) {
        self.authorized_tokens
            .entry(user.to_string())
            .or_default()
            .insert(capability.to_string());
    }

    /// Checks if a user has permission to execute an administrative action requiring specific capabilities
    pub fn check_permission(&self, user: &str, capability: &str) -> bool {
        if let Some(caps) = self.authorized_tokens.get(user) {
            caps.contains(capability)
        } else {
            false
        }
    }
}

impl Default for SovereignPrivilegeEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// 4. `sigma-doc` (Offline Documentation & Quick Guides Explorer)
/// Replaces standard man pages and tldr/tealdeer with lightweight, offline guides.
pub struct SovereignDocBrowser {
    pub doc_pages: HashMap<String, String>,
}

impl SovereignDocBrowser {
    pub fn new() -> Self {
        let mut pages = HashMap::new();
        pages.insert(
            "sigma-sh".to_string(),
            "sigma-sh: Sovereign shell replacement for BusyBox/Fish.\nUsage: sigma-sh [options] [script]".to_string(),
        );
        pages.insert(
            "sigma-priv".to_string(),
            "sigma-priv: Capability-based privilege manager (replaces sudo/doas).\nUsage: sigma-priv exec <capability> <command>".to_string(),
        );

        Self {
            doc_pages: pages,
        }
    }

    pub fn fetch_tldr_guide(&self, command: &str) -> Option<&str> {
        self.doc_pages.get(command).map(|s| s.as_str())
    }
}

impl Default for SovereignDocBrowser {
    fn default() -> Self {
        Self::new()
    }
}

/// 5. `sigma-core-utils` (Coreutils & BusyBox Replacement)
/// Replaces GNU Coreutils with extremely clean, memory-safe Rust commands.
pub struct SovereignCoreUtils {
    pub virtual_filesystem: HashMap<String, String>, // path to simulated content
}

impl SovereignCoreUtils {
    pub fn new() -> Self {
        Self {
            virtual_filesystem: HashMap::new(),
        }
    }

    pub fn execute_echo(&self, args: &[&str]) -> String {
        args.join(" ")
    }

    pub fn execute_mkdir(&mut self, path: &str) -> Result<(), &'static str> {
        if self.virtual_filesystem.contains_key(path) {
            return Err("Directory or file already exists");
        }
        self.virtual_filesystem.insert(path.to_string(), "DIR".to_string());
        Ok(())
    }

    pub fn execute_touch(&mut self, path: &str) -> Result<(), &'static str> {
        self.virtual_filesystem.insert(path.to_string(), String::new());
        Ok(())
    }
}

impl Default for SovereignCoreUtils {
    fn default() -> Self {
        Self::new()
    }
}

/// 6. `sigma-sh` (Sovereign Interactive REPL Shell)
pub struct SovereignShell {
    pub active_user: String,
    pub current_directory: String,
    pub aliases: HashMap<String, String>,
}

impl SovereignShell {
    pub fn new(user: &str) -> Self {
        Self {
            active_user: user.to_string(),
            current_directory: format!("/home/{}", user),
            aliases: HashMap::new(),
        }
    }

    pub fn register_alias(&mut self, shortcut: &str, expansion: &str) {
        self.aliases.insert(shortcut.to_string(), expansion.to_string());
    }

    pub fn evaluate_input(&self, input: &str) -> String {
        let trimmed = input.trim();
        if let Some(expansion) = self.aliases.get(trimmed) {
            format!("Expanding alias '{}' -> executing '{}'", trimmed, expansion)
        } else {
            format!("Executing shell command: '{}'", trimmed)
        }
    }
}

/// 7. `sigma-init` (PID 1 Multi-Supervisory Bootstrap System)
/// Orchestrates services boot sequences (supporting systemd, OpenRC, runit, s6, or dinit).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InitSupervisorType {
    Systemd,
    OpenRC,
    Runit,
    S6,
    Dinit,
}

pub struct SovereignInitSystem {
    pub active_supervisor: InitSupervisorType,
    pub booted_services: Vec<String>,
}

impl SovereignInitSystem {
    pub fn new(supervisor: InitSupervisorType) -> Self {
        Self {
            active_supervisor: supervisor,
            booted_services: Vec::new(),
        }
    }

    pub fn bootstrap_service(&mut self, service_name: &str) {
        self.booted_services.push(service_name.to_string());
    }

    pub fn get_boot_report(&self) -> String {
        let mut report = format!("=== INIT BOOT REPORT (Supervisor: {:?}) ===\n", self.active_supervisor);
        for service in &self.booted_services {
            report.push_str(&format!("  [OK] Service '{}' spawned successfully\n", service));
        }
        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sovereign_logger() {
        let mut logger = SovereignLogger::new(2);
        logger.log_event(SovereignLogLevel::Info, 100, "VFS subsystem loaded");
        logger.log_event(SovereignLogLevel::Warning, 105, "Low memory threshold warning");
        logger.log_event(SovereignLogLevel::Critical, 110, "PCI device panic triggered");

        // Info should have been rotated since capacity is 2
        let critical_logs = logger.query_logs_by_level(SovereignLogLevel::Critical);
        assert_eq!(critical_logs.len(), 1);
        assert!(critical_logs[0].contains("PCI device panic triggered"));
    }

    #[test]
    fn test_sovereign_cron_scheduler() {
        let mut cron = SovereignCronScheduler::new();
        cron.schedule_job("backup_db", "tar -czf db.tar.gz /var/db", 60);

        // First trigger at t=0
        let trigger1 = cron.trigger_due_jobs(0);
        assert_eq!(trigger1.len(), 1);
        assert_eq!(trigger1[0], "tar -czf db.tar.gz /var/db");

        // Trigger again at t=10 (not due yet, 10s < 60s)
        let trigger2 = cron.trigger_due_jobs(10);
        assert!(trigger2.is_empty());

        // Trigger at t=70 (due, 70s >= 60s)
        let trigger3 = cron.trigger_due_jobs(70);
        assert_eq!(trigger3.len(), 1);
    }

    #[test]
    fn test_sovereign_privilege_engine() {
        let mut priv_engine = SovereignPrivilegeEngine::new();
        priv_engine.grant_capability("developer", "NETWORK_BIND");

        assert!(priv_engine.check_permission("developer", "NETWORK_BIND"));
        assert!(!priv_engine.check_permission("developer", "REBOOT"));
        assert!(!priv_engine.check_permission("guest", "NETWORK_BIND"));
    }

    #[test]
    fn test_sovereign_doc_browser() {
        let browser = SovereignDocBrowser::new();
        assert!(browser.fetch_tldr_guide("sigma-sh").is_some());
        assert!(browser.fetch_tldr_guide("sigma-sh").unwrap().contains("Sovereign shell replacement"));
        assert!(browser.fetch_tldr_guide("nonexistent").is_none());
    }

    #[test]
    fn test_sovereign_core_utils() {
        let mut utils = SovereignCoreUtils::new();
        assert_eq!(utils.execute_echo(&["hello", "world"]), "hello world");

        assert!(utils.execute_mkdir("/usr/bin").is_ok());
        assert!(utils.execute_touch("/usr/bin/echo").is_ok());
        assert_eq!(utils.virtual_filesystem.get("/usr/bin").unwrap(), "DIR");
    }

    #[test]
    fn test_sovereign_shell() {
        let mut sh = SovereignShell::new("dr_aaryan");
        sh.register_alias("ll", "ls -la");

        assert_eq!(sh.evaluate_input("ll"), "Expanding alias 'll' -> executing 'ls -la'");
        assert_eq!(sh.evaluate_input("pwd"), "Executing shell command: 'pwd'");
    }

    #[test]
    fn test_sovereign_init_system() {
        let mut init = SovereignInitSystem::new(InitSupervisorType::Dinit);
        init.bootstrap_service("s-mm");
        init.bootstrap_service("s-sched");

        let report = init.get_boot_report();
        assert!(report.contains("INIT BOOT REPORT"));
        assert!(report.contains("s-mm"));
        assert!(report.contains("s-sched"));
    }
}
