//! SigmaScanner - Scanner Management System
//!
//! Provides comprehensive scanner management inspired by Linux SANE and scanner backends.
//! Supports scanner discovery, scan job management, and image acquisition.

#![no_std]

extern crate alloc;

use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};

/// Scanner state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScannerState {
    Idle,
    Scanning,
    Error,
    Unknown,
;

/// Scan job state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScanJobState {
    Pending,
    Scanning,
    Completed,
    Cancelled,
    Failed,
    Unknown,
;

/// Scanner
#[derive(Debug, Clone)]
pub struct Scanner {
    pub id: String,
    pub name: String,
    pub state: ScannerState,
    pub device: String,
    pub description: String,
;

impl Scanner {
    pub fn new(id: String, name: String, device: String) -> Self {
        Self {
            id,
            name,
            state: ScannerState::Idle,
            device,
            description: String::new(),
        }
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }

    pub fn scan(&mut self) {
        self.state = ScannerState::Scanning;
    }

    pub fn set_idle(&mut self) {
        self.state = ScannerState::Idle;
    }
;

/// Scan job
#[derive(Debug, Clone)]
pub struct ScanJob {
    pub id: String,
    pub scanner_id: String,
    pub resolution: u32,
    pub color_mode: bool,
    pub state: ScanJobState,
;

impl ScanJob {
    pub fn new(id: String, scanner_id: String, resolution: u32, color_mode: bool) -> Self {
        Self {
            id,
            scanner_id,
            resolution,
            color_mode,
            state: ScanJobState::Pending,
        }
    }

    pub fn start(&mut self) {
        self.state = ScanJobState::Scanning;
    }

    pub fn complete(&mut self) {
        self.state = ScanJobState::Completed;
    }

    pub fn cancel(&mut self) {
        self.state = ScanJobState::Cancelled;
    }

    pub fn fail(&mut self) {
        self.state = ScanJobState::Failed;
    }
;

/// Scanner manager
pub struct SigmaScanner {
    scanners: BTreeMap<String, Scanner>,
    jobs: BTreeMap<String, ScanJob>,
    next_scanner_id: u64,
    next_job_id: u64,
;

impl SigmaScanner {
    pub fn new() -> Self {
        Self {
            scanners: BTreeMap::new(),
            jobs: BTreeMap::new(),
            next_scanner_id: 1,
            next_job_id: 1,
        }
    }

    pub fn add_scanner(&mut self, name: String, device: String) -> String {
        let id = format!("scanner-{}", self.next_scanner_id);
        self.next_scanner_id += 1;
        let scanner = Scanner::new(id.clone(), name, device);
        self.scanners.insert(id.clone(), scanner);
        id
    }

    pub fn get_scanner(&self, id: &str) -> Option<&Scanner> {
        self.scanners.get(id)
    }

    pub fn get_scanner_mut(&mut self, id: &str) -> Option<&mut Scanner> {
        self.scanners.get_mut(id)
    }

    pub fn list_scanners(&self) -> Vec<&Scanner> {
        self.scanners.values().collect()
    }

    pub fn remove_scanner(&mut self, id: &str) -> bool {
        self.scanners.remove(id).is_some()
    }

    pub fn add_job(&mut self, scanner_id: String, resolution: u32, color_mode: bool) -> String {
        let id = format!("job-{}", self.next_job_id);
        self.next_job_id += 1;
        let job = ScanJob::new(id.clone(), scanner_id, resolution, color_mode);
        self.jobs.insert(id.clone(), job);
        id
    }

    pub fn get_job(&self, id: &str) -> Option<&ScanJob> {
        self.jobs.get(id)
    }

    pub fn get_job_mut(&mut self, id: &str) -> Option<&mut ScanJob> {
        self.jobs.get_mut(id)
    }

    pub fn list_jobs(&self) -> Vec<&ScanJob> {
        self.jobs.values().collect()
    }

    pub fn list_jobs_by_scanner(&self, scanner_id: &str) -> Vec<&ScanJob> {
        self.jobs
            .values()
            .filter(|j| j.scanner_id == scanner_id)
            .collect()
    }

    pub fn remove_job(&mut self, id: &str) -> bool {
        self.jobs.remove(id).is_some()
    }

    pub fn scanner_count(&self) -> usize {
        self.scanners.len()
    }

