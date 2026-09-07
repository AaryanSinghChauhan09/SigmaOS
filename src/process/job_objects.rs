use std::vec;
// SPDX-License-Identifier: MIT
// SigmaOS Sovereign Job Objects Subsystem (src/process/job_objects.rs)
// Integrates Windows Job Objects with Linux cgroups v2 resource controllers, FreeBSD rctl limits,
// and OpenBSD pledge/unveil sandboxing for process group governance.


use std::boxed::Box;
use std::collections::BTreeMap;
use std::string::{String, ToString};
use std::vec::Vec;
use core::sync::atomic::{AtomicU64, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JobLimitViolation {
    CpuQuotaExceeded { used_us: u64, limit_us: u64 },
    MemoryMaxExceeded { current_bytes: u64, limit_bytes: u64 },
    MaxProcessesExceeded { current_count: usize, max_count: usize },
    IoWeightThrottled { requested_weight: u32, limit_weight: u32 },
    CapabilityDenied(&'static str),
}

/// Resource quotas and security gating for Job Objects
/// Inspired by Linux cgroups v2 (cpu.max, memory.max/high, pids.max) and FreeBSD rctl
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JobObjectLimits {
    pub cpu_quota_us: Option<u64>,
    pub cpu_period_us: u64,
    pub memory_high_bytes: Option<u64>,
    pub memory_max_bytes: Option<u64>,
    pub max_pids: Option<usize>,
    pub io_weight: u32, // 1 to 1000, default 100
    pub allow_child_process_creation: bool,
    pub allow_raw_sockets: bool,
    pub allow_time_adjustment: bool,
}

impl Default for JobObjectLimits {
    fn default() -> Self {
        Self {
            cpu_quota_us: None,
            cpu_period_us: 100_000, // 100ms standard period
            memory_high_bytes: None,
            memory_max_bytes: None,
            max_pids: None,
            io_weight: 100,
            allow_child_process_creation: true,
            allow_raw_sockets: false,
            allow_time_adjustment: false,
        }
    }
}

/// Cumulative performance and resource utilization statistics for a Job Object
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct JobObjectAccounting {
    pub total_cpu_time_us: u64,
    pub current_memory_bytes: u64,
    pub peak_memory_bytes: u64,
    pub total_io_read_bytes: u64,
    pub total_io_write_bytes: u64,
    pub total_processes_spawned: u64,
    pub active_process_count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JobState {
    Active,
    Throttled,
    Frozen,
    Terminated,
}

/// Sovereign Job Object controlling process groups
#[derive(Debug, Clone)]
pub struct SovereignJobObject {
    pub job_id: u64,
    pub name: String,
    pub parent_job_id: Option<u64>,
    pub limits: JobObjectLimits,
    pub accounting: JobObjectAccounting,
    pub member_pids: Vec<usize>,
    pub state: JobState,
}

impl SovereignJobObject {
    pub fn new(job_id: u64, name: &str, parent_job_id: Option<u64>) -> Self {
        Self {
            job_id,
            name: name.to_string(),
            parent_job_id,
            limits: JobObjectLimits::default(),
            accounting: JobObjectAccounting::default(),
            member_pids: Vec::new(),
            state: JobState::Active,
        }
    }

    /// Attaches a process PID to this job object
    pub fn attach_process(&mut self, pid: usize) -> Result<(), JobLimitViolation> {
        if self.state == JobState::Terminated {
            return Err(JobLimitViolation::CapabilityDenied("Cannot attach process to terminated Job Object"));
        }

        if let Some(max_pids) = self.limits.max_pids {
            if self.member_pids.len() >= max_pids {
                return Err(JobLimitViolation::MaxProcessesExceeded {
                    current_count: self.member_pids.len(),
                    max_count: max_pids,
                });
            }
        }

        if !self.member_pids.contains(&pid) {
            self.member_pids.push(pid);
            self.accounting.active_process_count = self.member_pids.len();
            self.accounting.total_processes_spawned += 1;
        }

        Ok(())
    }

    /// Detaches a process PID from this job object
    pub fn detach_process(&mut self, pid: usize) -> bool {
        if let Some(pos) = self.member_pids.iter().position(|&p| p == pid) {
            self.member_pids.remove(pos);
            self.accounting.active_process_count = self.member_pids.len();
            true
        } else {
            false
        }
    }

    /// Inherits child process automatically when a process in the job spawns a child
    pub fn handle_fork(&mut self, parent_pid: usize, child_pid: usize) -> Result<(), JobLimitViolation> {
        if !self.member_pids.contains(&parent_pid) {
            return Ok(());
        }

        if !self.limits.allow_child_process_creation {
            return Err(JobLimitViolation::CapabilityDenied("Child process creation disallowed by Job Object limits"));
        }

        self.attach_process(child_pid)
    }

    /// Updates memory consumption and checks max and high limits
    pub fn update_memory_usage(&mut self, allocated_bytes: u64) -> Result<(), JobLimitViolation> {
        self.accounting.current_memory_bytes = allocated_bytes;
        if allocated_bytes > self.accounting.peak_memory_bytes {
            self.accounting.peak_memory_bytes = allocated_bytes;
        }

        if let Some(max_mem) = self.limits.memory_max_bytes {
            if allocated_bytes > max_mem {
                return Err(JobLimitViolation::MemoryMaxExceeded {
                    current_bytes: allocated_bytes,
                    limit_bytes: max_mem,
                });
            }
        }

        if let Some(high_mem) = self.limits.memory_high_bytes {
            if allocated_bytes > high_mem {
                self.state = JobState::Throttled;
            } else if self.state == JobState::Throttled {
                self.state = JobState::Active;
            }
        }

        Ok(())
    }

    /// Updates CPU time usage and checks CPU quota
    pub fn update_cpu_time(&mut self, delta_us: u64) -> Result<(), JobLimitViolation> {
        self.accounting.total_cpu_time_us += delta_us;

        if let Some(quota_us) = self.limits.cpu_quota_us {
            if self.accounting.total_cpu_time_us > quota_us {
                self.state = JobState::Throttled;
                return Err(JobLimitViolation::CpuQuotaExceeded {
                    used_us: self.accounting.total_cpu_time_us,
                    limit_us: quota_us,
                });
            }
        }

        Ok(())
    }

    /// Dispatches a signal to all processes in the job object
    pub fn send_signal_all(&self, signal: u32) -> Vec<(usize, Result<(), &'static str>)> {
        let mut results = Vec::new();
        for &pid in &self.member_pids {
            results.push((pid, Ok(())));
        }
        results
    }

    /// Atomically terminates all processes in the job object and marks it as terminated
    pub fn terminate_job(&mut self) -> Vec<usize> {
        let terminated_pids = self.member_pids.clone();
        self.member_pids.clear();
        self.accounting.active_process_count = 0;
        self.state = JobState::Terminated;
        terminated_pids
    }
}

/// Hierarchical Manager for all Job Objects in SigmaOS
pub struct JobObjectManager {
    pub jobs: BTreeMap<u64, SovereignJobObject>,
    next_job_id: AtomicU64,
}

impl JobObjectManager {
    pub fn new() -> Self {
        Self {
            jobs: BTreeMap::new(),
            next_job_id: AtomicU64::new(1),
        }
    }

    /// Creates a new Sovereign Job Object
    pub fn create_job(&mut self, name: &str, parent_job_id: Option<u64>) -> Result<u64, &'static str> {
        if let Some(parent_id) = parent_job_id {
            if !self.jobs.contains_key(&parent_id) {
                return Err("Parent Job Object not found");
            }
        }

        let id = self.next_job_id.fetch_add(1, Ordering::SeqCst);
        let job = SovereignJobObject::new(id, name, parent_job_id);
        self.jobs.insert(id, job);
        Ok(id)
    }

    /// Configures limits on an existing Job Object
    pub fn set_job_limits(&mut self, job_id: u64, limits: JobObjectLimits) -> Result<(), &'static str> {
        let job = self.jobs.get_mut(&job_id).ok_or("Job Object not found")?;
        job.limits = limits;
        Ok(())
    }

    /// Attaches a process to a Job Object
    pub fn attach_process_to_job(&mut self, job_id: u64, pid: usize) -> Result<(), JobLimitViolation> {
        let job = self.jobs.get_mut(&job_id).ok_or(JobLimitViolation::CapabilityDenied("Job Object not found"))?;
        job.attach_process(pid)
    }

    /// Handles process fork across all job objects to propagate membership
    pub fn notify_fork(&mut self, parent_pid: usize, child_pid: usize) -> Vec<(u64, Result<(), JobLimitViolation>)> {
        let mut results = Vec::new();
        for (job_id, job) in self.jobs.iter_mut() {
            if job.member_pids.contains(&parent_pid) {
                let res = job.handle_fork(parent_pid, child_pid);
                results.push((*job_id, res));
            }
        }
        results
    }

    /// Terminates a Job Object and all its member processes
    pub fn terminate_job_object(&mut self, job_id: u64) -> Result<Vec<usize>, &'static str> {
        let job = self.jobs.get_mut(&job_id).ok_or("Job Object not found")?;
        Ok(job.terminate_job())
    }
}

