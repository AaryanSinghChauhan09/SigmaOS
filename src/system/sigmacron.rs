// SPDX-License-Identifier: MIT
// Copyright (c) 2026 SigmaOS Contributors

//! SigmaCron - Job Scheduler
//!
//! SigmaCron provides job scheduling for SigmaOS, inspired by cron and systemd-timers.

#![no_std]
extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt;

/// Cron field types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CronField {
    /// Minute (0-59)
    Minute,
    /// Hour (0-23)
    Hour,
    /// Day of month (1-31)
    DayOfMonth,
    /// Month (1-12)
    Month,
    /// Day of week (0-6, 0=Sunday)
    DayOfWeek,
}

impl fmt::Display for CronField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CronField::Minute => write!(f, "minute"),
            CronField::Hour => write!(f, "hour"),
            CronField::DayOfMonth => write!(f, "day_of_month"),
            CronField::Month => write!(f, "month"),
            CronField::DayOfWeek => write!(f, "day_of_week"),
        }
    }
}

/// Cron job status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JobStatus {
    /// Job is enabled
    Enabled,
    /// Job is disabled
    Disabled,
    /// Job is running
    Running,
    /// Job failed
    Failed,
}

impl fmt::Display for JobStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JobStatus::Enabled => write!(f, "enabled"),
            JobStatus::Disabled => write!(f, "disabled"),
            JobStatus::Running => write!(f, "running"),
            JobStatus::Failed => write!(f, "failed"),
        }
    }
}

/// Cron job execution result
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JobResult {
    /// Job succeeded
    Success,
    /// Job failed
    Failure,
    /// Job timed out
    Timeout,
    /// Job was skipped
    Skipped,
}

impl fmt::Display for JobResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JobResult::Success => write!(f, "success"),
            JobResult::Failure => write!(f, "failure"),
            JobResult::Timeout => write!(f, "timeout"),
            JobResult::Skipped => write!(f, "skipped"),
        }
    }
}

/// Cron job
#[derive(Debug, Clone)]
pub struct CronJob {
    /// Job name
    pub name: String,
    /// Command to execute
    pub command: String,
    /// Arguments
    pub args: Vec<String>,
    /// Working directory
    pub working_dir: String,
    /// Environment variables
    pub env: BTreeMap<String, String>,
    /// Minute field (0-59, *)
    pub minute: String,
    /// Hour field (0-23, *)
    pub hour: String,
    /// Day of month field (1-31, *)
    pub day_of_month: String,
    /// Month field (1-12, *)
    pub month: String,
    /// Day of week field (0-6, *)
    pub day_of_week: String,
    /// Job status
    pub status: JobStatus,
    /// Enabled flag
    pub enabled: bool,
    /// Timeout in seconds (0 = no timeout)
    pub timeout: u32,
    /// Run as user
    pub user: String,
    /// Last run timestamp
    pub last_run: Option<u64>,
    /// Next run timestamp
    pub next_run: Option<u64>,
    /// Run count
    pub run_count: u32,
    /// Success count
    pub success_count: u32,
    /// Failure count
    pub failure_count: u32,
}

impl CronJob {
    /// Create a new cron job
    pub fn new(name: String, command: String) -> Self {
        Self {
            name,
            command,
            args: Vec::new(),
            working_dir: "/".to_string(),
            env: BTreeMap::new(),
            minute: "*".to_string(),
            hour: "*".to_string(),
            day_of_month: "*".to_string(),
            month: "*".to_string(),
            day_of_week: "*".to_string(),
            status: JobStatus::Enabled,
            enabled: true,
            timeout: 0,
            user: "root".to_string(),
            last_run: None,
            next_run: None,
            run_count: 0,
            success_count: 0,
            failure_count: 0,
        }
    }

    /// Set minute field
    pub fn with_minute(mut self, minute: String) -> Self {
        self.minute = minute;
        self
    }

    /// Set hour field
    pub fn with_hour(mut self, hour: String) -> Self {
        self.hour = hour;
        self
    }

    /// Set day of month field
    pub fn with_day_of_month(mut self, day: String) -> Self {
        self.day_of_month = day;
        self
    }

    /// Set month field
    pub fn with_month(mut self, month: String) -> Self {
        self.month = month;
        self
    }

    /// Set day of week field
    pub fn with_day_of_week(mut self, day: String) -> Self {
        self.day_of_week = day;
        self
    }

    /// Set working directory
    pub fn with_working_dir(mut self, dir: String) -> Self {
        self.working_dir = dir;
        self
    }

    /// Set user
    pub fn with_user(mut self, user: String) -> Self {
        self.user = user;
        self
    }

