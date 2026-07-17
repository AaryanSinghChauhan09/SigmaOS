#![no_std]
#![no_main]

/// OOP-based Crash Reporting Pipeline for SigmaOS
/// Implements crash reporting using OOP principles with traits and structs
/// No dependency on external crash reporting frameworks
/// Based on Roadmap Item 14: Crash reporting pipeline

use core::ptr::{self, NonNull};
use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

/// Report ID
pub type ReportID = usize;

/// Crash severity
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum CrashSeverity {
    Info = 0,
    Warning = 1,
    Error = 2,
    Critical = 3,
    Fatal = 4,
}

/// Crash report trait (OOP interface)
pub trait CrashReport {
    /// Get report ID
    fn id(&self) -> ReportID;
    /// Get application name
    fn application(&self) -> &[u8];
    /// Get crash severity
    fn severity(&self) -> CrashSeverity;
    /// Get crash message
    fn message(&self) -> &[u8];
    /// Get stack trace
    fn stack_trace(&self) -> &[u8];
    /// Get report info
    fn info(&self) -> ReportInfo;
}

/// Report info
#[repr(C)]
pub struct ReportInfo {
    pub id: ReportID,
    pub application: [u8; 64],
    pub severity: CrashSeverity,
    pub timestamp: u64,
    pub capability: ReportCapability,
}

impl ReportInfo {
    pub fn new(id: ReportID) -> Self {
        ReportInfo {
            id,
            application: [0; 64],
            severity: CrashSeverity::Error,
            timestamp: 0,
            capability: ReportCapability::new(),
        }
    }
}

/// Report capability
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ReportCapability {
    pub can_analyze: bool,
    pub can_delete: bool,
}

impl ReportCapability {
    pub fn new() -> Self {
        ReportCapability {
            can_analyze: false,
            can_delete: false,
        }
    }

    pub fn full() -> Self {
        ReportCapability {
            can_analyze: true,
            can_delete: true,
        }
    }
}

/// Simple crash report (OOP: Concrete report class)
#[repr(C)]
pub struct SimpleCrashReport {
    pub id: ReportID,
    pub application: [u8; 64],
    pub severity: CrashSeverity,
    pub message: [u8; 512],
    pub stack_trace: [u8; 1024],
    pub timestamp: u64,
    pub capability: ReportCapability,
}

impl SimpleCrashReport {
    pub fn new(id: ReportID, application: &[u8], severity: CrashSeverity, capability: ReportCapability) -> Self {
        let mut app_array = [0u8; 64];
        let app_len = application.len().min(63);

        unsafe {
            core::ptr::copy_nonoverlapping(application.as_ptr(), app_array.as_mut_ptr(), app_len);
        }

        SimpleCrashReport {
            id,
            application: app_array,
            severity,
            message: [0; 512],
            stack_trace: [0; 1024],
            timestamp: get_current_time(),
            capability,
        }
    }

    pub fn set_message(&mut self, message: &[u8]) {
        let len = message.len().min(511);
        unsafe {
            core::ptr::copy_nonoverlapping(message.as_ptr(), self.message.as_mut_ptr(), len);
        }
    }

    pub fn set_stack_trace(&mut self, trace: &[u8]) {
        let len = trace.len().min(1023);
        unsafe {
            core::ptr::copy_nonoverlapping(trace.as_ptr(), self.stack_trace.as_mut_ptr(), len);
        }
    }
}

impl CrashReport for SimpleCrashReport {
    fn id(&self) -> ReportID {
        self.id
    }

    fn application(&self) -> &[u8] {
        let len = self.application.iter().position(|&b| b == 0).unwrap_or(64);
        &self.application[..len]
    }

    fn severity(&self) -> CrashSeverity {
        self.severity
    }

    fn message(&self) -> &[u8] {
        let len = self.message.iter().position(|&b| b == 0).unwrap_or(512);
        &self.message[..len]
    }

    fn stack_trace(&self) -> &[u8] {
        let len = self.stack_trace.iter().position(|&b| b == 0).unwrap_or(1024);
        &self.stack_trace[..len]
    }

    fn info(&self) -> ReportInfo {
        ReportInfo {
            id: self.id,
            application: self.application,
            severity: self.severity,
            timestamp: self.timestamp,
            capability: self.capability,
        }
    }
}