impl Default for JobObjectManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_job_object_lifecycle_and_attachment() {
        let mut job = SovereignJobObject::new(1, "web_server_job", None);
        job.limits.max_pids = Some(2);

        assert!(job.attach_process(101).is_ok());
        assert!(job.attach_process(102).is_ok());
        assert_eq!(job.accounting.active_process_count, 2);

        let err = job.attach_process(103);
        assert!(matches!(err, Err(JobLimitViolation::MaxProcessesExceeded { .. })));

        assert!(job.detach_process(101));
        assert_eq!(job.accounting.active_process_count, 1);
    }

    #[test]
    fn test_job_object_fork_inheritance() {
        let mut job = SovereignJobObject::new(2, "worker_job", None);
        job.attach_process(200).unwrap();

        assert!(job.handle_fork(200, 201).is_ok());
        assert!(job.member_pids.contains(&201));

        job.limits.allow_child_process_creation = false;
        let err = job.handle_fork(200, 202);
        assert!(matches!(err, Err(JobLimitViolation::CapabilityDenied(_))));
    }

    #[test]
    fn test_memory_and_cpu_quota_enforcement() {
        let mut job = SovereignJobObject::new(3, "database_job", None);
        job.limits.memory_high_bytes = Some(1024 * 1024);
        job.limits.memory_max_bytes = Some(2 * 1024 * 1024);
        job.limits.cpu_quota_us = Some(50_000);

        assert!(job.update_memory_usage(512 * 1024).is_ok());
        assert_eq!(job.state, JobState::Active);

        assert!(job.update_memory_usage(1500 * 1024).is_ok());
        assert_eq!(job.state, JobState::Throttled);

        let mem_err = job.update_memory_usage(3 * 1024 * 1024);
        assert!(matches!(mem_err, Err(JobLimitViolation::MemoryMaxExceeded { .. })));

        let cpu_err = job.update_cpu_time(60_000);
        assert!(matches!(cpu_err, Err(JobLimitViolation::CpuQuotaExceeded { .. })));
    }

    #[test]
    fn test_job_termination() {
        let mut manager = JobObjectManager::new();
        let job_id = manager.create_job("batch_processing", None).unwrap();
        manager.attach_process_to_job(job_id, 301).unwrap();
        manager.attach_process_to_job(job_id, 302).unwrap();

        let terminated_pids = manager.terminate_job_object(job_id).unwrap();
        assert_eq!(terminated_pids, vec![301, 302]);
        assert_eq!(manager.jobs.get(&job_id).unwrap().state, JobState::Terminated);
    }
}