    /// Set timeout
    pub fn with_timeout(mut self, timeout: u32) -> Self {
        self.timeout = timeout;
        self
    }

    /// Add argument
    pub fn with_arg(mut self, arg: String) -> Self {
        self.args.push(arg);
        self
    }

    /// Add environment variable
    pub fn with_env(mut self, key: String, value: String) -> Self {
        self.env.insert(key, value);
        self
    }

    /// Enable job
    pub fn enable(&mut self) {
        self.enabled = true;
        self.status = JobStatus::Enabled;
    }

    /// Disable job
    pub fn disable(&mut self) {
        self.enabled = false;
        self.status = JobStatus::Disabled;
    }

    /// Record job run
    pub fn record_run(&mut self, result: JobResult, timestamp: u64) {
        self.last_run = Some(timestamp);
        self.run_count += 1;
        
        match result {
            JobResult::Success => self.success_count += 1,
            JobResult::Failure | JobResult::Timeout => self.failure_count += 1,
            JobResult::Skipped => {}
        }
    }

    /// Get cron expression
    pub fn cron_expression(&self) -> String {
        format!("{} {} {} {} {}",
            self.minute, self.hour, self.day_of_month, self.month, self.day_of_week)
    }
}

/// Cron schedule
#[derive(Debug, Clone)]
pub struct CronSchedule {
    /// Minute field (0-59)
    pub minute: String,
    /// Hour field (0-23)
    pub hour: String,
    /// Day of month field (1-31)
    pub day_of_month: String,
    /// Month field (1-12)
    pub month: String,
    /// Day of week field (0-6)
    pub day_of_week: String,
}

impl CronSchedule {
    /// Create a new cron schedule
    pub fn new(
        minute: String,
        hour: String,
        day_of_month: String,
        month: String,
        day_of_week: String,
    ) -> Self {
        Self {
            minute,
            hour,
            day_of_month,
            month,
            day_of_week,
        }
    }

    /// Create from cron expression
    pub fn from_expression(expr: &str) -> Result<Self, String> {
        let parts: Vec<&str> = expr.split_whitespace().collect();
        if parts.len() != 5 {
            return Err("Invalid cron expression".to_string());
        }
        
        Ok(Self::new(
            parts[0].to_string(),
            parts[1].to_string(),
            parts[2].to_string(),
            parts[3].to_string(),
            parts[4].to_string(),
        ))
    }

    /// Get as expression
    pub fn expression(&self) -> String {
        format!("{} {} {} {} {}",
            self.minute, self.hour, self.day_of_month, self.month, self.day_of_week)
    }
}

/// SigmaCron - Job Scheduler
pub struct SigmaCron {
    /// Cron jobs (name -> job)
    jobs: BTreeMap<String, CronJob>,
    /// Running state
    running: bool,
    /// Current timestamp
    current_time: u64,
}

impl SigmaCron {
    /// Create a new SigmaCron instance
    pub fn new() -> Self {
        Self {
            jobs: BTreeMap::new(),
            running: false,
            current_time: 0,
        }
    }

    /// Start the cron daemon
    pub fn start(&mut self) {
        self.running = true;
    }

    /// Stop the cron daemon
    pub fn stop(&mut self) {
        self.running = false;
    }

    /// Check if the daemon is running
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Add a cron job
    pub fn add_job(&mut self, job: CronJob) -> Result<(), String> {
        if self.jobs.contains_key(&job.name) {
            return Err(format!("Job '{}' already exists", job.name));
        }
        self.jobs.insert(job.name.clone(), job);
        Ok(())
    }

    /// Remove a cron job
    pub fn remove_job(&mut self, name: &str) -> Result<(), String> {
        if self.jobs.remove(name).is_none() {
            return Err(format!("Job '{}' not found", name));
        }
        Ok(())
    }

    /// Get a cron job
    pub fn get_job(&self, name: &str) -> Option<&CronJob> {
        self.jobs.get(name)
    }

    /// Get a cron job (mutable)
    pub fn get_job_mut(&mut self, name: &str) -> Option<&mut CronJob> {
        self.jobs.get_mut(name)
    }

    /// List all jobs
    pub fn list_jobs(&self) -> Vec<&CronJob> {
        self.jobs.values().collect()
    }

    /// List enabled jobs
    pub fn list_enabled_jobs(&self) -> Vec<&CronJob> {
        self.jobs.values().filter(|j| j.enabled).collect()
    }

    /// Enable a job
    pub fn enable_job(&mut self, name: &str) -> Result<(), String> {
        match self.jobs.get_mut(name) {
            Some(job) => {
                job.enable();
                Ok(())
            }
            None => Err(format!("Job '{}' not found", name)),
        }
    }

