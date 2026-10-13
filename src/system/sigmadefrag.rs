//! SigmaDefrag - Disk Defragmentation Tool
//!
//! Inspired by Linux e4defrag, xfs_fsr, and Btrfs defragmentation tools, this module
//! provides comprehensive filesystem defragmentation, extent reorganization, and
//! free space consolidation for SigmaOS.
//!
//! Features include:
//! - Fragmentation analysis and scoring (file, directory, and volume level)
//! - Defragmentation job lifecycle and progress tracking
//! - Extent consolidation and free-space defragmentation
//! - Automated threshold-based defragmentation policies
//! - Global instance and convenience API

#![no_std]

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, Ordering};

/// Defragmentation mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DefragMode {
    File,
    Directory,
    Filesystem,
    FreeSpace,
    Unknown,
}

/// Defragmentation job status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DefragStatus {
    Idle,
    Scanning,
    Defragmenting,
    Paused,
    Completed,
    Failed,
    Aborted,
}

/// Extent and fragmentation analysis report
#[derive(Debug, Clone)]
pub struct FragmentationReport {
    pub target_path: String,
    pub total_files: u64,
    pub fragmented_files: u64,
    pub total_extents: u64,
    pub ideal_extents: u64,
    pub fragmentation_score_pct: u8,
    pub free_space_fragmentation_pct: u8,
}

impl FragmentationReport {
    /// Create a new fragmentation report
    pub fn new(
        target_path: String,
        total_files: u64,
        fragmented_files: u64,
        total_extents: u64,
        ideal_extents: u64,
    ) -> Self {
        let score = if total_extents == 0 || ideal_extents == 0 {
            0
        } else if total_extents <= ideal_extents {
            0
        } else {
            let excess = total_extents - ideal_extents;
            let pct = ((excess as u128 * 100) / total_extents as u128) as u8;
            core::cmp::min(pct, 100)
        };

        Self {
            target_path,
            total_files,
            fragmented_files,
            total_extents,
            ideal_extents,
            fragmentation_score_pct: score,
            free_space_fragmentation_pct: 0,
        }
    }

    /// Check if defragmentation is recommended
    pub fn needs_defragmentation(&self, threshold_pct: u8) -> bool {
        self.fragmentation_score_pct >= threshold_pct
    }
}

/// Defragmentation task / job
#[derive(Debug, Clone)]
pub struct DefragJob {
    pub job_id: String,
    pub target_path: String,
    pub mode: DefragMode,
    pub status: DefragStatus,
    pub files_processed: u64,
    pub extents_merged: u64,
    pub bytes_relocated: u64,
    pub error_count: u32,
    pub start_timestamp: u64,
}

impl DefragJob {
    /// Create a new defragmentation job
    pub fn new(job_id: String, target_path: String, mode: DefragMode, timestamp: u64) -> Self {
        Self {
            job_id,
            target_path,
            mode,
            status: DefragStatus::Idle,
            files_processed: 0,
            extents_merged: 0,
            bytes_relocated: 0,
            error_count: 0,
            start_timestamp: timestamp,
        }
    }

    /// Update job progress
    pub fn update_progress(&mut self, files: u64, extents: u64, bytes: u64) {
        self.files_processed += files;
        self.extents_merged += extents;
        self.bytes_relocated += bytes;
    }

    /// Set status
    pub fn set_status(&mut self, status: DefragStatus) {
        self.status = status;
    }

    /// Check if job is currently active
    pub fn is_active(&self) -> bool {
        matches!(self.status, DefragStatus::Scanning | DefragStatus::Defragmenting)
    }
}

/// Main SigmaDefrag manager struct
pub struct SigmaDefrag {
    jobs: BTreeMap<String, DefragJob>,
    reports: BTreeMap<String, FragmentationReport>,
    running: AtomicBool,
    recommended_threshold_pct: u8,
}

impl SigmaDefrag {
    /// Create a new SigmaDefrag manager instance
    pub fn new() -> Self {
        Self {
            jobs: BTreeMap::new(),
            reports: BTreeMap::new(),
            running: AtomicBool::new(false),
            recommended_threshold_pct: 20,
        }
    }

    /// Start defragmentation service
    pub fn start(&self) {
        self.running.store(true, Ordering::SeqCst);
    }

    /// Stop defragmentation service
    pub fn stop(&self) {
        self.running.store(false, Ordering::SeqCst);
    }

