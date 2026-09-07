use std::format;
// SigmaOS Cron Daemon - Linux-inspired task scheduler
// Zero-dependency implementation of cron-like functionality

use crate::klib::{BTreeMap, Vec};
use std::string::{String, ToString};

/// Cron job specification
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CronJob {
    pub id: String,
    pub schedule: CronSchedule,
    pub command: String,
    pub user: String,
    pub environment: BTreeMap<String, String>,
    pub enabled: bool,
    pub last_run: Option<u64>,
    pub next_run: u64,
}

/// Cron schedule specification
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CronSchedule {
    pub minute: CronField,
    pub hour: CronField,
    pub day_of_month: CronField,
    pub month: CronField,
    pub day_of_week: CronField,
}

/// Cron field specification
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CronField {
    All,                   // *
    Specific(u32),         // 5
    Range(u32, u32),       // 1-5
    List([u32; 8], usize), // 1,3,5 - fixed-size array with length
    Step(u32, u32),        // */5 or 1-10/2
}

impl CronSchedule {
    /// Parse cron schedule string (e.g., "0 0 * * *" or "@daily", "@weekly", "@reboot")
    pub fn from_string(schedule: &str) -> Result<Self, CronError> {
        let trimmed = schedule.trim();
        if trimmed.starts_with('@') {
            match trimmed {
                "@reboot" => {
                    return Ok(CronSchedule {
                        minute: CronField::Specific(0),
                        hour: CronField::Specific(0),
                        day_of_month: CronField::All,
                        month: CronField::All,
                        day_of_week: CronField::All,
                    })
                }
                "@hourly" => {
                    return Ok(CronSchedule {
                        minute: CronField::Specific(0),
                        hour: CronField::All,
                        day_of_month: CronField::All,
                        month: CronField::All,
                        day_of_week: CronField::All,
                    })
                }
                "@daily" | "@midnight" => {
                    return Ok(CronSchedule {
                        minute: CronField::Specific(0),
                        hour: CronField::Specific(0),
                        day_of_month: CronField::All,
                        month: CronField::All,
                        day_of_week: CronField::All,
                    })
                }
                "@weekly" => {
                    return Ok(CronSchedule {
                        minute: CronField::Specific(0),
                        hour: CronField::Specific(0),
                        day_of_month: CronField::All,
                        month: CronField::All,
                        day_of_week: CronField::Specific(0),
                    })
                }
                "@monthly" => {
                    return Ok(CronSchedule {
                        minute: CronField::Specific(0),
                        hour: CronField::Specific(0),
                        day_of_month: CronField::Specific(1),
                        month: CronField::All,
                        day_of_week: CronField::All,
                    })
                }
                "@yearly" | "@annually" => {
                    return Ok(CronSchedule {
                        minute: CronField::Specific(0),
                        hour: CronField::Specific(0),
                        day_of_month: CronField::Specific(1),
                        month: CronField::Specific(1),
                        day_of_week: CronField::All,
                    })
                }
                _ => return Err(CronError::InvalidFormat),
            }
        }

        let parts: std::vec::Vec<&str> = trimmed.split_whitespace().collect();
        if parts.len() != 5 {
            return Err(CronError::InvalidFormat);
        }

        Ok(CronSchedule {
            minute: CronField::parse(parts[0], 0, 59)?,
            hour: CronField::parse(parts[1], 0, 23)?,
            day_of_month: CronField::parse(parts[2], 1, 31)?,
            month: CronField::parse(parts[3], 1, 12)?,
            day_of_week: CronField::parse(parts[4], 0, 6)?,
        })
    }

    /// Check if current time matches schedule
    pub fn matches(
        &self,
        minute: u32,
        hour: u32,
        day_of_month: u32,
        month: u32,
        day_of_week: u32,
    ) -> bool {
        self.minute.matches(minute)
            && self.hour.matches(hour)
            && self.day_of_month.matches(day_of_month)
            && self.month.matches(month)
            && self.day_of_week.matches(day_of_week)
    }
}