    /// Disable a job
    pub fn disable_job(&mut self, name: &str) -> Result<(), String> {
        match self.jobs.get_mut(name) {
            Some(job) => {
                job.disable();
                Ok(())
            }
            None => Err(format!("Job '{}' not found", name)),
        }
    }

    /// Set current time
    pub fn set_time(&mut self, time: u64) {
        self.current_time = time;
    }

    /// Get current time
    pub fn current_time(&self) -> u64 {
        self.current_time
    }

    /// Get job count
    pub fn job_count(&self) -> usize {
        self.jobs.len()
    }

    /// Get enabled job count
    pub fn enabled_job_count(&self) -> usize {
        self.jobs.values().filter(|j| j.enabled).count()
    }

    /// Get job statistics
    pub fn job_stats(&self) -> BTreeMap<String, String> {
        let mut stats = BTreeMap::new();
        stats.insert("total_jobs".to_string(), self.job_count().to_string());
        stats.insert("enabled_jobs".to_string(), self.enabled_job_count().to_string());
        stats.insert("disabled_jobs".to_string(), (self.job_count() - self.enabled_job_count()).to_string());
        stats
    }
}

impl Default for SigmaCron {
    fn default() -> Self {
        Self::new()
    }
}

/// Global SigmaCron instance (placeholder for actual global state)
static mut SIGMACRON_INSTANCE: Option<SigmaCron> = None;

/// Initialize the global SigmaCron instance
pub fn init_sigmacron() {
    unsafe {
        SIGMACRON_INSTANCE = Some(SigmaCron::new());
    }
}

/// Get the global SigmaCron instance
pub fn get_sigmacron() -> Option<&'static mut SigmaCron> {
    unsafe { SIGMACRON_INSTANCE.as_mut() }
}

/// Convenience function to add a cron job
pub fn add_cron_job(job: CronJob) -> Result<(), String> {
    if let Some(cron) = get_sigmacron() {
        cron.add_job(job)
    } else {
        Err("SigmaCron not initialized".to_string())
    }
}

/// Convenience function to enable a job
pub fn enable_cron_job(name: &str) -> Result<(), String> {
    if let Some(cron) = get_sigmacron() {
        cron.enable_job(name)
    } else {
        Err("SigmaCron not initialized".to_string())
    }
}