/// Crash pipeline trait (OOP interface)
pub trait CrashPipeline {
    /// Create report
    fn create_report(&mut self, application: &[u8], severity: CrashSeverity) -> Result<ReportID, CrashError>;
    /// Delete report
    fn delete_report(&mut self, id: ReportID) -> Result<(), CrashError>;
    /// Get report
    fn get_report(&self, id: ReportID) -> Option<&dyn CrashReport>;
    /// List reports by application
    fn list_reports(&self, application: &[u8]) -> Vec<ReportID>;
    /// Get pipeline statistics
    fn stats(&self) -> CrashStats;
}

/// Crash error types
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum CrashError {
    Success = 0,
    ReportNotFound = 1,
    PermissionDenied = 2,
}

/// Crash statistics
#[repr(C)]
pub struct CrashStats {
    pub total_reports: usize,
    pub by_severity: [usize; 5],
}

impl CrashStats {
    pub fn new() -> Self {
        CrashStats {
            total_reports: 0,
            by_severity: [0; 5],
        }
    }
}

/// Simple crash pipeline (OOP: Concrete pipeline class)
pub struct SimpleCrashPipeline {
    reports: Vec<Option<Box<dyn CrashReport>>>,
    next_id: AtomicUsize,
    stats: CrashStats,
    capability: PipelineCapability,
}

/// Pipeline capability
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PipelineCapability {
    pub can_create: bool,
    pub can_delete: bool,
}

impl PipelineCapability {
    pub fn new() -> Self {
        PipelineCapability {
            can_create: false,
            can_delete: false,
        }
    }

    pub fn full() -> Self {
        PipelineCapability {
            can_create: true,
            can_delete: true,
        }
    }
}

impl SimpleCrashPipeline {
    pub fn new(capability: PipelineCapability) -> Self {
        SimpleCrashPipeline {
            reports: Vec::new(),
            next_id: AtomicUsize::new(1),
            stats: CrashStats::new(),
            capability,
        }
    }
}

impl CrashPipeline for SimpleCrashPipeline {
    fn create_report(&mut self, application: &[u8], severity: CrashSeverity) -> Result<ReportID, CrashError> {
        if !self.capability.can_create {
            return Err(CrashError::PermissionDenied);
        }

        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let report = SimpleCrashReport::new(id, application, severity, ReportCapability::full());
        self.reports.push(Some(Box::new(report)));
        self.stats.total_reports += 1;
        self.stats.by_severity[severity as usize] += 1;
        Ok(id)
    }

    fn delete_report(&mut self, id: ReportID) -> Result<(), CrashError> {
        if !self.capability.can_delete {
            return Err(CrashError::PermissionDenied);
        }

        let mut index = None;
        let mut severity = CrashSeverity::Info;

        for (i, report_option) in self.reports.iter().enumerate() {
            if let Some(ref report) = *report_option {
                if report.id() == id {
                    index = Some(i);
                    severity = report.severity();
                    break;
                }
            }
        }

        if let Some(i) = index {
            self.reports[i] = None;
            self.stats.total_reports -= 1;
            self.stats.by_severity[severity as usize] -= 1;
            Ok(())
        } else {
            Err(CrashError::ReportNotFound)
        }
    }

    fn get_report(&self, id: ReportID) -> Option<&dyn CrashReport> {
        for report_option in &self.reports {
            if let Some(ref report) = *report_option {
                if report.id() == id {
                    return Some(report.as_ref());
                }
            }
        }
        None
    }

    fn list_reports(&self, application: &[u8]) -> Vec<ReportID> {
        let mut ids = Vec::new();

        for report_option in &self.reports {
            if let Some(ref report) = *report_option {
                if report.application() == application {
                    ids.push(report.id());
                }
            }
        }

        ids
    }

    fn stats(&self) -> CrashStats {
        self.stats
    }
}

/// Get current time (nanoseconds)
fn get_current_time() -> u64 {
    static mut COUNTER: u64 = 0;
    unsafe {
        COUNTER += 1_000_000;
        COUNTER
    }
}

/// Simple Vec implementation for no_std
struct Vec<T> {
    data: *mut T,
    len: usize,
    capacity: usize,
}

impl<T> Vec<T> {
    fn new() -> Self {
        Vec {
            data: core::ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }

    fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity {
                self.grow();
            }

            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }

    fn len(&self) -> usize {
        self.len
    }

    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 { 4 } else { self.capacity * 2 };
        let new_data = alloc(new_capacity * mem::size_of::<T>()) as *mut T;

        if !new_data.is_null() {
            for i in 0..self.len {
                core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1);
            }

            if self.capacity > 0 {
                free(self.data as *mut u8);
            }

            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}

// External allocator functions
extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}
