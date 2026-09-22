// Linux-inspired Cron Scheduler
// Provides periodic task scheduling with cron-like syntax

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Cron field (minute, hour, day of month, month, day of week)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CronField {
    Specific(u32),
    Range(u32, u32),
    List(Vec<u32>),
    All,       // *
    Step(u32), // */n
}

impl CronField {
    /// Parse a cron field string
    pub fn parse(s: &str, min: u32, max: u32) -> Result<Self, String> {
        if s == "*" {
            return Ok(CronField::All);
        }

        if s.starts_with("*/") {
            let step: u32 = s[2..].parse()
                .map_err(|_| format!("Invalid step value: {}", s))?;
            if step == 0 {
                return Err("Step value cannot be zero".to_string());
            }
            return Ok(CronField::Step(step));
        }

        if s.contains('-') {
            let parts: Vec<&str> = s.split('-').collect();
            if parts.len() != 2 {
                return Err(format!("Invalid range: {}", s));
            }
            let start: u32 = parts[0].parse()
                .map_err(|_| format!("Invalid range start: {}", parts[0]))?;
            let end: u32 = parts[1].parse()
                .map_err(|_| format!("Invalid range end: {}", parts[1]))?;
            if start < min || end > max || start > end {
                return Err(format!("Range out of bounds: {}", s));
            }
            return Ok(CronField::Range(start, end));
        }

        if s.contains(',') {
            let values: Result<Vec<u32>, _> = s.split(',')
                .map(|v| v.parse::<u32>())
                .collect();
            let values = values.map_err(|_| format!("Invalid list: {}", s))?;
            for &v in &values {
                if v < min || v > max {
                    return Err(format!("List value out of bounds: {}", v));
                }
            }
            return Ok(CronField::List(values));
        }

        let value: u32 = s.parse()
            .map_err(|_| format!("Invalid field value: {}", s))?;
        if value < min || value > max {
            return Err(format!("Value out of bounds: {}", value));
        }
        Ok(CronField::Specific(value))
    }

    /// Check if a value matches this field
    pub fn matches(&self, value: u32) -> bool {
        match self {
            CronField::Specific(v) => *v == value,
            CronField::Range(start, end) => value >= *start && value <= *end,
            CronField::List(values) => values.contains(&value),
            CronField::All => true,
            CronField::Step(step) => value % step == 0,
        }
    }
}

/// Cron schedule definition
#[derive(Debug, Clone)]
pub struct CronSchedule {
    pub minute: CronField,
    pub hour: CronField,
    pub day_of_month: CronField,
    pub month: CronField,
    pub day_of_week: CronField,
}

impl CronSchedule {
    /// Parse a cron schedule string (5 fields: minute hour dom month dow)
    pub fn parse(s: &str) -> Result<Self, String> {
        let fields: Vec<&str> = s.split_whitespace().collect();
        if fields.len() != 5 {
            return Err(format!("Expected 5 fields, got {}", fields.len()));
        }

        let minute = CronField::parse(fields[0], 0, 59)?;
        let hour = CronField::parse(fields[1], 0, 23)?;
        let day_of_month = CronField::parse(fields[2], 1, 31)?;
        let month = CronField::parse(fields[3], 1, 12)?;
        let day_of_week = CronField::parse(fields[4], 0, 6)?;

        Ok(Self {
            minute,
            hour,
            day_of_month,
            month,
            day_of_week,
        })
    }

    /// Check if a given time matches this schedule
    pub fn matches(&self, minute: u32, hour: u32, day_of_month: u32, month: u32, day_of_week: u32) -> bool {
        self.minute.matches(minute)
            && self.hour.matches(hour)
            && self.day_of_month.matches(day_of_month)
            && self.month.matches(month)
            && self.day_of_week.matches(day_of_week)
    }
}

/// Cron job
#[derive(Debug, Clone)]
pub struct CronJob {
    pub id: u64,
    pub schedule: CronSchedule,
    pub command: String,
    pub enabled: bool,
}

