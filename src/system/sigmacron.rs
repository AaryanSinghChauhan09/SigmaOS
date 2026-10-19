//! SigmaCron - Job Scheduling System
//!
//! Provides comprehensive job scheduling inspired by Linux cron and systemd timers.
//! Supports time-based scheduling, recurring jobs, and job execution management.

#![no_std]

extern crate alloc;

use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};

/// Job execution status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JobStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Skipped,
    Unknown,
}

/// Job schedule type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScheduleType {
    Once,
    minutely,
    Hourly,
    Daily,
    Weekly,
    Monthly,
    Yearly,
    CronExpression,
    Unknown,
}

/// Cron time specification
#[derive(Debug, Clone)]
pub struct CronSpec {
    pub minute: Option<String>,
    pub hour: Option<String>,
    pub day_of_month: Option<String>,
    pub month: Option<String>,
    pub day_of_week: Option<String>,
}

impl CronSpec {
    pub fn new() -> Self {
        Self {
            minute: None,
            hour: None,
            day_of_month: None,
            month: None,
            day_of_week: None,
        }
    }

    pub fn with_minute(mut self, minute: String) -> Self {
        self.minute = Some(minute);
        self
    }

    pub fn with_hour(mut self, hour: String) -> Self {
        self.hour = Some(hour);
        self
    }

    pub fn with_day_of_month(mut self, day: String) -> Self {
        self.day_of_month = Some(day);
        self
    }

    pub fn with_month(mut self, month: String) -> Self {
        self.month = Some(month);
        self
    }

    pub fn with_day_of_week(mut self, day: String) -> Self {
        self.day_of_week = Some(day);
        self
    }
}

impl Default for CronSpec {
    fn default() -> Self {
        Self::new()
    }
}

/// Scheduled job
#[derive(Debug, Clone)]
pub struct ScheduledJob {
    pub id: String,
    pub name: String,
    pub command: String,
    pub schedule_type: ScheduleType,
    pub cron_spec: Option<CronSpec>,
    pub status: JobStatus,
    pub enabled: bool,
    pub last_run: Option<u64>,
    pub next_run: Option<u64>,
    pub run_count: u32,
    pub fail_count: u32,
    pub environment: Vec<String>,
    pub working_directory: Option<String>,
}

impl ScheduledJob {
    pub fn new(id: String, name: String, command: String, schedule_type: ScheduleType) -> Self {
        Self {
            id,
            name,
            command,
            schedule_type,
            cron_spec: None,
            status: JobStatus::Pending,
            enabled: true,
            last_run: None,
            next_run: None,
            run_count: 0,
            fail_count: 0,
            environment: Vec::new(),
            working_directory: None,
        }
    }

    pub fn with_cron_spec(mut self, spec: CronSpec) -> Self {
        self.cron_spec = Some(spec);
        self
    }

    pub fn with_environment(mut self, env: Vec<String>) -> Self {
        self.environment = env;
        self
    }

    pub fn with_working_directory(mut self, dir: String) -> Self {
        self.working_directory = Some(dir);
        self
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn mark_running(&mut self) {
        self.status = JobStatus::Running;
    }

    pub fn mark_completed(&mut self) {
        self.status = JobStatus::Completed;
        self.run_count += 1;
    }

    pub fn mark_failed(&mut self) {
        self.status = JobStatus::Failed;
        self.fail_count += 1;
    }

    pub fn mark_skipped(&mut self) {
        self.status = JobStatus::Skipped;
    }
}

/// Cron manager
pub struct SigmaCron {
    jobs: BTreeMap<String, ScheduledJob>,
    next_job_id: u64,
}

impl SigmaCron {
    pub fn new() -> Self {
        Self {
            jobs: BTreeMap::new(),
            next_job_id: 1,
        }
    }