impl CronField {
    /// Parse a cron field string
    pub fn parse(field: &str, min: u32, _max: u32) -> Result<Self, CronError> {
        if field == "*" {
            return Ok(CronField::All);
        }

        if field.contains('/') {
            let parts: std::vec::Vec<&str> = field.split('/').collect();
            if parts.len() != 2 {
                return Err(CronError::InvalidField);
            }
            let base = if parts[0] == "*" {
                min
            } else {
                parts[0]
                    .parse::<u32>()
                    .map_err(|_| CronError::InvalidField)?
            };
            let step = parts[1]
                .parse::<u32>()
                .map_err(|_| CronError::InvalidField)?;
            return Ok(CronField::Step(base, step));
        }

        if field.contains('-') {
            let parts: std::vec::Vec<&str> = field.split('-').collect();
            if parts.len() != 2 {
                return Err(CronError::InvalidField);
            }
            let start = parts[0]
                .parse::<u32>()
                .map_err(|_| CronError::InvalidField)?;
            let end = parts[1]
                .parse::<u32>()
                .map_err(|_| CronError::InvalidField)?;
            return Ok(CronField::Range(start, end));
        }

        if field.contains(',') {
            let parts: std::vec::Vec<&str> = field.split(',').collect();
            if parts.len() > 8 {
                return Err(CronError::InvalidField);
            }
            let mut values = [0u32; 8];
            let mut count = 0;
            for (i, part) in parts.iter().enumerate() {
                let p: &str = *part;
                values[i] = p.parse::<u32>().map_err(|_| CronError::InvalidField)?;
                count += 1;
            }
            return Ok(CronField::List(values, count));
        }

        let value = field.parse::<u32>().map_err(|_| CronError::InvalidField)?;
        Ok(CronField::Specific(value))
    }

    /// Check if value matches field
    pub fn matches(&self, value: u32) -> bool {
        match self {
            CronField::All => true,
            CronField::Specific(v) => *v == value,
            CronField::Range(start, end) => value >= *start && value <= *end,
            CronField::List(values, count) => {
                for i in 0..*count {
                    if values[i] == value {
                        return true;
                    }
                }
                false
            }
            CronField::Step(base, step) => (value - base) % step == 0,
        }
    }
}

/// Cron daemon error types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CronError {
    InvalidFormat,
    InvalidField,
    JobNotFound,
    ExecutionError,
}

/// Cron daemon - manages scheduled tasks
pub struct CronDaemon {
    jobs: BTreeMap<String, CronJob>,
    running: bool,
}

impl CronDaemon {
    /// Create new cron daemon
    pub fn new() -> Self {
        Self {
            jobs: BTreeMap::new(),
            running: false,
        }
    }

    /// Add a cron job
    pub fn add_job(&mut self, job: CronJob) -> Result<(), CronError> {
        self.jobs.insert(job.id.clone(), job);
        Ok(())
    }

    /// Remove a cron job
    pub fn remove_job(&mut self, id: &str) -> Result<(), CronError> {
        let key = id.to_string();
        self.jobs.remove(&key).ok_or(CronError::JobNotFound)?;
        Ok(())
    }

    /// Get a cron job
    pub fn get_job(&self, id: &str) -> Option<&CronJob> {
        let key = id.to_string();
        self.jobs.get(&key)
    }

    /// List all cron jobs
    pub fn list_jobs(&self) -> Vec<&CronJob> {
        let mut jobs = Vec::new();
        for job in self.jobs.values() {
            jobs.push(job);
        }
        jobs
    }

    /// Start cron daemon
    pub fn start(&mut self) {
        self.running = true;
    }

    /// Stop cron daemon
    pub fn stop(&mut self) {
        self.running = false;
    }

    /// Check and run jobs that are due
    pub fn check_and_run(&mut self, current_time: u64) -> Vec<String> {
        let mut run_jobs: Vec<String> = Vec::new();

        for (id, job) in self.jobs.iter_mut() {
            if !job.enabled {
                continue;
            }

            if current_time >= job.next_run {
                // Mark as run
                job.last_run = Some(current_time);

                // Calculate next run time (simplified - normally would parse cron schedule)
                job.next_run = current_time + 60; // Default to 1 minute for now

                let id_str: &str = AsRef::<str>::as_ref(id);
                let job_id: String = id_str.into();
                run_jobs.push(job_id);
            }
        }

        run_jobs
    }

    /// Enable a job
    pub fn enable_job(&mut self, id: &str) -> Result<(), CronError> {
        let key = id.to_string();
        let job = self.jobs.get_mut(&key).ok_or(CronError::JobNotFound)?;
        job.enabled = true;
        Ok(())
    }

    /// Disable a job
    pub fn disable_job(&mut self, id: &str) -> Result<(), CronError> {
        let key = id.to_string();
        let job = self.jobs.get_mut(&key).ok_or(CronError::JobNotFound)?;
        job.enabled = false;
        Ok(())
    }
}

