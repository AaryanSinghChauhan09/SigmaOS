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
}