/// Convenience function to disable a job
pub fn disable_cron_job(name: &str) -> Result<(), String> {
    if let Some(cron) = get_sigmacron() {
        cron.disable_job(name)
    } else {
        Err("SigmaCron not initialized".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cron_field_display() {
        assert_eq!(CronField::Minute.to_string(), "minute");
        assert_eq!(CronField::Hour.to_string(), "hour");
    }

    #[test]
    fn test_job_status_display() {
        assert_eq!(JobStatus::Enabled.to_string(), "enabled");
        assert_eq!(JobStatus::Running.to_string(), "running");
    }

    #[test]
    fn test_job_result_display() {
        assert_eq!(JobResult::Success.to_string(), "success");
        assert_eq!(JobResult::Failure.to_string(), "failure");
    }

    #[test]
    fn test_cron_job() {
        let job = CronJob::new("test-job".to_string(), "/bin/echo".to_string())
            .with_minute("0".to_string())
            .with_hour("*/6".to_string())
            .with_day_of_month("*".to_string())
            .with_month("*".to_string())
            .with_day_of_week("*".to_string())
            .with_user("root".to_string())
            .with_timeout(300);

        assert_eq!(job.name, "test-job");
        assert_eq!(job.command, "/bin/echo");
        assert_eq!(job.minute, "0");
        assert_eq!(job.hour, "*/6");
        assert_eq!(job.user, "root");
        assert_eq!(job.timeout, 300);
    }

    #[test]
    fn test_cron_job_enable_disable() {
        let mut job = CronJob::new("test-job".to_string(), "/bin/echo".to_string());
        
        assert!(job.enabled);
        job.disable();
        assert!(!job.enabled);
        job.enable();
        assert!(job.enabled);
    }

    #[test]
    fn test_cron_job_record_run() {
        let mut job = CronJob::new("test-job".to_string(), "/bin/echo".to_string());
        
        job.record_run(JobResult::Success, 1000);
        assert_eq!(job.run_count, 1);
        assert_eq!(job.success_count, 1);
        assert_eq!(job.failure_count, 0);
        assert_eq!(job.last_run, Some(1000));
        
        job.record_run(JobResult::Failure, 2000);
        assert_eq!(job.run_count, 2);
        assert_eq!(job.success_count, 1);
        assert_eq!(job.failure_count, 1);
    }

    #[test]
    fn test_cron_schedule() {
        let schedule = CronSchedule::new(
            "0".to_string(),
            "*/6".to_string(),
            "*".to_string(),
            "*".to_string(),
            "*".to_string(),
        );
        
        assert_eq!(schedule.minute, "0");
        assert_eq!(schedule.hour, "*/6");
        assert_eq!(schedule.expression(), "0 */6 * * *");
    }

    #[test]
    fn test_cron_schedule_from_expression() {
        let schedule = CronSchedule::from_expression("0 */6 * * *").unwrap();
        
        assert_eq!(schedule.minute, "0");
        assert_eq!(schedule.hour, "*/6");
    }

    #[test]
    fn test_sigmacron() {
        let mut cron = SigmaCron::new();
        assert!(!cron.is_running());
        cron.start();
        assert!(cron.is_running());
        cron.stop();
        assert!(!cron.is_running());
    }

    #[test]
    fn test_sigmacron_add_job() {
        let mut cron = SigmaCron::new();
        
        let job = CronJob::new("test-job".to_string(), "/bin/echo".to_string());
        assert!(cron.add_job(job).is_ok());
        assert_eq!(cron.job_count(), 1);
    }

    #[test]
    fn test_sigmacron_duplicate_job() {
        let mut cron = SigmaCron::new();
        
        let job1 = CronJob::new("test-job".to_string(), "/bin/echo".to_string());
        let job2 = CronJob::new("test-job".to_string(), "/bin/ls".to_string());
        
        assert!(cron.add_job(job1).is_ok());
        assert!(cron.add_job(job2).is_err());
    }

    #[test]
    fn test_sigmacron_enable_disable() {
        let mut cron = SigmaCron::new();
        
        let job = CronJob::new("test-job".to_string(), "/bin/echo".to_string());
        cron.add_job(job).unwrap();
        
        assert!(cron.enable_job("test-job").is_ok());
        assert!(cron.disable_job("test-job").is_ok());
    }

    #[test]
    fn test_sigmacron_list_jobs() {
        let mut cron = SigmaCron::new();
        
        let job1 = CronJob::new("job1".to_string(), "/bin/echo".to_string());
        let job2 = CronJob::new("job2".to_string(), "/bin/ls".to_string());
        
        cron.add_job(job1).unwrap();
        cron.add_job(job2).unwrap();
        
        let jobs = cron.list_jobs();
        assert_eq!(jobs.len(), 2);
    }

    #[test]
    fn test_sigmacron_enabled_jobs() {
        let mut cron = SigmaCron::new();
        
        let mut job1 = CronJob::new("job1".to_string(), "/bin/echo".to_string());
        job1.disable();
        
        let job2 = CronJob::new("job2".to_string(), "/bin/ls".to_string());
        
        cron.add_job(job1).unwrap();
        cron.add_job(job2).unwrap();
        
        let enabled = cron.list_enabled_jobs();
        assert_eq!(enabled.len(), 1);
    }

    #[test]
    fn test_sigmacron_stats() {
        let mut cron = SigmaCron::new();
        
        let mut job1 = CronJob::new("job1".to_string(), "/bin/echo".to_string());
        job1.disable();
        
        let job2 = CronJob::new("job2".to_string(), "/bin/ls".to_string());
        
        cron.add_job(job1).unwrap();
        cron.add_job(job2).unwrap();
        
        let stats = cron.job_stats();
        assert_eq!(stats.get("total_jobs"), Some(&"2".to_string()));
        assert_eq!(stats.get("enabled_jobs"), Some(&"1".to_string()));
        assert_eq!(stats.get("disabled_jobs"), Some(&"1".to_string()));
    }

    #[test]
    fn test_add_cron_job_convenience() {
        init_sigmacron();
        
        let job = CronJob::new("test-job".to_string(), "/bin/echo".to_string());
        let result = add_cron_job(job);
        
        assert!(result.is_ok());
    }

    #[test]
    fn test_enable_cron_job_convenience() {
        init_sigmacron();
        
        let job = CronJob::new("test-job".to_string(), "/bin/echo".to_string());
        add_cron_job(job).unwrap();
        
        let result = enable_cron_job("test-job");
        assert!(result.is_ok());
    }

    #[test]
    fn test_disable_cron_job_convenience() {
        init_sigmacron();
        
        let job = CronJob::new("test-job".to_string(), "/bin/echo".to_string());
        add_cron_job(job).unwrap();
        
        let result = disable_cron_job("test-job");
        assert!(result.is_ok());
    }
}