impl CronJob {
    pub fn new(id: u64, schedule: CronSchedule, command: String) -> Self {
        Self {
            id,
            schedule,
            command,
            enabled: true,
        }
    }

    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Check if this job should run at the given time
    pub fn should_run(&self, minute: u32, hour: u32, day_of_month: u32, month: u32, day_of_week: u32) -> bool {
        self.enabled && self.schedule.matches(minute, hour, day_of_month, month, day_of_week)
    }
}

/// Cron manager for system-wide cron job management
pub struct CronManager {
    jobs: Arc<Mutex<HashMap<u64, CronJob>>>,
    next_job_id: Arc<Mutex<u64>>,
}

impl CronManager {
    pub fn new() -> Self {
        Self {
            jobs: Arc::new(Mutex::new(HashMap::new())),
            next_job_id: Arc::new(Mutex::new(1)),
        }
    }

    /// Create a new cron job
    pub fn create_job(&self, schedule: CronSchedule, command: String) -> u64 {
        let mut next_id = self.next_job_id.lock().unwrap();
        let job_id = *next_id;
        *next_id += 1;
        drop(next_id);

        let job = CronJob::new(job_id, schedule, command);
        let mut jobs = self.jobs.lock().unwrap();
        jobs.insert(job_id, job);

        job_id
    }

    /// Get a job by ID
    pub fn get_job(&self, job_id: u64) -> Option<CronJob> {
        let jobs = self.jobs.lock().unwrap();
        jobs.get(&job_id).cloned()
    }

    /// Remove a job
    pub fn remove_job(&self, job_id: u64) -> Result<(), String> {
        let mut jobs = self.jobs.lock().unwrap();
        match jobs.remove(&job_id) {
            Some(_) => Ok(()),
            None => Err(format!("Job {} not found", job_id)),
        }
    }

    /// Enable a job
    pub fn enable_job(&self, job_id: u64) -> Result<(), String> {
        let mut jobs = self.jobs.lock().unwrap();
        match jobs.get_mut(&job_id) {
            Some(job) => {
                job.enabled = true;
                Ok(())
            }
            None => Err(format!("Job {} not found", job_id)),
        }
    }

    /// Disable a job
    pub fn disable_job(&self, job_id: u64) -> Result<(), String> {
        let mut jobs = self.jobs.lock().unwrap();
        match jobs.get_mut(&job_id) {
            Some(job) => {
                job.enabled = false;
                Ok(())
            }
            None => Err(format!("Job {} not found", job_id)),
        }
    }

    /// Get jobs that should run at the given time
    pub fn get_jobs_to_run(&self, minute: u32, hour: u32, day_of_month: u32, month: u32, day_of_week: u32) -> Vec<CronJob> {
        let jobs = self.jobs.lock().unwrap();
        jobs.values()
            .filter(|job| job.should_run(minute, hour, day_of_month, month, day_of_week))
            .cloned()
            .collect()
    }

    /// Get all jobs
    pub fn get_all_jobs(&self) -> Vec<CronJob> {
        let jobs = self.jobs.lock().unwrap();
        jobs.values().cloned().collect()
    }

    /// Get job count
    pub fn job_count(&self) -> usize {
        let jobs = self.jobs.lock().unwrap();
        jobs.len()
    }
}

