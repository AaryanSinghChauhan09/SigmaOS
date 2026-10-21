#![no_std]
extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Cron manager for scheduled task execution
/// Inspired by Linux cron and systemd timers
#[derive(Debug, Clone)]
pub struct SigmaCronManager2 {
    jobs: Vec<CronJob>,
}

/// A cron job
#[derive(Debug, Clone)]
pub struct CronJob {
    id: String,
    name: String,
    command: String,
    schedule: CronSchedule,
    state: CronManager2JobState,
    user: String,
}

/// Cron schedule
#[derive(Debug, Clone)]
pub struct CronSchedule {
    minute: String,
    hour: String,
    day_of_month: String,
    month: String,
    day_of_week: String,
}

/// Job state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CronManager2JobState {
    Enabled,
    Disabled,
    Running,
    Failed,
}

impl SigmaCronManager2 {
    pub fn new() -> Self {
        Self {
            jobs: Vec::new(),
        }
    }

    /// Add a cron job
    pub fn add_job(&mut self, id: String, name: String, command: String, schedule: CronSchedule, user: String) -> Result<String, String> {
        if self.jobs.iter().any(|j| j.id == id) {
            return Err(format!("Job with id {} already exists", id));
        }
        
        let job = CronJob {
            id: id.clone(),
            name,
            command,
            schedule,
            state: CronManager2JobState::Enabled,
            user,
        };
        self.jobs.push(job);
        Ok(id)
    }

    /// Remove a job
    pub fn remove_job(&mut self, id: &str) -> Result<(), String> {
        let pos = self.jobs.iter().position(|j| j.id == id)
            .ok_or_else(|| format!("Job with id {} not found", id))?;
        self.jobs.remove(pos);
        Ok(())
    }

    /// Enable a job
    pub fn enable_job(&mut self, id: &str) -> Result<(), String> {
        let job = self.jobs.iter_mut().find(|j| j.id == id)
            .ok_or_else(|| format!("Job with id {} not found", id))?;
        job.state = CronManager2JobState::Enabled;
        Ok(())
    }

    /// Disable a job
    pub fn disable_job(&mut self, id: &str) -> Result<(), String> {
        let job = self.jobs.iter_mut().find(|j| j.id == id)
            .ok_or_else(|| format!("Job with id {} not found", id))?;
        job.state = CronManager2JobState::Disabled;
        Ok(())
    }

    /// Mark job as running
    pub fn mark_running(&mut self, id: &str) -> Result<(), String> {
        let job = self.jobs.iter_mut().find(|j| j.id == id)
            .ok_or_else(|| format!("Job with id {} not found", id))?;
        job.state = CronManager2JobState::Running;
        Ok(())
    }

    /// Mark job as failed
    pub fn mark_failed(&mut self, id: &str) -> Result<(), String> {
        let job = self.jobs.iter_mut().find(|j| j.id == id)
            .ok_or_else(|| format!("Job with id {} not found", id))?;
        job.state = CronManager2JobState::Failed;
        Ok(())
    }

    /// Update job schedule
    pub fn update_schedule(&mut self, id: &str, schedule: CronSchedule) -> Result<(), String> {
        let job = self.jobs.iter_mut().find(|j| j.id == id)
            .ok_or_else(|| format!("Job with id {} not found", id))?;
        job.schedule = schedule;
        Ok(())
    }

    /// Get all jobs
    pub fn get_jobs(&self) -> &[CronJob] {
        &self.jobs
    }

    /// Get enabled jobs
    pub fn get_enabled_jobs(&self) -> Vec<&CronJob> {
        self.jobs.iter().filter(|j| j.state == CronManager2JobState::Enabled).collect()
    }

    /// Get job by ID
    pub fn get_job(&self, id: &str) -> Option<&CronJob> {
        self.jobs.iter().find(|j| j.id == id)
    }

    /// Get jobs by user
    pub fn get_jobs_by_user(&self, user: &str) -> Vec<&CronJob> {
        self.jobs.iter().filter(|j| j.user == user).collect()
    }
}

impl Default for SigmaCronManager2 {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_job() {
        let mut manager = SigmaCronManager2::new();
        let schedule = CronSchedule {
            minute: "0".to_string(),
            hour: "2".to_string(),
            day_of_month: "*".to_string(),
            month: "*".to_string(),
            day_of_week: "*".to_string(),
        };
        let result = manager.add_job("job1".to_string(), "Backup".to_string(), "/usr/bin/backup".to_string(), schedule, "root".to_string());
        assert!(result.is_ok());
        assert_eq!(manager.get_jobs().len(), 1);
    }

    #[test]
    fn test_duplicate_job() {
        let mut manager = SigmaCronManager2::new();
        let schedule = CronSchedule {
            minute: "0".to_string(),
            hour: "2".to_string(),
            day_of_month: "*".to_string(),
            month: "*".to_string(),
            day_of_week: "*".to_string(),
        };
        manager.add_job("job1".to_string(), "Backup".to_string(), "/usr/bin/backup".to_string(), schedule.clone(), "root".to_string()).unwrap();
        let result = manager.add_job("job1".to_string(), "Other".to_string(), "/usr/bin/other".to_string(), schedule, "user".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_enable_disable_job() {
        let mut manager = SigmaCronManager2::new();
        let schedule = CronSchedule {
            minute: "0".to_string(),
            hour: "2".to_string(),
            day_of_month: "*".to_string(),
            month: "*".to_string(),
            day_of_week: "*".to_string(),
        };
        manager.add_job("job1".to_string(), "Backup".to_string(), "/usr/bin/backup".to_string(), schedule, "root".to_string()).unwrap();
        
        manager.disable_job("job1").unwrap();
        assert_eq!(manager.get_job("job1").unwrap().state, CronManager2JobState::Disabled);
        
        manager.enable_job("job1").unwrap();
        assert_eq!(manager.get_job("job1").unwrap().state, CronManager2JobState::Enabled);
    }

    #[test]
    fn test_get_jobs_by_user() {
        let mut manager = SigmaCronManager2::new();
        let schedule = CronSchedule {
            minute: "0".to_string(),
            hour: "2".to_string(),
            day_of_month: "*".to_string(),
            month: "*".to_string(),
            day_of_week: "*".to_string(),
        };
        manager.add_job("job1".to_string(), "Backup".to_string(), "/usr/bin/backup".to_string(), schedule.clone(), "root".to_string()).unwrap();
        manager.add_job("job2".to_string(), "Cleanup".to_string(), "/usr/bin/cleanup".to_string(), schedule, "user".to_string()).unwrap();
        
        let user_jobs = manager.get_jobs_by_user("root");
        assert_eq!(user_jobs.len(), 1);
        assert_eq!(user_jobs[0].id, "job1");
    }
}