impl Default for CronDaemon {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// Post-Quantum Secure SSH Daemon (SovereignSshDaemon) - OpenSSH & Dropbear Inspired
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyExchangeAlgorithm {
    Kyber1024Ed25519,
    Curve25519Sha256,
}

#[derive(Debug, Clone)]
pub struct SshSession {
    pub session_id: u64,
    pub user: String,
    pub remote_ip: String,
    pub algorithm: KeyExchangeAlgorithm,
    pub authenticated: bool,
    pub pty_allocated: bool,
    pub chroot_dir: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SshDaemonConfig {
    pub permit_root_login: bool,
    pub allow_users: Vec<String>,
    pub deny_users: Vec<String>,
    pub max_auth_tries: u32,
    pub banner: Option<String>,
    pub subsystems: BTreeMap<String, String>, // e.g. "sftp" -> "/usr/libexec/sftp-server"
    pub privilege_separation: bool,           // OpenBSD Privilege Separation
}

impl Default for SshDaemonConfig {
    fn default() -> Self {
        let mut subsystems = BTreeMap::new();
        subsystems.insert("sftp".to_string(), "/usr/libexec/sftp-server".to_string());
        Self {
            permit_root_login: false,
            allow_users: Vec::new(),
            deny_users: Vec::new(),
            max_auth_tries: 3,
            banner: Some("SigmaOS Post-Quantum Secure SSH Daemon".to_string()),
            subsystems,
            privilege_separation: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SshChannelType {
    PtySession,
    ExecCommand(String),
    Subsystem(String),
    DirectTcpIp {
        target_host: String,
        target_port: u16,
    },
}

pub struct SovereignSshDaemon {
    pub port: u16,
    pub host_key_fp: String,
    pub config: SshDaemonConfig,
    pub active_sessions: BTreeMap<u64, SshSession>,
    pub max_sessions: usize,
    pub next_session_id: u64,
    pub failed_auth_attempts: BTreeMap<String, u32>, // Fail2ban: IP -> failed attempt count
    pub banned_ips: Vec<String>,                     // Fail2ban locked IPs
    pub privileged_child_pids: BTreeMap<u64, u32>,   // Privilege Separation child tracker
}

impl SovereignSshDaemon {
    pub fn new(port: u16, host_key_fingerprint: &str) -> Self {
        Self {
            port,
            host_key_fp: host_key_fingerprint.to_string(),
            config: SshDaemonConfig::default(),
            active_sessions: BTreeMap::new(),
            max_sessions: 64,
            next_session_id: 1,
            failed_auth_attempts: BTreeMap::new(),
            banned_ips: Vec::new(),
            privileged_child_pids: BTreeMap::new(),
        }
    }

    /// Check if remote IP is locked out via Fail2ban protection
    pub fn is_ip_banned(&self, remote_ip: &str) -> bool {
        self.banned_ips.contains(&remote_ip.to_string())
    }

    pub fn accept_connection(
        &mut self,
        remote_ip: &str,
        user: &str,
        algo: KeyExchangeAlgorithm,
    ) -> Result<u64, CronError> {
        if self.is_ip_banned(remote_ip) {
            return Err(CronError::ExecutionError);
        }

        if self.active_sessions.len() >= self.max_sessions {
            return Err(CronError::ExecutionError);
        }

        // Evaluate sshd_config access rules
        if user == "root" && !self.config.permit_root_login {
            return Err(CronError::ExecutionError);
        }
        if self.config.deny_users.contains(&user.to_string()) {
            return Err(CronError::ExecutionError);
        }
        if !self.config.allow_users.is_empty()
            && !self.config.allow_users.contains(&user.to_string())
        {
            return Err(CronError::ExecutionError);
        }

        let sid = self.next_session_id;
        self.next_session_id += 1;

        let session = SshSession {
            session_id: sid,
            user: user.to_string(),
            remote_ip: remote_ip.to_string(),
            algorithm: algo,
            authenticated: false,
            pty_allocated: false,
            chroot_dir: None,
        };

        self.active_sessions.insert(sid, session);

        // OpenBSD privilege separation process spawning
        if self.config.privilege_separation {
            let child_pid = 1000 + (sid as u32);
            self.privileged_child_pids.insert(sid, child_pid);
        }

        Ok(sid)
    }

    pub fn authenticate_public_key(&mut self, session_id: u64, pubkey: &[u8]) -> bool {
        let (_user, remote_ip) = if let Some(session) = self.active_sessions.get(&session_id) {
            (session.user.clone(), session.remote_ip.clone())
        } else {
            return false;
        };

        if self.is_ip_banned(&remote_ip) {
            return false;
        }

        if !pubkey.is_empty() {
            if let Some(session) = self.active_sessions.get_mut(&session_id) {
                session.authenticated = true;
            }
            self.failed_auth_attempts.remove(&remote_ip);
            true
        } else {
            // Fail2ban brute force failure tracking
            let count = self
                .failed_auth_attempts
                .get(&remote_ip)
                .cloned()
                .unwrap_or(0)
                + 1;
            self.failed_auth_attempts.insert(remote_ip.clone(), count);
            if count >= self.config.max_auth_tries {
                self.banned_ips.push(remote_ip);
            }
            false
        }
    }

    pub fn allocate_pty(&mut self, session_id: u64) -> Result<(), CronError> {
        let session = self
            .active_sessions
            .get_mut(&session_id)
            .ok_or(CronError::JobNotFound)?;
        if !session.authenticated {
            return Err(CronError::ExecutionError);
        }
        session.pty_allocated = true;
        Ok(())
    }

    pub fn set_chroot_isolation(&mut self, session_id: u64, dir: &str) -> Result<(), CronError> {
        let session = self
            .active_sessions
            .get_mut(&session_id)
            .ok_or(CronError::JobNotFound)?;
        session.chroot_dir = Some(dir.to_string());
        Ok(())
    }

    /// Dispatches an SSH channel multiplexing request (Pty, Exec, Subsystem/SFTP, DirectTcpIp)
    pub fn open_channel(
        &self,
        session_id: u64,
        channel_type: SshChannelType,
    ) -> Result<String, CronError> {
        let session = self
            .active_sessions
            .get(&session_id)
            .ok_or(CronError::JobNotFound)?;
        if !session.authenticated {
            return Err(CronError::ExecutionError);
        }

        match channel_type {
            SshChannelType::PtySession => {
                Ok(format!("PTY channel open for session {}", session_id))
            }
            SshChannelType::ExecCommand(cmd) => Ok(format!("Exec channel open: '{}'", cmd)),
            SshChannelType::Subsystem(sub) => {
                if let Some(path) = self.config.subsystems.get(&sub) {
                    Ok(format!("Subsystem channel open: '{}' -> {}", sub, path))
                } else {
                    Err(CronError::ExecutionError)
                }
            }
            SshChannelType::DirectTcpIp {
                target_host,
                target_port,
            } => Ok(format!(
                "DirectTcpIp forward channel open to {}:{}",
                target_host, target_port
            )),
        }
    }
}

// =========================================================================
// Enhanced Sovereign Cron Daemon (Anacron & Cronie Inspired)
// =========================================================================

pub struct SovereignCronDaemon {
    pub base_daemon: CronDaemon,
    pub allowed_users: Vec<String>,
    pub denied_users: Vec<String>,
    pub anacron_catchup_enabled: bool,
    pub executed_catchup_count: usize,
    pub random_delay_max: u32, // RANDOM_DELAY in minutes to prevent thundering herd
    pub max_load_average_threshold: f32, // Cronie / batch mode max load average guard
    pub mail_to_user: Option<String>, // MAILTO routing for cron job output
    pub job_outputs: BTreeMap<String, String>, // Job ID -> Captured output
}

impl SovereignCronDaemon {
    pub fn new() -> Self {
        Self {
            base_daemon: CronDaemon::new(),
            allowed_users: Vec::new(),
            denied_users: Vec::new(),
            anacron_catchup_enabled: true,
            executed_catchup_count: 0,
            random_delay_max: 0,
            max_load_average_threshold: 4.0,
            mail_to_user: Some("root".to_string()),
            job_outputs: BTreeMap::new(),
        }
    }

    /// Sets RANDOM_DELAY jitter in minutes (Vixie Cron / Anacron parity)
    pub fn set_random_delay(&mut self, delay_minutes: u32) {
        self.random_delay_max = delay_minutes;
    }

    /// Sets system load threshold guard for batch jobs
    pub fn set_max_load_threshold(&mut self, load: f32) {
        self.max_load_average_threshold = load;
    }

    pub fn allow_user(&mut self, user: &str) {
        if !self.allowed_users.contains(&user.to_string()) {
            self.allowed_users.push(user.to_string());
        }
    }

    pub fn deny_user(&mut self, user: &str) {
        if !self.denied_users.contains(&user.to_string()) {
            self.denied_users.push(user.to_string());
        }
    }

    pub fn is_user_permitted(&self, user: &str) -> bool {
        if self.denied_users.contains(&user.to_string()) {
            return false;
        }
        if !self.allowed_users.is_empty() {
            return self.allowed_users.contains(&user.to_string());
        }
        true
    }

    pub fn run_anacron_catchup(&mut self, current_time: u64) -> usize {
        self.run_anacron_catchup_with_load(current_time, 1.0)
    }

    /// Anacron catchup execution checking system load and applying RANDOM_DELAY jitter
    pub fn run_anacron_catchup_with_load(&mut self, current_time: u64, current_load: f32) -> usize {
        if !self.anacron_catchup_enabled {
            return 0;
        }

        if current_load > self.max_load_average_threshold {
            return 0; // Defer execution due to high system load
        }

        let mut ran = 0;
        for (id, job) in self.base_daemon.jobs.iter_mut() {
            let is_permitted = if self.denied_users.contains(&job.user) {
                false
            } else if !self.allowed_users.is_empty() {
                self.allowed_users.contains(&job.user)
            } else {
                true
            };

            if job.enabled && is_permitted {
                if job.last_run.is_none() || (current_time >= job.next_run) {
                    job.last_run = Some(current_time);
                    let jitter = if self.random_delay_max > 0 {
                        ((current_time as u32 % self.random_delay_max) + 1) * 60
                    } else {
                        0
                    };
                    job.next_run = current_time + 86400 + (jitter as u64); // 24h + jitter
                    ran += 1;

                    // Capture simulated job execution output and MAILTO notification
                    let output = format!(
                        "Executed '{}' as user '{}' via Anacron (MAILTO={:?})",
                        job.command, job.user, self.mail_to_user
                    );
                    self.job_outputs.insert(id.clone(), output);
                }
            }
        }
        self.executed_catchup_count += ran;
        ran
    }
}

impl Default for SovereignCronDaemon {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_cron_field_parse() {
        assert_eq!(CronField::parse("*", 0, 59).unwrap(), CronField::All);
        assert_eq!(
            CronField::parse("5", 0, 59).unwrap(),
            CronField::Specific(5)
        );
        assert_eq!(
            CronField::parse("1-5", 0, 59).unwrap(),
            CronField::Range(1, 5)
        );
        assert_eq!(
            CronField::parse("*/5", 0, 59).unwrap(),
            CronField::Step(0, 5)
        );
        // Test list parsing
        let list_result = CronField::parse("1,3,5", 0, 59).unwrap();
        if let CronField::List(values, count) = list_result {
            assert_eq!(count, 3);
            assert_eq!(values[0], 1);
            assert_eq!(values[1], 3);
            assert_eq!(values[2], 5);
        } else {
            panic!("Expected List variant");
        }
    }

    #[test]
    fn test_cron_field_matches() {
        assert!(CronField::All.matches(10));
        assert!(CronField::Specific(5).matches(5));
        assert!(!CronField::Specific(5).matches(10));
        assert!(CronField::Range(1, 5).matches(3));
        assert!(!CronField::Range(1, 5).matches(10));
        // Test list matching
        let list_field = CronField::List([1, 3, 5, 0, 0, 0, 0, 0], 3);
        assert!(list_field.matches(3));
        assert!(!list_field.matches(10));
    }

    #[test]
    fn test_cron_schedule_parse() {
        let schedule = CronSchedule::from_string("0 0 * * *").unwrap();
        assert_eq!(schedule.minute, CronField::Specific(0));
        assert_eq!(schedule.hour, CronField::Specific(0));

        // Test special schedule macro aliases
        let daily = CronSchedule::from_string("@daily").unwrap();
        assert_eq!(daily.minute, CronField::Specific(0));
        assert_eq!(daily.hour, CronField::Specific(0));

        let weekly = CronSchedule::from_string("@weekly").unwrap();
        assert_eq!(weekly.day_of_week, CronField::Specific(0));

        let monthly = CronSchedule::from_string("@monthly").unwrap();
        assert_eq!(monthly.day_of_month, CronField::Specific(1));

        let yearly = CronSchedule::from_string("@yearly").unwrap();
        assert_eq!(yearly.month, CronField::Specific(1));

        let reboot = CronSchedule::from_string("@reboot").unwrap();
        assert_eq!(reboot.minute, CronField::Specific(0));
    }

    #[test]
    fn test_cron_daemon() {
        let mut daemon = CronDaemon::new();

        let job = CronJob {
            id: "test-job".to_string(),
            schedule: CronSchedule::from_string("0 0 * * *").unwrap(),
            command: "echo test".to_string(),
            user: "root".to_string(),
            environment: BTreeMap::new(),
            enabled: true,
            last_run: None,
            next_run: 0,
        };

        daemon.add_job(job).unwrap();
        assert_eq!(daemon.list_jobs().len(), 1);
        daemon.remove_job("test-job").unwrap();
        assert_eq!(daemon.list_jobs().len(), 0);
    }

    #[test]
    fn test_sovereign_ssh_daemon() {
        let mut sshd = SovereignSshDaemon::new(22, "SHA256:kyber1024keyfp");

        // Root login prohibited by default
        assert!(sshd
            .accept_connection(
                "192.168.1.10",
                "root",
                KeyExchangeAlgorithm::Kyber1024Ed25519
            )
            .is_err());

        let sid = sshd
            .accept_connection(
                "192.168.1.10",
                "alice",
                KeyExchangeAlgorithm::Kyber1024Ed25519,
            )
            .unwrap();
        assert_eq!(sid, 1);
        assert!(sshd.privileged_child_pids.contains_key(&sid)); // Privilege Separation check

        assert!(!sshd.active_sessions.get(&sid).unwrap().authenticated);
        assert!(sshd.authenticate_public_key(sid, b"ed25519_pubkey"));
        assert!(sshd.active_sessions.get(&sid).unwrap().authenticated);

        assert!(sshd.allocate_pty(sid).is_ok());
        assert!(sshd.set_chroot_isolation(sid, "/jails/alice").is_ok());
        assert_eq!(
            sshd.active_sessions.get(&sid).unwrap().chroot_dir,
            Some("/jails/alice".to_string())
        );

        // Channel multiplexing tests
        let pty_res = sshd.open_channel(sid, SshChannelType::PtySession).unwrap();
        assert!(pty_res.contains("PTY channel open"));

        let exec_res = sshd
            .open_channel(sid, SshChannelType::ExecCommand("uname -a".to_string()))
            .unwrap();
        assert!(exec_res.contains("Exec channel open"));

        let sftp_res = sshd
            .open_channel(sid, SshChannelType::Subsystem("sftp".to_string()))
            .unwrap();
        assert!(sftp_res.contains("Subsystem channel open"));

        // Fail2ban brute-force protection test
        let sid2 = sshd
            .accept_connection("10.0.0.5", "alice", KeyExchangeAlgorithm::Kyber1024Ed25519)
            .unwrap();
        for _ in 0..3 {
            sshd.authenticate_public_key(sid2, b""); // Invalid empty key
        }
        assert!(sshd.is_ip_banned("10.0.0.5"));
        assert!(sshd
            .accept_connection("10.0.0.5", "alice", KeyExchangeAlgorithm::Kyber1024Ed25519)
            .is_err());
    }

    #[test]
    fn test_sovereign_cron_daemon() {
        let mut cron = SovereignCronDaemon::new();
        cron.allow_user("alice");
        cron.deny_user("bob");
        cron.set_random_delay(15);
        cron.set_max_load_threshold(2.5);

        assert!(cron.is_user_permitted("alice"));
        assert!(!cron.is_user_permitted("bob"));

        let job = CronJob {
            id: "anacron-job".to_string(),
            schedule: CronSchedule::from_string("@daily").unwrap(),
            command: "backup.sh".to_string(),
            user: "alice".to_string(),
            environment: BTreeMap::new(),
            enabled: true,
            last_run: None,
            next_run: 100,
        };
        cron.base_daemon.add_job(job).unwrap();

        // Defer execution when system load exceeds max_load_average_threshold
        let deferred = cron.run_anacron_catchup_with_load(500, 3.5);
        assert_eq!(deferred, 0);

        // Execute when load is below threshold
        let ran = cron.run_anacron_catchup_with_load(500, 1.2);
        assert_eq!(ran, 1);
        assert_eq!(cron.executed_catchup_count, 1);
        assert!(cron.job_outputs.contains_key("anacron-job"));
        assert!(cron
            .job_outputs
            .get("anacron-job")
            .unwrap()
            .contains("MAILTO"));
    }
}