    pub fn create_job(
        &mut self,
        name: String,
        command: String,
        schedule_type: ScheduleType,
    ) -> String {
        let id = format!("job-{}", self.next_job_id);
        self.next_job_id += 1;
        let job = ScheduledJob::new(id.clone(), name, command, schedule_type);
        self.jobs.insert(id.clone(), job);
        id
    }

    pub fn get_job(&self, id: &str) -> Option<&ScheduledJob> {
        self.jobs.get(id)
    }

    pub fn get_job_mut(&mut self, id: &str) -> Option<&mut ScheduledJob> {
        self.jobs.get_mut(id)
    }

    pub fn list_jobs(&self) -> Vec<&ScheduledJob> {
        self.jobs.values().collect()
    }

    pub fn list_jobs_by_status(&self, status: JobStatus) -> Vec<&ScheduledJob> {
        self.jobs.values().filter(|j| j.status == status).collect()
    }

    pub fn list_enabled_jobs(&self) -> Vec<&ScheduledJob> {
        self.jobs.values().filter(|j| j.enabled).collect()
    }

    pub fn list_disabled_jobs(&self) -> Vec<&ScheduledJob> {
        self.jobs.values().filter(|j| !j.enabled).collect()
    }

    pub fn remove_job(&mut self, id: &str) -> bool {
        self.jobs.remove(id).is_some()
    }

    pub fn enable_job(&mut self, id: &str) -> bool {
        if let Some(job) = self.jobs.get_mut(id) {
            job.enable();
            true
        } else {
            false
        }
    }

    pub fn disable_job(&mut self, id: &str) -> bool {
        if let Some(job) = self.jobs.get_mut(id) {
            job.disable();
            true
        } else {
            false
        }
    }

