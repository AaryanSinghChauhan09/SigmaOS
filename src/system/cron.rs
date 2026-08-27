// SigmaOS Cron Daemon - Linux-inspired task scheduler
// Zero-dependency implementation of cron-like functionality

extern crate alloc;

use crate::klib::{BTreeMap, Vec};
use alloc::string::String;

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
    Step(u32, u32),       // */5 or 1-10/2
}

impl CronSchedule {
    /// Parse cron schedule string (e.g., "0 0 * * *")
    pub fn from_string(schedule: &str) -> Result<Self, CronError> {
        let parts: alloc::vec::Vec<&str> = schedule.split_whitespace().collect();
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
    pub fn matches(&self, minute: u32, hour: u32, day_of_month: u32, month: u32, day_of_week: u32) -> bool {
        self.minute.matches(minute) &&
        self.hour.matches(hour) &&
        self.day_of_month.matches(day_of_month) &&
        self.month.matches(month) &&
        self.day_of_week.matches(day_of_week)
    }
}

impl CronField {
    /// Parse a cron field string
    pub fn parse(field: &str, min: u32, max: u32) -> Result<Self, CronError> {
        if field == "*" {
            return Ok(CronField::All);
        }

        if field.contains('/') {
            let parts: alloc::vec::Vec<&str> = field.split('/').collect();
            if parts.len() != 2 {
                return Err(CronError::InvalidField);
            }
            let base = if parts[0] == "*" { min } else { parts[0].parse::<u32>().map_err(|_| CronError::InvalidField)? };
            let step = parts[1].parse::<u32>().map_err(|_| CronError::InvalidField)?;
            return Ok(CronField::Step(base, step));
        }

        if field.contains('-') {
            let parts: alloc::vec::Vec<&str> = field.split('-').collect();
            if parts.len() != 2 {
                return Err(CronError::InvalidField);
            }
            let start = parts[0].parse::<u32>().map_err(|_| CronError::InvalidField)?;
            let end = parts[1].parse::<u32>().map_err(|_| CronError::InvalidField)?;
            return Ok(CronField::Range(start, end));
        }

        if field.contains(',') {
            let parts: alloc::vec::Vec<&str> = field.split(',').collect();
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
            },
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
        self.jobs.remove(id).ok_or(CronError::JobNotFound)?;
        Ok(())
    }

    /// Get a cron job
    pub fn get_job(&self, id: &str) -> Option<&CronJob> {
        self.jobs.get(id)
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
        let job = self.jobs.get_mut(id).ok_or(CronError::JobNotFound)?;
        job.enabled = true;
        Ok(())
    }

    /// Disable a job
    pub fn disable_job(&mut self, id: &str) -> Result<(), CronError> {
        let job = self.jobs.get_mut(id).ok_or(CronError::JobNotFound)?;
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

pub struct SovereignSshDaemon {
    pub port: u16,
    pub host_key_fp: String,
    pub active_sessions: BTreeMap<u64, SshSession>,
    pub max_sessions: usize,
    pub next_session_id: u64,
}

impl SovereignSshDaemon {
    pub fn new(port: u16, host_key_fingerprint: &str) -> Self {
        Self {
            port,
            host_key_fp: host_key_fingerprint.to_string(),
            active_sessions: BTreeMap::new(),
            max_sessions: 64,
            next_session_id: 1,
        }
    }

    pub fn accept_connection(&mut self, remote_ip: &str, user: &str, algo: KeyExchangeAlgorithm) -> Result<u64, CronError> {
        if self.active_sessions.len() >= self.max_sessions {
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
        Ok(sid)
    }

    pub fn authenticate_public_key(&mut self, session_id: u64, pubkey: &[u8]) -> bool {
        if let Some(session) = self.active_sessions.get_mut(&session_id) {
            // Simulated Ed25519/Dilithium signature verification
            if !pubkey.is_empty() {
                session.authenticated = true;
                return true;
            }
        }
        false
    }

    pub fn allocate_pty(&mut self, session_id: u64) -> Result<(), CronError> {
        let session = self.active_sessions.get_mut(&session_id).ok_or(CronError::JobNotFound)?;
        if !session.authenticated {
            return Err(CronError::ExecutionError);
        }
        session.pty_allocated = true;
        Ok(())
    }

    pub fn set_chroot_isolation(&mut self, session_id: u64, dir: &str) -> Result<(), CronError> {
        let session = self.active_sessions.get_mut(&session_id).ok_or(CronError::JobNotFound)?;
        session.chroot_dir = Some(dir.to_string());
        Ok(())
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
}

impl SovereignCronDaemon {
    pub fn new() -> Self {
        Self {
            base_daemon: CronDaemon::new(),
            allowed_users: Vec::new(),
            denied_users: Vec::new(),
            anacron_catchup_enabled: true,
            executed_catchup_count: 0,
        }
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
        if !self.anacron_catchup_enabled {
            return 0;
        }

        let mut ran = 0;
        for job in self.base_daemon.jobs.values_mut() {
            let permitted = if self.denied_users.contains(&job.user) {
                false
            } else if !self.allowed_users.is_empty() {
                self.allowed_users.contains(&job.user)
            } else {
                true
            };
            if job.enabled && permitted {
                if job.last_run.is_none() || (current_time > job.next_run) {
                    job.last_run = Some(current_time);
                    job.next_run = current_time + 86400; // 24h
                    ran += 1;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cron_field_parse() {
        assert_eq!(CronField::parse("*", 0, 59).unwrap(), CronField::All);
        assert_eq!(CronField::parse("5", 0, 59).unwrap(), CronField::Specific(5));
        assert_eq!(CronField::parse("1-5", 0, 59).unwrap(), CronField::Range(1, 5));
        assert_eq!(CronField::parse("*/5", 0, 59).unwrap(), CronField::Step(0, 5));
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
        let sid = sshd.accept_connection("192.168.1.10", "alice", KeyExchangeAlgorithm::Kyber1024Ed25519).unwrap();
        assert_eq!(sid, 1);

        assert!(!sshd.active_sessions.get(&sid).unwrap().authenticated);
        assert!(sshd.authenticate_public_key(sid, b"ed25519_pubkey"));
        assert!(sshd.active_sessions.get(&sid).unwrap().authenticated);

        assert!(sshd.allocate_pty(sid).is_ok());
        assert!(sshd.set_chroot_isolation(sid, "/jails/alice").is_ok());
        assert_eq!(sshd.active_sessions.get(&sid).unwrap().chroot_dir, Some("/jails/alice".to_string()));
    }

    #[test]
    fn test_sovereign_cron_daemon() {
        let mut cron = SovereignCronDaemon::new();
        cron.allow_user("alice");
        cron.deny_user("bob");

        assert!(cron.is_user_permitted("alice"));
        assert!(!cron.is_user_permitted("bob"));

        let job = CronJob {
            id: "anacron-job".to_string(),
            schedule: CronSchedule::from_string("0 0 * * *").unwrap(),
            command: "backup.sh".to_string(),
            user: "alice".to_string(),
            environment: BTreeMap::new(),
            enabled: true,
            last_run: None,
            next_run: 100,
        };
        cron.base_daemon.add_job(job).unwrap();

        let ran = cron.run_anacron_catchup(500);
        assert_eq!(ran, 1);
        assert_eq!(cron.executed_catchup_count, 1);
    }
}