impl Default for CronManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cron_field_specific() {
        let field = CronField::parse("5", 0, 59).unwrap();
        assert!(field.matches(5));
        assert!(!field.matches(6));
    }

    #[test]
    fn test_cron_field_range() {
        let field = CronField::parse("5-10", 0, 59).unwrap();
        assert!(field.matches(5));
        assert!(field.matches(7));
        assert!(field.matches(10));
        assert!(!field.matches(4));
        assert!(!field.matches(11));
    }

    #[test]
    fn test_cron_field_list() {
        let field = CronField::parse("1,3,5", 0, 59).unwrap();
        assert!(field.matches(1));
        assert!(field.matches(3));
        assert!(field.matches(5));
        assert!(!field.matches(2));
    }

    #[test]
    fn test_cron_field_all() {
        let field = CronField::parse("*", 0, 59).unwrap();
        assert!(field.matches(0));
        assert!(field.matches(30));
        assert!(field.matches(59));
    }

    #[test]
    fn test_cron_field_step() {
        let field = CronField::parse("*/5", 0, 59).unwrap();
        assert!(field.matches(0));
        assert!(field.matches(5));
        assert!(field.matches(10));
        assert!(!field.matches(3));
    }

    #[test]
    fn test_cron_field_invalid() {
        assert!(CronField::parse("70", 0, 59).is_err());
        assert!(CronField::parse("*/0", 0, 59).is_err());
        assert!(CronField::parse("10-5", 0, 59).is_err());
    }

    #[test]
    fn test_cron_schedule_parse() {
        let schedule = CronSchedule::parse("0 * * * *").unwrap();
        assert!(matches!(schedule.minute, CronField::Specific(0)));
        assert!(matches!(schedule.hour, CronField::All));
    }

    #[test]
    fn test_cron_schedule_matches() {
        let schedule = CronSchedule::parse("30 12 * * *").unwrap();
        assert!(schedule.matches(30, 12, 1, 1, 0));
        assert!(!schedule.matches(29, 12, 1, 1, 0));
        assert!(!schedule.matches(30, 11, 1, 1, 0));
    }

    #[test]
    fn test_cron_job() {
        let schedule = CronSchedule::parse("0 * * * *").unwrap();
        let job = CronJob::new(1, schedule, "echo test".to_string());

        assert!(job.should_run(0, 12, 1, 1, 0));
        assert!(!job.should_run(1, 12, 1, 1, 0));
    }

    #[test]
    fn test_cron_job_disabled() {
        let schedule = CronSchedule::parse("0 * * * *").unwrap();
        let job = CronJob::new(1, schedule, "echo test".to_string())
            .with_enabled(false);

        assert!(!job.should_run(0, 12, 1, 1, 0));
    }

    #[test]
    fn test_cron_manager() {
        let manager = CronManager::new();

        let schedule = CronSchedule::parse("0 * * * *").unwrap();
        let job_id = manager.create_job(schedule, "echo test".to_string());

        assert_eq!(job_id, 1);
        assert_eq!(manager.job_count(), 1);

        let job = manager.get_job(job_id).unwrap();
        assert_eq!(job.command, "echo test");
    }

    #[test]
    fn test_cron_manager_enable_disable() {
        let manager = CronManager::new();

        let schedule = CronSchedule::parse("0 * * * *").unwrap();
        let job_id = manager.create_job(schedule, "echo test".to_string());

        manager.disable_job(job_id).unwrap();
        let job = manager.get_job(job_id).unwrap();
        assert!(!job.enabled);

        manager.enable_job(job_id).unwrap();
        let job = manager.get_job(job_id).unwrap();
        assert!(job.enabled);
    }

    #[test]
    fn test_cron_manager_get_jobs_to_run() {
        let manager = CronManager::new();

        let schedule1 = CronSchedule::parse("0 * * * *").unwrap();
        let schedule2 = CronSchedule::parse("30 * * * *").unwrap();

        manager.create_job(schedule1, "job1".to_string());
        manager.create_job(schedule2, "job2".to_string());

        let jobs = manager.get_jobs_to_run(0, 12, 1, 1, 0);
        assert_eq!(jobs.len(), 1);
        assert_eq!(jobs[0].command, "job1");

        let jobs = manager.get_jobs_to_run(30, 12, 1, 1, 0);
        assert_eq!(jobs.len(), 1);
        assert_eq!(jobs[0].command, "job2");
    }
}