    /// Check if service is running
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }

    /// Set default recommended threshold
    pub fn set_threshold(&mut self, threshold_pct: u8) {
        self.recommended_threshold_pct = threshold_pct;
    }

    /// Store a fragmentation analysis report
    pub fn record_report(&mut self, report: FragmentationReport) {
        self.reports.insert(report.target_path.clone(), report);
    }

    /// Get report for a path
    pub fn get_report(&self, target_path: &str) -> Option<&FragmentationReport> {
        self.reports.get(target_path)
    }

    /// Start a new defragmentation job
    pub fn create_job(&mut self, job: DefragJob) -> bool {
        if self.jobs.contains_key(&job.job_id) {
            false
        } else {
            self.jobs.insert(job.job_id.clone(), job);
            true
        }
    }

    /// Get job by ID
    pub fn get_job(&self, job_id: &str) -> Option<&DefragJob> {
        self.jobs.get(job_id)
    }

    /// Get mutable job by ID
    pub fn get_job_mut(&mut self, job_id: &str) -> Option<&mut DefragJob> {
        self.jobs.get_mut(job_id)
    }

    /// Pause a defrag job
    pub fn pause_job(&mut self, job_id: &str) -> bool {
        if let Some(job) = self.jobs.get_mut(job_id) {
            if job.is_active() {
                job.set_status(DefragStatus::Paused);
                return true;
            }
        }
        false
    }

    /// Resume a defrag job
    pub fn resume_job(&mut self, job_id: &str) -> bool {
        if let Some(job) = self.jobs.get_mut(job_id) {
            if job.status == DefragStatus::Paused {
                job.set_status(DefragStatus::Defragmenting);
                return true;
            }
        }
        false
    }

    /// Abort a defrag job
    pub fn abort_job(&mut self, job_id: &str) -> bool {
        if let Some(job) = self.jobs.get_mut(job_id) {
            job.set_status(DefragStatus::Aborted);
            true
        } else {
            false
        }
    }

    /// List all jobs
    pub fn list_jobs(&self) -> Vec<&DefragJob> {
        self.jobs.values().collect()
    }

    /// List active jobs
    pub fn list_active_jobs(&self) -> Vec<&DefragJob> {
        self.jobs.values().filter(|j| j.is_active()).collect()
    }

    /// Job count
    pub fn job_count(&self) -> usize {
        self.jobs.len()
    }
}

impl Default for SigmaDefrag {
    fn default() -> Self {
        Self::new()
    }
}

/// Global SigmaDefrag instance (placeholder for global state)
static mut SIGMA_DEFRAG_INSTANCE: Option<SigmaDefrag> = None;

/// Initialize global SigmaDefrag instance
pub fn init_sigmadefrag() {
    unsafe {
        SIGMA_DEFRAG_INSTANCE = Some(SigmaDefrag::new());
    }
}

/// Get global SigmaDefrag instance
pub fn get_sigmadefrag() -> Option<&'static SigmaDefrag> {
    unsafe {
        SIGMA_DEFRAG_INSTANCE.as_ref()
    }
}

/// Get mutable global SigmaDefrag instance
pub fn get_sigmadefrag_mut() -> Option<&'static mut SigmaDefrag> {
    unsafe {
        SIGMA_DEFRAG_INSTANCE.as_mut()
    }
}

/// Convenience function to start a defragmentation job
pub fn start_defrag_job(job: DefragJob) -> bool {
    if let Some(mgr) = get_sigmadefrag_mut() {
        mgr.create_job(job)
    } else {
        false
    }
}

/// Convenience function to inspect job status
pub fn get_defrag_status(job_id: &str) -> Option<DefragStatus> {
    if let Some(mgr) = get_sigmadefrag() {
        mgr.get_job(job_id).map(|j| j.status)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fragmentation_report_scoring() {
        let report = FragmentationReport::new(
            "/home/user/largefile.bin".to_string(),
            1,
            1,
            200,
            100,
        );
        assert_eq!(report.fragmentation_score_pct, 50);
        assert!(report.needs_defragmentation(20));
        assert!(!report.needs_defragmentation(60));
    }

    #[test]
    fn test_defrag_job_lifecycle() {
        let mut job = DefragJob::new(
            "job-1".to_string(),
            "/home".to_string(),
            DefragMode::Filesystem,
            1000,
        );
        assert_eq!(job.status, DefragStatus::Idle);
        assert!(!job.is_active());

        job.set_status(DefragStatus::Defragmenting);
        assert!(job.is_active());

        job.update_progress(10, 50, 1024 * 1024);
        assert_eq!(job.files_processed, 10);
        assert_eq!(job.extents_merged, 50);
        assert_eq!(job.bytes_relocated, 1024 * 1024);

        job.set_status(DefragStatus::Completed);
        assert_eq!(job.status, DefragStatus::Completed);
        assert!(!job.is_active());
    }

    #[test]
    fn test_sigmadefrag_manager() {
        let mut mgr = SigmaDefrag::new();
        mgr.start();
        assert!(mgr.is_running());

        let rep = FragmentationReport::new(
            "/var".to_string(),
            500,
            100,
            1000,
            800,
        );
        mgr.record_report(rep);
        assert!(mgr.get_report("/var").is_some());

        let mut job = DefragJob::new(
            "j1".to_string(),
            "/var".to_string(),
            DefragMode::Filesystem,
            100,
        );
        job.set_status(DefragStatus::Defragmenting);
        assert!(mgr.create_job(job));
        assert_eq!(mgr.job_count(), 1);
        assert_eq!(mgr.list_active_jobs().len(), 1);

        assert!(mgr.pause_job("j1"));
        assert_eq!(mgr.get_job("j1").unwrap().status, DefragStatus::Paused);

        assert!(mgr.resume_job("j1"));
        assert_eq!(mgr.get_job("j1").unwrap().status, DefragStatus::Defragmenting);

        assert!(mgr.abort_job("j1"));
        assert_eq!(mgr.get_job("j1").unwrap().status, DefragStatus::Aborted);

        mgr.stop();
        assert!(!mgr.is_running());
    }

    #[test]
    fn test_global_instance() {
        init_sigmadefrag();
        let job = DefragJob::new(
            "global-j".to_string(),
            "/".to_string(),
            DefragMode::Directory,
            50,
        );
        assert!(start_defrag_job(job));
        assert_eq!(get_defrag_status("global-j"), Some(DefragStatus::Idle));
    }
}