    pub fn run_job(&mut self, id: &str) -> bool {
        if let Some(job) = self.jobs.get_mut(id) {
            if job.enabled {
                job.mark_running();
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn complete_job(&mut self, id: &str) -> bool {
        if let Some(job) = self.jobs.get_mut(id) {
            job.mark_completed();
            true
        } else {
            false
        }
    }

    pub fn fail_job(&mut self, id: &str) -> bool {
        if let Some(job) = self.jobs.get_mut(id) {
            job.mark_failed();
            true
        } else {
            false
        }
    }

    pub fn skip_job(&mut self, id: &str) -> bool {
        if let Some(job) = self.jobs.get_mut(id) {
            job.mark_skipped();
            true
        } else {
            false
        }
    }

    pub fn set_cron_spec(&mut self, id: &str, spec: CronSpec) -> bool {
        if let Some(job) = self.jobs.get_mut(id) {
            job.cron_spec = Some(spec);
            true
        } else {
            false
        }
    }

    pub fn job_count(&self) -> usize {
        self.jobs.len()
    }

    pub fn enabled_job_count(&self) -> usize {
        self.jobs.values().filter(|j| j.enabled).count()
    }

    pub fn running_job_count(&self) -> usize {
        self.jobs
            .values()
            .filter(|j| j.status == JobStatus::Running)
            .count()
    }

    pub fn failed_job_count(&self) -> usize {
        self.jobs
            .values()
            .filter(|j| j.status == JobStatus::Failed)
            .count()
    }
}

impl Default for SigmaCron {
    fn default() -> Self {
        Self::new()
    }
}

static mut SIGMACRON_INSTANCE: Option<SigmaCron> = None;

pub fn init_sigmacron() {
    unsafe {
        SIGMACRON_INSTANCE = Some(SigmaCron::new());
    }
}

pub fn get_sigmacron() -> &'static SigmaCron {
    unsafe {
        SIGMACRON_INSTANCE
            .as_ref()
            .expect("SigmaCron not initialized")
    }
}

pub fn get_sigmacron_mut() -> &'static mut SigmaCron {
    unsafe {
        SIGMACRON_INSTANCE
            .as_mut()
            .expect("SigmaCron not initialized")
    }
}

pub fn create_job(name: String, command: String, schedule_type: ScheduleType) -> String {
    let manager = get_sigmacron_mut();
    manager.create_job(name, command, schedule_type)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cron_spec_creation() {
        let spec = CronSpec::new()
            .with_minute("*".to_string())
            .with_hour("*".to_string())
            .with_day_of_month("*".to_string())
            .with_month("*".to_string())
            .with_day_of_week("*".to_string());

        assert_eq!(spec.minute, Some("*".to_string()));
        assert_eq!(spec.hour, Some("*".to_string()));
    }

    #[test]
    fn test_scheduled_job_creation() {
        let job = ScheduledJob::new(
            "job-1".to_string(),
            "backup".to_string(),
            "/usr/bin/backup".to_string(),
            ScheduleType::Daily,
        );
        assert_eq!(job.id, "job-1");
        assert_eq!(job.name, "backup");
        assert_eq!(job.command, "/usr/bin/backup");
        assert_eq!(job.schedule_type, ScheduleType::Daily);
        assert_eq!(job.status, JobStatus::Pending);
        assert!(job.enabled);
    }

    #[test]
    fn test_scheduled_job_builder() {
        let cron_spec = CronSpec::new().with_minute("0".to_string()).with_hour("2".to_string());
        let job = ScheduledJob::new(
            "job-1".to_string(),
            "backup".to_string(),
            "/usr/bin/backup".to_string(),
            ScheduleType::CronExpression,
        )
        .with_cron_spec(cron_spec)
        .with_environment(vec!["PATH=/usr/bin".to_string()])
        .with_working_directory("/home/user".to_string());

        assert!(job.cron_spec.is_some());
        assert_eq!(job.environment.len(), 1);
        assert_eq!(job.working_directory, Some("/home/user".to_string()));
    }

    #[test]
    fn test_job_state_transitions() {
        let mut job = ScheduledJob::new(
            "job-1".to_string(),
            "test".to_string(),
            "/bin/test".to_string(),
            ScheduleType::Daily,
        );
        assert_eq!(job.status, JobStatus::Pending);

        job.mark_running();
        assert_eq!(job.status, JobStatus::Running);

        job.mark_completed();
        assert_eq!(job.status, JobStatus::Completed);
        assert_eq!(job.run_count, 1);

        job.mark_running();
        job.mark_failed();
        assert_eq!(job.status, JobStatus::Failed);
        assert_eq!(job.fail_count, 1);

        job.mark_skipped();
        assert_eq!(job.status, JobStatus::Skipped);
    }

    #[test]
    fn test_enable_disable_job() {
        let mut job = ScheduledJob::new(
            "job-1".to_string(),
            "test".to_string(),
            "/bin/test".to_string(),
            ScheduleType::Daily,
        );
        assert!(job.enabled);

        job.disable();
        assert!(!job.enabled);

        job.enable();
        assert!(job.enabled);
    }

    #[test]
    fn test_cron_manager() {
        let manager = SigmaCron::new();
        assert_eq!(manager.job_count(), 0);
    }

    #[test]
    fn test_create_job() {
        let mut manager = SigmaCron::new();
        let id = manager.create_job(
            "backup".to_string(),
            "/usr/bin/backup".to_string(),
            ScheduleType::Daily,
        );
        assert_eq!(id, "job-1");
        assert_eq!(manager.job_count(), 1);
    }

    #[test]
    fn test_enable_disable_job_manager() {
        let mut manager = SigmaCron::new();
        let id = manager.create_job(
            "backup".to_string(),
            "/usr/bin/backup".to_string(),
            ScheduleType::Daily,
        );

        assert!(manager.disable_job(&id));
        assert!(!manager.get_job(&id).unwrap().enabled);

        assert!(manager.enable_job(&id));
        assert!(manager.get_job(&id).unwrap().enabled);
    }

    #[test]
    fn test_run_complete_fail_job() {
        let mut manager = SigmaCron::new();
        let id = manager.create_job(
            "backup".to_string(),
            "/usr/bin/backup".to_string(),
            ScheduleType::Daily,
        );

        assert!(manager.run_job(&id));
        assert_eq!(manager.get_job(&id).unwrap().status, JobStatus::Running);

        assert!(manager.complete_job(&id));
        assert_eq!(manager.get_job(&id).unwrap().status, JobStatus::Completed);
        assert_eq!(manager.get_job(&id).unwrap().run_count, 1);

        manager.run_job(&id);
        assert!(manager.fail_job(&id));
        assert_eq!(manager.get_job(&id).unwrap().status, JobStatus::Failed);
        assert_eq!(manager.get_job(&id).unwrap().fail_count, 1);
    }

    #[test]
    fn test_list_jobs_by_status() {
        let mut manager = SigmaCron::new();
        let id1 = manager.create_job("job1".to_string(), "/bin/test1".to_string(), ScheduleType::Daily);
        let id2 = manager.create_job("job2".to_string(), "/bin/test2".to_string(), ScheduleType::Hourly);
        manager.run_job(&id1);
        manager.run_job(&id2);
        manager.complete_job(&id1);

        let completed = manager.list_jobs_by_status(JobStatus::Completed);
        assert_eq!(completed.len(), 1);

        let running = manager.list_jobs_by_status(JobStatus::Running);
        assert_eq!(running.len(), 1);
    }

    #[test]
    fn test_list_enabled_disabled_jobs() {
        let mut manager = SigmaCron::new();
        let id1 = manager.create_job("job1".to_string(), "/bin/test1".to_string(), ScheduleType::Daily);
        let id2 = manager.create_job("job2".to_string(), "/bin/test2".to_string(), ScheduleType::Hourly);
        manager.disable_job(&id2);

        let enabled = manager.list_enabled_jobs();
        assert_eq!(enabled.len(), 1);

        let disabled = manager.list_disabled_jobs();
        assert_eq!(disabled.len(), 1);
    }

    #[test]
    fn test_set_cron_spec() {
        let mut manager = SigmaCron::new();
        let id = manager.create_job(
            "backup".to_string(),
            "/usr/bin/backup".to_string(),
            ScheduleType::CronExpression,
        );

        let spec = CronSpec::new().with_minute("0".to_string()).with_hour("2".to_string());
        assert!(manager.set_cron_spec(&id, spec));
        assert!(manager.get_job(&id).unwrap().cron_spec.is_some());
    }

    #[test]
    fn test_job_counts() {
        let mut manager = SigmaCron::new();
        let id1 = manager.create_job("job1".to_string(), "/bin/test1".to_string(), ScheduleType::Daily);
        let id2 = manager.create_job("job2".to_string(), "/bin/test2".to_string(), ScheduleType::Hourly);
        let id3 = manager.create_job("job3".to_string(), "/bin/test3".to_string(), ScheduleType::Weekly);
        manager.disable_job(&id3);
        manager.run_job(&id1);
        manager.run_job(&id2);
        manager.fail_job(&id2);

        assert_eq!(manager.job_count(), 3);
        assert_eq!(manager.enabled_job_count(), 2);
        assert_eq!(manager.running_job_count(), 1);
        assert_eq!(manager.failed_job_count(), 1);
    }

    #[test]
    fn test_global_instance() {
        init_sigmacron();
        let manager = get_sigmacron();
        assert_eq!(manager.job_count(), 0);

        let manager_mut = get_sigmacron_mut();
        manager_mut.create_job("test".to_string(), "/bin/test".to_string(), ScheduleType::Daily);
        assert_eq!(manager.job_count(), 1);
    }

    #[test]
    fn test_create_job_convenience() {
        init_sigmacron();
        let id = create_job("test".to_string(), "/bin/test".to_string(), ScheduleType::Daily);
        assert_eq!(id, "job-1");

        let manager = get_sigmacron();
        assert!(manager.get_job(&id).is_some());
    }
}