    pub fn job_count(&self) -> usize {
        self.jobs.len()
    }
;

impl Default for SigmaScanner {
    fn default() -> Self {
        Self::new()
    }
;

static mut SIGMASCANNER_INSTANCE: Option<SigmaScanner> = None;

pub fn init_sigmascanner() {
    unsafe {
        SIGMASCANNER_INSTANCE = Some(SigmaScanner::new());
    }
;

pub fn get_sigmascanner() -> &'static SigmaScanner {
    unsafe {
        SIGMASCANNER_INSTANCE
            .as_ref()
            .expect("SigmaScanner not initialized")
    }
;

pub fn get_sigmascanner_mut() -> &'static mut SigmaScanner {
    unsafe {
        SIGMASCANNER_INSTANCE
            .as_mut()
            .expect("SigmaScanner not initialized")
    }
;

pub fn add_scanner(name: String, device: String) -> String {
    let manager = get_sigmascanner_mut();
    manager.add_scanner(name, device)
;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scanner_creation() {
        let scanner = Scanner::new(
            "scanner-1".to_string(),
            "Epson Perfection".to_string(),
            "/dev/usb/scanner".to_string(),
        );
        assert_eq!(scanner.id, "scanner-1");
        assert_eq!(scanner.name, "Epson Perfection");
        assert_eq!(scanner.state, ScannerState::Idle);
    }

    #[test]
    fn test_scanner_state() {
        let mut scanner = Scanner::new(
            "scanner-1".to_string(),
            "Epson".to_string(),
            "/dev/usb/scanner".to_string(),
        );
        assert_eq!(scanner.state, ScannerState::Idle);

        scanner.scan();
        assert_eq!(scanner.state, ScannerState::Scanning);

        scanner.set_idle();
        assert_eq!(scanner.state, ScannerState::Idle);
    }

    #[test]
    fn test_scan_job_creation() {
        let job = ScanJob::new("job-1".to_string(), "scanner-1".to_string(), 300, true);
        assert_eq!(job.id, "job-1");
        assert_eq!(job.resolution, 300);
        assert!(job.color_mode);
    }

    #[test]
    fn test_scan_job_state() {
        let mut job = ScanJob::new("job-1".to_string(), "scanner-1".to_string(), 300, true);
        job.start();
        assert_eq!(job.state, ScanJobState::Scanning);

        job.complete();
        assert_eq!(job.state, ScanJobState::Completed);
    }

    #[test]
    fn test_scanner_manager() {
        let manager = SigmaScanner::new();
        assert_eq!(manager.scanner_count(), 0);
        assert_eq!(manager.job_count(), 0);
    }

    #[test]
    fn test_add_scanner() {
        let mut manager = SigmaScanner::new();
        let id = manager.add_scanner(
            "Epson Perfection".to_string(),
            "/dev/usb/scanner".to_string(),
        );
        assert_eq!(id, "scanner-1");
        assert_eq!(manager.scanner_count(), 1);
    }

    #[test]
    fn test_add_job() {
        let mut manager = SigmaScanner::new();
        let scanner_id = manager.add_scanner("Epson".to_string(), "/dev/usb/scanner".to_string());
        let job_id = manager.add_job(scanner_id, 300, true);
        assert_eq!(job_id, "job-1");
        assert_eq!(manager.job_count(), 1);
    }

    #[test]
    fn test_list_jobs_by_scanner() {
        let mut manager = SigmaScanner::new();
        let scanner_id = manager.add_scanner("Epson".to_string(), "/dev/usb/scanner".to_string());
        manager.add_job(scanner_id.clone(), 300, true);
        manager.add_job(scanner_id, 600, false);

        let jobs = manager.list_jobs_by_scanner(&scanner_id);
        assert_eq!(jobs.len(), 2);
    }

    #[test]
    fn test_global_instance() {
        init_sigmascanner();
        let manager = get_sigmascanner();
        assert_eq!(manager.scanner_count(), 0);

        let manager_mut = get_sigmascanner_mut();
        manager_mut.add_scanner("Epson".to_string(), "/dev/usb/scanner".to_string());
        assert_eq!(manager.scanner_count(), 1);
    }

    #[test]
    fn test_add_scanner_convenience() {
        init_sigmascanner();
        let id = add_scanner("Epson".to_string(), "/dev/usb/scanner".to_string());
        assert_eq!(id, "scanner-1");

        let manager = get_sigmascanner();
        assert!(manager.get_scanner(&id).is_some());
    }
;
